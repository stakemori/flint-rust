extern crate libc;
extern crate gmp;

// use self::gmp::mpz::mp_limb_t;
// use std::ops::{Shl, Shr, ShlAssign, ShrAssign};
use std::mem::uninitialized;
use bindings::*;
use self::libc::c_long;
use fmpq::Fmpq;
use std::fmt;

#[derive(Debug)]
pub struct FmpqMat {
    fmpq_mat: fmpq_mat_struct,
}

impl fmt::Display for FmpqMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vv = Vec::new();
        for i in 0..self.nrows() {
            let v: Vec<String> = (0..self.ncols())
                .map(|j| format!("{}", self.entry_cloned(i, j)))
                .collect();
            vv.push(format!("[{}]", v.join(", ")));
        }
        write!(f, "{}", vv.join("\n"))
    }
}


impl From<Vec<Vec<Fmpq>>> for FmpqMat {
    fn from(vecs: Vec<Vec<Fmpq>>) -> FmpqMat {
        let nr = vecs.len() as i64;
        let nc = vecs.last().unwrap().len() as i64;
        let mut res = Self::new(nr, nc);
        for (i, v) in vecs.iter().enumerate() {
            for (j, a) in v.iter().enumerate() {
                res.set_entry(i as isize, j as isize, a);
            }
        }
        res
    }
}

impl Drop for FmpqMat {
    fn drop(&mut self) {
        unsafe {
            fmpq_mat_clear(&mut self.fmpq_mat);
        }
    }
}


impl FmpqMat {
    pub fn new(rows: mp_limb_signed_t, cols: mp_limb_signed_t) -> Self {
        unsafe {
            let mut fmpq_mat: fmpq_mat_struct = uninitialized();
            fmpq_mat_init(&mut fmpq_mat, rows, cols);
            FmpqMat { fmpq_mat: fmpq_mat }
        }
    }

    pub fn as_raw(&self) -> &fmpq_mat_struct {
        &self.fmpq_mat
    }

    pub fn as_raw_mut(&mut self) -> &mut fmpq_mat_struct {
        &mut self.fmpq_mat
    }

    pub fn ncols(&self) -> isize {
        self.fmpq_mat.c as isize
    }

    pub fn nrows(&self) -> isize {
        self.fmpq_mat.r as isize
    }


    pub fn entry_raw(&self, r: isize, c: isize) -> &fmpq {
        unsafe {
            debug_assert!(r < self.nrows());
            debug_assert!(c < self.ncols());
            let a = (*self.fmpq_mat.rows.offset(r)).offset(c);
            &*a
        }
    }

    pub fn entry_raw_mut(&self, r: isize, c: isize) -> &mut fmpq {
        unsafe {
            debug_assert!(r < self.nrows());
            debug_assert!(c < self.ncols());
            let a = (*self.fmpq_mat.rows.offset(r)).offset(c);
            &mut *a
        }
    }

    pub fn entry_cloned(&self, r: isize, c: isize) -> Fmpq {
        unsafe {
            let p = self.entry_raw(r, c);
            let mut res = Fmpq::new();
            fmpq_set(res.as_raw_mut(), p);
            res
        }
    }

    pub fn set_entry(&mut self, r: isize, c: isize, x: &Fmpq) {
        unsafe {
            fmpq_set(self.entry_raw_mut(r, c), x.as_raw());
        }
    }

    pub fn rref_mut(&mut self, mat: &Self) -> c_long {
        unsafe { fmpq_mat_rref(self.as_raw_mut(), mat.as_raw()) }
    }

    pub fn right_kernel_basis(&self) -> Vec<Vec<Fmpq>> {
        let m = self.ncols();
        let mut mat = Self::new(self.nrows() as i64, m as i64);
        let r = mat.rref_mut(self) as isize;
        let idcs: Vec<isize> = (0..r)
            .map(|i| {
                (0..m)
                    .filter(|&j| unsafe {
                        !int_to_bool!(fmpq_is_zero(mat.entry_raw(i, j)))
                    })
                    .next()
                    .unwrap()
            })
            .collect();
        (0..m)
            .filter(|&i| idcs.iter().all(|&x| x != i))
            .map(|i| {
                let mut v: Vec<_> = (0..m).map(|_| Fmpq::new()).collect();
                v[i as usize].set_one();
                for (k, &j) in idcs.iter().enumerate() {
                    v[j as usize].set(mat.entry_raw(k as isize, i));
                    v[j as usize].negate();
                }
                v
            })
            .collect()
    }

    pub fn is_zero(&self) -> bool {
        unsafe { int_to_bool!(fmpq_mat_is_zero(self.as_raw())) }
    }

    impl_mut_c_wrapper!(
        mul_mut,
        fmpq_mat_mul,
        (x: SelfRef, y: SelfRef),
        doc = "self = x * y"
    );
    impl_mut_c_wrapper!(
        transpose_mut,
        fmpq_mat_transpose,
        (x: SelfRef),
        doc = "self = x^t"
    );

    impl_mut_c_wrapper_w_rtype!(
        rref,
        fmpq_mat_rref,
        c_long,
        (x: SelfRef),
        doc = "Call `fmpq_mat_rref(self, x)`"
    );
}
