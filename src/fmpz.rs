extern crate libc;

use bindings::*;
use std;
use self::libc::{c_int, c_ulong, c_long};
use std::ffi::CString;
use std::fmt;
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};
use std::cmp::Ordering::{self, Greater, Less, Equal};

#[derive(Debug, Clone)]
pub struct Fmpz {
    pub fmpz: fmpz_t,
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


macro_rules! int_to_ord {
    ($cmp: expr) => {
        {
            let cmp = $cmp;
            if cmp == 0 {
                Equal
            } else if cmp < 0 {
                Less
            } else {
                Greater
            }
        }
    }
}

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

macro_rules! define_assign {
    ($trait:ident, $meth:ident, $func:ident) =>
    {
        impl<'a> $trait<&'a Self> for Fmpz {
            fn $meth(&mut self, other: &Fmpz) {
                unsafe {
                    $func(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
                }
            }
        }
    };

    ($trait:ident, $meth:ident, $func:ident, $typ:ty) =>
    {
        impl $trait<$typ> for Fmpz {
            fn $meth(&mut self, other: $typ) {
                unsafe {
                    $func(self.as_mut_ptr(), self.as_ptr(), other);
                }
            }
        }
    }
}

define_assign!(AddAssign, add_assign, fmpz_add);
define_assign!(MulAssign, mul_assign, fmpz_mul);
define_assign!(SubAssign, sub_assign, fmpz_sub);
define_assign!(DivAssign, div_assign, fmpz_fdiv_q);

define_assign!(AddAssign, add_assign, fmpz_add_ui, c_ulong);
define_assign!(MulAssign, mul_assign, fmpz_mul_ui, c_ulong);
define_assign!(MulAssign, mul_assign, fmpz_mul_si, c_long);

impl From<c_long> for Fmpz {
    fn from(x: c_long) -> Fmpz {
        Fmpz::from_si(x)
    }
}

impl From<c_ulong> for Fmpz {
    fn from(x: c_ulong) -> Fmpz {
        Fmpz::from_ui(x)
    }
}

impl Fmpz {
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

    pub fn get_str(&self, base: usize) -> String {
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

    /// Return jacobi symbol self mod p
    pub fn jacobi(&self, p: &Self) -> i64 {
        unsafe { fmpz_jacobi(self.as_ptr(), p.as_ptr()) as i64 }
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
        write!(f, "{}", self.get_str(10))
    }
}
