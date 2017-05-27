#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::os::raw::{c_ulong, c_long, c_char, c_int};
pub type mp_limb_t = c_ulong;
pub type mp_limb_signed_t = c_long;
pub type fmpz = mp_limb_signed_t;
pub type fmpz_t = [fmpz; 1usize];
pub type fmpzptr = *mut fmpz;
pub type fmpzconstptr = *const fmpz;

#[repr(C)]
pub struct fmpz_factor_struct {
    pub sign: c_int,
    pub p: *mut fmpz,
    pub exp: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
}

#[link(name = "flint_wrapper")]
extern "C" {
    pub fn fmpz_add(f: fmpzptr, g: fmpzconstptr, h: fmpzconstptr);
    pub fn wrapped_fmpz_clear(f: fmpzptr);
    pub fn wrapped_fmpz_init(f: fmpzptr);
    pub fn wrapped_fmpz_init_set(f: fmpzptr, g: fmpzconstptr);
    pub fn warpped_fmpz_init_set_si(f: fmpzptr, g: mp_limb_signed_t);
    pub fn wrapped_fmpz_set_si(f: fmpzptr, val: mp_limb_signed_t);
    pub fn wrapped_fmpz_set_ui(f: fmpzptr, val: mp_limb_t);
    pub fn fmpz_mul(f: fmpzptr, g: fmpzconstptr, h: fmpzconstptr);
    pub fn fmpz_get_str(str: *mut c_char, b: c_int, f: fmpzconstptr) -> *const c_char;
    pub fn fmpz_sizeinbase(f: fmpzconstptr, b: c_int) -> usize;
    pub fn fmpz_pow_ui(f: fmpzptr, g: fmpzconstptr, exp: mp_limb_t);
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: fmpzconstptr);
}
