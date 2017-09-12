macro_rules! int_to_bool {
    ($a: expr) => {
        $a != 0
    }
}

macro_rules! is_even {
    ($expr: expr) => {($expr) & 1 == 0}
}

macro_rules! define_assign {
    ($t:ty, $trait:ident, $meth:ident, $func:ident) =>
    {
        impl<'a> $trait<&'a Self> for $t {
            fn $meth(&mut self, other: &$t) {
                unsafe {
                    $func(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
                }
            }
        }
    };

    ($t:ty, $trait:ident, $meth:ident, $func:ident) =>
    {
        impl<'a> $trait<&'a mut Self> for $t {
            fn $meth(&mut self, other: &mut $t) {
                unsafe {
                    $func(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
                }
            }
        }
    };

}

macro_rules! define_assign_wref {
    ($t:ty, $trait:ident, $meth:ident, $func:ident, $ty:ty) =>
    {
        impl<'a> $trait<&'a $ty> for $t {
            fn $meth(&mut self, other: &$ty) {
                unsafe {
                    $func(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
                }
            }
        }
    };
}

macro_rules! define_assign_c {
    ($t:ty, $trait:ident, $meth:ident, $func:ident, $typ:ty) =>
    {
        impl $trait<$typ> for $t {
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


macro_rules! impl_neg {
    ($t: ty, $cfunc: ident) => {
        impl<'b> Neg for &'b $t {
            type Output = $t;
            fn neg(self) -> $t {
                unsafe {
                    let mut a: $t = Default::default();
                    $cfunc(a.as_mut_ptr(), self.as_ptr());
                    a
                }
            }
        }
    };
}

macro_rules! impl_operator_c  {
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

macro_rules! impl_c_wrapper_w_rtype {
    ($meth: ident, $c_func: ident, $rtyp: ty, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) -> $rtyp {
            unsafe {
                $c_func(self.as_ptr(), $(__ref_or_val!($t, $x)),*) as $rtyp
            }
        }
    };
}

macro_rules! impl_mut_c_wrapper {
    ($meth: ident, $c_func: ident, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) {
            unsafe {
                $c_func(self.as_mut_ptr(), $(__ref_or_val!($t, $x)),*);
            }
        }
    };
}

macro_rules! __ann_type {
    (SelfRef) => {&Self};
    (SelfRefMut) => {&mut Self};
    (FmpzRef) => {&Fmpz};
    (FmpzRefMut) => {&mut Fmpz};
    (Si) => {c_long};
    (Ui) => {c_ulong};
    ($t: ident) => {$t};
}

macro_rules! __ref_or_val {
    (Si, $val: expr) => {$val};
    (Ui, $val: expr) => {$val};
    (SelfRef, $val: expr) => {$val.as_ptr()};
    (SelfRefMut, $val: expr) => {$val.as_mut_ptr()};
    (FmpzRef, $val: expr) => {$val.as_ptr()};
    (FmpzRefMut, $val: expr) => {$val.as_mut_ptr()};
    ($t: ident, $val: expr) =>  {$val};
}

macro_rules! impl_self_mut_call_c {
    ($meth: ident, $c_func: ident, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) {
            unsafe {
                $c_func(self.as_mut_ptr(), self.as_ptr(), $(__ref_or_val!($t, $x)),*);
            }
        }
    }
}
