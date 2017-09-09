#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate libc;
extern crate gmp;

use self::gmp::mpq::mpq_struct;
use self::gmp::mpz::{mpz_struct, mp_limb_t};
use self::libc::{c_long, c_char, c_int, c_void, c_double, c_uint};
// TODO: Make sure this is correct.
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


pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;

// Rand
#[repr(C)]
struct __mpz_struct {
    _mp_alloc: c_int,
    _mp_size: c_int,
    _mp_d: *mut mp_limb_t,
}


type mpz_t = [__mpz_struct; 1usize];

#[repr(u32)]
enum gmp_randalg_t {
    GMP_RAND_ALG_DEFAULT = 0,
}

#[repr(C)]
union __gmp_randstate_struct__bindgen_ty_1 {
    _mp_lc: *mut c_void,
}

#[repr(C)]
struct __gmp_randstate_struct {
    _mp_seed: mpz_t,
    _mp_alg: gmp_randalg_t,
    _mp_algdata: __gmp_randstate_struct__bindgen_ty_1,
}

type gmp_randstate_t = [__gmp_randstate_struct; 1usize];

#[repr(C)]
pub struct flint_rand_s {
    gmp_state: gmp_randstate_t,
    gmp_init: c_int,
    __randval: mp_limb_t,
    __randval2: mp_limb_t,
}

// preinvn
#[repr(C)]
pub struct fmpz_preinvn_struct {
    dinv: mp_ptr,
    n: mp_limb_signed_t,
    norm: mp_limb_t,
}

// CRT
#[repr(C)]
#[derive(Debug, Clone)]
pub struct nmod_t {
    n: mp_limb_t,
    ninv: mp_limb_t,
    norm: mp_limb_t,
}

#[repr(C)]
pub struct fmpz_comb_struct {
    primes: *const mp_limb_t,
    num_primes: mp_limb_signed_t,
    n: mp_limb_signed_t,
    comb: *mut *mut fmpz,
    res: *mut *mut fmpz,
    mod_: *mut nmod_t,
}

#[repr(C)]
pub struct fmpz_comb_temp_struct {
    n: mp_limb_signed_t,
    comb_temp: *mut *mut fmpz,
    temp: fmpz_t,
    temp2: fmpz_t,
}

// fmpq
#[repr(C)]
#[derive(Debug, Clone)]
pub struct fmpq {
    pub num: fmpz,
    pub den: fmpz,
}

pub type fmpq_t = [fmpq; 1usize];
extern "C" {
    pub fn _fmpq_cmp(
        p: *mut fmpz,
        q: *mut fmpz,
        r: *mut fmpz,
        s: *mut fmpz,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    #[link_name="wrapped_fmpq_clear"]
    pub fn fmpq_clear(x: *mut fmpq);

    #[link_name="wrapped_fmpq_init"]
    pub fn fmpq_init(x: *mut fmpq);

    #[link_name="wrapped_fmpq_zero"]
    pub fn fmpq_zero(x: *mut fmpq);

    #[link_name="wrapped_fmpq_one"]
    pub fn fmpq_one(x: *mut fmpq);

    #[link_name="wrapped_fmpq_equal"]
    pub fn fmpq_equal(x: *const fmpq, y: *const fmpq) -> c_int;

    #[link_name="wrapped_fmpq_sgn"]
    pub fn fmpq_sgn(x: *const fmpq) -> c_int;

    #[link_name="wrapped_fmpq_is_zero"]
    pub fn fmpq_is_zero(x: *const fmpq) -> c_int;

    #[link_name="wrapped_fmpq_is_one"]
    pub fn fmpq_is_one(x: *const fmpq) -> c_int;

    #[link_name="wrapped_fmpq_set"]
    pub fn fmpq_set(x: *mut fmpq, y: *const fmpq);

    #[link_name="wrapped_fmpq_neg"]
    pub fn fmpq_neg(x: *mut fmpq, y: *const fmpq);

    #[link_name="wrapped_fmpq_abs"]
    pub fn fmpq_abs(x: *mut fmpq, y: *const fmpq);

    pub fn fmpq_cmp(x: *const fmpq, y: *const fmpq) -> c_int;

    pub fn _fmpq_canonicalise(num: *mut fmpz, den: *mut fmpz);

    pub fn fmpq_canonicalise(res: *mut fmpq);

    pub fn _fmpq_is_canonical(num: *mut fmpz, den: *mut fmpz) -> c_int;

    pub fn fmpq_is_canonical(x: *const fmpq) -> c_int;

    pub fn _fmpq_set_si(rnum: *mut fmpz, rden: *mut fmpz, p: mp_limb_signed_t, q: mp_limb_t);

    pub fn fmpq_set_si(res: *mut fmpq, p: mp_limb_signed_t, q: mp_limb_t);

    pub fn fmpq_set_fmpz_frac(res: *mut fmpq, p: *const fmpz, q: *const fmpz);

    pub fn flint_mpq_init_set_readonly(z: *mut mpq_struct, f: *mut fmpq);

    pub fn flint_mpq_clear_readonly(z: *mut mpq_struct);

    pub fn fmpq_init_set_readonly(f: *mut fmpq, z: *mut mpq_struct);

    pub fn fmpq_clear_readonly(f: *mut fmpq);

    pub fn _fmpq_get_str(str: *mut c_char, b: c_int, num: *mut fmpz, den: *mut fmpz)
        -> *mut c_char;

    pub fn fmpq_get_str(str: *mut c_char, b: c_int, x: *mut fmpq) -> *mut c_char;

    pub fn _fmpq_randtest(
        num: *mut fmpz,
        den: *mut fmpz,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );

    pub fn fmpq_randtest(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);

    pub fn fmpq_randtest_not_zero(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);

    pub fn _fmpq_randbits(
        num: *mut fmpz,
        den: *mut fmpz,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );

    pub fn fmpq_randbits(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);

    pub fn _fmpq_add(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *mut fmpz,
        op1den: *mut fmpz,
        op2num: *mut fmpz,
        op2den: *mut fmpz,
    );

    pub fn fmpq_add(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn _fmpq_add_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *mut fmpz,
        q: *mut fmpz,
        r: mp_limb_signed_t,
    );

    pub fn fmpq_add_si(res: *mut fmpq, op1: *const fmpq, c: mp_limb_signed_t);

    pub fn _fmpq_add_fmpz(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *mut fmpz,
        q: *mut fmpz,
        r: *mut fmpz,
    );

    pub fn fmpq_add_fmpz(res: *mut fmpq, op1: *const fmpq, c: *const fmpz);

    pub fn _fmpq_sub(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );

    pub fn fmpq_sub(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn _fmpq_sub_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *mut fmpz,
        q: *mut fmpz,
        r: mp_limb_signed_t,
    );

    pub fn fmpq_sub_si(res: *mut fmpq, op1: *const fmpq, c: mp_limb_signed_t);

    pub fn _fmpq_sub_fmpz(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *mut fmpz,
        q: *mut fmpz,
        r: *mut fmpz,
    );

    pub fn fmpq_sub_fmpz(res: *mut fmpq, op1: *const fmpq, c: *const fmpz);

    pub fn _fmpq_mul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );

    pub fn fmpq_mul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn fmpq_mul_fmpz(res: *mut fmpq, op: *const fmpq, x: *const fmpz);

    pub fn _fmpq_pow_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        opnum: *const fmpz,
        opden: *const fmpz,
        e: mp_limb_signed_t,
    );

    pub fn fmpq_pow_si(rop: *mut fmpq, op: *const fmpq, e: mp_limb_signed_t);

    pub fn _fmpq_addmul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );

    pub fn fmpq_addmul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn _fmpq_submul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );

    pub fn fmpq_submul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn fmpq_inv(dest: *mut fmpq, src: *const fmpq);

    pub fn fmpq_div(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);

    pub fn fmpq_div_fmpz(res: *mut fmpq, op: *const fmpq, x: *const fmpz);

    pub fn fmpq_mul_2exp(res: *mut fmpq, x: *const fmpq, exp: mp_limb_t);

    pub fn fmpq_div_2exp(res: *mut fmpq, x: *const fmpq, exp: mp_limb_t);

    pub fn _fmpq_mod_fmpz(res: *mut fmpz, num: *mut fmpz, den: *mut fmpz, mod_: *mut fmpz)
        -> c_int;

    pub fn fmpq_mod_fmpz(res: *mut fmpz, x: *mut fmpq, mod_: *mut fmpz) -> c_int;

    pub fn _fmpq_reconstruct_fmpz(
        num: *mut fmpz,
        den: *mut fmpz,
        a: *mut fmpz,
        m: *mut fmpz,
    ) -> c_int;

    pub fn fmpq_reconstruct_fmpz(res: *mut fmpq, a: *mut fmpz, m: *mut fmpz) -> c_int;

    pub fn _fmpq_reconstruct_fmpz_2(
        n: *mut fmpz,
        d: *mut fmpz,
        a: *mut fmpz,
        m: *mut fmpz,
        N: *mut fmpz,
        D: *mut fmpz,
    ) -> c_int;

    pub fn fmpq_reconstruct_fmpz_2(
        res: *mut fmpq,
        a: *mut fmpz,
        m: *mut fmpz,
        N: *mut fmpz,
        D: *mut fmpz,
    ) -> c_int;

    pub fn fmpq_height_bits(x: *const fmpq) -> mp_limb_t;

    pub fn fmpq_height(height: *mut fmpz, x: *const fmpq);

    pub fn _fmpq_next_calkin_wilf(rnum: *mut fmpz, rden: *mut fmpz, num: *mut fmpz, den: *mut fmpz);

    pub fn fmpq_next_calkin_wilf(res: *mut fmpq, x: *mut fmpq);

    pub fn _fmpq_next_signed_calkin_wilf(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        num: *mut fmpz,
        den: *mut fmpz,
    );

    pub fn fmpq_next_signed_calkin_wilf(res: *mut fmpq, x: *mut fmpq);

    pub fn _fmpq_next_minimal(rnum: *mut fmpz, rden: *mut fmpz, num: *mut fmpz, den: *mut fmpz);

    pub fn fmpq_next_minimal(res: *mut fmpq, x: *mut fmpq);

    pub fn _fmpq_next_signed_minimal(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        num: *mut fmpz,
        den: *mut fmpz,
    );

    pub fn fmpq_next_signed_minimal(res: *mut fmpq, x: *mut fmpq);

    pub fn fmpq_get_cfrac(
        c: *mut fmpz,
        rem: *mut fmpq,
        x: *const fmpq,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpq_set_cfrac(x: *mut fmpq, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpq_cfrac_bound(x: *mut fmpq) -> mp_limb_signed_t;
    pub fn fmpq_dedekind_sum_naive(s: *mut fmpq, h: *mut fmpz, k: *mut fmpz);

    pub fn fmpq_dedekind_sum_coprime_large(s: *mut fmpq, h: *mut fmpz, k: *mut fmpz);

    pub fn fmpq_dedekind_sum_coprime_d(h: f64, k: f64) -> f64;

    pub fn fmpq_dedekind_sum_coprime(s: *mut fmpq, h: *mut fmpz, k: *mut fmpz);

    pub fn fmpq_dedekind_sum(s: *mut fmpq, h: *mut fmpz, k: *mut fmpz);

    pub fn _fmpq_harmonic_ui(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);

    pub fn fmpq_harmonic_ui(x: *mut fmpq, n: mp_limb_t);
}

// fmpz_poly
#[repr(C)]
#[derive(Debug, Clone)]
pub struct fmpz_poly_struct {
    pub coeffs: *mut fmpz,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type fmpz_poly_t = [fmpz_poly_struct; 1usize];
#[repr(C)]
#[derive(Debug, Clone)]
pub struct fmpz_poly_powers_precomp_struct {
    pub powers: *mut *mut fmpz,
    pub len: mp_limb_signed_t,
}
pub type fmpz_poly_powers_precomp_t = [fmpz_poly_powers_precomp_struct; 1usize];
#[repr(C)]
#[derive(Debug, Clone)]
pub struct fmpz_poly_factor_struct {
    pub c: fmpz,
    pub p: *mut fmpz_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct nmod_poly_struct {
    pub coeffs: mp_ptr,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub mod_: nmod_t,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct nmod_poly_factor_struct {
    pub p: *mut nmod_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}


pub type fmpz_poly_factor_t = [fmpz_poly_factor_struct; 1usize];
extern "C" {
    pub fn fmpz_poly_init(poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_init2(poly: *mut fmpz_poly_struct, alloc: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_realloc(poly: *mut fmpz_poly_struct, alloc: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_fit_length(poly: *mut fmpz_poly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_clear(poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_normalise(poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_set(poly1: *mut fmpz_poly_struct, poly2: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_set_ui(poly: *mut fmpz_poly_struct, c: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_set_si(poly: *mut fmpz_poly_struct, c: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_set_fmpz(poly: *mut fmpz_poly_struct, c: *mut fmpz);
}
extern "C" {
    pub fn fmpz_poly_set_mpz(poly: *mut fmpz_poly_struct, c: *mut mpz_struct);
}
extern "C" {
    pub fn _fmpz_poly_set_str(
        poly: *mut fmpz,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_set_str(
        poly: *mut fmpz_poly_struct,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_get_str(
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmpz_poly_get_str(poly: *mut fmpz_poly_struct) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fmpz_poly_get_str_pretty(
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmpz_poly_get_str_pretty(
        poly: *mut fmpz_poly_struct,
        x: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmpz_poly_zero_coeffs(
        poly: *mut fmpz_poly_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_swap(poly1: *mut fmpz_poly_struct, poly2: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_reverse(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_reverse(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_set_trunc(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_randtest(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_randtest_unsigned(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_randtest_not_zero(
        f: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_get_coeff_si(
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_poly_set_coeff_si(
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_get_coeff_ui(poly: *mut fmpz_poly_struct, n: mp_limb_signed_t) -> mp_limb_t;
}
extern "C" {
    pub fn fmpz_poly_set_coeff_ui(poly: *mut fmpz_poly_struct, n: mp_limb_signed_t, x: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_set_coeff_fmpz(poly: *mut fmpz_poly_struct, n: mp_limb_signed_t, x: *mut fmpz);
}
extern "C" {
    pub fn fmpz_poly_get_coeff_fmpz(x: *mut fmpz, poly: *mut fmpz_poly_struct, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_equal(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_equal_trunc(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_add(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_add(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_add_series(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sub(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_sub(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_sub_series(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_neg(res: *mut fmpz_poly_struct, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_scalar_mul_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_mul_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_mul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_addmul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_submul_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_fdiv_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_fdiv_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_fdiv_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_tdiv_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_tdiv_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_tdiv_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_divexact_ui(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_divexact_si(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_divexact_fmpz(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        x: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_fdiv_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        exp: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_tdiv_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        exp: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_scalar_mul_2exp(
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        exp: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_bit_pack(
        arr: mp_ptr,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        bit_size: mp_limb_t,
        negate: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _fmpz_poly_bit_unpack(
        poly: *mut fmpz,
        len: mp_limb_signed_t,
        arr: mp_srcptr,
        bit_size: mp_limb_t,
        negate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_bit_unpack_unsigned(
        poly: *mut fmpz,
        len: mp_limb_signed_t,
        arr: mp_srcptr,
        bit_size: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_bit_pack(f: *mut fmpz, poly: *mut fmpz_poly_struct, bit_size: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_bit_unpack(poly: *mut fmpz_poly_struct, f: *mut fmpz, bit_size: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_bit_unpack_unsigned(
        poly: *mut fmpz_poly_struct,
        f: *mut fmpz,
        bit_size: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mul_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mul_classical(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_mullow_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mullow_classical(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mulhigh_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        start: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mulhigh_classical(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        start: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mulmid_classical(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mulmid_classical(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_mul_karatsuba(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_mul_karatsuba(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mullow_karatsuba_n(
        res: *mut fmpz,
        poly1: *const fmpz,
        poly2: *const fmpz,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mullow_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mulhigh_karatsuba_n(
        res: *mut fmpz,
        poly1: *const fmpz,
        poly2: *const fmpz,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mulhigh_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        length: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mul_KS(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mul_KS(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_mullow_KS(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mullow_KS(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mul_SS(
        output: *mut fmpz,
        input1: *const fmpz,
        length1: mp_limb_signed_t,
        input2: *const fmpz,
        length2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mul_SS(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_mullow_SS(
        output: *mut fmpz,
        input1: *const fmpz,
        length1: mp_limb_signed_t,
        input2: *const fmpz,
        length2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mullow_SS(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_mul(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mul(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_mullow(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mullow(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_mulhigh_n(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sqr_KS(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_sqr_KS(rop: *mut fmpz_poly_struct, op: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_sqr_karatsuba(rop: *mut fmpz_poly_struct, op: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_sqr_karatsuba(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn _fmpz_poly_sqr_classical(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_sqr_classical(rop: *mut fmpz_poly_struct, op: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_sqr(rop: *mut fmpz, op: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_sqr(rop: *mut fmpz_poly_struct, op: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_sqrlow_KS(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_sqrlow_KS(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sqrlow_karatsuba_n(res: *mut fmpz, poly: *const fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_sqrlow_karatsuba_n(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sqrlow_classical(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_sqrlow_classical(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sqrlow(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_sqrlow(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow_multinomial(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_pow_multinomial(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow_binomial(res: *mut fmpz, poly: *const fmpz, e: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_pow_binomial(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_pow_binexp(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow_addchains(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: *const ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn fmpz_poly_pow_addchains(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow_small(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_pow(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t, e: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_pow(res: *mut fmpz_poly_struct, poly: *mut fmpz_poly_struct, e: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_pow_trunc(
        res: *mut fmpz,
        poly: *const fmpz,
        e: mp_limb_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_pow_trunc(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        e: mp_limb_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_shift_left(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_shift_right(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_shift_left(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_shift_right(
        res: *mut fmpz_poly_struct,
        poly: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_2norm(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_2norm(res: *mut fmpz, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_2norm_normalised_bits(poly: *const fmpz, len: mp_limb_signed_t) -> mp_limb_t;
}
extern "C" {
    pub fn _fmpz_poly_gcd_subresultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_gcd_subresultant(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_gcd_heuristic(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_gcd_heuristic(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_gcd_modular(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_gcd_modular(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_gcd(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_gcd(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_lcm(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_lcm(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_resultant_modular(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_resultant_modular(
        res: *mut fmpz,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_resultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_resultant(
        res: *mut fmpz,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_xgcd_modular(
        r: *mut fmpz,
        s: *mut fmpz,
        t: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_xgcd_modular(
        r: *mut fmpz,
        s: *mut fmpz_poly_struct,
        t: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_discriminant(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_discriminant(res: *mut fmpz, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_content(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_content(res: *mut fmpz, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_primitive_part(res: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_primitive_part(res: *mut fmpz_poly_struct, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_is_squarefree(
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_is_squarefree(poly: *mut fmpz_poly_struct) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_divrem_basecase(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_divrem_divconquer_recursive(
        Q: *mut fmpz,
        BQ: *mut fmpz,
        W: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_divrem_divconquer(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_divrem(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_divrem(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_div_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_div_basecase(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_divremlow_divconquer_recursive(
        Q: *mut fmpz,
        QB: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_div_divconquer_recursive(
        Q: *mut fmpz,
        temp: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_div_divconquer(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_div_divconquer(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_div(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_div(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_preinvert(B_inv: *mut fmpz, B: *const fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_preinvert(B_inv: *mut fmpz_poly_struct, B: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_div_preinv(
        Q: *mut fmpz,
        A: *const fmpz,
        len1: mp_limb_signed_t,
        B: *const fmpz,
        B_inv: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_div_preinv(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
        B_inv: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_divrem_preinv(
        Q: *mut fmpz,
        A: *mut fmpz,
        len1: mp_limb_signed_t,
        B: *const fmpz,
        B_inv: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_divrem_preinv(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
        B_inv: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_powers_precompute(B: *const fmpz, len: mp_limb_signed_t) -> *mut *mut fmpz;
}
extern "C" {
    pub fn fmpz_poly_powers_precompute(
        pinv: *mut fmpz_poly_powers_precomp_struct,
        poly: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_powers_clear(powers: *mut *mut fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_powers_clear(pinv: *mut fmpz_poly_powers_precomp_struct);
}
extern "C" {
    pub fn _fmpz_poly_rem_powers_precomp(
        A: *mut fmpz,
        m: mp_limb_signed_t,
        B: *const fmpz,
        n: mp_limb_signed_t,
        powers: *const *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_rem_powers_precomp(
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
        B_inv: *mut fmpz_poly_powers_precomp_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_rem_basecase(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_rem_basecase(
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_rem(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_rem(
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_div_root(Q: *mut fmpz_poly_struct, A: *mut fmpz_poly_struct, c: *mut fmpz);
}
extern "C" {
    pub fn _fmpz_poly_div_root(Q: *mut fmpz, A: *const fmpz, len: mp_limb_signed_t, c: *mut fmpz);
}
extern "C" {
    pub fn _fmpz_poly_inv_series_basecase(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_inv_series_basecase(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_inv_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_inv_series_newton(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_inv_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_inv_series(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_div_series(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: mp_limb_signed_t,
        B: *const fmpz,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_div_series(
        Q: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_divides(
        q: *mut fmpz,
        a: *const fmpz,
        len1: mp_limb_signed_t,
        b: *const fmpz,
        len2: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_divides(
        q: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
        b: *mut fmpz_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_pseudo_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        d: *mut mp_limb_t,
        A: *const fmpz,
        A_len: mp_limb_signed_t,
        B: *const fmpz,
        B_len: mp_limb_signed_t,
        inv: *mut fmpz_preinvn_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_divrem_basecase(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        d: *mut mp_limb_t,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_pseudo_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        d: *mut mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *mut fmpz_preinvn_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_divrem_divconquer(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        d: *mut mp_limb_t,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_pseudo_divrem_cohen(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_divrem_cohen(
        Q: *mut fmpz_poly_struct,
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_pseudo_rem_cohen(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_rem_cohen(
        R: *mut fmpz_poly_struct,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_pseudo_div(
        Q: *mut fmpz,
        d: *mut mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *mut fmpz_preinvn_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_div(
        Q: *mut fmpz_poly_struct,
        d: *mut mp_limb_t,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_pseudo_rem(
        R: *mut fmpz,
        d: *mut mp_limb_t,
        A: *const fmpz,
        lenA: mp_limb_signed_t,
        B: *const fmpz,
        lenB: mp_limb_signed_t,
        inv: *mut fmpz_preinvn_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_pseudo_rem(
        R: *mut fmpz_poly_struct,
        d: *mut mp_limb_t,
        A: *mut fmpz_poly_struct,
        B: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_derivative(rpoly: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_derivative(res: *mut fmpz_poly_struct, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_evaluate_divconquer_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_divconquer_fmpz(
        res: *mut fmpz,
        poly: *mut fmpz_poly_struct,
        a: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_evaluate_horner_fmpz(
        res: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        a: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_horner_fmpz(res: *mut fmpz, f: *mut fmpz_poly_struct, a: *mut fmpz);
}
extern "C" {
    pub fn _fmpz_poly_evaluate_fmpz(
        res: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        a: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_fmpz(res: *mut fmpz, f: *mut fmpz_poly_struct, a: *mut fmpz);
}
extern "C" {
    pub fn _fmpz_poly_evaluate_horner_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *mut fmpz,
        aden: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_horner_fmpq(res: *mut fmpq, f: *mut fmpz_poly_struct, a: *mut fmpq);
}
extern "C" {
    pub fn _fmpz_poly_evaluate_divconquer_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *mut fmpz,
        aden: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_divconquer_fmpq(
        res: *mut fmpq,
        f: *mut fmpz_poly_struct,
        a: *mut fmpq,
    );
}
extern "C" {
    pub fn _fmpz_poly_evaluate_fmpq(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        anum: *mut fmpz,
        aden: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_evaluate_fmpq(res: *mut fmpq, f: *mut fmpz_poly_struct, a: *mut fmpq);
}
extern "C" {
    pub fn fmpz_poly_evaluate_mpq(
        res: *mut mpq_struct,
        f: *mut fmpz_poly_struct,
        a: *mut mpq_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_evaluate_mod(
        poly: *const fmpz,
        len: mp_limb_signed_t,
        a: mp_limb_t,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn fmpz_poly_evaluate_mod(
        poly: *mut fmpz_poly_struct,
        a: mp_limb_t,
        n: mp_limb_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn _fmpz_poly_compose_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose_horner(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_compose_divconquer(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose_divconquer(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_compose(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_taylor_shift_horner(poly: *mut fmpz, c: *mut fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_taylor_shift_horner(
        g: *mut fmpz_poly_struct,
        f: *mut fmpz_poly_struct,
        c: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_taylor_shift_divconquer(poly: *mut fmpz, c: *mut fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_taylor_shift_divconquer(
        g: *mut fmpz_poly_struct,
        f: *mut fmpz_poly_struct,
        c: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_taylor_shift_multi_mod(poly: *mut fmpz, c: *mut fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_taylor_shift_multi_mod(
        g: *mut fmpz_poly_struct,
        f: *mut fmpz_poly_struct,
        c: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_taylor_shift(poly: *mut fmpz, c: *mut fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_taylor_shift(g: *mut fmpz_poly_struct, f: *mut fmpz_poly_struct, c: *mut fmpz);
}
extern "C" {
    pub fn _fmpz_poly_compose_series_brent_kung(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose_series_brent_kung(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_compose_series_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose_series_horner(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_compose_series(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_compose_series(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        poly2: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_revert_series_lagrange(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_revert_series_lagrange(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_revert_series_lagrange_fast(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_revert_series_lagrange_fast(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_revert_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_revert_series_newton(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_revert_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_revert_series(
        Qinv: *mut fmpz_poly_struct,
        Q: *mut fmpz_poly_struct,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_sqrt_classical(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_sqrt_classical(
        b: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_sqrt(
        res: *mut fmpz,
        poly: *const fmpz,
        len: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fmpz_poly_sqrt(
        b: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fmpz_poly_signature(
        r1: *mut mp_limb_signed_t,
        r2: *mut mp_limb_signed_t,
        poly: *const fmpz,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_signature(
        r1: *mut mp_limb_signed_t,
        r2: *mut mp_limb_signed_t,
        poly: *mut fmpz_poly_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_get_nmod_poly(res: *mut nmod_poly_struct, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_set_nmod_poly(res: *mut fmpz_poly_struct, poly: *mut nmod_poly_struct);
}
extern "C" {
    pub fn fmpz_poly_set_nmod_poly_unsigned(
        res: *mut fmpz_poly_struct,
        poly: *mut nmod_poly_struct,
    );
}
extern "C" {
    pub fn _fmpz_poly_CRT_ui_precomp(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        m1: *mut fmpz,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        m2: mp_limb_t,
        m2inv: mp_limb_t,
        m1m2: *mut fmpz,
        c: mp_limb_t,
        sign: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _fmpz_poly_CRT_ui(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: mp_limb_signed_t,
        m1: *mut fmpz,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        m2: mp_limb_t,
        m2inv: mp_limb_t,
        sign: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn fmpz_poly_CRT_ui(
        res: *mut fmpz_poly_struct,
        poly1: *mut fmpz_poly_struct,
        m1: *mut fmpz,
        poly2: *mut nmod_poly_struct,
        sign: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn _fmpz_poly_product_roots_fmpz_vec(poly: *mut fmpz, xs: *const fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_product_roots_fmpz_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpz,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_monomial_to_newton(poly: *mut fmpz, roots: *const fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn _fmpz_poly_newton_to_monomial(poly: *mut fmpz, roots: *const fmpz, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_evaluate_fmpz_vec(
        res: *mut fmpz,
        f: *mut fmpz_poly_struct,
        a: *const fmpz,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_interpolate_fmpz_vec(
        poly: *mut fmpz_poly_struct,
        xs: *const fmpz,
        ys: *const fmpz,
        n: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_build_tree(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        fac: *mut nmod_poly_factor_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_lift(
        Gout: *mut fmpz_poly_struct,
        Hout: *mut fmpz_poly_struct,
        Aout: *mut fmpz_poly_struct,
        Bout: *mut fmpz_poly_struct,
        f: *mut fmpz_poly_struct,
        g: *mut fmpz_poly_struct,
        h: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
        b: *mut fmpz_poly_struct,
        p: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_hensel_lift_without_inverse(
        G: *mut fmpz,
        H: *mut fmpz,
        f: *const fmpz,
        lenF: mp_limb_signed_t,
        g: *const fmpz,
        lenG: mp_limb_signed_t,
        h: *const fmpz,
        lenH: mp_limb_signed_t,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_lift_without_inverse(
        Gout: *mut fmpz_poly_struct,
        Hout: *mut fmpz_poly_struct,
        f: *mut fmpz_poly_struct,
        g: *mut fmpz_poly_struct,
        h: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
        b: *mut fmpz_poly_struct,
        p: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn _fmpz_poly_hensel_lift_only_inverse(
        A: *mut fmpz,
        B: *mut fmpz,
        G: *const fmpz,
        lenG: mp_limb_signed_t,
        H: *const fmpz,
        lenH: mp_limb_signed_t,
        a: *const fmpz,
        lenA: mp_limb_signed_t,
        b: *const fmpz,
        lenB: mp_limb_signed_t,
        p: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_lift_only_inverse(
        Aout: *mut fmpz_poly_struct,
        Bout: *mut fmpz_poly_struct,
        G: *mut fmpz_poly_struct,
        H: *mut fmpz_poly_struct,
        a: *mut fmpz_poly_struct,
        b: *mut fmpz_poly_struct,
        p: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_lift_tree_recursive(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        j: mp_limb_signed_t,
        inv: mp_limb_signed_t,
        p0: *mut fmpz,
        p1: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_poly_hensel_lift_tree(
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        r: mp_limb_signed_t,
        p: *mut fmpz,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        inv: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_hensel_start_lift(
        lifted_fac: *mut fmpz_poly_factor_struct,
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        local_fac: *mut nmod_poly_factor_struct,
        target_exp: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _fmpz_poly_hensel_continue_lift(
        lifted_fac: *mut fmpz_poly_factor_struct,
        link: *mut mp_limb_signed_t,
        v: *mut fmpz_poly_t,
        w: *mut fmpz_poly_t,
        f: *mut fmpz_poly_struct,
        prev: mp_limb_signed_t,
        curr: mp_limb_signed_t,
        N: mp_limb_signed_t,
        p: *mut fmpz,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_poly_hensel_lift_once(
        lifted_fac: *mut fmpz_poly_factor_struct,
        f: *mut fmpz_poly_struct,
        local_fac: *mut nmod_poly_factor_struct,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _fmpz_poly_bound_roots(bound: *mut fmpz, poly: *const fmpz, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_bound_roots(bound: *mut fmpz, poly: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_cyclotomic(
        a: *mut fmpz,
        n: mp_limb_t,
        factors: mp_ptr,
        num_factors: mp_limb_signed_t,
        phi: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_poly_cyclotomic(poly: *mut fmpz_poly_struct, n: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_cos_minpoly(f: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_cos_minpoly(f: *mut fmpz_poly_struct, n: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_swinnerton_dyer(T: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_swinnerton_dyer(poly: *mut fmpz_poly_struct, n: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_chebyshev_t(coeffs: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_chebyshev_t(poly: *mut fmpz_poly_struct, n: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_chebyshev_u(coeffs: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn fmpz_poly_chebyshev_u(poly: *mut fmpz_poly_struct, n: mp_limb_t);
}
extern "C" {
    pub fn _fmpz_poly_eta_qexp(f: *mut fmpz, e: mp_limb_signed_t, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_eta_qexp(f: *mut fmpz_poly_struct, e: mp_limb_signed_t, n: mp_limb_signed_t);
}
extern "C" {
    pub fn _fmpz_poly_theta_qexp(f: *mut fmpz, e: mp_limb_signed_t, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_theta_qexp(f: *mut fmpz_poly_struct, e: mp_limb_signed_t, n: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_factor_init(fac: *mut fmpz_poly_factor_struct);
}
extern "C" {
    pub fn fmpz_poly_factor_init2(fac: *mut fmpz_poly_factor_struct, alloc: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_factor_realloc(fac: *mut fmpz_poly_factor_struct, alloc: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_factor_fit_length(fac: *mut fmpz_poly_factor_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_poly_factor_clear(fac: *mut fmpz_poly_factor_struct);
}
extern "C" {
    pub fn fmpz_poly_factor_set(
        res: *mut fmpz_poly_factor_struct,
        fac: *mut fmpz_poly_factor_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_factor_insert(
        fac: *mut fmpz_poly_factor_struct,
        p: *mut fmpz_poly_struct,
        exp: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_factor_concat(
        res: *mut fmpz_poly_factor_struct,
        fac: *mut fmpz_poly_factor_struct,
    );
}
extern "C" {
    pub fn fmpz_poly_factor_print(fac: *mut fmpz_poly_factor_struct);
}
extern "C" {
    pub fn fmpz_poly_factor_zassenhaus_recombination(
        final_fac: *mut fmpz_poly_factor_struct,
        lifted_fac: *mut fmpz_poly_factor_struct,
        F: *mut fmpz_poly_struct,
        P: *mut fmpz,
        exp: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_factor_squarefree(fac: *mut fmpz_poly_factor_struct, F: *mut fmpz_poly_struct);
}
extern "C" {
    pub fn _fmpz_poly_factor_zassenhaus(
        final_fac: *mut fmpz_poly_factor_struct,
        exp: mp_limb_signed_t,
        f: *mut fmpz_poly_struct,
        cutoff: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_poly_factor_zassenhaus(fac: *mut fmpz_poly_factor_struct, G: *mut fmpz_poly_struct);
}


// fmpz_mat
#[repr(C)]
#[derive(Debug, Clone)]
pub struct fmpz_mat_struct {
    pub entries: *mut fmpz,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fmpz,
}

pub type fmpz_mat_t = [fmpz_mat_struct; 1usize];
extern "C" {
    pub fn fmpz_mat_init(mat: *mut fmpz_mat_struct, rows: mp_limb_signed_t, cols: mp_limb_signed_t);
}
extern "C" {
    pub fn fmpz_mat_init_set(mat: *mut fmpz_mat_struct, src: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_swap(mat1: *mut fmpz_mat_struct, mat2: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_set(mat1: *mut fmpz_mat_struct, mat2: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_clear(mat: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_equal(mat1: *const fmpz_mat_struct, mat2: *const fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_zero(mat: *const fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_one(mat: *const fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_zero(mat: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_one(mat: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_window_init(
        window: *mut fmpz_mat_struct,
        mat: *mut fmpz_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_window_clear(window: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_concat_horizontal(
        res: *mut fmpz_mat_struct,
        mat1: *mut fmpz_mat_struct,
        mat2: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_concat_vertical(
        res: *mut fmpz_mat_struct,
        mat1: *mut fmpz_mat_struct,
        mat2: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_randbits(mat: *mut fmpz_mat_struct, state: *mut flint_rand_s, bits: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mat_randtest(mat: *mut fmpz_mat_struct, state: *mut flint_rand_s, bits: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mat_randtest_unsigned(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randintrel(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randsimdioph(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
        bits2: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randntrulike(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
        q: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randntrulike2(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
        q: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randajtai(mat: *mut fmpz_mat_struct, state: *mut flint_rand_s, alpha: f64);
}
extern "C" {
    pub fn fmpz_mat_randrank(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        rank: mp_limb_signed_t,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randdet(mat: *mut fmpz_mat_struct, state: *mut flint_rand_s, det: *mut fmpz);
}
extern "C" {
    pub fn fmpz_mat_randops(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        count: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_randpermdiag(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        diag: *const fmpz,
        n: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_max_bits(mat: *mut fmpz_mat_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_transpose(B: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_add(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_sub(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_neg(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_scalar_mul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_mul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_mul_ui(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, c: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mat_scalar_addmul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_addmul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_addmul_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_submul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_submul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_submul_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_divexact_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_divexact_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_divexact_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_mul_2exp(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        exp: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_tdiv_q_2exp(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        exp: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_scalar_mod_fmpz(B: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct, m: *mut fmpz);
}
extern "C" {
    pub fn fmpz_mat_mul(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_mul_classical(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_mul_classical_inline(
        C: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn _fmpz_mat_mul_multi_mod(
        C: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
        bits: mp_limb_t,
    );
}
extern "C" {
    pub fn fmpz_mat_mul_multi_mod(
        C: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_sqr_bodrato(B: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_sqr(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_pow(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, exp: mp_limb_t);
}
extern "C" {
    pub fn fmpz_mat_content(ret: *mut fmpz, A: *const fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_find_pivot_any(
        mat: *mut fmpz_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_fflu(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        perm: *mut mp_limb_signed_t,
        A: *mut fmpz_mat_struct,
        rank_check: c_int,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_rref(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_rref_fflu(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_rref_mul(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_is_in_rref_with_rank(
        A: *mut fmpz_mat_struct,
        den: *mut fmpz,
        rank: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_rref_mod(
        perm: *mut mp_limb_signed_t,
        A: *mut fmpz_mat_struct,
        p: *mut fmpz,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_trace(trace: *mut fmpz, mat: *const fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_det(det: *mut fmpz, A: *const fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_det_cofactor(det: *mut fmpz, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn _fmpz_mat_det_cofactor_2x2(det: *mut fmpz, x: *const *mut fmpz);
}
extern "C" {
    pub fn _fmpz_mat_det_cofactor_3x3(det: *mut fmpz, x: *const *mut fmpz);
}
extern "C" {
    pub fn _fmpz_mat_det_cofactor_4x4(det: *mut fmpz, x: *const *mut fmpz);
}
extern "C" {
    pub fn fmpz_mat_det_bareiss(det: *mut fmpz, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_det_modular(det: *mut fmpz, A: *mut fmpz_mat_struct, proved: c_int);
}
extern "C" {
    pub fn fmpz_mat_det_modular_accelerated(det: *mut fmpz, A: *mut fmpz_mat_struct, proved: c_int);
}
extern "C" {
    pub fn fmpz_mat_det_modular_given_divisor(
        det: *mut fmpz,
        A: *mut fmpz_mat_struct,
        d: *mut fmpz,
        proved: c_int,
    );
}
extern "C" {
    pub fn fmpz_mat_det_bound(bound: *mut fmpz, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_det_divisor(d: *mut fmpz, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn _fmpz_mat_charpoly(cp: *mut fmpz, mat: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_charpoly(cp: *mut fmpz_poly_struct, mat: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_rank(A: *const fmpz_mat_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_solve_bound(
        N: *mut fmpz,
        D: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_solve(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_solve_cramer(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_solve_fflu(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_solve_fflu_precomp(
        X: *mut fmpz_mat_struct,
        perm: *const mp_limb_signed_t,
        FFLU: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_solve_dixon(
        X: *mut fmpz_mat_struct,
        mod_: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_nullspace(
        res: *mut fmpz_mat_struct,
        mat: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_inv(B: *mut fmpz_mat_struct, den: *mut fmpz, A: *mut fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_hnf(H: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_hnf_transform(
        H: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
    );
}
extern "C" {
    pub fn fmpz_mat_hnf_classical(H: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_hnf_xgcd(H: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_hnf_minors(H: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_hnf_modular(H: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct, D: *mut fmpz);
}
extern "C" {
    pub fn fmpz_mat_hnf_pernet_stein(
        H: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_in_hnf(A: *mut fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_snf(S: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_snf_diagonal(S: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_snf_kannan_bachem(S: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_snf_iliopoulos(
        S: *mut fmpz_mat_struct,
        A: *mut fmpz_mat_struct,
        mod_: *mut fmpz,
    );
}
extern "C" {
    pub fn fmpz_mat_is_in_snf(A: *mut fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_hadamard(A: *mut fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_hadamard(A: *mut fmpz_mat_struct) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_gram(B: *mut fmpz_mat_struct, A: *mut fmpz_mat_struct);
}
extern "C" {
    pub fn fmpz_mat_is_reduced(A: *mut fmpz_mat_struct, delta: f64, eta: f64) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_reduced_gram(A: *mut fmpz_mat_struct, delta: f64, eta: f64) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_reduced_with_removal(
        A: *mut fmpz_mat_struct,
        delta: f64,
        eta: f64,
        gs_B: *mut fmpz,
        newd: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn fmpz_mat_is_reduced_gram_with_removal(
        A: *mut fmpz_mat_struct,
        delta: f64,
        eta: f64,
        gs_B: *mut fmpz,
        newd: c_int,
    ) -> c_int;
}

extern "C" {
    // fmpz : Arbitrary precision integers. Functions related to mpz, mpf and IO are excluded.
    pub fn fmpz_init2(f: *mut fmpz, limbs: mp_limb_t);

    pub fn fmpz_randbits(f: *mut fmpz, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest(f: *mut fmpz, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest_unsigned(f: *mut fmpz, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest_not_zero(f: *mut fmpz, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randm(f: *mut fmpz, state: *mut flint_rand_s, m: fmpzptr);
    pub fn fmpz_randtest_mod(f: *mut fmpz, state: *mut flint_rand_s, m: *const fmpz);
    pub fn fmpz_randtest_mod_signed(f: *mut fmpz, state: *mut flint_rand_s, m: *const fmpz);

    pub fn fmpz_get_si(f: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_get_ui(f: *const fmpz) -> mp_limb_t;
    pub fn fmpz_set_d(f: *mut fmpz, c: c_double);
    pub fn fmpz_get_d(f: *const fmpz) -> c_double;
    pub fn fmpz_get_d_2exp(exp: *mut mp_limb_signed_t, f: *const fmpz) -> c_double;
    pub fn fmpz_get_str(str: *mut c_char, b: c_int, f: fmpzptr) -> *const c_char;
    pub fn fmpz_set_str(f: fmpzmutptr, str: *const c_char, b: c_int) -> c_int;


    pub fn fmpz_sizeinbase(f: fmpzptr, b: c_int) -> usize;
    pub fn fmpz_bits(f: *const fmpz) -> mp_limb_t;
    pub fn fmpz_sgn(f: *const fmpz) -> c_int;
    pub fn fmpz_val2(x: *const fmpz) -> mp_limb_t;
    pub fn fmpz_set(f: *mut fmpz, g: *const fmpz);
    pub fn fmpz_abs_fits_ui(f: *const fmpz) -> c_int;
    pub fn fmpz_fits_si(f: *const fmpz) -> c_int;
    pub fn fmpz_setbit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_tstbit(f: *mut fmpz, i: mp_limb_t) -> c_int;
    pub fn fmpz_abs_lbound_ui_2exp(
        exp: *mut mp_limb_signed_t,
        x: *const fmpz,
        bits: c_int,
    ) -> mp_limb_t;
    pub fn fmpz_abs_ubound_ui_2exp(
        exp: *mut mp_limb_signed_t,
        x: *const fmpz,
        bits: c_int,
    ) -> mp_limb_t;


    pub fn fmpz_cmp(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_cmp_ui(f: *const fmpz, g: mp_limb_t) -> c_int;
    pub fn fmpz_cmp_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_cmpabs(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_equal(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_equal_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_equal_ui(f: *const fmpz, g: mp_limb_t) -> c_int;

    pub fn fmpz_abs(f1: *mut fmpz, f2: *const fmpz);
    pub fn fmpz_add(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_add_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_sub(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_sub_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_mul(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_mul_si(f: fmpzmutptr, g: fmpzptr, x: mp_limb_signed_t);
    pub fn fmpz_mul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_mul_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_addmul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_addmul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_submul(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_submul_ui(f: fmpzmutptr, g: fmpzptr, x: mp_limb_t);
    pub fn fmpz_cdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_cdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_cdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_fdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_fdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_fdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_fdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_fdiv_qr(f: *mut fmpz, s: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_r(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_tdiv_q(f: fmpzmutptr, g: fmpzptr, h: fmpzptr);
    pub fn fmpz_tdiv_qr(f: *mut fmpz, s: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_tdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_tdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_tdiv_ui(g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_divexact(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_divexact_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_divexact_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_divisible(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_divisible_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_mod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_mod_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_mods(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_ui(g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_preinvn_init(inv: *mut fmpz_preinvn_struct, f: *mut fmpz);
    pub fn fmpz_preinvn_clear(inv: *mut fmpz_preinvn_struct);
    pub fn fmpz_fdiv_qr_preinvn(
        f: *mut fmpz,
        s: *mut fmpz,
        g: *const fmpz,
        h: *const fmpz,
        inv: *const fmpz_preinvn_struct,
    );

    pub fn fmpz_pow_ui(f: fmpzmutptr, g: fmpzptr, exp: mp_limb_t);
    pub fn fmpz_powm_ui(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t, m: *const fmpz);
    pub fn fmpz_powm(f: *mut fmpz, g: *const fmpz, e: *const fmpz, m: *const fmpz);

    pub fn fmpz_clog(x: *const fmpz, b: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_clog_ui(x: *const fmpz, b: mp_limb_t) -> mp_limb_signed_t;

    pub fn fmpz_flog(x: *const fmpz, b: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_flog_ui(x: *const fmpz, b: mp_limb_t) -> mp_limb_signed_t;
    pub fn fmpz_dlog(x: *const fmpz) -> c_double;


    pub fn fmpz_sqrtmod(b: *mut fmpz, a: *const fmpz, p: *const fmpz) -> c_int;
    pub fn fmpz_sqrt(f: *mut fmpz, g: *const fmpz);
    pub fn fmpz_sqrtrem(f: *mut fmpz, r: *mut fmpz, g: *const fmpz);
    pub fn fmpz_is_square(f: *const fmpz) -> c_int;
    pub fn fmpz_root(r: *mut fmpz, f: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_fac_ui(f: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_fib_ui(f: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_bin_uiui(res: *mut fmpz, n: mp_limb_t, k: mp_limb_t);
    pub fn fmpz_rfac_ui(r: *mut fmpz, x: *const fmpz, n: mp_limb_t);
    pub fn fmpz_rfac_uiui(r: *mut fmpz, x: mp_limb_t, n: mp_limb_t);
    pub fn fmpz_mul_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, h: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_mul_si_tdiv_q_2exp(
        f: *mut fmpz,
        g: *const fmpz,
        x: mp_limb_signed_t,
        exp: mp_limb_t,
    );

    pub fn _fmpz_remove(x: *mut fmpz, f: *const fmpz, finv: c_double) -> mp_limb_signed_t;
    pub fn fmpz_remove(rop: *mut fmpz, op: *const fmpz, f: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_invmod(f: *mut fmpz, g: *const fmpz, h: *const fmpz) -> c_int;
    pub fn fmpz_jacobi(a: *const fmpz, p: *const fmpz) -> c_int;

    pub fn fmpz_bit_pack(
        arr: mp_ptr,
        shift: mp_limb_t,
        bits: mp_limb_t,
        coeff: *mut fmpz,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;
    pub fn fmpz_bit_unpack(
        coeff: *mut fmpz,
        arr: *const mp_limb_t,
        shift: mp_limb_t,
        bits: mp_limb_t,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;
    pub fn fmpz_bit_unpack_unsigned(
        coeff: *mut fmpz,
        arr: *const mp_limb_t,
        shift: mp_limb_t,
        bits: mp_limb_t,
    );

    pub fn fmpz_complement(r: *mut fmpz, f: *const fmpz);
    pub fn fmpz_combit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_clrbit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_and(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_or(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_xor(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_popcnt(c: *const fmpz) -> mp_limb_t;

    pub fn fmpz_CRT_ui(
        out: *mut fmpz,
        r1: *mut fmpz,
        m1: *mut fmpz,
        r2: mp_limb_t,
        m2: mp_limb_t,
        sign: c_int,
    );
    pub fn fmpz_CRT(
        out: *mut fmpz,
        r1: *const fmpz,
        m1: *const fmpz,
        r2: *mut fmpz,
        m2: *mut fmpz,
        sign: c_int,
    );
    pub fn fmpz_multi_mod_ui(
        out: *mut mp_limb_t,
        in_: *const fmpz,
        comb: *const fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
    );
    pub fn fmpz_multi_CRT_ui(
        output: *mut fmpz,
        residues: mp_srcptr,
        comb: *const fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
        sign: c_int,
    );
    pub fn fmpz_comb_init(
        comb: *mut fmpz_comb_struct,
        primes: mp_srcptr,
        num_primes: mp_limb_signed_t,
    );
    pub fn fmpz_comb_temp_init(temp: *mut fmpz_comb_temp_struct, comb: *const fmpz_comb_struct);
    pub fn fmpz_comb_clear(comb: *mut fmpz_comb_struct);
    pub fn fmpz_comb_temp_clear(temp: *mut fmpz_comb_temp_struct);

    pub fn fmpz_is_strong_probabprime(n: *const fmpz, a: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime(p: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime_lucas(n: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime_BPSW(n: *const fmpz) -> c_int;
    pub fn fmpz_is_prime_pseudosquare(n: *const fmpz) -> c_int;
    pub fn fmpz_is_prime_pocklington(
        F: *mut fmpz,
        R: *mut fmpz,
        n: *const fmpz,
        pm1: mp_ptr,
        num_pm1: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_np1_trial_factors(
        n: *const fmpz,
        pp1: mp_ptr,
        num_pp1: *mut mp_limb_signed_t,
        limit: mp_limb_t,
    );
    pub fn fmpz_is_prime_morrison(
        F: *mut fmpz,
        R: *mut fmpz,
        n: *const fmpz,
        pm1: mp_ptr,
        num_pm1: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_nm1_trial_factors(
        n: *const fmpz,
        pm1: mp_ptr,
        num_pm1: *mut mp_limb_signed_t,
        limit: mp_limb_t,
    );
    pub fn fmpz_is_prime(p: *const fmpz) -> c_int;

    pub fn fmpz_lucas_chain(
        Vm: *mut fmpz,
        Vm1: *mut fmpz,
        A: *const fmpz,
        m: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_lucas_chain_full(
        Vm: *mut fmpz,
        Vm1: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        m: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_lucas_chain_double(
        U2m: *mut fmpz,
        U2m1: *mut fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_lucas_chain_add(
        Umn: *mut fmpz,
        Umn1: *mut fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        Un: *const fmpz,
        Un1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_lucas_chain_mul(
        Ukm: *mut fmpz,
        Ukm1: *mut fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        k: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_lucas_chain_VtoU(
        Um: *mut fmpz,
        Um1: *mut fmpz,
        Vm: *const fmpz,
        Vm1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        Dinv: *const fmpz,
        n: *const fmpz,
    );
    pub fn fmpz_divisor_in_residue_class_lenstra(
        fac: *mut fmpz,
        n: *const fmpz,
        r: *const fmpz,
        s: *const fmpz,
    ) -> c_int;


    pub fn fmpz_primorial(res: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_euler_phi(res: *mut fmpz, n: *const fmpz);
    pub fn fmpz_factor_euler_phi(res: *mut fmpz, fac: *const fmpz_factor_struct);
    pub fn fmpz_factor_moebius_mu(fac: *const fmpz_factor_struct) -> c_int;
    pub fn fmpz_moebius_mu(n: *const fmpz) -> c_int;
    pub fn fmpz_factor_divisor_sigma(res: *mut fmpz, fac: *const fmpz_factor_struct, k: mp_limb_t);
    pub fn fmpz_divisor_sigma(res: *mut fmpz, n: *mut fmpz, k: mp_limb_t);

    // fmpz factor: Factorisation of arbitrary precision integers
    pub fn _fmpz_factor_append_ui(factor: *mut fmpz_factor_struct, p: mp_limb_t, exp: mp_limb_t);
    pub fn _fmpz_factor_append(factor: *mut fmpz_factor_struct, p: *mut fmpz, exp: mp_limb_t);
    pub fn fmpz_factor_trial_range(
        factor: *mut fmpz_factor_struct,
        n: *const fmpz,
        start: mp_limb_t,
        num_primes: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_pp1(
        factor: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: fmpzptr);
    pub fn fmpz_factor_si(factor: *mut fmpz_factor_struct, n: mp_limb_signed_t);
    pub fn fmpz_factor_expand_iterative(n: fmpzmutptr, factor: *const fmpz_factor_struct);

    // 57. ulong extras: Arithmetic for single word unsigned integers
    pub fn n_randlimb(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randbits(state: *mut flint_rand_s, bits: c_uint) -> mp_limb_t;
    pub fn n_randtest_bits(state: *mut flint_rand_s, bits: c_int) -> mp_limb_t;
    pub fn n_randint(state: *mut flint_rand_s, limit: mp_limb_t) -> mp_limb_t;
    pub fn n_randtest(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randtest_not_zero(state: *mut flint_rand_s) -> mp_limb_t;
    pub fn n_randprime(state: *mut flint_rand_s, bits: mp_limb_t, proved: c_int) -> mp_limb_t;
    pub fn n_randtest_prime(state: *mut flint_rand_s, proved: c_int) -> mp_limb_t;

    pub fn n_pow(n: mp_limb_t, exp: mp_limb_t) -> mp_limb_t;
    pub fn n_flog(n: mp_limb_t, b: mp_limb_t) -> mp_limb_t;
    pub fn n_clog(n: mp_limb_t, b: mp_limb_t) -> mp_limb_t;

    pub fn n_remove(n: *mut mp_limb_t, p: mp_limb_t) -> c_int;
    pub fn n_jacobi(x: mp_limb_signed_t, y: mp_limb_t) -> c_int;
}


#[link(name = "flint_wrapper")]
extern "C" {
    #[link_name = "wrapped_fmpz_clear"]
    pub fn fmpz_clear(f: fmpzmutptr);
    #[link_name = "wrapped_fmpz_init"]
    pub fn fmpz_init(f: fmpzmutptr);
    #[link_name = "wrapped_fmpz_init_set"]
    pub fn fmpz_init_set(f: fmpzmutptr, g: fmpzptr);
    #[link_name = "wrapped_fmpz_init_set_si"]
    pub fn fmpz_init_set_si(f: fmpzmutptr, g: mp_limb_signed_t);
    #[link_name = "wrapped_fmpz_init_set_ui"]
    pub fn fmpz_init_set_ui(f: fmpzmutptr, g: mp_limb_t);
    #[link_name = "wrapped_fmpz_set_si"]
    pub fn fmpz_set_si(f: fmpzmutptr, val: mp_limb_signed_t);
    #[link_name = "wrapped_fmpz_set_ui"]
    pub fn fmpz_set_ui(f: fmpzmutptr, val: mp_limb_t);
    #[link_name = "wrapped_fmpz_neg_ui"]
    pub fn fmpz_neg_ui(f: *mut fmpz, val: mp_limb_t);
    #[link_name = "wrapped_fmpz_set_uiui"]
    pub fn fmpz_set_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);
    #[link_name = "wrapped_fmpz_neg_uiui"]
    pub fn fmpz_neg_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);
    #[link_name = "wrapped_fmpz_set_ui_smod"]
    pub fn fmpz_set_ui_smod(f: *mut fmpz, x: mp_limb_t, m: mp_limb_t);

    #[link_name = "wrapped_fmpz_zero"]
    pub fn fmpz_zero(f: *mut fmpz);
    #[link_name = "wrapped_fmpz_one"]
    pub fn fmpz_one(f: *mut fmpz);

    #[link_name = "wrapped_fmpz_is_zero"]
    pub fn fmpz_is_zero(f: *const fmpz) -> c_int;
    #[link_name = "wrapped_fmpz_is_one"]
    pub fn fmpz_is_one(f: *const fmpz) -> c_int;
    #[link_name = "wrapped_fmpz_is_pm1"]
    pub fn fmpz_is_pm1(f: *const fmpz) -> c_int;
    #[link_name = "wrapped_fmpz_is_even"]
    pub fn fmpz_is_even(f: *const fmpz) -> c_int;
    #[link_name = "wrapped_fmpz_is_odd"]
    pub fn fmpz_is_odd(f: *const fmpz) -> c_int;

    #[link_name = "wrapped_fmpz_neg"]
    pub fn fmpz_neg(f1: *mut fmpz, f2: *const fmpz);
    #[link_name = "wrapped_fmpz_mul2_uiui"]
    pub fn fmpz_mul2_uiui(f: *mut fmpz, g: *const fmpz, h1: mp_limb_t, h2: mp_limb_t);
    #[link_name = "wrapped_fmpz_divexact2_uiui"]
    pub fn fmpz_divexact2_uiui(f: *mut fmpz, g: *const fmpz, h1: mp_limb_t, h2: mp_limb_t);
    #[link_name = "wrapped_fmpz_negmod"]
    pub fn fmpz_negmod(r: *mut fmpz, a: *const fmpz, mod_: *const fmpz);


    #[link_name = "wrapped_flint_randinit"]
    pub fn flint_randinit(state: *mut flint_rand_s);
    #[link_name = "wrapped_flint_randclear"]
    pub fn flint_randclear(state: *mut flint_rand_s);
}
