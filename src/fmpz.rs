use gmp::mpz::Mpz;
use bindings::*;
use std;
use libc::{c_int, c_ulong, c_long};
use std::ffi::CString;
use std::fmt;
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Shr, Shl, ShlAssign, ShrAssign, BitAnd,
               BitOr, BitXor, Mul, Add, Sub};
use std::cmp::Ordering::{self, Greater, Less, Equal};
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Fmpz {
    fmpz: fmpz_t,
}

impl Drop for Fmpz {
    fn drop(&mut self) {
        unsafe {
            fmpz_clear(self.as_mut_ptr());
        }
    }
}

impl Default for Fmpz {
    fn default() -> Self {
        Fmpz::new()
    }
}

impl_part_eq_c!(Fmpz, c_ulong, fmpz_cmp_ui);
impl_part_cmp_c!(Fmpz, c_ulong, fmpz_cmp_ui);

impl_part_eq_c!(Fmpz, c_long, fmpz_cmp_si);
impl_part_cmp_c!(Fmpz, c_long, fmpz_cmp_si);

impl PartialEq for Fmpz {
    fn eq(&self, other: &Fmpz) -> bool {
        unsafe { fmpz_equal(self.as_ptr(), other.as_ptr()) != 0 }
    }
}

impl PartialOrd for Fmpz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(int_to_ord!(
            unsafe { fmpz_cmp(self.as_ptr(), other.as_ptr()) }
        ))
    }
}

define_assign!(Fmpz, AddAssign, add_assign, fmpz_add);
define_assign!(Fmpz, MulAssign, mul_assign, fmpz_mul);
define_assign!(Fmpz, SubAssign, sub_assign, fmpz_sub);
define_assign!(Fmpz, DivAssign, div_assign, fmpz_fdiv_q);

define_assign_c!(Fmpz, AddAssign, add_assign, fmpz_add_ui, c_ulong);
define_assign_c!(Fmpz, SubAssign, sub_assign, fmpz_sub_ui, c_ulong);
define_assign_c!(Fmpz, MulAssign, mul_assign, fmpz_mul_ui, c_ulong);
define_assign_c!(Fmpz, MulAssign, mul_assign, fmpz_mul_si, c_long);

define_assign_c!(Fmpz, ShlAssign, shl_assign, fmpz_mul_2exp, c_ulong);
define_assign_c!(Fmpz, ShrAssign, shr_assign, fmpz_fdiv_q_2exp, c_ulong);

impl_operator!(BitAnd, Fmpz, bitand, fmpz_and);
impl_operator!(BitOr, Fmpz, bitor, fmpz_or);
impl_operator!(BitXor, Fmpz, bitxor, fmpz_xor);

impl_operator!(Mul, Fmpz, mul, fmpz_mul);
impl_operator!(Add, Fmpz, add, fmpz_add);
impl_operator!(Sub, Fmpz, sub, fmpz_sub);

impl_operator_c!(Shl, Fmpz, shl, c_ulong, fmpz_mul_2exp);
impl_operator_c!(Shr, Fmpz, shr, c_ulong, fmpz_fdiv_q_2exp);

impl From<c_long> for Fmpz {
    fn from(x: c_long) -> Fmpz {
        Fmpz::from_si(x)
    }
}

impl<'a> From<&'a Mpz> for Fmpz {
    fn from(x: &Mpz) -> Fmpz {
        unsafe {
            let mut a = Fmpz::new();
            fmpz_set_mpz(a.as_mut_ptr(), x.inner());
            a
        }
    }
}

impl<'a> From<&'a Fmpz> for Mpz {
    fn from(x: &Fmpz) -> Mpz {
        unsafe {
            let mut a = Mpz::new();
            fmpz_get_mpz(a.inner_mut(), x.as_ptr());
            a
        }
    }
}

impl Fmpz {
    pub fn is_even(&self) -> bool {
        unsafe { int_to_bool!(fmpz_is_even(self.as_ptr())) }
    }

    pub fn as_mut_ptr(&mut self) -> fmpzmutptr {
        &mut self.fmpz[0] as fmpzmutptr
    }

    pub fn as_ptr(&self) -> fmpzptr {
        &self.fmpz[0] as fmpzptr
    }

    fn uninitialized() -> fmpz_t {
        unsafe {
            let a: fmpz = std::mem::uninitialized();
            [a] as fmpz_t
        }
    }

    /// Return uninitialized Fmpz.
    pub fn new() -> Fmpz {
        unsafe {
            let mut a = Fmpz::uninitialized();
            fmpz_init(a.as_mut_ptr());
            Fmpz { fmpz: a }
        }
    }

    pub fn from_si(g: c_long) -> Fmpz {
        unsafe {
            let mut a = Fmpz::uninitialized();
            fmpz_init_set_si(a.as_mut_ptr(), g);
            Fmpz { fmpz: a }
        }
    }

    pub fn from_ui(g: c_ulong) -> Fmpz {
        unsafe {
            let mut a = Fmpz::uninitialized();
            fmpz_init_set_ui(a.as_mut_ptr(), g);
            Fmpz { fmpz: a }
        }
    }

    /// self = val
    pub fn set(&mut self, val: &Fmpz) {
        unsafe {
            fmpz_set(self.as_mut_ptr(), val.as_ptr());
        }
    }

    /// self = val
    pub fn set_si(&mut self, val: c_long) {
        unsafe {
            fmpz_set_si(self.as_mut_ptr(), val);
        }
    }

    /// self = val
    pub fn set_ui(&mut self, val: c_ulong) {
        unsafe {
            fmpz_set_ui(self.as_mut_ptr(), val);
        }
    }

    /// self = n + m
    pub fn add_mut(&mut self, n: &Fmpz, m: &Fmpz) {
        unsafe {
            fmpz_add(self.as_mut_ptr(), n.as_ptr(), m.as_ptr());
        }
    }

    /// self = n + m
    pub fn add_ui_mut(&mut self, n: &Self, m: c_ulong) {
        unsafe {
            fmpz_add_ui(self.as_mut_ptr(), n.as_ptr(), m);
        }
    }

    /// self = n * m
    pub fn mul_mut(&mut self, n: &Fmpz, m: &Fmpz) {
        unsafe {
            fmpz_mul(self.as_mut_ptr(), n.as_ptr(), m.as_ptr());
        }
    }

    /// self = n * m
    pub fn mul_ui_mut(&mut self, n: &Fmpz, m: c_long) {
        unsafe {
            fmpz_mul_si(self.as_mut_ptr(), n.as_ptr(), m);
        }
    }

    /// self = g/h. Rounds up towards infinity.
    pub fn cdiv_q_mut(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_cdiv_q(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }

    /// self = g/h. Rounds up towards zero.
    pub fn tdiv_q_mut(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_tdiv_q(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }

    /// self = g/h. Rounds up towards -infinity.
    pub fn fdiv_q_mut(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_fdiv_q(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }


    /// self = g^exp
    pub fn pow_ui_mut(&mut self, g: &Fmpz, exp: c_ulong) {
        unsafe {
            fmpz_pow_ui(self.as_mut_ptr(), g.as_ptr(), exp);
        }
    }

    pub fn get_si_unchecked(&self) -> c_long {
        unsafe { fmpz_get_si(self.as_ptr()) }
    }

    pub fn get_ui_unchecked(&self) -> c_ulong {
        unsafe { fmpz_get_ui(self.as_ptr()) }
    }

    pub fn to_slong(&self) -> Option<c_long> {
        if unsafe { fmpz_fits_si(self.as_ptr()) != 0 } {
            Some(self.get_si_unchecked())
        } else {
            None
        }
    }

    pub fn to_ulong(&self) -> Option<c_ulong> {
        if unsafe { fmpz_abs_fits_ui(self.as_ptr()) != 0 } {
            Some(self.get_ui_unchecked())
        } else {
            None
        }
    }

    pub fn get_string(&self, base: usize) -> String {
        // taken from rust-gmp (cf. https://crates.io/crates/rust-gmp)
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = fmpz_sizeinbase(self.as_ptr(), base as c_int) as usize + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: Vec<u8> = Vec::with_capacity(len);
            vector.set_len(len);

            fmpz_get_str(vector.as_mut_ptr() as *mut _, base as c_int, self.as_ptr());

            let first_nul = vector.iter().position(|i| i == &0).unwrap_or(len);
            vector.truncate(first_nul);
            String::from_utf8(vector).unwrap()
        }
    }

    pub fn from_str(s: &str, base: usize) -> Result<Fmpz, ParseFmpzError> {
        // taken from rust-gmp (cf. https://crates.io/crates/rust-gmp)
        let s = CString::new(s.to_string()).map_err(
            |_| ParseFmpzError { _priv: () },
        )?;
        unsafe {
            assert!(base == 0 || (base >= 2 && base <= 62));
            let mut n = Fmpz::new();
            let r = fmpz_set_str(n.as_mut_ptr(), s.as_ptr(), base as c_int);
            if r == 0 {
                Ok(n)
            } else {
                Err(ParseFmpzError { _priv: () })
            }
        }
    }

    /// Prime factoriazation of self.
    pub fn to_factor(&self) -> FmpzFactor {
        let mut fac = FmpzFactor::new();
        fac.factor_mut(self);
        fac
    }

    impl_mut_c_wrapper!(
        fdiv_r_2exp_mut,
        fmpz_fdiv_r_2exp,
        (x: SelfRef, y: Ui),
        doc = "`self = x mod 2**y`"
    );
    impl_mut_c_wrapper!(
        sub_ui_mut,
        fmpz_sub_ui,
        (x: FmpzRef, y: Ui),
        doc = "`self = x - y`"
    );
    impl_mut_c_wrapper!(
        mod_ui_mut,
        fmpz_mod_ui,
        (x: FmpzRef, y: Ui),
        doc = "`self = x % y`"
    );

    impl_mut_c_wrapper!(
        addmul_mut,
        fmpz_addmul,
        (x: FmpzRef, y: FmpzRef),
        doc = "`self += x * y`"
    );

    impl_mut_c_wrapper!(
        submul_mut,
        fmpz_submul,
        (x: FmpzRef, y: FmpzRef),
        doc = "`self -= x * y`"
    );

    impl_mut_c_wrapper!(
        addmul_ui_mut,
        fmpz_addmul_ui,
        (x: FmpzRef, y: Ui),
        doc = "`self += x * y`"
    );

    impl_mut_c_wrapper!(
        submul_ui_mut,
        fmpz_submul_ui,
        (x: FmpzRef, y: Ui),
        doc = "`self -= x * y`"
    );

    pub fn bits(&self) -> c_ulong {
        unsafe { fmpz_bits(self.as_ptr()) }
    }

    impl_c_wrapper_w_rtype!(
        jacobi,
        fmpz_jacobi,
        i32,
        (p: SelfRef),
        doc = "Return jacobi symbol self mod p"
    );

    impl_c_wrapper_w_rtype!(
        sgn,
        fmpz_sgn,
        i32,
        (),
        doc = "Return -1 if `self < 0`, +1 if `self > 0` 0 otherwise."
    );

    /// Return `valuation(op, f)` and set `self = op/f^e`, where e is the valuation.
    pub fn remove(&mut self, op: &Self, f: &Self) -> mp_limb_signed_t {
        unsafe { fmpz_remove(self.as_mut_ptr(), op.as_ptr(), f.as_ptr()) }
    }

    pub fn set_remove(&mut self, f: &Self) -> c_long {
        unsafe { fmpz_remove(self.as_mut_ptr(), self.as_ptr(), f.as_ptr()) }
    }

    /// Return the hilbert symbol of `hilb(a, b, p)`, where `p` is an odd prime.
    pub fn hilbert_symbol_odd(a: &Self, b: &Self, p: &Self) -> i32 {
        let mut tmp1 = Fmpz::new();
        let mut tmp2 = Fmpz::new();
        Self::_hilbert_symbol_odd(a, b, p, &mut tmp1, &mut tmp2)
    }

    pub fn _hilbert_symbol_odd(
        a: &Self,
        b: &Self,
        p: &Self,
        tmp1: &mut Self,
        tmp2: &mut Self,
    ) -> i32 {
        let val_a = tmp1.remove(a, p);
        let val_b = tmp2.remove(b, p);
        match (is_even!(val_a), is_even!(val_b)) {
            (true, true) => 1,
            (true, false) => tmp1.jacobi(p),
            (false, true) => tmp2.jacobi(p),
            (false, false) => {
                let cond = {
                    *tmp1 *= tmp2 as &Fmpz;
                    tmp2.sub_ui_mut(p, 1);
                    *tmp2 >>= 1;
                    tmp2.is_even()
                };
                if cond {
                    tmp1.jacobi(p)
                } else {
                    -tmp1.jacobi(p)
                }
            }
        }
    }

    pub fn hilbert_symbol_2(a: &Self, b: &Self) -> i32 {
        let two: Fmpz = From::from(2);
        let mut tmp1 = Fmpz::new();
        let mut tmp2 = Fmpz::new();
        Self::_hilbert_symbol_2(a, b, &two, &mut tmp1, &mut tmp2)
    }

    pub fn _hilbert_symbol_2(
        a: &Self,
        b: &Self,
        two: &Self,
        tmp1: &mut Self,
        tmp2: &mut Self,
    ) -> i32 {
        let val_a = tmp1.remove(a, two);
        let val_b = tmp2.remove(b, two);
        let a_eps = tmp1._eps();
        let b_eps = tmp2._eps();
        let a_om = tmp1._omega(a_eps);
        let b_om = tmp2._omega(b_eps);
        let mut res_expt = 0;
        if !is_even!(a_eps) && !is_even!(b_eps) {
            res_expt += 1;
        }
        match (is_even!(val_a), is_even!(val_b)) {
            (true, false) => {
                res_expt += a_om;
            }
            (false, true) => {
                res_expt += b_om;
            }
            (false, false) => {
                res_expt += a_om + b_om;
            }
            _ => (),
        }
        if is_even!(res_expt) { 1 } else { -1 }
    }

    fn _omega(&mut self, eps: i32) -> i32 {
        if eps == 0 {
            *self >>= 1;
            if self.is_even() { 0 } else { 1 }
        } else {
            *self -= 1;
            *self >>= 1;
            if self.is_even() { 1 } else { 0 }
        }
    }

    fn _eps(&mut self) -> i32 {
        *self -= 1;
        *self >>= 1;
        if self.is_even() { 0 } else { 1 }
    }
}

#[derive(Debug)]
pub struct ParseFmpzError {
    _priv: (),
}


pub struct FmpzFactor {
    factor_struct: fmpz_factor_struct,
}

impl Drop for FmpzFactor {
    fn drop(&mut self) {
        unsafe {
            fmpz_factor_clear(&mut self.factor_struct);
        }
    }
}

impl Default for FmpzFactor {
    fn default() -> Self {
        FmpzFactor::new()
    }
}


impl FmpzFactor {
    pub fn new() -> FmpzFactor {
        unsafe {
            let mut a = std::mem::uninitialized();
            fmpz_factor_init(&mut a);
            FmpzFactor { factor_struct: a }
        }
    }

    pub fn factor_mut(&mut self, n: &Fmpz) {
        unsafe { fmpz_factor(&mut self.factor_struct, n.as_ptr()) };
    }

    pub fn factor_si_mut(&mut self, n: c_long) {
        unsafe {
            fmpz_factor_si(&mut self.factor_struct, n);
        }
    }

    /// Evaluates an integer in factored form back to n.
    pub fn factor_expand_iterative(&self, n: &mut Fmpz) {
        unsafe {
            fmpz_factor_expand_iterative(n.as_mut_ptr(), &self.factor_struct);
        }
    }

    pub fn to_vec(&self) -> Vec<(Fmpz, c_long)> {
        let mut v: Vec<(Fmpz, c_long)> = Vec::new();
        let n_p = self.factor_struct.p;
        let exp_p = self.factor_struct.exp;
        for i in 0..self.factor_struct.num {
            let j = i as isize;
            let n = unsafe { Fmpz { fmpz: [*n_p.offset(j)] } };
            let exp = unsafe { *exp_p.offset(j) as c_long };
            v.push((n, exp))
        }
        v
    }
}

impl fmt::Display for Fmpz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_string(10))
    }
}

impl Serialize for Fmpz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.get_string(32), serializer)
    }
}

impl<'de> Deserialize<'de> for Fmpz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let a = String::deserialize(deserializer)?;
        Ok(Fmpz::from_str(&a, 32).unwrap())
    }
}
