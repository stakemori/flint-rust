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

macro_rules! impl_part_eq {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialEq<$c_type> for $t {
            fn eq(&self, other: &$c_type) -> bool {
                unsafe { $c_func(self.as_ptr(), *other) == 0 }
            }
        }
    };
}

macro_rules! impl_part_cmp {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialOrd<$c_type> for $t {
            fn partial_cmp(&self, other: &$c_type) -> Option<Ordering> {
                Some(int_to_ord!(unsafe { $c_func(self.as_ptr(), *other) }))
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
