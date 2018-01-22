use bindings::*;
use libc::{c_long, c_ulong};
use std;
use fmpz::Fmpz;
use std::fmt;
use std::cmp::Ordering::{self, Greater, Less, Equal};
use std::ops::*;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};


#[derive(Debug)]
pub struct Fmpq {
    fmpq: fmpq_t,
}

impl Clone for Fmpq {
    fn clone(&self) -> Self {
        let mut a = Fmpq::new();
        a.set(self);
        a
    }

    fn clone_from(&mut self, other: &Self) {
        self.set(other);
    }
}

impl Serialize for Fmpq {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let a = (self.num_new(), self.den_new());
        a.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Fmpq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        type FmpzTuple = (Fmpz, Fmpz);
        let a = FmpzTuple::deserialize(deserializer)?;
        Ok(From::from((&a.0, &a.1)))
    }
}

impl_operator!(Mul, Fmpq, mul, fmpq_mul);
impl_operator!(Add, Fmpq, add, fmpq_add);
impl_operator!(Sub, Fmpq, sub, fmpq_sub);
impl_operator!(Div, Fmpq, div, fmpq_div);

define_assign_wref!(Fmpq, AddAssign, add_assign, fmpq_add, Fmpq);
define_assign_wref!(Fmpq, MulAssign, mul_assign, fmpq_mul, Fmpq);
define_assign_wref!(Fmpq, SubAssign, sub_assign, fmpq_sub, Fmpq);
define_assign_wref!(Fmpq, DivAssign, div_assign, fmpq_div, Fmpq);

define_assign_with_ptr!(Fmpq, AddAssign, add_assign, fmpq_add, fmpq);
define_assign_with_ptr!(Fmpq, MulAssign, mul_assign, fmpq_mul, fmpq);
define_assign_with_ptr!(Fmpq, SubAssign, sub_assign, fmpq_sub, fmpq);
define_assign_with_ptr!(Fmpq, DivAssign, div_assign, fmpq_div, fmpq);

define_assign_c!(Fmpq, AddAssign, add_assign, fmpq_add_si, c_long);
define_assign_c!(Fmpq, SubAssign, sub_assign, fmpq_sub_si, c_long);
define_assign_c!(Fmpq, MulAssign, mul_assign, fmpq_mul_si, c_long);
define_assign_c!(Fmpq, DivAssign, div_assign, fmpq_div_si, c_long);

define_assign_wref!(Fmpq, AddAssign, add_assign, fmpq_add_fmpz, Fmpz);
define_assign_wref!(Fmpq, SubAssign, sub_assign, fmpq_sub_fmpz, Fmpz);
define_assign_wref!(Fmpq, MulAssign, mul_assign, fmpq_mul_fmpz, Fmpz);
define_assign_wref!(Fmpq, DivAssign, div_assign, fmpq_div_fmpz, Fmpz);

define_assign_with_ptr!(Fmpq, AddAssign, add_assign, fmpq_add_fmpz, fmpz);
define_assign_with_ptr!(Fmpq, MulAssign, mul_assign, fmpq_mul_fmpz, fmpz);
define_assign_with_ptr!(Fmpq, SubAssign, sub_assign, fmpq_sub_fmpz, fmpz);
define_assign_with_ptr!(Fmpq, DivAssign, div_assign, fmpq_div_fmpz, fmpz);

define_assign_c!(Fmpq, ShlAssign, shl_assign, fmpq_mul_2exp, c_ulong);
define_assign_c!(Fmpq, ShrAssign, shr_assign, fmpq_div_2exp, c_ulong);

impl_operator_c!(Add, Fmpq, add, c_long, fmpq_add_si);
impl_operator_c!(Sub, Fmpq, sub, c_long, fmpq_sub_si);
impl_operator_c!(Mul, Fmpq, mul, c_long, fmpq_mul_si);
impl_operator_c!(Div, Fmpq, div, c_long, fmpq_div_si);
impl_operator_c!(Shl, Fmpq, shl, c_ulong, fmpq_mul_2exp);
impl_operator_c!(Shr, Fmpq, shr, c_ulong, fmpq_div_2exp);

impl Default for Fmpq {
    fn default() -> Self {
        Fmpq::new()
    }
}

impl Deref for Fmpq {
    type Target = fmpq;

    fn deref(&self) -> &fmpq {
        self.as_raw()
    }
}

impl DerefMut for Fmpq {
    fn deref_mut(&mut self) -> &mut fmpq {
        self.as_raw_mut()
    }
}

impl_neg!(Fmpq, fmpq_neg);

impl fmt::Display for Fmpq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut num = Fmpz::new();
        let mut den = Fmpz::new();
        self.num(&mut num);
        self.den(&mut den);
        if den == 1_i64 {
            write!(f, "{}", num)
        } else {
            write!(f, "{}/{}", num, den)
        }
    }
}

impl PartialEq for Fmpq {
    fn eq(&self, other: &Fmpq) -> bool {
        debug_assert!(self.is_canonical() && other.is_canonical());
        unsafe { fmpq_equal(self.as_raw(), other.as_raw()) != 0 }
    }
}

impl Eq for Fmpq {}

impl PartialOrd for Fmpq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(int_to_ord!(
            unsafe { fmpq_cmp(self.as_raw(), other.as_raw()) }
        ))
    }
}

impl From<(c_long, c_ulong)> for Fmpq {
    fn from(x: (c_long, c_ulong)) -> Fmpq {
        unsafe {
            let mut a = Fmpq::new();
            fmpq_set_si(a.as_raw_mut(), x.0, x.1);
            a
        }
    }
}

impl From<(Fmpz, Fmpz)> for Fmpq {
    fn from(x: (Fmpz, Fmpz)) -> Self {
        Into::into((&x.0, &x.1))
    }
}

impl From<c_long> for Fmpq {
    fn from(x: c_long) -> Fmpq {
        From::from((x, 1))
    }
}

impl<'a> From<(&'a Fmpz, &'a Fmpz)> for Fmpq {
    fn from(x: (&'a Fmpz, &'a Fmpz)) -> Self {
        unsafe {
            let mut a = Fmpq::new();
            fmpq_set_fmpz_frac(a.as_raw_mut(), x.0.as_raw(), x.1.as_raw());
            a
        }
    }
}

impl<'a> From<&'a Fmpz> for Fmpq {
    fn from(x: &Fmpz) -> Self {
        unsafe {
            let mut res = Fmpq::new();
            fmpz_set(res.num_as_raw_mut(), x.as_raw());
            fmpz_one(res.den_as_raw_mut());
            res
        }
    }
}

impl Drop for Fmpq {
    fn drop(&mut self) {
        unsafe {
            fmpq_clear(self.as_raw_mut());
        }
    }
}

#[derive(Debug)]
pub struct InverseNotExist;

impl Fmpq {
    fn uninitialized() -> fmpq_t {
        unsafe {
            let a: fmpq = std::mem::uninitialized();
            [a] as fmpq_t
        }
    }

    /// Return new rational, which is set to zero.
    pub fn new() -> Self {
        unsafe {
            let mut a = Self::uninitialized();
            fmpq_init(a.as_mut_ptr());
            Fmpq { fmpq: a }
        }
    }

    pub fn as_raw_mut(&mut self) -> &mut fmpq {
        &mut self.fmpq[0]
    }

    pub fn as_raw(&self) -> &fmpq {
        &self.fmpq[0]
    }

    pub fn sgn(&self) -> i32 {
        unsafe { fmpq_sgn(self.as_raw()) as i32 }
    }

    pub fn num(&self, num: &mut Fmpz) {
        unsafe {
            fmpz_set(num.as_raw_mut(), self.num_as_raw());
        }
    }

    pub fn den(&self, den: &mut Fmpz) {
        unsafe {
            fmpz_set(den.as_raw_mut(), self.den_as_raw());
        }
    }

    pub fn num_as_raw(&self) -> &fmpz {
        &self.fmpq[0].num
    }

    pub fn num_new(&self) -> Fmpz {
        let mut a = Fmpz::new();
        unsafe {
            fmpz_set(a.as_raw_mut(), self.num_as_raw());
        }
        a
    }

    /// Assuming `self` has a canonical form, checks if `self` is integral.
    pub fn is_integral(&self) -> bool {
        unsafe { int_to_bool!(fmpz_is_one(self.den_as_raw())) }
    }

    pub fn to_slong(&self) -> Option<c_long> {
        if self.is_integral() {
            unsafe {
                if int_to_bool!(fmpz_fits_si(self.num_as_raw())) {
                    Some(fmpz_get_si(self.num_as_raw()))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn den_new(&self) -> Fmpz {
        let mut a = Fmpz::new();
        unsafe {
            fmpz_set(a.as_raw_mut(), self.den_as_raw());
        }
        a
    }

    pub fn num_as_raw_mut(&mut self) -> &mut fmpz {
        &mut self.fmpq[0].num
    }

    pub fn den_as_raw(&self) -> &fmpz {
        &self.fmpq[0].den
    }

    pub fn den_as_raw_mut(&mut self) -> &mut fmpz {
        &mut self.fmpq[0].den
    }

    pub fn is_zero(&self) -> bool {
        int_to_bool!(unsafe { fmpq_is_zero(self.as_raw()) })
    }

    pub fn is_one(&self) -> bool {
        int_to_bool!(unsafe { fmpq_is_one(self.as_raw()) })
    }

    pub fn is_canonical(&self) -> bool {
        int_to_bool!(unsafe { fmpq_is_canonical(self.as_raw()) })
    }

    pub fn set_ui(&mut self, num: c_ulong) {
        unsafe {
            fmpz_set_ui(self.num_as_raw_mut(), num);
            fmpz_set_ui(self.den_as_raw_mut(), 1);
        }
    }

    pub fn set_fmpz(&mut self, num: &Fmpz) {
        unsafe {
            fmpz_set(self.num_as_raw_mut(), num.as_raw());
            fmpz_set_ui(self.den_as_raw_mut(), 1);
        }
    }

    impl_mut_c_wrapper!(
        canonicalise_mut,
        fmpq_canonicalise,
        (),
        doc = "Canonicalize self"
    );
    impl_mut_c_wrapper!(
        sub_si_mut,
        fmpq_sub_si,
        (x: fmpqref, y: Si),
        doc = "`self = x - y`"
    );
    impl_mut_c_wrapper!(
        add_si_mut,
        fmpq_add_si,
        (x: fmpqref, y: Si),
        doc = "`self = x + y`"
    );
    impl_mut_c_wrapper!(
        mul_si_mut,
        fmpq_mul_si,
        (x: fmpqref, y: Si),
        doc = "`self = x * y`"
    );
    impl_mut_c_wrapper!(
        div_si_mut,
        fmpq_div_si,
        (x: fmpqref, y: Si),
        doc = "`self = x / y`"
    );

    impl_mut_c_wrapper!(
        add_mut,
        fmpq_add,
        (x: fmpqref, y: fmpqref),
        doc = "`self = x + y`"
    );
    impl_mut_c_wrapper!(
        sub_mut,
        fmpq_sub,
        (x: fmpqref, y: fmpqref),
        doc = "`self = x - y`"
    );
    impl_mut_c_wrapper!(
        mul_mut,
        fmpq_mul,
        (x: fmpqref, y: fmpqref),
        doc = "`self = x * y`"
    );
    impl_mut_c_wrapper!(
        div_mut,
        fmpq_div,
        (x: fmpqref, y: fmpqref),
        doc = "`self = x / y`"
    );

    impl_mut_c_wrapper!(
        sub_fmpz_mut,
        fmpq_sub_fmpz,
        (x: fmpqref, y: fmpzref),
        doc = "`self = x - y`"
    );
    impl_mut_c_wrapper!(
        add_fmpz_mut,
        fmpq_add_fmpz,
        (x: fmpqref, y: fmpzref),
        doc = "`self = x + y`"
    );
    impl_mut_c_wrapper!(
        mul_fmpz_mut,
        fmpq_mul_fmpz,
        (x: fmpqref, y: fmpzref),
        doc = "`self = x * y`"
    );
    impl_mut_c_wrapper!(
        div_fmpz_mut,
        fmpq_div_fmpz,
        (x: fmpqref, y: fmpzref),
        doc = "`self = x / y`"
    );

    impl_mut_c_wrapper!(abs_mut, fmpq_abs, (x: fmpqref), doc = "`self = abs(x)`");
    impl_mut_c_wrapper!(inv_mut, fmpq_inv, (x: fmpqref), doc = "`self = x^(-1)`");
    impl_mut_c_wrapper!(neg_mut, fmpq_neg, (x: fmpqref), doc = "`self = -x`");
    impl_mut_c_wrapper!(set, fmpq_set, (x: fmpqref), doc = "`self = x`");
    impl_mut_c_wrapper!(set_zero, fmpq_zero,(),);
    impl_mut_c_wrapper!(set_one, fmpq_one,(),);
    impl_mut_c_wrapper!(set_si, fmpq_set_si, (p: Si, q: Ui), doc = "`self = p/q`");
    impl_mut_c_wrapper!(
        set_fmpz_frac,
        fmpq_set_fmpz_frac,
        (p: fmpzref, q: fmpzref),
        doc = "`self = p/q`"
    );
    impl_mut_c_wrapper!(
        pow_si_mut,
        fmpq_pow_si,
        (x: fmpqref, e: c_long),
        doc = "`self = x^e`"
    );
    impl_mut_c_wrapper!(
        add_mul_mut,
        fmpq_addmul,
        (x: fmpqref, y: fmpqref),
        doc = "`self += x*y`"
    );
    impl_mut_c_wrapper!(sub_mul_mut, fmpq_submul, (x: fmpqref, y: fmpqref),);

    impl_self_mut_call_c!(negate, fmpq_neg, (), doc = "`self = -self`");
    impl_self_mut_call_c!(
        set_pow_si,
        fmpq_pow_si,
        (e: c_long),
        doc = "`self = self^e`"
    );
    impl_self_mut_call_c!(set_inv, fmpq_inv, (), doc = "`self = self^(-1)`");

    /// Call `fmpz_remove` on the numerator of `self` and return the valuation.
    pub fn set_num_remove(&mut self, f: &Fmpz) -> c_long {
        unsafe { fmpz_remove(self.num_as_raw_mut(), self.num_as_raw(), f.as_raw()) }
    }

    /// Similar to `set_num_remove` for denominator.
    pub fn set_den_remove(&mut self, f: &Fmpz) -> c_long {
        unsafe { fmpz_remove(self.den_as_raw_mut(), self.den_as_raw(), f.as_raw()) }
    }

    pub fn mod_fmpz_mut(&self, res: &mut Fmpz, m: &Fmpz) -> Result<(), InverseNotExist> {
        unsafe {
            let a = fmpq_mod_fmpz(res.as_raw_mut(), self.as_raw(), m.as_raw());
            if a == 1 { Ok(()) } else { Err(InverseNotExist) }
        }
    }
}
