extern crate libc;

use bindings::*;

#[derive(Debug, Clone)]
pub struct FmpzMat {
    fmpz_mat: fmpz_mat_struct,
}

impl Drop for FmpzMat {
    fn drop(&mut self) {
        unsafe {
            fmpz_mat_clear(&mut self.fmpz_mat);
        }
    }
}
