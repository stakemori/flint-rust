use bindings::*;
use std;
use libc::c_int;
use std::ffi::CString;

#[derive(Debug)]
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
    fn as_mut_ptr(&mut self) -> fmpzmutptr {
        self.fmpz.as_mut_ptr()
    }

    fn as_ptr(&self) -> fmpzptr {
        self.fmpz.as_ptr()
    }

    fn uninitialized() -> fmpz_t {
        unsafe {
            let a = std::mem::uninitialized();
            [a] as fmpz_t
        }
    }

    /// Return uninitialized Fmpz.
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

    /// self = val
    pub fn set_si(&mut self, val: mp_limb_signed_t) {
        unsafe {
            wrapped_fmpz_set_si(self.as_mut_ptr(), val);
        }
    }

    /// self = val
    pub fn set_ui(&mut self, val: mp_limb_t) {
        unsafe {
            wrapped_fmpz_set_ui(self.as_mut_ptr(), val);
        }
    }

    /// self = g + h
    pub fn add(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_add(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }
    /// self = g * h
    pub fn mul(&mut self, g: &Fmpz, h: &Fmpz) {
        unsafe {
            fmpz_mul(self.as_mut_ptr(), g.as_ptr(), h.as_ptr());
        }
    }


    /// self = g^exp
    pub fn pow_ui(&mut self, g: &Fmpz, exp: mp_limb_t) {
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
            match String::from_utf8(vector) {
                Ok(s) => s,
                Err(_) => panic!("FMpz returned invalid UTF-8!"),
            }
        }
    }

    pub fn from_str(s: &str, base: usize) -> Result<Fmpz, ParseFmpzError> {
        // taken from rust-gmp (cf. https://crates.io/crates/rust-gmp)
        let s = CString::new(s.to_string()).map_err(|_| ParseFmpzError { _priv: () })?;
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
}

#[derive(Debug)]
pub struct ParseFmpzError {
    _priv: ()
}


pub struct FmpzFactor {
    factor_struct: fmpz_factor_struct,
}

impl Drop for FmpzFactor {
    fn drop(&mut self) {
        unsafe{
            fmpz_factor_clear(&mut self.factor_struct);
        }
    }
}

impl FmpzFactor {
    pub fn new() -> FmpzFactor {
        unsafe{
            let mut a = std::mem::uninitialized();
            fmpz_factor_init(&mut a);
            FmpzFactor{factor_struct: a}
        }
    }

    pub fn factor(&mut self, n: &Fmpz) {
        unsafe{
            fmpz_factor(&mut self.factor_struct, n.as_ptr())
        };
    }

    pub fn to_vec(&self) ->  Vec<(Fmpz, mp_limb_signed_t)> {
        let mut v: Vec<(Fmpz, mp_limb_signed_t)> = Vec::new();
        let n_p = self.factor_struct.p;
        let exp_p = self.factor_struct.exp;
        for i in 0..self.factor_struct.num {
            let j = i as isize;
            let n = unsafe{
                Fmpz{fmpz: [*n_p.offset(j)]}
            };
            let exp = unsafe{
                *exp_p.offset(j) as mp_limb_signed_t
            };
            v.push((n, exp))
        }
        v
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut res = Fmpz::new();
        let a = Fmpz::from_str("239023902390239032920930920", 10).unwrap();
        let b = Fmpz::from_si(344349839938948);
        res.mul(&a, &b);
        println!("{}", res.get_str(10));
        res.pow_ui(&a, 12);
        let mut fac = FmpzFactor::new();
        println!("{}", res.get_str(10));
        fac.factor(&b);
        println!("{:?}", fac.to_vec());
    }
}
