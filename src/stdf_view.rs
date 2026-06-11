//
// stdf_view.rs
// Borrowing counterpart to the owned records in stdf_types.rs.
//
// RecordView mirrors StdfRecord with scalar Cn/Sn as Cow<str>: borrowed for
// ASCII, allocated as Latin-1 otherwise. Arrays use the shared readers, so a
// view is full fidelity and into_owned rebuilds the owned record exactly.
//

use crate::stdf_types::*;
use std::borrow::Cow;

/// Read Cn (u8 + bytes) as a `Cow<str>`, borrowing for ASCII payloads.
#[inline(always)]
pub(crate) fn read_cn_str<'a>(raw_data: &'a [u8], pos: &mut usize) -> Cow<'a, str> {
    let count = read_uint8(raw_data, pos) as usize;
    if count == 0 {
        return Cow::Borrowed("");
    }
    let min_pos = std::cmp::min(*pos + count, raw_data.len());
    let slice = &raw_data[*pos..min_pos];
    *pos = min_pos;
    cow_from_bytes(slice)
}

/// Read Sn (u16 + bytes) as a `Cow<str>`, borrowing for ASCII payloads.
#[inline(always)]
pub(crate) fn read_sn_str<'a>(
    raw_data: &'a [u8],
    pos: &mut usize,
    order: &ByteOrder,
) -> Cow<'a, str> {
    let count = read_u2(raw_data, pos, order) as usize;
    if count == 0 {
        return Cow::Borrowed("");
    }
    let min_pos = std::cmp::min(*pos + count, raw_data.len());
    let slice = &raw_data[*pos..min_pos];
    *pos = min_pos;
    cow_from_bytes(slice)
}

#[inline(always)]
fn cow_from_bytes(slice: &[u8]) -> Cow<'_, str> {
    // borrow ASCII directly, else allocate the Latin-1 mapping (== owned path)
    match ascii_str(slice) {
        Some(s) => Cow::Borrowed(s),
        None => Cow::Owned(bytes_to_string(slice)),
    }
}

// View structs: each mirrors its owned counterpart with scalar Cn/Sn as
// `Cow<'a, str>`. Arrays and binary fields stay owned (shared readers).

/// Borrowing view of [`ATR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`ATRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct ATRView<'a> {
    pub mod_tim: U4,
    pub cmd_line: Cow<'a, str>,
}

/// Borrowing view of [`VUR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`VURView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct VURView<'a> {
    pub upd_nam: Cow<'a, str>,
}

/// Borrowing view of [`MIR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`MIRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct MIRView<'a> {
    pub setup_t: U4,
    pub start_t: U4,
    pub stat_num: U1,
    pub mode_cod: C1,
    pub rtst_cod: C1,
    pub prot_cod: C1,
    pub burn_tim: U2,
    pub cmod_cod: C1,
    pub lot_id: Cow<'a, str>,
    pub part_typ: Cow<'a, str>,
    pub node_nam: Cow<'a, str>,
    pub tstr_typ: Cow<'a, str>,
    pub job_nam: Cow<'a, str>,
    pub job_rev: Cow<'a, str>,
    pub sblot_id: Cow<'a, str>,
    pub oper_nam: Cow<'a, str>,
    pub exec_typ: Cow<'a, str>,
    pub exec_ver: Cow<'a, str>,
    pub test_cod: Cow<'a, str>,
    pub tst_temp: Cow<'a, str>,
    pub user_txt: Cow<'a, str>,
    pub aux_file: Cow<'a, str>,
    pub pkg_typ: Cow<'a, str>,
    pub famly_id: Cow<'a, str>,
    pub date_cod: Cow<'a, str>,
    pub facil_id: Cow<'a, str>,
    pub floor_id: Cow<'a, str>,
    pub proc_id: Cow<'a, str>,
    pub oper_frq: Cow<'a, str>,
    pub spec_nam: Cow<'a, str>,
    pub spec_ver: Cow<'a, str>,
    pub flow_id: Cow<'a, str>,
    pub setup_id: Cow<'a, str>,
    pub dsgn_rev: Cow<'a, str>,
    pub eng_id: Cow<'a, str>,
    pub rom_cod: Cow<'a, str>,
    pub serl_num: Cow<'a, str>,
    pub supr_nam: Cow<'a, str>,
}

/// Borrowing view of [`MRR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`MRRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct MRRView<'a> {
    pub finish_t: U4,
    pub disp_cod: C1,
    pub usr_desc: Cow<'a, str>,
    pub exc_desc: Cow<'a, str>,
}

/// Borrowing view of [`HBR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`HBRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct HBRView<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub hbin_num: U2,
    pub hbin_cnt: U4,
    pub hbin_pf: C1,
    pub hbin_nam: Cow<'a, str>,
}

/// Borrowing view of [`SBR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`SBRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct SBRView<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub sbin_num: U2,
    pub sbin_cnt: U4,
    pub sbin_pf: C1,
    pub sbin_nam: Cow<'a, str>,
}

/// Borrowing view of [`PMR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`PMRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct PMRView<'a> {
    pub pmr_indx: U2,
    pub chan_typ: U2,
    pub chan_nam: Cow<'a, str>,
    pub phy_nam: Cow<'a, str>,
    pub log_nam: Cow<'a, str>,
    pub head_num: U1,
    pub site_num: U1,
}

/// Borrowing view of [`PGR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`PGRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct PGRView<'a> {
    pub grp_indx: U2,
    pub grp_nam: Cow<'a, str>,
    pub indx_cnt: U2,
    pub pmr_indx: KxU2,
}

/// Borrowing view of [`SDR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`SDRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct SDRView<'a> {
    pub head_num: U1,
    pub site_grp: U1,
    pub site_cnt: U1,
    pub site_num: KxU1,
    pub hand_typ: Cow<'a, str>,
    pub hand_id: Cow<'a, str>,
    pub card_typ: Cow<'a, str>,
    pub card_id: Cow<'a, str>,
    pub load_typ: Cow<'a, str>,
    pub load_id: Cow<'a, str>,
    pub dib_typ: Cow<'a, str>,
    pub dib_id: Cow<'a, str>,
    pub cabl_typ: Cow<'a, str>,
    pub cabl_id: Cow<'a, str>,
    pub cont_typ: Cow<'a, str>,
    pub cont_id: Cow<'a, str>,
    pub lasr_typ: Cow<'a, str>,
    pub lasr_id: Cow<'a, str>,
    pub extr_typ: Cow<'a, str>,
    pub extr_id: Cow<'a, str>,
}

/// Borrowing view of [`PSR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`PSRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct PSRView<'a> {
    pub cont_flg: B1,
    pub psr_indx: U2,
    pub psr_nam: Cow<'a, str>,
    pub opt_flg: B1,
    pub totp_cnt: U2,
    pub locp_cnt: U2,
    pub pat_bgn: KxU8,
    pub pat_end: KxU8,
    pub pat_file: KxCn,
    pub pat_lbl: KxCn,
    pub file_uid: KxCn,
    pub atpg_dsc: KxCn,
    pub src_id: KxCn,
}

/// Borrowing view of [`CNR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`CNRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct CNRView<'a> {
    pub chn_num: U2,
    pub bit_pos: U4,
    pub cell_nam: Cow<'a, str>,
}

/// Borrowing view of [`SSR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`SSRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct SSRView<'a> {
    pub ssr_nam: Cow<'a, str>,
    pub chn_cnt: U2,
    pub chn_list: KxU2,
}

/// Borrowing view of [`CDR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`CDRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct CDRView<'a> {
    pub cont_flg: B1,
    pub cdr_indx: U2,
    pub chn_nam: Cow<'a, str>,
    pub chn_len: U4,
    pub sin_pin: U2,
    pub sout_pin: U2,
    pub mstr_cnt: U1,
    pub m_clks: KxU2,
    pub slav_cnt: U1,
    pub s_clks: KxU2,
    pub inv_val: U1,
    pub lst_cnt: U2,
    pub cell_lst: KxSn,
}

/// Borrowing view of [`WIR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`WIRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct WIRView<'a> {
    pub head_num: U1,
    pub site_grp: U1,
    pub start_t: U4,
    pub wafer_id: Cow<'a, str>,
}

/// Borrowing view of [`WRR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`WRRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct WRRView<'a> {
    pub head_num: U1,
    pub site_grp: U1,
    pub finish_t: U4,
    pub part_cnt: U4,
    pub rtst_cnt: U4,
    pub abrt_cnt: U4,
    pub good_cnt: U4,
    pub func_cnt: U4,
    pub wafer_id: Cow<'a, str>,
    pub fabwf_id: Cow<'a, str>,
    pub frame_id: Cow<'a, str>,
    pub mask_id: Cow<'a, str>,
    pub usr_desc: Cow<'a, str>,
    pub exc_desc: Cow<'a, str>,
}

/// Borrowing view of [`PRR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`PRRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct PRRView<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub part_flg: B1,
    pub num_test: U2,
    pub hard_bin: U2,
    pub soft_bin: U2,
    pub x_coord: I2,
    pub y_coord: I2,
    pub test_t: U4,
    pub part_id: Cow<'a, str>,
    pub part_txt: Cow<'a, str>,
    pub part_fix: Bn,
}

/// Borrowing view of [`TSR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`TSRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct TSRView<'a> {
    pub head_num: U1,
    pub site_num: U1,
    pub test_typ: C1,
    pub test_num: U4,
    pub exec_cnt: U4,
    pub fail_cnt: U4,
    pub alrm_cnt: U4,
    pub test_nam: Cow<'a, str>,
    pub seq_name: Cow<'a, str>,
    pub test_lbl: Cow<'a, str>,
    pub opt_flag: B1,
    pub test_tim: R4,
    pub test_min: R4,
    pub test_max: R4,
    pub tst_sums: R4,
    pub tst_sqrs: R4,
}

/// Borrowing view of [`PTR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`PTRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct PTRView<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    pub result: R4,
    pub test_txt: Cow<'a, str>,
    pub alarm_id: Cow<'a, str>,
    pub opt_flag: Option<B1>,
    pub res_scal: Option<I1>,
    pub llm_scal: Option<I1>,
    pub hlm_scal: Option<I1>,
    pub lo_limit: Option<R4>,
    pub hi_limit: Option<R4>,
    pub units: Option<Cow<'a, str>>,
    pub c_resfmt: Option<Cow<'a, str>>,
    pub c_llmfmt: Option<Cow<'a, str>>,
    pub c_hlmfmt: Option<Cow<'a, str>>,
    pub lo_spec: Option<R4>,
    pub hi_spec: Option<R4>,
}

/// Borrowing view of [`MPR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`MPRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct MPRView<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub parm_flg: B1,
    pub rtn_icnt: U2,
    pub rslt_cnt: U2,
    pub rtn_stat: KxN1,
    pub rtn_rslt: KxR4,
    pub test_txt: Cow<'a, str>,
    pub alarm_id: Cow<'a, str>,
    pub opt_flag: Option<B1>,
    pub res_scal: Option<I1>,
    pub llm_scal: Option<I1>,
    pub hlm_scal: Option<I1>,
    pub lo_limit: Option<R4>,
    pub hi_limit: Option<R4>,
    pub start_in: Option<R4>,
    pub incr_in: Option<R4>,
    pub rtn_indx: Option<KxU2>,
    pub units: Option<Cow<'a, str>>,
    pub units_in: Option<Cow<'a, str>>,
    pub c_resfmt: Option<Cow<'a, str>>,
    pub c_llmfmt: Option<Cow<'a, str>>,
    pub c_hlmfmt: Option<Cow<'a, str>>,
    pub lo_spec: Option<R4>,
    pub hi_spec: Option<R4>,
}

/// Borrowing view of [`FTR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`FTRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct FTRView<'a> {
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub test_flg: B1,
    pub opt_flag: B1,
    pub cycl_cnt: U4,
    pub rel_vadr: U4,
    pub rept_cnt: U4,
    pub num_fail: U4,
    pub xfail_ad: I4,
    pub yfail_ad: I4,
    pub vect_off: I2,
    pub rtn_icnt: U2,
    pub pgm_icnt: U2,
    pub rtn_indx: KxU2,
    pub rtn_stat: KxN1,
    pub pgm_indx: KxU2,
    pub pgm_stat: KxN1,
    pub fail_pin: Dn,
    pub vect_nam: Cow<'a, str>,
    pub time_set: Cow<'a, str>,
    pub op_code: Cow<'a, str>,
    pub test_txt: Cow<'a, str>,
    pub alarm_id: Cow<'a, str>,
    pub prog_txt: Cow<'a, str>,
    pub rslt_txt: Cow<'a, str>,
    pub patg_num: U1,
    pub spin_map: Dn,
}

/// Borrowing view of [`STR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`STRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct STRView<'a> {
    pub cont_flg: B1,
    pub test_num: U4,
    pub head_num: U1,
    pub site_num: U1,
    pub psr_ref: U2,
    pub test_flg: B1,
    pub log_typ: Cow<'a, str>,
    pub test_txt: Cow<'a, str>,
    pub alarm_id: Cow<'a, str>,
    pub prog_txt: Cow<'a, str>,
    pub rslt_txt: Cow<'a, str>,
    pub z_val: U1,
    pub fmu_flg: B1,
    pub mask_map: Dn,
    pub fal_map: Dn,
    pub cyc_cnt_t: U8,
    pub totf_cnt: U4,
    pub totl_cnt: U4,
    pub cyc_base: U8,
    pub bit_base: U4,
    pub cond_cnt: U2,
    pub lim_cnt: U2,
    pub cyc_size: U1,
    pub pmr_size: U1,
    pub chn_size: U1,
    pub pat_size: U1,
    pub bit_size: U1,
    pub u1_size: U1,
    pub u2_size: U1,
    pub u3_size: U1,
    pub utx_size: U1,
    pub cap_bgn: U2,
    pub lim_indx: KxU2,
    pub lim_spec: KxU4,
    pub cond_lst: KxCn,
    pub cyc_cnt: U2,
    pub cyc_ofst: KxUf,
    pub pmr_cnt: U2,
    pub pmr_indx: KxUf,
    pub chn_cnt: U2,
    pub chn_num: KxUf,
    pub exp_cnt: U2,
    pub exp_data: KxU1,
    pub cap_cnt: U2,
    pub cap_data: KxU1,
    pub new_cnt: U2,
    pub new_data: KxU1,
    pub pat_cnt: U2,
    pub pat_num: KxUf,
    pub bpos_cnt: U2,
    pub bit_pos: KxUf,
    pub usr1_cnt: U2,
    pub usr1: KxUf,
    pub usr2_cnt: U2,
    pub usr2: KxUf,
    pub usr3_cnt: U2,
    pub usr3: KxUf,
    pub txt_cnt: U2,
    pub user_txt: KxCf,
}

/// Borrowing view of [`BPS`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`BPSView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct BPSView<'a> {
    pub seq_name: Cow<'a, str>,
}

/// Borrowing view of [`DTR`]; scalar Cn/Sn fields are `Cow<str>`. Convert with [`DTRView::into_owned`].
#[derive(Debug, Clone, PartialEq)]
pub struct DTRView<'a> {
    pub text_dat: Cow<'a, str>,
}

// Borrowing byte-walkers: decode field data into a `*View` (or the owned struct
// for records with no scalar strings). The owned read_from_bytes walkers are
// the owned-path equivalent, kept in sync by the differential test.

// --- records without scalar strings: parsed straight into the owned type ---

#[inline]
pub(crate) fn parse_far(raw_data: &[u8], _order: &ByteOrder) -> FAR {
    let pos = &mut 0;
    FAR {
        cpu_type: read_uint8(raw_data, pos),
        stdf_ver: read_uint8(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_pcr(raw_data: &[u8], order: &ByteOrder) -> PCR {
    let pos = &mut 0;
    let mut v = PCR::new();
    v.head_num = read_uint8(raw_data, pos);
    v.site_num = read_uint8(raw_data, pos);
    v.part_cnt = read_u4(raw_data, pos, order);
    if *pos + 4 <= raw_data.len() {
        v.rtst_cnt = read_u4(raw_data, pos, order);
    }
    if *pos + 4 <= raw_data.len() {
        v.abrt_cnt = read_u4(raw_data, pos, order);
    }
    if *pos + 4 <= raw_data.len() {
        v.good_cnt = read_u4(raw_data, pos, order);
    }
    if *pos + 4 <= raw_data.len() {
        v.func_cnt = read_u4(raw_data, pos, order);
    }
    v
}

#[inline]
pub(crate) fn parse_pmr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> PMRView<'a> {
    let pos = &mut 0;
    let pmr_indx = read_u2(raw_data, pos, order);
    let mut chan_typ = 0;
    if *pos + 2 <= raw_data.len() {
        chan_typ = read_u2(raw_data, pos, order);
    }
    let chan_nam = read_cn_str(raw_data, pos);
    let phy_nam = read_cn_str(raw_data, pos);
    let log_nam = read_cn_str(raw_data, pos);
    let mut head_num = 1;
    if *pos < raw_data.len() {
        head_num = read_uint8(raw_data, pos);
    }
    let mut site_num = 1;
    if *pos < raw_data.len() {
        site_num = read_uint8(raw_data, pos);
    }
    PMRView {
        pmr_indx,
        chan_typ,
        chan_nam,
        phy_nam,
        log_nam,
        head_num,
        site_num,
    }
}

#[inline]
pub(crate) fn parse_pgr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> PGRView<'a> {
    let pos = &mut 0;
    let grp_indx = read_u2(raw_data, pos, order);
    let grp_nam = read_cn_str(raw_data, pos);
    let indx_cnt = read_u2(raw_data, pos, order);
    let pmr_indx = read_kx_u2(raw_data, pos, order, indx_cnt);
    PGRView {
        grp_indx,
        grp_nam,
        indx_cnt,
        pmr_indx,
    }
}

#[inline]
pub(crate) fn parse_plr(raw_data: &[u8], order: &ByteOrder) -> PLR {
    let pos = &mut 0;
    let mut v = PLR::new();
    v.grp_cnt = read_u2(raw_data, pos, order);
    v.grp_indx = read_kx_u2(raw_data, pos, order, v.grp_cnt);
    v.grp_mode = read_kx_u2(raw_data, pos, order, v.grp_cnt);
    v.grp_radx = read_kx_u1(raw_data, pos, v.grp_cnt);
    v.pgm_char = read_kx_cn(raw_data, pos, v.grp_cnt);
    v.rtn_char = read_kx_cn(raw_data, pos, v.grp_cnt);
    v.pgm_chal = read_kx_cn(raw_data, pos, v.grp_cnt);
    v.rtn_chal = read_kx_cn(raw_data, pos, v.grp_cnt);
    v
}

#[inline]
pub(crate) fn parse_rdr(raw_data: &[u8], order: &ByteOrder) -> RDR {
    let pos = &mut 0;
    let num_bins = read_u2(raw_data, pos, order);
    RDR {
        num_bins,
        rtst_bin: read_kx_u2(raw_data, pos, order, num_bins),
    }
}

#[inline]
pub(crate) fn parse_nmr(raw_data: &[u8], order: &ByteOrder) -> NMR {
    let pos = &mut 0;
    let mut v = NMR::new();
    v.cont_flg = [read_uint8(raw_data, pos)];
    v.totm_cnt = read_u2(raw_data, pos, order);
    v.locm_cnt = read_u2(raw_data, pos, order);
    v.pmr_indx = read_kx_u2(raw_data, pos, order, v.locm_cnt);
    v.atpg_nam = read_kx_cn(raw_data, pos, v.locm_cnt);
    v
}

#[inline]
pub(crate) fn parse_wcr(raw_data: &[u8], order: &ByteOrder) -> WCR {
    let pos = &mut 0;
    let mut v = WCR::new();
    v.wafr_siz = read_r4(raw_data, pos, order);
    v.die_ht = read_r4(raw_data, pos, order);
    v.die_wid = read_r4(raw_data, pos, order);
    v.wf_units = read_uint8(raw_data, pos);
    if *pos < raw_data.len() {
        v.wf_flat = read_uint8(raw_data, pos) as char;
    }
    if *pos + 2 <= raw_data.len() {
        v.center_x = read_i2(raw_data, pos, order);
    }
    if *pos + 2 <= raw_data.len() {
        v.center_y = read_i2(raw_data, pos, order);
    }
    if *pos < raw_data.len() {
        v.pos_x = read_uint8(raw_data, pos) as char;
    }
    if *pos < raw_data.len() {
        v.pos_y = read_uint8(raw_data, pos) as char;
    }
    v
}

#[inline]
pub(crate) fn parse_pir(raw_data: &[u8], _order: &ByteOrder) -> PIR {
    let pos = &mut 0;
    PIR {
        head_num: read_uint8(raw_data, pos),
        site_num: read_uint8(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_gdr(raw_data: &[u8], order: &ByteOrder) -> GDR {
    let pos = &mut 0;
    let fld_cnt = read_u2(raw_data, pos, order);
    GDR {
        fld_cnt,
        gen_data: read_vn(raw_data, pos, order, fld_cnt),
    }
}

#[inline]
pub(crate) fn parse_reserved(raw_data: &[u8], _order: &ByteOrder) -> ReservedRec {
    ReservedRec {
        raw_data: raw_data.to_vec(),
    }
}

// --- records with scalar strings: parsed into a borrowing view ---

#[inline]
pub(crate) fn parse_atr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> ATRView<'a> {
    let pos = &mut 0;
    ATRView {
        mod_tim: read_u4(raw_data, pos, order),
        cmd_line: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_vur_view<'a>(raw_data: &'a [u8], _order: &ByteOrder) -> VURView<'a> {
    let pos = &mut 0;
    VURView {
        upd_nam: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_mir_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> MIRView<'a> {
    let pos = &mut 0;
    let setup_t = read_u4(raw_data, pos, order);
    let start_t = read_u4(raw_data, pos, order);
    let stat_num = read_uint8(raw_data, pos);
    let mut mode_cod = ' ';
    if *pos < raw_data.len() {
        mode_cod = read_uint8(raw_data, pos) as char;
    }
    let mut rtst_cod = ' ';
    if *pos < raw_data.len() {
        rtst_cod = read_uint8(raw_data, pos) as char;
    }
    let mut prot_cod = ' ';
    if *pos < raw_data.len() {
        prot_cod = read_uint8(raw_data, pos) as char;
    }
    let mut burn_tim = 65535;
    if *pos + 2 <= raw_data.len() {
        burn_tim = read_u2(raw_data, pos, order);
    }
    let mut cmod_cod = ' ';
    if *pos < raw_data.len() {
        cmod_cod = read_uint8(raw_data, pos) as char;
    }
    MIRView {
        setup_t,
        start_t,
        stat_num,
        mode_cod,
        rtst_cod,
        prot_cod,
        burn_tim,
        cmod_cod,
        lot_id: read_cn_str(raw_data, pos),
        part_typ: read_cn_str(raw_data, pos),
        node_nam: read_cn_str(raw_data, pos),
        tstr_typ: read_cn_str(raw_data, pos),
        job_nam: read_cn_str(raw_data, pos),
        job_rev: read_cn_str(raw_data, pos),
        sblot_id: read_cn_str(raw_data, pos),
        oper_nam: read_cn_str(raw_data, pos),
        exec_typ: read_cn_str(raw_data, pos),
        exec_ver: read_cn_str(raw_data, pos),
        test_cod: read_cn_str(raw_data, pos),
        tst_temp: read_cn_str(raw_data, pos),
        user_txt: read_cn_str(raw_data, pos),
        aux_file: read_cn_str(raw_data, pos),
        pkg_typ: read_cn_str(raw_data, pos),
        famly_id: read_cn_str(raw_data, pos),
        date_cod: read_cn_str(raw_data, pos),
        facil_id: read_cn_str(raw_data, pos),
        floor_id: read_cn_str(raw_data, pos),
        proc_id: read_cn_str(raw_data, pos),
        oper_frq: read_cn_str(raw_data, pos),
        spec_nam: read_cn_str(raw_data, pos),
        spec_ver: read_cn_str(raw_data, pos),
        flow_id: read_cn_str(raw_data, pos),
        setup_id: read_cn_str(raw_data, pos),
        dsgn_rev: read_cn_str(raw_data, pos),
        eng_id: read_cn_str(raw_data, pos),
        rom_cod: read_cn_str(raw_data, pos),
        serl_num: read_cn_str(raw_data, pos),
        supr_nam: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_mrr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> MRRView<'a> {
    let pos = &mut 0;
    let finish_t = read_u4(raw_data, pos, order);
    let mut disp_cod = ' ';
    if *pos < raw_data.len() {
        disp_cod = read_uint8(raw_data, pos) as char;
    }
    MRRView {
        finish_t,
        disp_cod,
        usr_desc: read_cn_str(raw_data, pos),
        exc_desc: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_hbr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> HBRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let hbin_num = read_u2(raw_data, pos, order);
    let hbin_cnt = read_u4(raw_data, pos, order);
    let mut hbin_pf = ' ';
    if *pos < raw_data.len() {
        hbin_pf = read_uint8(raw_data, pos) as char;
    }
    HBRView {
        head_num,
        site_num,
        hbin_num,
        hbin_cnt,
        hbin_pf,
        hbin_nam: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_sbr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> SBRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let sbin_num = read_u2(raw_data, pos, order);
    let sbin_cnt = read_u4(raw_data, pos, order);
    let mut sbin_pf = ' ';
    if *pos < raw_data.len() {
        sbin_pf = read_uint8(raw_data, pos) as char;
    }
    SBRView {
        head_num,
        site_num,
        sbin_num,
        sbin_cnt,
        sbin_pf,
        sbin_nam: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_sdr_view<'a>(raw_data: &'a [u8], _order: &ByteOrder) -> SDRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let site_grp = read_uint8(raw_data, pos);
    let site_cnt = read_uint8(raw_data, pos);
    let site_num = read_kx_u1(raw_data, pos, site_cnt as u16);
    SDRView {
        head_num,
        site_grp,
        site_cnt,
        site_num,
        hand_typ: read_cn_str(raw_data, pos),
        hand_id: read_cn_str(raw_data, pos),
        card_typ: read_cn_str(raw_data, pos),
        card_id: read_cn_str(raw_data, pos),
        load_typ: read_cn_str(raw_data, pos),
        load_id: read_cn_str(raw_data, pos),
        dib_typ: read_cn_str(raw_data, pos),
        dib_id: read_cn_str(raw_data, pos),
        cabl_typ: read_cn_str(raw_data, pos),
        cabl_id: read_cn_str(raw_data, pos),
        cont_typ: read_cn_str(raw_data, pos),
        cont_id: read_cn_str(raw_data, pos),
        lasr_typ: read_cn_str(raw_data, pos),
        lasr_id: read_cn_str(raw_data, pos),
        extr_typ: read_cn_str(raw_data, pos),
        extr_id: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_psr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> PSRView<'a> {
    let pos = &mut 0;
    let cont_flg = [read_uint8(raw_data, pos)];
    let psr_indx = read_u2(raw_data, pos, order);
    let psr_nam = read_cn_str(raw_data, pos);
    let opt_flg = [read_uint8(raw_data, pos)];
    let totp_cnt = read_u2(raw_data, pos, order);
    let locp_cnt = read_u2(raw_data, pos, order);
    PSRView {
        cont_flg,
        psr_indx,
        psr_nam,
        opt_flg,
        totp_cnt,
        locp_cnt,
        pat_bgn: read_kx_u8(raw_data, pos, order, locp_cnt),
        pat_end: read_kx_u8(raw_data, pos, order, locp_cnt),
        pat_file: read_kx_cn(raw_data, pos, locp_cnt),
        pat_lbl: read_kx_cn(raw_data, pos, locp_cnt),
        file_uid: read_kx_cn(raw_data, pos, locp_cnt),
        atpg_dsc: read_kx_cn(raw_data, pos, locp_cnt),
        src_id: read_kx_cn(raw_data, pos, locp_cnt),
    }
}

#[inline]
pub(crate) fn parse_cnr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> CNRView<'a> {
    let pos = &mut 0;
    CNRView {
        chn_num: read_u2(raw_data, pos, order),
        bit_pos: read_u4(raw_data, pos, order),
        cell_nam: read_sn_str(raw_data, pos, order),
    }
}

#[inline]
pub(crate) fn parse_ssr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> SSRView<'a> {
    let pos = &mut 0;
    let ssr_nam = read_cn_str(raw_data, pos);
    let chn_cnt = read_u2(raw_data, pos, order);
    SSRView {
        ssr_nam,
        chn_cnt,
        chn_list: read_kx_u2(raw_data, pos, order, chn_cnt),
    }
}

#[inline]
pub(crate) fn parse_cdr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> CDRView<'a> {
    let pos = &mut 0;
    let cont_flg = [read_uint8(raw_data, pos)];
    let cdr_indx = read_u2(raw_data, pos, order);
    let chn_nam = read_cn_str(raw_data, pos);
    let chn_len = read_u4(raw_data, pos, order);
    let sin_pin = read_u2(raw_data, pos, order);
    let sout_pin = read_u2(raw_data, pos, order);
    let mstr_cnt = read_uint8(raw_data, pos);
    let m_clks = read_kx_u2(raw_data, pos, order, mstr_cnt as u16);
    let slav_cnt = read_uint8(raw_data, pos);
    let s_clks = read_kx_u2(raw_data, pos, order, slav_cnt as u16);
    let mut inv_val = 255;
    if *pos < raw_data.len() {
        inv_val = read_uint8(raw_data, pos);
    }
    let lst_cnt = read_u2(raw_data, pos, order);
    let cell_lst = read_kx_sn(raw_data, pos, order, lst_cnt);
    CDRView {
        cont_flg,
        cdr_indx,
        chn_nam,
        chn_len,
        sin_pin,
        sout_pin,
        mstr_cnt,
        m_clks,
        slav_cnt,
        s_clks,
        inv_val,
        lst_cnt,
        cell_lst,
    }
}

#[inline]
pub(crate) fn parse_wir_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> WIRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let mut site_grp = 255;
    if *pos < raw_data.len() {
        site_grp = read_uint8(raw_data, pos);
    }
    let start_t = read_u4(raw_data, pos, order);
    WIRView {
        head_num,
        site_grp,
        start_t,
        wafer_id: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_wrr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> WRRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let mut site_grp = 255;
    if *pos < raw_data.len() {
        site_grp = read_uint8(raw_data, pos);
    }
    let finish_t = read_u4(raw_data, pos, order);
    let part_cnt = read_u4(raw_data, pos, order);
    let mut rtst_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        rtst_cnt = read_u4(raw_data, pos, order);
    }
    let mut abrt_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        abrt_cnt = read_u4(raw_data, pos, order);
    }
    let mut good_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        good_cnt = read_u4(raw_data, pos, order);
    }
    let mut func_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        func_cnt = read_u4(raw_data, pos, order);
    }
    WRRView {
        head_num,
        site_grp,
        finish_t,
        part_cnt,
        rtst_cnt,
        abrt_cnt,
        good_cnt,
        func_cnt,
        wafer_id: read_cn_str(raw_data, pos),
        fabwf_id: read_cn_str(raw_data, pos),
        frame_id: read_cn_str(raw_data, pos),
        mask_id: read_cn_str(raw_data, pos),
        usr_desc: read_cn_str(raw_data, pos),
        exc_desc: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_prr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> PRRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let part_flg = [read_uint8(raw_data, pos)];
    let num_test = read_u2(raw_data, pos, order);
    let hard_bin = read_u2(raw_data, pos, order);
    let mut soft_bin = 65535;
    if *pos + 2 <= raw_data.len() {
        soft_bin = read_u2(raw_data, pos, order);
    }
    let mut x_coord = -32768;
    if *pos + 2 <= raw_data.len() {
        x_coord = read_i2(raw_data, pos, order);
    }
    let mut y_coord = -32768;
    if *pos + 2 <= raw_data.len() {
        y_coord = read_i2(raw_data, pos, order);
    }
    let mut test_t = 0;
    if *pos + 4 <= raw_data.len() {
        test_t = read_u4(raw_data, pos, order);
    }
    PRRView {
        head_num,
        site_num,
        part_flg,
        num_test,
        hard_bin,
        soft_bin,
        x_coord,
        y_coord,
        test_t,
        part_id: read_cn_str(raw_data, pos),
        part_txt: read_cn_str(raw_data, pos),
        part_fix: read_bn(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_tsr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> TSRView<'a> {
    let pos = &mut 0;
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let mut test_typ = ' ';
    if *pos < raw_data.len() {
        test_typ = read_uint8(raw_data, pos) as char;
    }
    let test_num = read_u4(raw_data, pos, order);
    let mut exec_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        exec_cnt = read_u4(raw_data, pos, order);
    }
    let mut fail_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        fail_cnt = read_u4(raw_data, pos, order);
    }
    let mut alrm_cnt = 4_294_967_295;
    if *pos + 4 <= raw_data.len() {
        alrm_cnt = read_u4(raw_data, pos, order);
    }
    let test_nam = read_cn_str(raw_data, pos);
    let seq_name = read_cn_str(raw_data, pos);
    let test_lbl = read_cn_str(raw_data, pos);
    let opt_flag = [read_uint8(raw_data, pos)];
    TSRView {
        head_num,
        site_num,
        test_typ,
        test_num,
        exec_cnt,
        fail_cnt,
        alrm_cnt,
        test_nam,
        seq_name,
        test_lbl,
        opt_flag,
        test_tim: read_r4(raw_data, pos, order),
        test_min: read_r4(raw_data, pos, order),
        test_max: read_r4(raw_data, pos, order),
        tst_sums: read_r4(raw_data, pos, order),
        tst_sqrs: read_r4(raw_data, pos, order),
    }
}

#[inline]
pub(crate) fn parse_ptr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> PTRView<'a> {
    let pos = &mut 0;
    let test_num = read_u4(raw_data, pos, order);
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let test_flg = [read_uint8(raw_data, pos)];
    let parm_flg = [read_uint8(raw_data, pos)];
    let result = read_r4(raw_data, pos, order);
    let test_txt = read_cn_str(raw_data, pos);
    let alarm_id = read_cn_str(raw_data, pos);
    let mut opt_flag = None;
    let mut res_scal = None;
    let mut llm_scal = None;
    let mut hlm_scal = None;
    let mut lo_limit = None;
    let mut hi_limit = None;
    let mut units = None;
    let mut c_resfmt = None;
    let mut c_llmfmt = None;
    let mut c_hlmfmt = None;
    let mut lo_spec = None;
    let mut hi_spec = None;
    // Trailing optionals. An absent length-checked field ends parsing
    // (break 'opt). Order/count-checked fields just stay None.
    'opt: {
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        opt_flag = Some([read_uint8(raw_data, pos)]);
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        res_scal = Some(read_i1(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        llm_scal = Some(read_i1(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        hlm_scal = Some(read_i1(raw_data, pos));
        if *pos + 4 <= raw_data.len() {
            lo_limit = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            hi_limit = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        units = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_resfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_llmfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_hlmfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 4 <= raw_data.len() {
            lo_spec = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            hi_spec = Some(read_r4(raw_data, pos, order));
        }
    }
    PTRView {
        test_num,
        head_num,
        site_num,
        test_flg,
        parm_flg,
        result,
        test_txt,
        alarm_id,
        opt_flag,
        res_scal,
        llm_scal,
        hlm_scal,
        lo_limit,
        hi_limit,
        units,
        c_resfmt,
        c_llmfmt,
        c_hlmfmt,
        lo_spec,
        hi_spec,
    }
}

#[inline]
pub(crate) fn parse_mpr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> MPRView<'a> {
    let pos = &mut 0;
    let test_num = read_u4(raw_data, pos, order);
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let test_flg = [read_uint8(raw_data, pos)];
    let parm_flg = [read_uint8(raw_data, pos)];
    let rtn_icnt = read_u2(raw_data, pos, order);
    let rslt_cnt = read_u2(raw_data, pos, order);
    let rtn_stat = read_kx_n1(raw_data, pos, rtn_icnt);
    let rtn_rslt = read_kx_r4(raw_data, pos, order, rslt_cnt);
    let test_txt = read_cn_str(raw_data, pos);
    let alarm_id = read_cn_str(raw_data, pos);
    let mut opt_flag = None;
    let mut res_scal = None;
    let mut llm_scal = None;
    let mut hlm_scal = None;
    let mut lo_limit = None;
    let mut hi_limit = None;
    let mut start_in = None;
    let mut incr_in = None;
    let mut rtn_indx = None;
    let mut units = None;
    let mut units_in = None;
    let mut c_resfmt = None;
    let mut c_llmfmt = None;
    let mut c_hlmfmt = None;
    let mut lo_spec = None;
    let mut hi_spec = None;
    'opt: {
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        opt_flag = Some([read_uint8(raw_data, pos)]);
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        res_scal = Some(read_i1(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        llm_scal = Some(read_i1(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        hlm_scal = Some(read_i1(raw_data, pos));
        if *pos + 4 <= raw_data.len() {
            lo_limit = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            hi_limit = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            start_in = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            incr_in = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 2 * rtn_icnt as usize <= raw_data.len() {
            rtn_indx = Some(read_kx_u2(raw_data, pos, order, rtn_icnt));
        }
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        units = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        units_in = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_resfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_llmfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 1 > raw_data.len() {
            break 'opt;
        }
        c_hlmfmt = Some(read_cn_str(raw_data, pos));
        if *pos + 4 <= raw_data.len() {
            lo_spec = Some(read_r4(raw_data, pos, order));
        }
        if *pos + 4 <= raw_data.len() {
            hi_spec = Some(read_r4(raw_data, pos, order));
        }
    }
    MPRView {
        test_num,
        head_num,
        site_num,
        test_flg,
        parm_flg,
        rtn_icnt,
        rslt_cnt,
        rtn_stat,
        rtn_rslt,
        test_txt,
        alarm_id,
        opt_flag,
        res_scal,
        llm_scal,
        hlm_scal,
        lo_limit,
        hi_limit,
        start_in,
        incr_in,
        rtn_indx,
        units,
        units_in,
        c_resfmt,
        c_llmfmt,
        c_hlmfmt,
        lo_spec,
        hi_spec,
    }
}

#[inline]
pub(crate) fn parse_ftr_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> FTRView<'a> {
    let pos = &mut 0;
    let test_num = read_u4(raw_data, pos, order);
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let test_flg = [read_uint8(raw_data, pos)];
    let opt_flag = [read_uint8(raw_data, pos)];
    let cycl_cnt = read_u4(raw_data, pos, order);
    let rel_vadr = read_u4(raw_data, pos, order);
    let rept_cnt = read_u4(raw_data, pos, order);
    let num_fail = read_u4(raw_data, pos, order);
    let xfail_ad = read_i4(raw_data, pos, order);
    let yfail_ad = read_i4(raw_data, pos, order);
    let vect_off = read_i2(raw_data, pos, order);
    let rtn_icnt = read_u2(raw_data, pos, order);
    let pgm_icnt = read_u2(raw_data, pos, order);
    let rtn_indx = read_kx_u2(raw_data, pos, order, rtn_icnt);
    let rtn_stat = read_kx_n1(raw_data, pos, rtn_icnt);
    let pgm_indx = read_kx_u2(raw_data, pos, order, pgm_icnt);
    let pgm_stat = read_kx_n1(raw_data, pos, pgm_icnt);
    let fail_pin = read_dn(raw_data, pos, order);
    let vect_nam = read_cn_str(raw_data, pos);
    let time_set = read_cn_str(raw_data, pos);
    let op_code = read_cn_str(raw_data, pos);
    let test_txt = read_cn_str(raw_data, pos);
    let alarm_id = read_cn_str(raw_data, pos);
    let prog_txt = read_cn_str(raw_data, pos);
    let rslt_txt = read_cn_str(raw_data, pos);
    let mut patg_num = 255;
    if *pos < raw_data.len() {
        patg_num = read_uint8(raw_data, pos);
    }
    let spin_map = read_dn(raw_data, pos, order);
    FTRView {
        test_num,
        head_num,
        site_num,
        test_flg,
        opt_flag,
        cycl_cnt,
        rel_vadr,
        rept_cnt,
        num_fail,
        xfail_ad,
        yfail_ad,
        vect_off,
        rtn_icnt,
        pgm_icnt,
        rtn_indx,
        rtn_stat,
        pgm_indx,
        pgm_stat,
        fail_pin,
        vect_nam,
        time_set,
        op_code,
        test_txt,
        alarm_id,
        prog_txt,
        rslt_txt,
        patg_num,
        spin_map,
    }
}

#[inline]
pub(crate) fn parse_str_view<'a>(raw_data: &'a [u8], order: &ByteOrder) -> STRView<'a> {
    let pos = &mut 0;
    let cont_flg = [read_uint8(raw_data, pos)];
    let test_num = read_u4(raw_data, pos, order);
    let head_num = read_uint8(raw_data, pos);
    let site_num = read_uint8(raw_data, pos);
    let psr_ref = read_u2(raw_data, pos, order);
    let test_flg = [read_uint8(raw_data, pos)];
    let log_typ = read_cn_str(raw_data, pos);
    let test_txt = read_cn_str(raw_data, pos);
    let alarm_id = read_cn_str(raw_data, pos);
    let prog_txt = read_cn_str(raw_data, pos);
    let rslt_txt = read_cn_str(raw_data, pos);
    let z_val = read_uint8(raw_data, pos);
    let fmu_flg = [read_uint8(raw_data, pos)];
    let mask_map = read_dn(raw_data, pos, order);
    let fal_map = read_dn(raw_data, pos, order);
    let cyc_cnt_t = read_u8(raw_data, pos, order);
    let totf_cnt = read_u4(raw_data, pos, order);
    let totl_cnt = read_u4(raw_data, pos, order);
    let cyc_base = read_u8(raw_data, pos, order);
    let bit_base = read_u4(raw_data, pos, order);
    let cond_cnt = read_u2(raw_data, pos, order);
    let lim_cnt = read_u2(raw_data, pos, order);
    let cyc_size = read_uint8(raw_data, pos);
    let pmr_size = read_uint8(raw_data, pos);
    let chn_size = read_uint8(raw_data, pos);
    let pat_size = read_uint8(raw_data, pos);
    let bit_size = read_uint8(raw_data, pos);
    let u1_size = read_uint8(raw_data, pos);
    let u2_size = read_uint8(raw_data, pos);
    let u3_size = read_uint8(raw_data, pos);
    let utx_size = read_uint8(raw_data, pos);
    let cap_bgn = read_u2(raw_data, pos, order);
    let lim_indx = read_kx_u2(raw_data, pos, order, lim_cnt);
    let lim_spec = read_kx_u4(raw_data, pos, order, lim_cnt);
    let cond_lst = read_kx_cn(raw_data, pos, cond_cnt);
    let cyc_cnt = read_u2(raw_data, pos, order);
    let cyc_ofst = read_kx_uf(raw_data, pos, order, cyc_cnt, cyc_size);
    let pmr_cnt = read_u2(raw_data, pos, order);
    let pmr_indx = read_kx_uf(raw_data, pos, order, pmr_cnt, pmr_size);
    let chn_cnt = read_u2(raw_data, pos, order);
    let chn_num = read_kx_uf(raw_data, pos, order, chn_cnt, chn_size);
    let exp_cnt = read_u2(raw_data, pos, order);
    let exp_data = read_kx_u1(raw_data, pos, exp_cnt);
    let cap_cnt = read_u2(raw_data, pos, order);
    let cap_data = read_kx_u1(raw_data, pos, cap_cnt);
    let new_cnt = read_u2(raw_data, pos, order);
    let new_data = read_kx_u1(raw_data, pos, new_cnt);
    let pat_cnt = read_u2(raw_data, pos, order);
    let pat_num = read_kx_uf(raw_data, pos, order, pat_cnt, pat_size);
    let bpos_cnt = read_u2(raw_data, pos, order);
    let bit_pos = read_kx_uf(raw_data, pos, order, bpos_cnt, bit_size);
    let usr1_cnt = read_u2(raw_data, pos, order);
    let usr1 = read_kx_uf(raw_data, pos, order, usr1_cnt, u1_size);
    let usr2_cnt = read_u2(raw_data, pos, order);
    let usr2 = read_kx_uf(raw_data, pos, order, usr2_cnt, u2_size);
    let usr3_cnt = read_u2(raw_data, pos, order);
    let usr3 = read_kx_uf(raw_data, pos, order, usr3_cnt, u3_size);
    let txt_cnt = read_u2(raw_data, pos, order);
    let user_txt = read_kx_cf(raw_data, pos, txt_cnt, utx_size);
    STRView {
        cont_flg,
        test_num,
        head_num,
        site_num,
        psr_ref,
        test_flg,
        log_typ,
        test_txt,
        alarm_id,
        prog_txt,
        rslt_txt,
        z_val,
        fmu_flg,
        mask_map,
        fal_map,
        cyc_cnt_t,
        totf_cnt,
        totl_cnt,
        cyc_base,
        bit_base,
        cond_cnt,
        lim_cnt,
        cyc_size,
        pmr_size,
        chn_size,
        pat_size,
        bit_size,
        u1_size,
        u2_size,
        u3_size,
        utx_size,
        cap_bgn,
        lim_indx,
        lim_spec,
        cond_lst,
        cyc_cnt,
        cyc_ofst,
        pmr_cnt,
        pmr_indx,
        chn_cnt,
        chn_num,
        exp_cnt,
        exp_data,
        cap_cnt,
        cap_data,
        new_cnt,
        new_data,
        pat_cnt,
        pat_num,
        bpos_cnt,
        bit_pos,
        usr1_cnt,
        usr1,
        usr2_cnt,
        usr2,
        usr3_cnt,
        usr3,
        txt_cnt,
        user_txt,
    }
}

#[inline]
pub(crate) fn parse_bps_view<'a>(raw_data: &'a [u8], _order: &ByteOrder) -> BPSView<'a> {
    let pos = &mut 0;
    BPSView {
        seq_name: read_cn_str(raw_data, pos),
    }
}

#[inline]
pub(crate) fn parse_dtr_view<'a>(raw_data: &'a [u8], _order: &ByteOrder) -> DTRView<'a> {
    let pos = &mut 0;
    DTRView {
        text_dat: read_cn_str(raw_data, pos),
    }
}

// into_owned: rebuild the owned record exactly from a view.

impl ATRView<'_> {
    /// Allocate the borrowed strings into an owned [`ATR`].
    pub fn into_owned(self) -> ATR {
        ATR {
            mod_tim: self.mod_tim,
            cmd_line: self.cmd_line.into_owned(),
        }
    }
}

impl VURView<'_> {
    /// Allocate the borrowed strings into an owned [`VUR`].
    pub fn into_owned(self) -> VUR {
        VUR {
            upd_nam: self.upd_nam.into_owned(),
        }
    }
}

impl MIRView<'_> {
    /// Allocate the borrowed strings into an owned [`MIR`].
    pub fn into_owned(self) -> MIR {
        MIR {
            setup_t: self.setup_t,
            start_t: self.start_t,
            stat_num: self.stat_num,
            mode_cod: self.mode_cod,
            rtst_cod: self.rtst_cod,
            prot_cod: self.prot_cod,
            burn_tim: self.burn_tim,
            cmod_cod: self.cmod_cod,
            lot_id: self.lot_id.into_owned(),
            part_typ: self.part_typ.into_owned(),
            node_nam: self.node_nam.into_owned(),
            tstr_typ: self.tstr_typ.into_owned(),
            job_nam: self.job_nam.into_owned(),
            job_rev: self.job_rev.into_owned(),
            sblot_id: self.sblot_id.into_owned(),
            oper_nam: self.oper_nam.into_owned(),
            exec_typ: self.exec_typ.into_owned(),
            exec_ver: self.exec_ver.into_owned(),
            test_cod: self.test_cod.into_owned(),
            tst_temp: self.tst_temp.into_owned(),
            user_txt: self.user_txt.into_owned(),
            aux_file: self.aux_file.into_owned(),
            pkg_typ: self.pkg_typ.into_owned(),
            famly_id: self.famly_id.into_owned(),
            date_cod: self.date_cod.into_owned(),
            facil_id: self.facil_id.into_owned(),
            floor_id: self.floor_id.into_owned(),
            proc_id: self.proc_id.into_owned(),
            oper_frq: self.oper_frq.into_owned(),
            spec_nam: self.spec_nam.into_owned(),
            spec_ver: self.spec_ver.into_owned(),
            flow_id: self.flow_id.into_owned(),
            setup_id: self.setup_id.into_owned(),
            dsgn_rev: self.dsgn_rev.into_owned(),
            eng_id: self.eng_id.into_owned(),
            rom_cod: self.rom_cod.into_owned(),
            serl_num: self.serl_num.into_owned(),
            supr_nam: self.supr_nam.into_owned(),
        }
    }
}

impl MRRView<'_> {
    /// Allocate the borrowed strings into an owned [`MRR`].
    pub fn into_owned(self) -> MRR {
        MRR {
            finish_t: self.finish_t,
            disp_cod: self.disp_cod,
            usr_desc: self.usr_desc.into_owned(),
            exc_desc: self.exc_desc.into_owned(),
        }
    }
}

impl HBRView<'_> {
    /// Allocate the borrowed strings into an owned [`HBR`].
    pub fn into_owned(self) -> HBR {
        HBR {
            head_num: self.head_num,
            site_num: self.site_num,
            hbin_num: self.hbin_num,
            hbin_cnt: self.hbin_cnt,
            hbin_pf: self.hbin_pf,
            hbin_nam: self.hbin_nam.into_owned(),
        }
    }
}

impl SBRView<'_> {
    /// Allocate the borrowed strings into an owned [`SBR`].
    pub fn into_owned(self) -> SBR {
        SBR {
            head_num: self.head_num,
            site_num: self.site_num,
            sbin_num: self.sbin_num,
            sbin_cnt: self.sbin_cnt,
            sbin_pf: self.sbin_pf,
            sbin_nam: self.sbin_nam.into_owned(),
        }
    }
}

impl PMRView<'_> {
    /// Allocate the borrowed strings into an owned [`PMR`].
    pub fn into_owned(self) -> PMR {
        PMR {
            pmr_indx: self.pmr_indx,
            chan_typ: self.chan_typ,
            chan_nam: self.chan_nam.into_owned(),
            phy_nam: self.phy_nam.into_owned(),
            log_nam: self.log_nam.into_owned(),
            head_num: self.head_num,
            site_num: self.site_num,
        }
    }
}

impl PGRView<'_> {
    /// Allocate the borrowed strings into an owned [`PGR`].
    pub fn into_owned(self) -> PGR {
        PGR {
            grp_indx: self.grp_indx,
            grp_nam: self.grp_nam.into_owned(),
            indx_cnt: self.indx_cnt,
            pmr_indx: self.pmr_indx,
        }
    }
}

impl SDRView<'_> {
    /// Allocate the borrowed strings into an owned [`SDR`].
    pub fn into_owned(self) -> SDR {
        SDR {
            head_num: self.head_num,
            site_grp: self.site_grp,
            site_cnt: self.site_cnt,
            site_num: self.site_num,
            hand_typ: self.hand_typ.into_owned(),
            hand_id: self.hand_id.into_owned(),
            card_typ: self.card_typ.into_owned(),
            card_id: self.card_id.into_owned(),
            load_typ: self.load_typ.into_owned(),
            load_id: self.load_id.into_owned(),
            dib_typ: self.dib_typ.into_owned(),
            dib_id: self.dib_id.into_owned(),
            cabl_typ: self.cabl_typ.into_owned(),
            cabl_id: self.cabl_id.into_owned(),
            cont_typ: self.cont_typ.into_owned(),
            cont_id: self.cont_id.into_owned(),
            lasr_typ: self.lasr_typ.into_owned(),
            lasr_id: self.lasr_id.into_owned(),
            extr_typ: self.extr_typ.into_owned(),
            extr_id: self.extr_id.into_owned(),
        }
    }
}

impl PSRView<'_> {
    /// Allocate the borrowed strings into an owned [`PSR`].
    pub fn into_owned(self) -> PSR {
        PSR {
            cont_flg: self.cont_flg,
            psr_indx: self.psr_indx,
            psr_nam: self.psr_nam.into_owned(),
            opt_flg: self.opt_flg,
            totp_cnt: self.totp_cnt,
            locp_cnt: self.locp_cnt,
            pat_bgn: self.pat_bgn,
            pat_end: self.pat_end,
            pat_file: self.pat_file,
            pat_lbl: self.pat_lbl,
            file_uid: self.file_uid,
            atpg_dsc: self.atpg_dsc,
            src_id: self.src_id,
        }
    }
}

impl CNRView<'_> {
    /// Allocate the borrowed strings into an owned [`CNR`].
    pub fn into_owned(self) -> CNR {
        CNR {
            chn_num: self.chn_num,
            bit_pos: self.bit_pos,
            cell_nam: self.cell_nam.into_owned(),
        }
    }
}

impl SSRView<'_> {
    /// Allocate the borrowed strings into an owned [`SSR`].
    pub fn into_owned(self) -> SSR {
        SSR {
            ssr_nam: self.ssr_nam.into_owned(),
            chn_cnt: self.chn_cnt,
            chn_list: self.chn_list,
        }
    }
}

impl CDRView<'_> {
    /// Allocate the borrowed strings into an owned [`CDR`].
    pub fn into_owned(self) -> CDR {
        CDR {
            cont_flg: self.cont_flg,
            cdr_indx: self.cdr_indx,
            chn_nam: self.chn_nam.into_owned(),
            chn_len: self.chn_len,
            sin_pin: self.sin_pin,
            sout_pin: self.sout_pin,
            mstr_cnt: self.mstr_cnt,
            m_clks: self.m_clks,
            slav_cnt: self.slav_cnt,
            s_clks: self.s_clks,
            inv_val: self.inv_val,
            lst_cnt: self.lst_cnt,
            cell_lst: self.cell_lst,
        }
    }
}

impl WIRView<'_> {
    /// Allocate the borrowed strings into an owned [`WIR`].
    pub fn into_owned(self) -> WIR {
        WIR {
            head_num: self.head_num,
            site_grp: self.site_grp,
            start_t: self.start_t,
            wafer_id: self.wafer_id.into_owned(),
        }
    }
}

impl WRRView<'_> {
    /// Allocate the borrowed strings into an owned [`WRR`].
    pub fn into_owned(self) -> WRR {
        WRR {
            head_num: self.head_num,
            site_grp: self.site_grp,
            finish_t: self.finish_t,
            part_cnt: self.part_cnt,
            rtst_cnt: self.rtst_cnt,
            abrt_cnt: self.abrt_cnt,
            good_cnt: self.good_cnt,
            func_cnt: self.func_cnt,
            wafer_id: self.wafer_id.into_owned(),
            fabwf_id: self.fabwf_id.into_owned(),
            frame_id: self.frame_id.into_owned(),
            mask_id: self.mask_id.into_owned(),
            usr_desc: self.usr_desc.into_owned(),
            exc_desc: self.exc_desc.into_owned(),
        }
    }
}

impl PRRView<'_> {
    /// Allocate the borrowed strings into an owned [`PRR`].
    pub fn into_owned(self) -> PRR {
        PRR {
            head_num: self.head_num,
            site_num: self.site_num,
            part_flg: self.part_flg,
            num_test: self.num_test,
            hard_bin: self.hard_bin,
            soft_bin: self.soft_bin,
            x_coord: self.x_coord,
            y_coord: self.y_coord,
            test_t: self.test_t,
            part_id: self.part_id.into_owned(),
            part_txt: self.part_txt.into_owned(),
            part_fix: self.part_fix,
        }
    }
}

impl TSRView<'_> {
    /// Allocate the borrowed strings into an owned [`TSR`].
    pub fn into_owned(self) -> TSR {
        TSR {
            head_num: self.head_num,
            site_num: self.site_num,
            test_typ: self.test_typ,
            test_num: self.test_num,
            exec_cnt: self.exec_cnt,
            fail_cnt: self.fail_cnt,
            alrm_cnt: self.alrm_cnt,
            test_nam: self.test_nam.into_owned(),
            seq_name: self.seq_name.into_owned(),
            test_lbl: self.test_lbl.into_owned(),
            opt_flag: self.opt_flag,
            test_tim: self.test_tim,
            test_min: self.test_min,
            test_max: self.test_max,
            tst_sums: self.tst_sums,
            tst_sqrs: self.tst_sqrs,
        }
    }
}

impl PTRView<'_> {
    /// Allocate the borrowed strings into an owned [`PTR`].
    pub fn into_owned(self) -> PTR {
        PTR {
            test_num: self.test_num,
            head_num: self.head_num,
            site_num: self.site_num,
            test_flg: self.test_flg,
            parm_flg: self.parm_flg,
            result: self.result,
            test_txt: self.test_txt.into_owned(),
            alarm_id: self.alarm_id.into_owned(),
            opt_flag: self.opt_flag,
            res_scal: self.res_scal,
            llm_scal: self.llm_scal,
            hlm_scal: self.hlm_scal,
            lo_limit: self.lo_limit,
            hi_limit: self.hi_limit,
            units: self.units.map(Cow::into_owned),
            c_resfmt: self.c_resfmt.map(Cow::into_owned),
            c_llmfmt: self.c_llmfmt.map(Cow::into_owned),
            c_hlmfmt: self.c_hlmfmt.map(Cow::into_owned),
            lo_spec: self.lo_spec,
            hi_spec: self.hi_spec,
        }
    }
}

impl MPRView<'_> {
    /// Allocate the borrowed strings into an owned [`MPR`].
    pub fn into_owned(self) -> MPR {
        MPR {
            test_num: self.test_num,
            head_num: self.head_num,
            site_num: self.site_num,
            test_flg: self.test_flg,
            parm_flg: self.parm_flg,
            rtn_icnt: self.rtn_icnt,
            rslt_cnt: self.rslt_cnt,
            rtn_stat: self.rtn_stat,
            rtn_rslt: self.rtn_rslt,
            test_txt: self.test_txt.into_owned(),
            alarm_id: self.alarm_id.into_owned(),
            opt_flag: self.opt_flag,
            res_scal: self.res_scal,
            llm_scal: self.llm_scal,
            hlm_scal: self.hlm_scal,
            lo_limit: self.lo_limit,
            hi_limit: self.hi_limit,
            start_in: self.start_in,
            incr_in: self.incr_in,
            rtn_indx: self.rtn_indx,
            units: self.units.map(Cow::into_owned),
            units_in: self.units_in.map(Cow::into_owned),
            c_resfmt: self.c_resfmt.map(Cow::into_owned),
            c_llmfmt: self.c_llmfmt.map(Cow::into_owned),
            c_hlmfmt: self.c_hlmfmt.map(Cow::into_owned),
            lo_spec: self.lo_spec,
            hi_spec: self.hi_spec,
        }
    }
}

impl FTRView<'_> {
    /// Allocate the borrowed strings into an owned [`FTR`].
    pub fn into_owned(self) -> FTR {
        FTR {
            test_num: self.test_num,
            head_num: self.head_num,
            site_num: self.site_num,
            test_flg: self.test_flg,
            opt_flag: self.opt_flag,
            cycl_cnt: self.cycl_cnt,
            rel_vadr: self.rel_vadr,
            rept_cnt: self.rept_cnt,
            num_fail: self.num_fail,
            xfail_ad: self.xfail_ad,
            yfail_ad: self.yfail_ad,
            vect_off: self.vect_off,
            rtn_icnt: self.rtn_icnt,
            pgm_icnt: self.pgm_icnt,
            rtn_indx: self.rtn_indx,
            rtn_stat: self.rtn_stat,
            pgm_indx: self.pgm_indx,
            pgm_stat: self.pgm_stat,
            fail_pin: self.fail_pin,
            vect_nam: self.vect_nam.into_owned(),
            time_set: self.time_set.into_owned(),
            op_code: self.op_code.into_owned(),
            test_txt: self.test_txt.into_owned(),
            alarm_id: self.alarm_id.into_owned(),
            prog_txt: self.prog_txt.into_owned(),
            rslt_txt: self.rslt_txt.into_owned(),
            patg_num: self.patg_num,
            spin_map: self.spin_map,
        }
    }
}

impl STRView<'_> {
    /// Allocate the borrowed strings into an owned [`STR`].
    pub fn into_owned(self) -> STR {
        STR {
            cont_flg: self.cont_flg,
            test_num: self.test_num,
            head_num: self.head_num,
            site_num: self.site_num,
            psr_ref: self.psr_ref,
            test_flg: self.test_flg,
            log_typ: self.log_typ.into_owned(),
            test_txt: self.test_txt.into_owned(),
            alarm_id: self.alarm_id.into_owned(),
            prog_txt: self.prog_txt.into_owned(),
            rslt_txt: self.rslt_txt.into_owned(),
            z_val: self.z_val,
            fmu_flg: self.fmu_flg,
            mask_map: self.mask_map,
            fal_map: self.fal_map,
            cyc_cnt_t: self.cyc_cnt_t,
            totf_cnt: self.totf_cnt,
            totl_cnt: self.totl_cnt,
            cyc_base: self.cyc_base,
            bit_base: self.bit_base,
            cond_cnt: self.cond_cnt,
            lim_cnt: self.lim_cnt,
            cyc_size: self.cyc_size,
            pmr_size: self.pmr_size,
            chn_size: self.chn_size,
            pat_size: self.pat_size,
            bit_size: self.bit_size,
            u1_size: self.u1_size,
            u2_size: self.u2_size,
            u3_size: self.u3_size,
            utx_size: self.utx_size,
            cap_bgn: self.cap_bgn,
            lim_indx: self.lim_indx,
            lim_spec: self.lim_spec,
            cond_lst: self.cond_lst,
            cyc_cnt: self.cyc_cnt,
            cyc_ofst: self.cyc_ofst,
            pmr_cnt: self.pmr_cnt,
            pmr_indx: self.pmr_indx,
            chn_cnt: self.chn_cnt,
            chn_num: self.chn_num,
            exp_cnt: self.exp_cnt,
            exp_data: self.exp_data,
            cap_cnt: self.cap_cnt,
            cap_data: self.cap_data,
            new_cnt: self.new_cnt,
            new_data: self.new_data,
            pat_cnt: self.pat_cnt,
            pat_num: self.pat_num,
            bpos_cnt: self.bpos_cnt,
            bit_pos: self.bit_pos,
            usr1_cnt: self.usr1_cnt,
            usr1: self.usr1,
            usr2_cnt: self.usr2_cnt,
            usr2: self.usr2,
            usr3_cnt: self.usr3_cnt,
            usr3: self.usr3,
            txt_cnt: self.txt_cnt,
            user_txt: self.user_txt,
        }
    }
}

impl BPSView<'_> {
    /// Allocate the borrowed strings into an owned [`BPS`].
    pub fn into_owned(self) -> BPS {
        BPS {
            seq_name: self.seq_name.into_owned(),
        }
    }
}

impl DTRView<'_> {
    /// Allocate the borrowed strings into an owned [`DTR`].
    pub fn into_owned(self) -> DTR {
        DTR {
            text_dat: self.text_dat.into_owned(),
        }
    }
}

// RecordView: borrowing counterpart to StdfRecord.

/// `RecordView<'a>` is the borrowing counterpart to [`StdfRecord`]. It mirrors
/// the same variants but keeps scalar Cn/Sn strings as `Cow<'a, str>`: a
/// pure-ASCII payload is borrowed from the record bytes, while non-ASCII bytes
/// are decoded as Latin-1 into an owned `String`, matching the owned parse
/// path exactly.
///
/// Records that have no scalar Cn/Sn fields (`FAR`, `PCR`, `PLR`, `RDR`,
/// `NMR`, `WCR`, `PIR`, `EPS`, `GDR`, `ReservedRec`) have nothing to borrow, so
/// their variants wrap the owned struct directly.
///
/// Obtain one with [`RawDataElement::view`], then either inspect it in place or
/// call [`RecordView::into_owned`] to materialize a fully owned [`StdfRecord`].
/// A view is for inspection or conversion only; for serialization or long-term
/// storage convert to the owned record first.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordView<'a> {
    // rec type 0
    FAR(FAR),
    ATR(ATRView<'a>),
    VUR(VURView<'a>),
    // rec type 1
    MIR(MIRView<'a>),
    MRR(MRRView<'a>),
    PCR(PCR),
    HBR(HBRView<'a>),
    SBR(SBRView<'a>),
    PMR(PMRView<'a>),
    PGR(PGRView<'a>),
    PLR(PLR),
    RDR(RDR),
    SDR(SDRView<'a>),
    PSR(PSRView<'a>),
    NMR(NMR),
    CNR(CNRView<'a>),
    SSR(SSRView<'a>),
    CDR(CDRView<'a>),
    // rec type 2
    WIR(WIRView<'a>),
    WRR(WRRView<'a>),
    WCR(WCR),
    // rec type 5
    PIR(PIR),
    PRR(PRRView<'a>),
    // rec type 10
    TSR(TSRView<'a>),
    // rec type 15
    PTR(PTRView<'a>),
    MPR(MPRView<'a>),
    FTR(FTRView<'a>),
    STR(STRView<'a>),
    // rec type 20
    BPS(BPSView<'a>),
    EPS(EPS),
    // rec type 50
    GDR(GDR),
    DTR(DTRView<'a>),
    // rec type 180 / 181: Reserved
    ReservedRec(ReservedRec),
    InvalidRec(RecordHeader),
}

impl<'a> RecordView<'a> {
    /// Parse the field data (without header) of a record into a borrowing view.
    /// `header` selects the variant; `raw_data` is borrowed by the result.
    #[inline]
    pub fn from_bytes(
        header: RecordHeader,
        raw_data: &'a [u8],
        order: &ByteOrder,
    ) -> RecordView<'a> {
        match (header.typ, header.sub) {
            // rec type 15
            (15, 10) => RecordView::PTR(parse_ptr_view(raw_data, order)),
            (15, 15) => RecordView::MPR(parse_mpr_view(raw_data, order)),
            (15, 20) => RecordView::FTR(parse_ftr_view(raw_data, order)),
            (15, 30) => RecordView::STR(parse_str_view(raw_data, order)),
            // rec type 5
            (5, 10) => RecordView::PIR(parse_pir(raw_data, order)),
            (5, 20) => RecordView::PRR(parse_prr_view(raw_data, order)),
            // rec type 2
            (2, 10) => RecordView::WIR(parse_wir_view(raw_data, order)),
            (2, 20) => RecordView::WRR(parse_wrr_view(raw_data, order)),
            (2, 30) => RecordView::WCR(parse_wcr(raw_data, order)),
            // rec type 50
            (50, 10) => RecordView::GDR(parse_gdr(raw_data, order)),
            (50, 30) => RecordView::DTR(parse_dtr_view(raw_data, order)),
            // rec type 0
            (0, 10) => RecordView::FAR(parse_far(raw_data, order)),
            (0, 20) => RecordView::ATR(parse_atr_view(raw_data, order)),
            (0, 30) => RecordView::VUR(parse_vur_view(raw_data, order)),
            // rec type 1
            (1, 10) => RecordView::MIR(parse_mir_view(raw_data, order)),
            (1, 20) => RecordView::MRR(parse_mrr_view(raw_data, order)),
            (1, 30) => RecordView::PCR(parse_pcr(raw_data, order)),
            (1, 40) => RecordView::HBR(parse_hbr_view(raw_data, order)),
            (1, 50) => RecordView::SBR(parse_sbr_view(raw_data, order)),
            (1, 60) => RecordView::PMR(parse_pmr_view(raw_data, order)),
            (1, 62) => RecordView::PGR(parse_pgr_view(raw_data, order)),
            (1, 63) => RecordView::PLR(parse_plr(raw_data, order)),
            (1, 70) => RecordView::RDR(parse_rdr(raw_data, order)),
            (1, 80) => RecordView::SDR(parse_sdr_view(raw_data, order)),
            (1, 90) => RecordView::PSR(parse_psr_view(raw_data, order)),
            (1, 91) => RecordView::NMR(parse_nmr(raw_data, order)),
            (1, 92) => RecordView::CNR(parse_cnr_view(raw_data, order)),
            (1, 93) => RecordView::SSR(parse_ssr_view(raw_data, order)),
            (1, 94) => RecordView::CDR(parse_cdr_view(raw_data, order)),
            // rec type 10
            (10, 30) => RecordView::TSR(parse_tsr_view(raw_data, order)),
            // rec type 20
            (20, 10) => RecordView::BPS(parse_bps_view(raw_data, order)),
            (20, 20) => RecordView::EPS(EPS::new()),
            // rec type 180 / 181: Reserved
            (180 | 181, _) => RecordView::ReservedRec(parse_reserved(raw_data, order)),
            // not matched
            _ => RecordView::InvalidRec(header),
        }
    }

    /// Materialize a fully owned [`StdfRecord`], allocating the strings that
    /// were borrowed. The result is identical to parsing the same bytes with
    /// the owned `read_from_bytes` path.
    pub fn into_owned(self) -> StdfRecord {
        match self {
            RecordView::FAR(r) => StdfRecord::FAR(r),
            RecordView::ATR(r) => StdfRecord::ATR(r.into_owned()),
            RecordView::VUR(r) => StdfRecord::VUR(r.into_owned()),
            RecordView::MIR(r) => StdfRecord::MIR(r.into_owned()),
            RecordView::MRR(r) => StdfRecord::MRR(r.into_owned()),
            RecordView::PCR(r) => StdfRecord::PCR(r),
            RecordView::HBR(r) => StdfRecord::HBR(r.into_owned()),
            RecordView::SBR(r) => StdfRecord::SBR(r.into_owned()),
            RecordView::PMR(r) => StdfRecord::PMR(r.into_owned()),
            RecordView::PGR(r) => StdfRecord::PGR(r.into_owned()),
            RecordView::PLR(r) => StdfRecord::PLR(r),
            RecordView::RDR(r) => StdfRecord::RDR(r),
            RecordView::SDR(r) => StdfRecord::SDR(r.into_owned()),
            RecordView::PSR(r) => StdfRecord::PSR(r.into_owned()),
            RecordView::NMR(r) => StdfRecord::NMR(r),
            RecordView::CNR(r) => StdfRecord::CNR(r.into_owned()),
            RecordView::SSR(r) => StdfRecord::SSR(r.into_owned()),
            RecordView::CDR(r) => StdfRecord::CDR(r.into_owned()),
            RecordView::WIR(r) => StdfRecord::WIR(r.into_owned()),
            RecordView::WRR(r) => StdfRecord::WRR(r.into_owned()),
            RecordView::WCR(r) => StdfRecord::WCR(r),
            RecordView::PIR(r) => StdfRecord::PIR(r),
            RecordView::PRR(r) => StdfRecord::PRR(r.into_owned()),
            RecordView::TSR(r) => StdfRecord::TSR(r.into_owned()),
            RecordView::PTR(r) => StdfRecord::PTR(r.into_owned()),
            RecordView::MPR(r) => StdfRecord::MPR(r.into_owned()),
            RecordView::FTR(r) => StdfRecord::FTR(r.into_owned()),
            RecordView::STR(r) => StdfRecord::STR(r.into_owned()),
            RecordView::BPS(r) => StdfRecord::BPS(r.into_owned()),
            RecordView::EPS(r) => StdfRecord::EPS(r),
            RecordView::GDR(r) => StdfRecord::GDR(r),
            RecordView::DTR(r) => StdfRecord::DTR(r.into_owned()),
            RecordView::ReservedRec(r) => StdfRecord::ReservedRec(r),
            RecordView::InvalidRec(h) => StdfRecord::InvalidRec(h),
        }
    }
}

impl RawDataElement {
    /// Borrow the field data and parse it into a [`RecordView`], borrowing
    /// scalar strings directly from `self.raw_data` when they are ASCII.
    ///
    /// ```
    /// use rust_stdf::{ByteOrder, RawDataElement, RecordHeader, RecordView};
    ///
    /// // a DTR record carrying the text "OK"
    /// let raw = RawDataElement {
    ///     offset: 0,
    ///     header: RecordHeader { typ: 50, sub: 30, len: 3 },
    ///     raw_data: vec![2, b'O', b'K'],
    ///     byte_order: ByteOrder::LittleEndian,
    /// };
    /// if let RecordView::DTR(dtr) = raw.view() {
    ///     assert_eq!(&*dtr.text_dat, "OK"); // borrowed from raw_data, no allocation
    ///     let owned = dtr.into_owned(); // materialize an owned DTR
    ///     assert_eq!(owned.text_dat, "OK");
    /// }
    /// ```
    #[inline]
    pub fn view(&self) -> RecordView<'_> {
        RecordView::from_bytes(self.header, &self.raw_data, &self.byte_order)
    }
}
