//
// diff_view_test.rs
// Differential tests proving the borrowing parse path (RecordView) decodes
// every record identically to the owned path, and that the streaming readers
// agree with get_record_iter.
//
// By default the tests run against the bundled demo files. Point the first two
// at any other STDF datalog with DIFF_FILE:
//
//     DIFF_FILE=path/to/file.stdf cargo test --test diff_view_test -- --nocapture
//

use rust_stdf::{stdf_file::StdfReader, RawDataElement, StdfRecord};
use std::path::PathBuf;

fn target_files() -> Vec<PathBuf> {
    if let Ok(p) = std::env::var("DIFF_FILE") {
        return vec![PathBuf::from(p)];
    }
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("demo_stdf");
    ["lot2.stdf", "lot3.stdf"]
        .iter()
        .map(|f| base.join(f))
        .collect()
}

// `view().into_owned()` must reproduce `read_from_bytes` field-for-field.
#[test]
fn view_into_owned_matches_read_from_bytes() {
    for path in target_files() {
        let p = path.display().to_string();
        let mut reader = StdfReader::new(&path).unwrap_or_else(|e| panic!("cannot open {p}: {e}"));

        let mut count: u64 = 0;
        for raw in reader.get_rawdata_iter() {
            let raw = raw.expect("read error while iterating raw records");

            let owned: StdfRecord = (&raw).into();
            let via_view: StdfRecord = raw.view().into_owned();

            // NaN-tolerant compare: derived PartialEq treats NaN != NaN, so
            // byte-identical float fields would spuriously fail. Debug
            // formatting renders NaN identically, sidestepping that.
            assert_eq!(
                format!("{owned:?}"),
                format!("{via_view:?}"),
                "{p}: record #{count} differs (read_from_bytes vs view().into_owned())"
            );
            count += 1;
        }
        assert!(count > 0, "no records read from {p}");
        eprintln!("{p}: {count} records matched (view vs read_from_bytes)");
    }
}

// The new streaming API must yield the same records as the public
// `get_record_iter`, so adding it broke nothing for existing callers.
#[test]
fn read_record_matches_record_iter() {
    for path in target_files() {
        let p = path.display().to_string();

        // legacy public path
        let mut r1 = StdfReader::new(&path).unwrap_or_else(|e| panic!("cannot open {p}: {e}"));
        let legacy: Vec<String> = r1
            .get_record_iter()
            .map(|r| format!("{:?}", r.expect("read error")))
            .collect();

        // buffer-reusing borrowing path
        let mut r2 = StdfReader::new(&path).unwrap_or_else(|e| panic!("cannot open {p}: {e}"));
        let mut raw = RawDataElement::default();
        let mut streamed: Vec<String> = Vec::with_capacity(legacy.len());
        while r2
            .read_record(&mut raw)
            .expect("read error while streaming records")
        {
            streamed.push(format!("{:?}", raw.view().into_owned()));
        }

        assert!(!legacy.is_empty(), "no records read from {p}");
        assert_eq!(
            legacy.len(),
            streamed.len(),
            "{p}: record count differs between get_record_iter and read_record"
        );
        for (i, (a, b)) in legacy.iter().zip(&streamed).enumerate() {
            assert_eq!(
                a, b,
                "{p}: record #{i} differs (get_record_iter vs read_record)"
            );
        }
        eprintln!(
            "{p}: {} records matched (get_record_iter vs read_record)",
            legacy.len()
        );
    }
}

// read_view must yield the same records as get_record_iter, over both the
// borrow fast-path (uncompressed) and the copy fallback (compressed).
#[test]
fn read_view_matches_record_iter() {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("demo_stdf");
    for name in ["lot2.stdf", "lot2.stdf.gz"] {
        let path = base.join(name);
        let p = path.display().to_string();

        let mut r1 = StdfReader::new(&path).unwrap_or_else(|e| panic!("cannot open {p}: {e}"));
        let legacy: Vec<String> = r1
            .get_record_iter()
            .map(|r| format!("{:?}", r.expect("read error")))
            .collect();

        let mut r2 = StdfReader::new(&path).unwrap_or_else(|e| panic!("cannot open {p}: {e}"));
        let mut viewed: Vec<String> = Vec::with_capacity(legacy.len());
        while let Some(res) = r2.read_view() {
            let view = res.expect("read error while streaming views");
            viewed.push(format!("{:?}", view.into_owned()));
        }

        assert!(!legacy.is_empty(), "no records read from {p}");
        assert_eq!(legacy.len(), viewed.len(), "{p}: record count differs");
        for (i, (a, b)) in legacy.iter().zip(&viewed).enumerate() {
            assert_eq!(
                a, b,
                "{p}: record #{i} differs (read_view vs get_record_iter)"
            );
        }
        eprintln!(
            "{p}: {} records matched (read_view vs get_record_iter)",
            legacy.len()
        );
    }
}

// A tiny BufReader forces records across the buffer boundary, deterministically
// exercising read_view's copy fallback on the uncompressed path.
#[test]
fn read_view_straddle_fallback() {
    use rust_stdf::CompressType;
    use std::fs::File;
    use std::io::BufReader;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("demo_stdf");
    path.push("lot2.stdf");

    let mut r1 = StdfReader::new(&path).unwrap();
    let legacy: Vec<String> = r1
        .get_record_iter()
        .map(|r| format!("{:?}", r.expect("read error")))
        .collect();

    let br = BufReader::with_capacity(16, File::open(&path).unwrap());
    let mut r2 = StdfReader::from(br, &CompressType::Uncompressed).unwrap();
    let mut viewed: Vec<String> = Vec::with_capacity(legacy.len());
    while let Some(res) = r2.read_view() {
        viewed.push(format!("{:?}", res.expect("read error").into_owned()));
    }

    assert_eq!(legacy.len(), viewed.len(), "straddle: record count differs");
    for (i, (a, b)) in legacy.iter().zip(&viewed).enumerate() {
        assert_eq!(a, b, "straddle: record #{i} differs");
    }
    eprintln!("straddle fallback: {} records matched", legacy.len());
}

// Switching from read_view to the owned iterator mid-stream must continue
// correctly, i.e. the deferred consume is settled by the next read_header.
#[test]
fn read_view_interleaves_with_owned_reader() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("demo_stdf");
    path.push("lot2.stdf");

    let mut r1 = StdfReader::new(&path).unwrap();
    let full: Vec<String> = r1
        .get_record_iter()
        .map(|r| format!("{:?}", r.expect("read error")))
        .collect();

    let mut r2 = StdfReader::new(&path).unwrap();
    let mut mixed: Vec<String> = Vec::with_capacity(full.len());
    // first 100 records via read_view (leaves a deferred consume pending)
    for _ in 0..100 {
        match r2.read_view() {
            Some(res) => mixed.push(format!("{:?}", res.expect("read error").into_owned())),
            None => break,
        }
    }
    // the rest via the owned iterator
    for rec in r2.get_record_iter() {
        mixed.push(format!("{:?}", rec.expect("read error")));
    }

    assert_eq!(full.len(), mixed.len(), "interleave: record count differs");
    for (i, (a, b)) in full.iter().zip(&mixed).enumerate() {
        assert_eq!(a, b, "interleave: record #{i} differs");
    }
    eprintln!("interleave: {} records matched", full.len());
}
