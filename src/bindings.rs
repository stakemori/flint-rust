#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate libc;

use self::libc::{c_ulong, c_long, c_char, c_int, c_void, c_double};
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
    gmp_init: ::std::os::raw::c_int,
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
    pub fn fmpz_abs_lbound_ui_2exp(exp: *mut mp_limb_signed_t,
                                   x: *const fmpz,
                                   bits: c_int)
                                   -> mp_limb_t;
    pub fn fmpz_abs_ubound_ui_2exp(exp: *mut mp_limb_signed_t,
                                   x: *const fmpz,
                                   bits: c_int)
                                   -> mp_limb_t;


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
    pub fn fmpz_fdiv_qr_preinvn(f: *mut fmpz,
                                s: *mut fmpz,
                                g: *const fmpz,
                                h: *const fmpz,
                                inv: *const fmpz_preinvn_struct);

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
    pub fn fmpz_mul_si_tdiv_q_2exp(f: *mut fmpz,
                                   g: *const fmpz,
                                   x: mp_limb_signed_t,
                                   exp: mp_limb_t);

    pub fn _fmpz_remove(x: *mut fmpz, f: *const fmpz, finv: c_double) -> mp_limb_signed_t;
    pub fn fmpz_remove(rop: *mut fmpz, op: *const fmpz, f: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_invmod(f: *mut fmpz, g: *const fmpz, h: *const fmpz) -> c_int;
    pub fn fmpz_jacobi(a: *const fmpz, p: *const fmpz) -> c_int;

    pub fn fmpz_bit_pack(arr: mp_ptr,
                         shift: mp_limb_t,
                         bits: mp_limb_t,
                         coeff: *mut fmpz,
                         negate: c_int,
                         borrow: c_int)
                         -> c_int;
    pub fn fmpz_bit_unpack(coeff: *mut fmpz,
                           arr: *const mp_limb_t,
                           shift: mp_limb_t,
                           bits: mp_limb_t,
                           negate: c_int,
                           borrow: c_int)
                           -> c_int;
    pub fn fmpz_bit_unpack_unsigned(coeff: *mut fmpz,
                                    arr: *const mp_limb_t,
                                    shift: mp_limb_t,
                                    bits: mp_limb_t);

    pub fn fmpz_complement(r: *mut fmpz, f: *const fmpz);
    pub fn fmpz_combit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_clrbit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_and(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_or(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_xor(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_popcnt(c: *const fmpz) -> mp_limb_t;

    pub fn fmpz_CRT_ui(out: *mut fmpz,
                       r1: *mut fmpz,
                       m1: *mut fmpz,
                       r2: mp_limb_t,
                       m2: mp_limb_t,
                       sign: c_int);
    pub fn fmpz_CRT(out: *mut fmpz,
                    r1: *const fmpz,
                    m1: *const fmpz,
                    r2: *mut fmpz,
                    m2: *mut fmpz,
                    sign: c_int);
    pub fn fmpz_multi_mod_ui(out: *mut mp_limb_t,
                             in_: *const fmpz,
                             comb: *const fmpz_comb_struct,
                             temp: *mut fmpz_comb_temp_struct);
    pub fn fmpz_multi_CRT_ui(output: *mut fmpz,
                             residues: mp_srcptr,
                             comb: *const fmpz_comb_struct,
                             temp: *mut fmpz_comb_temp_struct,
                             sign: c_int);
    pub fn fmpz_comb_init(comb: *mut fmpz_comb_struct,
                          primes: mp_srcptr,
                          num_primes: mp_limb_signed_t);
    pub fn fmpz_comb_temp_init(temp: *mut fmpz_comb_temp_struct, comb: *const fmpz_comb_struct);
    pub fn fmpz_comb_clear(comb: *mut fmpz_comb_struct);
    pub fn fmpz_comb_temp_clear(temp: *mut fmpz_comb_temp_struct);

    pub fn fmpz_is_strong_probabprime(n: *const fmpz, a: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime(p: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime_lucas(n: *const fmpz) -> c_int;
    pub fn fmpz_is_probabprime_BPSW(n: *const fmpz) -> c_int;
    pub fn fmpz_is_prime_pseudosquare(n: *const fmpz) -> c_int;
    pub fn fmpz_is_prime_pocklington(F: *mut fmpz,
                                     R: *mut fmpz,
                                     n: *const fmpz,
                                     pm1: mp_ptr,
                                     num_pm1: mp_limb_signed_t)
                                     -> c_int;
    pub fn _fmpz_np1_trial_factors(n: *const fmpz,
                                   pp1: mp_ptr,
                                   num_pp1: *mut mp_limb_signed_t,
                                   limit: mp_limb_t);
    pub fn fmpz_is_prime_morrison(F: *mut fmpz,
                                  R: *mut fmpz,
                                  n: *const fmpz,
                                  pm1: mp_ptr,
                                  num_pm1: mp_limb_signed_t)
                                  -> c_int;
    pub fn _fmpz_nm1_trial_factors(n: *const fmpz,
                                   pm1: mp_ptr,
                                   num_pm1: *mut mp_limb_signed_t,
                                   limit: mp_limb_t);
    pub fn fmpz_is_prime(p: *const fmpz) -> c_int;

    pub fn fmpz_lucas_chain(Vm: *mut fmpz,
                            Vm1: *mut fmpz,
                            A: *const fmpz,
                            m: *const fmpz,
                            n: *const fmpz);
    pub fn fmpz_lucas_chain_full(Vm: *mut fmpz,
                                 Vm1: *mut fmpz,
                                 A: *const fmpz,
                                 B: *const fmpz,
                                 m: *const fmpz,
                                 n: *const fmpz);
    pub fn fmpz_lucas_chain_double(U2m: *mut fmpz,
                                   U2m1: *mut fmpz,
                                   Um: *const fmpz,
                                   Um1: *const fmpz,
                                   A: *const fmpz,
                                   B: *const fmpz,
                                   n: *const fmpz);
    pub fn fmpz_lucas_chain_add(Umn: *mut fmpz,
                                Umn1: *mut fmpz,
                                Um: *const fmpz,
                                Um1: *const fmpz,
                                Un: *const fmpz,
                                Un1: *const fmpz,
                                A: *const fmpz,
                                B: *const fmpz,
                                n: *const fmpz);
    pub fn fmpz_lucas_chain_mul(Ukm: *mut fmpz,
                                Ukm1: *mut fmpz,
                                Um: *const fmpz,
                                Um1: *const fmpz,
                                A: *const fmpz,
                                B: *const fmpz,
                                k: *const fmpz,
                                n: *const fmpz);
    pub fn fmpz_lucas_chain_VtoU(Um: *mut fmpz,
                                 Um1: *mut fmpz,
                                 Vm: *const fmpz,
                                 Vm1: *const fmpz,
                                 A: *const fmpz,
                                 B: *const fmpz,
                                 Dinv: *const fmpz,
                                 n: *const fmpz);
    pub fn fmpz_divisor_in_residue_class_lenstra(fac: *mut fmpz,
                                                 n: *const fmpz,
                                                 r: *const fmpz,
                                                 s: *const fmpz)
                                                 -> c_int;


    pub fn fmpz_primorial(res: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_euler_phi(res: *mut fmpz, n: *const fmpz);
    pub fn fmpz_factor_euler_phi(res: *mut fmpz, fac: *const fmpz_factor_struct);
    pub fn fmpz_factor_moebius_mu(fac: *const fmpz_factor_struct) -> c_int;
    pub fn fmpz_moebius_mu(n: *const fmpz) -> c_int;
    pub fn fmpz_factor_divisor_sigma(res: *mut fmpz,
                                     fac: *const fmpz_factor_struct,
                                     k: mp_limb_t);
    pub fn fmpz_divisor_sigma(res: *mut fmpz, n: *mut fmpz, k: mp_limb_t);

    // fmpz factor: Factorisation of arbitrary precision integers
    pub fn _fmpz_factor_append_ui(factor: *mut fmpz_factor_struct, p: mp_limb_t, exp: mp_limb_t);
    pub fn _fmpz_factor_append(factor: *mut fmpz_factor_struct, p: *mut fmpz, exp: mp_limb_t);
    pub fn fmpz_factor_trial_range(factor: *mut fmpz_factor_struct,
                                   n: *const fmpz,
                                   start: mp_limb_t,
                                   num_primes: mp_limb_t)
                                   -> c_int;
    pub fn fmpz_factor_pp1(factor: *mut fmpz,
                           n: *const fmpz,
                           B1: mp_limb_t,
                           B2_sqrt: mp_limb_t,
                           c: mp_limb_t)
                           -> c_int;
    pub fn fmpz_factor_init(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor_clear(factor: *mut fmpz_factor_struct);
    pub fn fmpz_factor(factor: *mut fmpz_factor_struct, n: fmpzptr);
    pub fn fmpz_factor_si(factor: *mut fmpz_factor_struct, n: mp_limb_signed_t);
    pub fn fmpz_factor_expand_iterative(n: fmpzmutptr, factor: *const fmpz_factor_struct);
}

#[link(name = "fmpz_wrapper")]
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

    pub fn bench_square_sum_native(n: c_ulong);
}
