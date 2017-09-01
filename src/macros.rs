macro_rules! is_even {
    ($expr: expr) => {($expr) & 1 == 0}
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

macro_rules! impl_part_eq_c {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialEq<$c_type> for $t {
            fn eq(&self, other: &$c_type) -> bool {
                unsafe { $c_func(self.as_ptr(), *other) == 0 }
            }
        }
    };
}

macro_rules! impl_part_cmp_c {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialOrd<$c_type> for $t {
            fn partial_cmp(&self, other: &$c_type) -> Option<Ordering> {
                Some(int_to_ord!(unsafe { $c_func(self.as_ptr(), *other) }))
            }
        }
    }
}

macro_rules! impl_operator {
    ($tr: ident, $t: ty, $method: ident, $cfunc: ident) => {
        impl<'a> $tr for &'a $t {
            type Output = $t;
            fn $method(self, other: &$t) -> $t {
                let mut res: $t = Default::default();
                unsafe{
                    $cfunc(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
                }
                res
            }
        }

    }
}

macro_rules! imp_operator_c {
    ($tr: ident, $t: ty, $method: ident, $ct: ty, $cfunc: ident) => {
        impl<'a> $tr<$ct> for &'a $t {
            type Output = $t;
            fn $method(self, other: $ct) -> $t {
                let mut res: $t = Default::default();
                unsafe {
                    $cfunc(res.as_mut_ptr(), self.as_ptr(), other);
                }
                res
            }
        }

    }
}

// copied from rust-gmp/macros.rs
macro_rules! impl_c_wrapper {
    ($meth: ident, $c_func: ident, Ui, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: c_ulong) {
            unsafe {
                $c_func(self.as_mut_ptr(), x);
            }
        }
    };
    ($meth: ident, $c_func: ident, Si, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: c_long) {
            unsafe {
                $c_func(self.as_mut_ptr(), x);
            }
        }
    };
    ($meth: ident, $c_func: ident, $t1: ty, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: &$t1) {
            unsafe {
                $c_func(self.as_mut_ptr(), x.as_ptr());
            }
        }
    };
    ($meth: ident, $c_func: ident, $t: ty, Ui, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: &$t, y: c_ulong) {
            unsafe {
                $c_func(self.as_mut_ptr(), x.as_ptr(), y);
            }
        }
    };
    ($meth: ident, $c_func: ident, $t: ty, Si, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: &$t, y: c_long) {
            unsafe {
                $c_func(self.as_mut_ptr(), x.as_ptr(), y);
            }
        }
    };
    ($meth: ident, $c_func: ident, $t1: ty, $t2: ty, $doc: expr) => {
        #[doc = $doc]
        pub fn $meth(&mut self, x: &$t1, y: &$t2) {
            unsafe {
                $c_func(self.as_mut_ptr(), x.as_ptr(), y.as_ptr());
            }
        }
    };
}
