//
// view_record_tests.rs
// Layout-pinning tests for record types that the bundled demo files do not
// contain, so the hand-written view walkers (the MPR/PTR optional block, the
// STR/FTR arrays, the conditional scalar defaults) have automated coverage.
//
// Each test builds a record's field bytes by hand and asserts the decoded
// `RecordView`. A test fails if a field's order, size, or default drifts from
// what the byte layout encodes.
//

use rust_stdf::{ByteOrder, KxUf, RawDataElement, RecordHeader, RecordView, StdfRecord};

/// Little-endian field-byte builder, one method per STDF data type.
#[derive(Default)]
struct Buf(Vec<u8>);

impl Buf {
    fn u1(mut self, v: u8) -> Self {
        self.0.push(v);
        self
    }
    fn i1(mut self, v: i8) -> Self {
        self.0.push(v as u8);
        self
    }
    fn u2(mut self, v: u16) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u4(mut self, v: u32) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u8v(mut self, v: u64) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn i2(mut self, v: i16) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn i4(mut self, v: i32) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn r4(mut self, v: f32) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    /// Cn: 1-byte length prefix then the bytes.
    fn cn(mut self, s: &str) -> Self {
        self.0.push(s.len() as u8);
        self.0.extend_from_slice(s.as_bytes());
        self
    }
    /// Dn: 2-byte bit count then ceil(bits/8) data bytes.
    fn dn(mut self, bits: u16, bytes: &[u8]) -> Self {
        self.0.extend_from_slice(&bits.to_le_bytes());
        self.0.extend_from_slice(bytes);
        self
    }
    /// Sn: 2-byte length prefix then the bytes.
    fn sn(mut self, s: &str) -> Self {
        self.0.extend_from_slice(&(s.len() as u16).to_le_bytes());
        self.0.extend_from_slice(s.as_bytes());
        self
    }
    /// Bn: 1-byte length prefix then the bytes.
    fn bn(mut self, bytes: &[u8]) -> Self {
        self.0.push(bytes.len() as u8);
        self.0.extend_from_slice(bytes);
        self
    }
}

fn element(typ: u8, sub: u8, data: Buf) -> RawDataElement {
    let raw_data = data.0;
    RawDataElement {
        offset: 0,
        header: RecordHeader {
            typ,
            sub,
            len: raw_data.len() as u16,
        },
        raw_data,
        byte_order: ByteOrder::LittleEndian,
    }
}

#[test]
fn mpr_all_optional_fields_present() {
    let data = Buf::default()
        .u4(1000) // test_num
        .u1(1) // head_num
        .u1(2) // site_num
        .u1(0) // test_flg
        .u1(0) // parm_flg
        .u2(2) // rtn_icnt
        .u2(2) // rslt_cnt
        .u1(0x21) // rtn_stat nibbles -> [1, 2]
        .r4(1.5) // rtn_rslt[0]
        .r4(2.5) // rtn_rslt[1]
        .cn("Tst") // test_txt
        .cn("") // alarm_id
        .u1(0) // opt_flag
        .i1(-1) // res_scal
        .i1(-2) // llm_scal
        .i1(-3) // hlm_scal
        .r4(0.0) // lo_limit
        .r4(10.0) // hi_limit
        .r4(0.0) // start_in
        .r4(1.0) // incr_in
        .u2(10) // rtn_indx[0]
        .u2(20) // rtn_indx[1]
        .cn("V") // units
        .cn("s") // units_in
        .cn("%f") // c_resfmt
        .cn("") // c_llmfmt
        .cn("") // c_hlmfmt
        .r4(-1.0) // lo_spec
        .r4(11.0); // hi_spec
    let rde = element(15, 15, data);
    let RecordView::MPR(m) = rde.view() else {
        panic!("expected MPR view");
    };
    assert_eq!(m.test_num, 1000);
    assert_eq!((m.head_num, m.site_num), (1, 2));
    assert_eq!((m.rtn_icnt, m.rslt_cnt), (2, 2));
    assert_eq!(m.rtn_stat, vec![1, 2]);
    assert_eq!(m.rtn_rslt, vec![1.5, 2.5]);
    assert_eq!(&*m.test_txt, "Tst");
    assert_eq!(&*m.alarm_id, "");
    assert_eq!(m.opt_flag, Some([0]));
    assert_eq!(
        (m.res_scal, m.llm_scal, m.hlm_scal),
        (Some(-1), Some(-2), Some(-3))
    );
    assert_eq!((m.lo_limit, m.hi_limit), (Some(0.0), Some(10.0)));
    assert_eq!((m.start_in, m.incr_in), (Some(0.0), Some(1.0)));
    assert_eq!(m.rtn_indx, Some(vec![10, 20]));
    assert_eq!(m.units.as_deref(), Some("V"));
    assert_eq!(m.units_in.as_deref(), Some("s"));
    assert_eq!(m.c_resfmt.as_deref(), Some("%f"));
    assert_eq!(m.c_llmfmt.as_deref(), Some(""));
    assert_eq!(m.c_hlmfmt.as_deref(), Some(""));
    assert_eq!((m.lo_spec, m.hi_spec), (Some(-1.0), Some(11.0)));
}

#[test]
fn mpr_truncated_optionals_default_to_none() {
    // Stop right after the mandatory fields: the optional block's first
    // length check fails, so every optional stays None.
    let data = Buf::default()
        .u4(7)
        .u1(1)
        .u1(1)
        .u1(0)
        .u1(0)
        .u2(1) // rtn_icnt
        .u2(1) // rslt_cnt
        .u1(0x03) // rtn_stat -> [3]
        .r4(9.0) // rtn_rslt[0]
        .cn("x") // test_txt
        .cn(""); // alarm_id
    let rde = element(15, 15, data);
    let RecordView::MPR(m) = rde.view() else {
        panic!("expected MPR view");
    };
    assert_eq!(m.rtn_stat, vec![3]);
    assert_eq!(m.rtn_rslt, vec![9.0]);
    assert_eq!(m.opt_flag, None);
    assert_eq!(m.res_scal, None);
    assert_eq!(m.lo_limit, None);
    assert_eq!(m.rtn_indx, None);
    assert_eq!(m.units, None);
    assert_eq!(m.hi_spec, None);
}

#[test]
fn ftr_arrays_and_bitfields() {
    let data = Buf::default()
        .u4(5) // test_num
        .u1(1) // head_num
        .u1(1) // site_num
        .u1(0) // test_flg
        .u1(0) // opt_flag
        .u4(100) // cycl_cnt
        .u4(0) // rel_vadr
        .u4(0) // rept_cnt
        .u4(3) // num_fail
        .i4(-1) // xfail_ad
        .i4(-2) // yfail_ad
        .i2(-5) // vect_off
        .u2(2) // rtn_icnt
        .u2(0) // pgm_icnt
        .u2(7) // rtn_indx[0]
        .u2(8) // rtn_indx[1]
        .u1(0x21) // rtn_stat nibbles -> [1, 2]
        .dn(4, &[0x0A]) // fail_pin: 4 bits -> 1 byte
        .cn("VEC") // vect_nam
        .cn("") // time_set
        .cn("") // op_code
        .cn("FT") // test_txt
        .cn("") // alarm_id
        .cn("") // prog_txt
        .cn("") // rslt_txt
        .u1(7) // patg_num
        .dn(0, &[]); // spin_map: 0 bits
    let rde = element(15, 20, data);
    let RecordView::FTR(f) = rde.view() else {
        panic!("expected FTR view");
    };
    assert_eq!(f.test_num, 5);
    assert_eq!(f.num_fail, 3);
    assert_eq!((f.xfail_ad, f.yfail_ad, f.vect_off), (-1, -2, -5));
    assert_eq!(f.rtn_indx, vec![7, 8]);
    assert_eq!(f.rtn_stat, vec![1, 2]);
    assert_eq!(f.fail_pin, vec![0x0A]);
    assert_eq!(&*f.vect_nam, "VEC");
    assert_eq!(&*f.test_txt, "FT");
    assert_eq!(f.patg_num, 7);
    assert!(f.spin_map.is_empty());
}

#[test]
fn ftr_missing_patg_num_keeps_default() {
    // Stop before patg_num: it must keep its 255 default and spin_map empty.
    let data = Buf::default()
        .u4(5)
        .u1(1)
        .u1(1)
        .u1(0)
        .u1(0)
        .u4(0)
        .u4(0)
        .u4(0)
        .u4(0)
        .i4(0)
        .i4(0)
        .i2(0)
        .u2(0) // rtn_icnt
        .u2(0) // pgm_icnt
        .dn(0, &[]) // fail_pin
        .cn("") // vect_nam
        .cn("") // time_set
        .cn("") // op_code
        .cn("") // test_txt
        .cn("") // alarm_id
        .cn("") // prog_txt
        .cn(""); // rslt_txt (stop here)
    let rde = element(15, 20, data);
    let RecordView::FTR(f) = rde.view() else {
        panic!("expected FTR view");
    };
    assert_eq!(f.patg_num, 255);
    assert!(f.spin_map.is_empty());
}

#[test]
fn pmr_conditional_defaults() {
    // chan_typ present, head_num / site_num present.
    let present = element(
        1,
        60,
        Buf::default()
            .u2(100) // pmr_indx
            .u2(5) // chan_typ
            .cn("CH") // chan_nam
            .cn("PH") // phy_nam
            .cn("LG") // log_nam
            .u1(3) // head_num
            .u1(4), // site_num
    );
    let RecordView::PMR(p) = present.view() else {
        panic!("expected PMR view");
    };
    assert_eq!(p.pmr_indx, 100);
    assert_eq!(p.chan_typ, 5);
    assert_eq!(&*p.chan_nam, "CH");
    assert_eq!((p.head_num, p.site_num), (3, 4));

    // only pmr_indx: chan_typ falls back to 0, head_num / site_num to 1.
    let bare = element(1, 60, Buf::default().u2(7));
    let RecordView::PMR(p) = bare.view() else {
        panic!("expected PMR view");
    };
    assert_eq!(p.pmr_indx, 7);
    assert_eq!(p.chan_typ, 0);
    assert_eq!((p.head_num, p.site_num), (1, 1));
}

#[test]
fn plr_string_arrays() {
    let data = Buf::default()
        .u2(2) // grp_cnt
        .u2(1) // grp_indx[0]
        .u2(2) // grp_indx[1]
        .u2(0) // grp_mode[0]
        .u2(0) // grp_mode[1]
        .u1(10) // grp_radx[0]
        .u1(16) // grp_radx[1]
        .cn("A") // pgm_char[0]
        .cn("B") // pgm_char[1]
        .cn("") // rtn_char[0]
        .cn("") // rtn_char[1]
        .cn("") // pgm_chal[0]
        .cn("") // pgm_chal[1]
        .cn("") // rtn_chal[0]
        .cn(""); // rtn_chal[1]
    let rde = element(1, 63, data);
    let RecordView::PLR(p) = rde.view() else {
        panic!("expected PLR view");
    };
    assert_eq!(p.grp_cnt, 2);
    assert_eq!(p.grp_indx, vec![1, 2]);
    assert_eq!(p.grp_radx, vec![10, 16]);
    assert_eq!(p.pgm_char, vec!["A".to_string(), "B".to_string()]);
}

#[test]
fn str_scalar_walk_and_kxuf_array() {
    let data = Buf::default()
        .u1(0) // cont_flg
        .u4(9) // test_num
        .u1(1) // head_num
        .u1(1) // site_num
        .u2(0) // psr_ref
        .u1(0) // test_flg
        .cn("") // log_typ
        .cn("S") // test_txt
        .cn("") // alarm_id
        .cn("") // prog_txt
        .cn("") // rslt_txt
        .u1(0) // z_val
        .u1(0) // fmu_flg
        .dn(0, &[]) // mask_map
        .dn(0, &[]) // fal_map
        .u8v(0) // cyc_cnt_t
        .u4(0) // totf_cnt
        .u4(0) // totl_cnt
        .u8v(0) // cyc_base
        .u4(0) // bit_base
        .u2(1) // cond_cnt
        .u2(0) // lim_cnt
        .u1(4) // cyc_size -> KxUf::F4
        .u1(1) // pmr_size
        .u1(1) // chn_size
        .u1(1) // pat_size
        .u1(1) // bit_size
        .u1(1) // u1_size
        .u1(1) // u2_size
        .u1(1) // u3_size
        .u1(0) // utx_size
        .u2(0) // cap_bgn
        // lim_indx / lim_spec are empty (lim_cnt = 0)
        .cn("X=1") // cond_lst[0]
        .u2(1) // cyc_cnt
        .u4(42) // cyc_ofst[0] as u32 (cyc_size = 4)
        .u2(0) // pmr_cnt
        .u2(0) // chn_cnt
        .u2(0) // exp_cnt
        .u2(0) // cap_cnt
        .u2(0) // new_cnt
        .u2(0) // pat_cnt
        .u2(0) // bpos_cnt
        .u2(0) // usr1_cnt
        .u2(0) // usr2_cnt
        .u2(0) // usr3_cnt
        .u2(0); // txt_cnt
    let rde = element(15, 30, data);
    let RecordView::STR(s) = rde.view() else {
        panic!("expected STR view");
    };
    assert_eq!(s.test_num, 9);
    assert_eq!(&*s.test_txt, "S");
    assert_eq!(s.cond_cnt, 1);
    assert_eq!(s.cond_lst, vec!["X=1".to_string()]);
    assert_eq!(s.cyc_size, 4);
    assert_eq!(s.cyc_cnt, 1);
    assert_eq!(s.cyc_ofst, KxUf::F4(vec![42]));
}

/// Assert the owned and borrowing walkers decode `data` to the same record.
fn view_eq_owned(name: &str, typ: u8, sub: u8, data: Buf) {
    let rde = element(typ, sub, data);
    let owned: StdfRecord = (&rde).into();
    let viewed: StdfRecord = rde.view().into_owned();
    assert_eq!(
        format!("{owned:?}"),
        format!("{viewed:?}"),
        "{name}: view().into_owned() differs from read_from_bytes"
    );
}

// One generated test per record type: build a representative record by hand and
// assert the borrowing walker decodes it identically to the owned walker. This
// covers every view walker without depending on the demo files (which are
// excluded from the published package).
macro_rules! view_eq_owned_tests {
    ($($name:ident => ($typ:expr, $sub:expr, $buf:expr),)+) => {
        $(
            #[test]
            fn $name() {
                view_eq_owned(stringify!($name), $typ, $sub, $buf);
            }
        )+
    };
}

view_eq_owned_tests! {
    far => (0, 10, Buf::default().u1(1).u1(4)),
    atr => (0, 20, Buf::default().u4(123).cn("prog --flag")),
    vur => (0, 30, Buf::default().cn("V4-2007")),
    mir => (1, 10, Buf::default().u4(1).u4(2).u1(1).u1(b' ').u1(b' ').u1(b' ').u2(0)
        .u1(b' ').cn("LOT").cn("PART").cn("NODE").cn("TESTER").cn("JOB").cn("REV")),
    mrr => (1, 20, Buf::default().u4(9).u1(b'P').cn("user desc").cn("exec desc")),
    pcr => (1, 30, Buf::default().u1(1).u1(1).u4(100).u4(1).u4(2).u4(3).u4(4)),
    hbr => (1, 40, Buf::default().u1(1).u1(1).u2(5).u4(50).u1(b'P').cn("HBIN5")),
    sbr => (1, 50, Buf::default().u1(1).u1(1).u2(7).u4(70).u1(b'F').cn("SBIN7")),
    pmr => (1, 60, Buf::default().u2(100).u2(5).cn("CH").cn("PHY").cn("LOG").u1(3).u1(4)),
    pgr => (1, 62, Buf::default().u2(5).cn("grp").u2(3).u2(1).u2(2).u2(3)),
    plr => (1, 63, Buf::default().u2(2).u2(1).u2(2).u2(0).u2(0).u1(10).u1(16)
        .cn("A").cn("B").cn("").cn("").cn("").cn("").cn("").cn("")),
    rdr => (1, 70, Buf::default().u2(2).u2(10).u2(20)),
    sdr => (1, 80, Buf::default().u1(1).u1(1).u1(2).u1(10).u1(20)
        .cn("HAND").cn("HID").cn("CARD").cn("CID")),
    psr => (1, 90, Buf::default().u1(0).u2(1).cn("psr").u1(0).u2(2).u2(1)
        .u8v(100).u8v(200).cn("f.pat").cn("lbl").cn("uid").cn("dsc").cn("src")),
    nmr => (1, 91, Buf::default().u1(0).u2(2).u2(2).u2(1).u2(2).cn("A").cn("B")),
    cnr => (1, 92, Buf::default().u2(3).u4(99).sn("scan_cell")),
    ssr => (1, 93, Buf::default().cn("ssr").u2(2).u2(1).u2(2)),
    cdr => (1, 94, Buf::default().u1(0).u2(1).cn("chain").u4(50).u2(1).u2(2)
        .u1(1).u2(7).u1(1).u2(8).u1(0).u2(2).sn("c0").sn("c1")),
    wir => (2, 10, Buf::default().u1(1).u1(255).u4(9).cn("W01")),
    wrr => (2, 20, Buf::default().u1(1).u1(255).u4(9).u4(100).u4(1).u4(2).u4(3).u4(4)
        .cn("W01").cn("FAB").cn("FRAME").cn("MASK").cn("ud").cn("ed")),
    wcr => (2, 30, Buf::default().r4(200.0).r4(5.0).r4(5.0).u1(1).u1(b'D')
        .i2(0).i2(0).u1(b'R').u1(b'U')),
    pir => (5, 10, Buf::default().u1(1).u1(2)),
    prr => (5, 20, Buf::default().u1(1).u1(2).u1(0).u2(10).u2(1).u2(1).i2(5).i2(6)
        .u4(100).cn("P1").cn("desc").bn(&[1, 2, 3])),
    tsr => (10, 30, Buf::default().u1(1).u1(1).u1(b'P').u4(1000).u4(100).u4(2).u4(1)
        .cn("test").cn("seq").cn("lbl").u1(0).r4(0.5).r4(0.1).r4(0.9).r4(50.0).r4(30.0)),
    ptr => (15, 10, Buf::default().u4(1000).u1(1).u1(1).u1(0).u1(0).r4(1.5)
        .cn("test_txt").cn("alarm").u1(2).i1(0).i1(0).i1(0).r4(0.0).r4(10.0)
        .cn("V").cn("%f").cn("").cn("").r4(-1.0).r4(11.0)),
    mpr => (15, 15, Buf::default().u4(7).u1(1).u1(1).u1(0).u1(0).u2(1).u2(1)
        .u1(0x03).r4(9.0).cn("x").cn("")),
    ftr => (15, 20, Buf::default().u4(5).u1(1).u1(1).u1(0).u1(0).u4(100).u4(0).u4(0)
        .u4(3).i4(-1).i4(-2).i2(-5).u2(2).u2(0).u2(7).u2(8).u1(0x21).dn(4, &[0x0A])
        .cn("VEC").cn("").cn("").cn("FT").cn("").cn("").cn("").u1(7).dn(0, &[])),
    str => (15, 30, Buf::default().u1(0).u4(9).u1(1).u1(1).u2(0).u1(0)
        .cn("").cn("S").cn("").cn("").cn("").u1(0).u1(0).dn(0, &[]).dn(0, &[])
        .u8v(0).u4(0).u4(0).u8v(0).u4(0).u2(1).u2(0).u1(4).u1(1).u1(1).u1(1).u1(1)
        .u1(1).u1(1).u1(1).u1(0).u2(0).cn("X=1").u2(1).u4(42).u2(0).u2(0).u2(0)
        .u2(0).u2(0).u2(0).u2(0).u2(0).u2(0).u2(0).u2(0)),
    bps => (20, 10, Buf::default().cn("segment")),
    eps => (20, 20, Buf::default()),
    gdr => (50, 10, Buf::default().u2(2).u1(1).u1(42).u1(10).cn("text")),
    dtr => (50, 30, Buf::default().cn("some datalog text")),
    reserved => (180, 0, Buf::default().u1(1).u1(2).u1(3)),
}
