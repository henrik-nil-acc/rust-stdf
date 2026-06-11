# rust-stdf

[Documentation](https://docs.rs/rust-stdf/)
 
A Rust STDF library for process STDF datalogs of Version V4 and V4-2007.

```
# Cargo.toml
[dependencies]
rust-stdf = "0.3.1"
```
## Features

Available features are listed below:
 - `gzip`: gzip compression (.gz) support powered by `flate2`
 - `bzip`: bzip compression (.bz2) support powered by `bzip2`
 - `zipfile`: zip compression (.zip) support powered by `zip`
 - `atdf`: ATDF reader + STDF -> ATDF convertor (in dev)
 - `serialize`: serialize STDF records by `serde`

***Note***: *`zipfile` feature contains unsafe Rust code, and STDF Reader will only open the first file in the zip archive with no password.*

`rust-stdf` enable `gzip` and `bzip` by default, you can also control features by yourself.

```
rust-stdf = { version="0.3.1", default-features = false, features = ["gzip", ...]}
```

---

## Example

Here is a simple example to show you how to iterate records in a STDF V4 file. There is a rather complex example in the [github repo](https://github.com/noonchen/rust-stdf/tree/main/example) shows how to use existing APIs to convert STDF to Excel xlsx file.

```rust
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};

fn main() {
    let stdf_path = "demo_file.stdf";   // "demo_file.stdf.gz" "demo_file.stdf.bz2"
    let mut reader = match StdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // we will count total DUT# in the file
    // and put test result of PTR named
    // "continuity test" in a vector.
    let mut dut_count: u64 = 0;
    let mut continuity_rlt = vec![];

    // use type filter to work on certain types,
    // use `|` to combine multiple typs
    let rec_types = REC_PIR | REC_PTR;
    // iterator starts from current file position,
    // if file hits EOF, it will NOT redirect to 0.
    for rec in reader
        .get_record_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.is_type(rec_types))
    {
        match rec {
            StdfRecord::PIR(_) => {dut_count += 1;}
            StdfRecord::PTR(ref ptr_rec) => {
                if ptr_rec.test_txt == "continuity test" {
                    continuity_rlt.push(ptr_rec.result);
                }
            }
            _ => {}
        }
    }
    println!("Total duts {} \n continuity result {:?}",
            dut_count,
            continuity_rlt);
}
```

## Borrowed record views

`get_record_iter` yields owned `StdfRecord`s, allocating a `String` for every
text field. `read_view` instead borrows each record straight from the reader's
buffer as a `RecordView`, with scalar text fields exposed as `Cow<str>` that
borrow in place for ASCII. The common path does no per-record copy and no
string allocation, though array fields still allocate their vectors. The view
is valid until the next read. Call `into_owned()` on the records you keep.

```rust
use rust_stdf::{stdf_file::*, RecordView};

let mut reader = StdfReader::new("demo_file.stdf").unwrap();
while let Some(rec) = reader.read_view() {
    if let RecordView::PTR(ptr) = rec.unwrap() {
        // `ptr.test_txt` borrows from the reader buffer, no string allocation
        println!("{} = {}", ptr.test_txt, ptr.result);
    }
}
```

To own the raw bytes (e.g. to keep `offset`/`header`), use
`read_record(&mut RawDataElement)` then `raw.view()`.
