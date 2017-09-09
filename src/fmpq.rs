use bindings::*;
use libc::{c_long, c_ulong};
use std;
use fmpz::Fmpz;
use std::fmt;
use std::cmp::Ordering::{self, Greater, Less, Equal};
use std::ops::*;

#[derive(Debug, Clone)]
pub struct Fmpq {
    pub fmpq: fmpq_t,
}

impl_operator!(Mul, Fmpq, mul, fmpq_mul);
impl_operator!(Add, Fmpq, add, fmpq_add);
impl_operator!(Sub, Fmpq, sub, fmpq_sub);
impl_operator!(Div, Fmpq, div, fmpq_div);

define_assign!(Fmpq, AddAssign, add_assign, fmpq_add);
define_assign!(Fmpq, MulAssign, mul_assign, fmpq_mul);
define_assign!(Fmpq, SubAssign, sub_assign, fmpq_sub);
define_assign!(Fmpq, DivAssign, div_assign, fmpq_div);

define_assign_c!(Fmpq, AddAssign, add_assign, fmpq_add_si, c_long);
define_assign_c!(Fmpq, SubAssign, sub_assign, fmpq_sub_si, c_long);
define_assign_wref!(Fmpq, AddAssign, add_assign, fmpq_add_fmpz, Fmpz);
define_assign_wref!(Fmpq, SubAssign, sub_assign, fmpq_sub_fmpz, Fmpz);
define_assign_wref!(Fmpq, MulAssign, mul_assign, fmpq_mul_fmpz, Fmpz);
define_assign_wref!(Fmpq, DivAssign, div_assign, fmpq_div_fmpz, Fmpz);

define_assign_c!(Fmpq, ShlAssign, shl_assign, fmpq_mul_2exp, c_ulong);
define_assign_c!(Fmpq, ShrAssign, shr_assign, fmpq_div_2exp, c_ulong);

imp_operator_c!(Shl, Fmpq, shl, c_ulong, fmpq_mul_2exp);
imp_operator_c!(Shr, Fmpq, shr, c_ulong, fmpq_div_2exp);

impl Default for Fmpq {
    fn default() -> Self {
        Fmpq::new()
    }
}

impl fmt::Display for Fmpq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut num = Fmpz::new();
        let mut den = Fmpz::new();
        self.set_num_den(&mut num, &mut den);
        write!(f, "{}/{}", num, den)
    }
}

impl PartialEq for Fmpq {
    fn eq(&self, other: &Fmpq) -> bool {
        unsafe { fmpq_equal(self.as_ptr(), other.as_ptr()) != 0 }
    }
}

impl PartialOrd for Fmpq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(int_to_ord!(
            unsafe { fmpq_cmp(self.as_ptr(), other.as_ptr()) }
        ))
    }
}

impl From<(c_long, c_ulong)> for Fmpq {
    fn from(x: (c_long, c_ulong)) -> Fmpq {
        unsafe {
            let mut a = Fmpq::new();
            fmpq_set_si(a.as_mut_ptr(), x.0, x.1);
            a
        }
    }
}

impl<'a> From<(&'a Fmpz, &'a Fmpz)> for Fmpq {
    fn from(x: (&'a Fmpz, &'a Fmpz)) -> Self {
        unsafe {
            let mut a = Fmpq::new();
            fmpq_set_fmpz_frac(a.as_mut_ptr(), x.0.as_ptr(), x.1.as_ptr());
            a
        }
    }
}

impl Drop for Fmpq {
    fn drop(&mut self) {
        unsafe {
            fmpq_clear(self.as_mut_ptr());
        }
    }
}

impl Fmpq {
    fn uninitialized() -> fmpq_t {
        unsafe {
            let a: fmpq = std::mem::uninitialized();
            [a] as fmpq_t
        }
    }

    pub fn new() -> Self {
        unsafe {
            let mut a = Self::uninitialized();
            fmpq_init(a.as_mut_ptr());
            Fmpq { fmpq: a }
        }
    }

    pub fn as_mut_ptr(&mut self) -> &mut fmpq {
        &mut self.fmpq[0]
    }

    pub fn as_ptr(&self) -> *const fmpq {
        &self.fmpq[0]
    }

    pub fn sgn(&self) -> i32 {
        unsafe { fmpq_sgn(self.as_ptr()) as i32 }
    }

    pub fn set_num_den(&self, num: &mut Fmpz, den: &mut Fmpz) {
        unsafe {
            fmpz_set(num.as_mut_ptr(), self.num_as_ptr());
            fmpz_set(den.as_mut_ptr(), self.den_as_ptr());
        }
    }

    pub fn num_as_ptr(&self) -> fmpzptr {
        &self.fmpq[0].num
    }

    pub fn num_as_mut_ptr(&mut self) -> fmpzptr {
        &mut self.fmpq[0].num
    }

    pub fn den_as_ptr(&self) -> fmpzptr {
        &self.fmpq[0].den
    }

    pub fn den_as_mut_ptr(&mut self) -> fmpzptr {
        &mut self.fmpq[0].den
    }

    pub fn is_zero(&self) -> bool {
        int_to_bool!(unsafe { fmpq_is_zero(self.as_ptr()) })
    }

    pub fn is_one(&self) -> bool {
        int_to_bool!(unsafe { fmpq_is_one(self.as_ptr()) })
    }
    /// Canonicalize self
    impl_mut_c_wrapper!(canonicalise_mut, fmpq_canonicalise,);
    /// `self = x - y`
    impl_mut_c_wrapper!(sub_si_mut, fmpq_sub_si, x: SelfRef, y: Si);
    /// `self = x + y`
    impl_mut_c_wrapper!(add_si_mut, fmpq_add_si, x: SelfRef, y: Si);
    /// `self = x - y`
    impl_mut_c_wrapper!(sub_fmpz_mut, fmpq_sub_fmpz, x: SelfRef, y: FmpzRef);
    /// `self = x + y`
    impl_mut_c_wrapper!(add_fmpz_mut, fmpq_add_fmpz, x: SelfRef, y: FmpzRef);
    /// `self = x * y`
    impl_mut_c_wrapper!(mul_fmpz_mut, fmpq_mul_fmpz, x: SelfRef, y: FmpzRef);
    /// `self = x / y`
    impl_mut_c_wrapper!(div_fmpz_mut, fmpq_div_fmpz, x: SelfRef, y: FmpzRef);

    /// `self = abs(x)`
    impl_mut_c_wrapper!(abs_mut, fmpq_abs, x: SelfRef);
    /// `self = x^(-1)`
    impl_mut_c_wrapper!(inv_mut, fmpq_inv, x: SelfRef);
    /// `self = -x`
    impl_mut_c_wrapper!(neg_mut, fmpq_neg, x: SelfRef);
    /// `self = x`
    impl_mut_c_wrapper!(set, fmpq_set, x: SelfRef);
    impl_mut_c_wrapper!(set_zero, fmpq_zero,);
    impl_mut_c_wrapper!(set_one, fmpq_one,);
    /// `self = p/q`
    impl_mut_c_wrapper!(set_si, fmpq_set_si, p: Si, q: Ui);
    /// `self = p/q`
    impl_mut_c_wrapper!(set_fmpz_frac, fmpq_set_fmpz_frac, p: FmpzRef, q: FmpzRef);
    /// `self = x^e`
    impl_mut_c_wrapper!(pow_si_mut, fmpq_pow_si, x: SelfRef, e: c_long);
    /// `self += x*y`
    impl_mut_c_wrapper!(add_mul_mut, fmpq_addmul, x: SelfRef, y: SelfRef);
    impl_mut_c_wrapper!(sub_mul_mut, fmpq_submul, x: SelfRef, y: SelfRef);

    impl_self_mut_call_c!(negate, fmpq_neg,);
    impl_self_mut_call_c!(set_pow_si, fmpq_pow_si, e: c_long);
}
