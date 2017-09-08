use bindings::*;
use libc::{c_long, c_ulong};
use std;
use fmpz::Fmpz;
use std::fmt;
use std::cmp::Ordering::{self, Greater, Less, Equal};

#[derive(Debug, Clone)]
pub struct Fmpq {
    pub fmpq: fmpq_t,
}

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

    pub fn set_num_den(&self, num: &mut Fmpz, dnm: &mut Fmpz) {
        unsafe {
            fmpq_get_fmpz(self.as_ptr(), num.as_mut_ptr(), dnm.as_mut_ptr());
        }
    }
}
