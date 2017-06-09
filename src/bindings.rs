#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::os::raw::{c_ulong, c_long, c_char, c_int};
pub type mp_limb_t = c_ulong;
pub type mp_limb_signed_t = c_long;
pub type fmpz = mp_limb_signed_t;
pub type fmpz_t = [fmpz; 1usize];
pub type fmpzmutptr = *mut fmpz;
pub type fmpzptr = *const fmpz;

#[repr(C)]
pub struct fmpz_factor_struct {
    pub sign: c_int,
    pub p: *mut fmpz,
    pub exp: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
}

extern "C" {
    pub fn fmpz_add(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_add_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_sub(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_sub_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_mul(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_get_str(str: *mut c_char, b: c_int, f: fmpzptr) -> *const c_char;
    pub fn fmpz_sizeinbase(f: fmpzptr, b: c_int) -> usize;
    pub fn fmpz_pow_ui(f: fmpzmutptr, g: fmpzptr, exp: mp_limb_t);
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: fmpzptr);
    pub fn fmpz_factor_si(factor: *mut fmpz_factor_struct, n: mp_limb_signed_t);
    pub fn fmpz_set_str(f: fmpzmutptr, str: *const c_char, b: c_int) -> c_int;
    pub fn fmpz_mul_si(f: fmpzmutptr, g: fmpzptr, x: mp_limb_signed_t);
    pub fn fmpz_mul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_addmul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_submul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_addmul(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_submul(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_factor_expand_iterative(n: fmpzmutptr, factor: *const fmpz_factor_struct);
    pub fn fmpz_fdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_tdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_cdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
}

#[link(name = "fmpz_wrapper")]
extern "C" {
    pub fn wrapped_fmpz_clear(f: fmpzmutptr);
    pub fn wrapped_fmpz_init(f: fmpzmutptr);
    pub fn wrapped_fmpz_init_set(f: fmpzmutptr, g: fmpzptr);
    pub fn wrapped_fmpz_init_set_si(f: fmpzmutptr, g: mp_limb_signed_t);
    pub fn wrapped_fmpz_init_set_ui(f: fmpzmutptr, g: mp_limb_t);
    pub fn wrapped_fmpz_set_si(f: fmpzmutptr, val: mp_limb_signed_t);
    pub fn wrapped_fmpz_set_ui(f: fmpzmutptr, val: mp_limb_t);
    pub fn bench_square_sum_native(n: c_ulong);
}
