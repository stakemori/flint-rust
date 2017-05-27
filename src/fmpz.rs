use bindings::*;
use std;
use libc::c_int;

pub struct Fmpz {
    fmpz: fmpz_t,
}

impl Drop for Fmpz {
    fn drop(&mut self) {
        unsafe {
            wrapped_fmpz_clear(self.as_mut_ptr());
        }
    }
}

impl Fmpz {
    fn as_mut_ptr(&mut self) -> fmpzptr {
        self.fmpz.as_mut_ptr()
    }

    fn as_ptr(&self) -> fmpzconstptr {
        self.fmpz.as_ptr()
    }

    fn uninitialized() -> fmpz_t {
        unsafe {
            let a = std::mem::uninitialized();
            [a] as fmpz_t
        }
    }

    pub fn new() -> Fmpz {
        unsafe {
            let mut a = Fmpz::uninitialized();
            wrapped_fmpz_init(a.as_mut_ptr());
            Fmpz { fmpz: a }
        }
    }

    pub fn from_si(g: mp_limb_signed_t) -> Fmpz {
        unsafe {
            let mut a = Fmpz::uninitialized();
            warpped_fmpz_init_set_si(a.as_mut_ptr(), g);
            Fmpz { fmpz: a }
        }
    }

    pub fn fmpz_set_si(&mut self, val: mp_limb_signed_t) {
        unsafe {
            wrapped_fmpz_set_si(self.as_mut_ptr(), val);
        }
    }

    pub fn fmpz_set_ui(&mut self, val: mp_limb_t) {
        unsafe {
            wrapped_fmpz_set_ui(self.as_mut_ptr(), val);
        }
    }

    /// self = g + h
    pub fn fmpz_add(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_add(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }
    /// self = g * h
    pub fn fmpz_mul(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_mul(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }


    /// self = g^exp
    pub fn fmpz_pow_ui(&mut self, g: &Fmpz, exp: mp_limb_t) {
        unsafe{
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
            match String::from_utf8(vector) {
                Ok(s) => s,
                Err(_) => panic!("FMpz returned invalid UTF-8!"),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut res = Fmpz::new();
        let a = Fmpz::from_si(12);
        let b = Fmpz::from_si(20);
        res.fmpz_mul(&a, &b);
        println!("{}", res.get_str(10));
        res.fmpz_pow_ui(&a, 12);
        println!("{}", res.get_str(10));
    }
}
