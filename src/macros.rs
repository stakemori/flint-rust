macro_rules! int_to_bool {
    ($a: expr) => {
        $a != 0
    }
}

macro_rules! is_even {
    ($expr: expr) => {($expr) & 1 == 0}
}

macro_rules! div_check {
    (DivAssign, $x: ident) => {debug_assert!(!$x.is_zero());};
    ($a: ident, $x: ident) => {}
}

macro_rules! div_check_c {
    (DivAssign, $x: ident) => {debug_assert!($x != 0);};
    ($a: ident, $x: ident) => {}
}

macro_rules! define_assign_wref {
    ($t:ty, $trait:ident, $meth:ident, $func:ident, $ty:ty) =>
    {
        impl<'a> $trait<&'a $ty> for $t {
            fn $meth(&mut self, other: &$ty) {
                unsafe {
                    div_check!($trait, other);
                    $func(self.as_raw_mut(), self.as_raw(), other.as_raw());
                }
            }
        }
    };
}

macro_rules! define_assign_with_ptr {
    ($t:ty, $trait:ident, $meth:ident, $func:ident, $ty:ty) =>
    {
        impl<'a> $trait<&'a $ty> for $t {
            fn $meth(&mut self, other: &$ty) {
                unsafe {
                    $func(self.as_raw_mut(), self.as_raw(), other);
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
                    div_check_c!($trait, other);
                    $func(self.as_raw_mut(), self.as_raw(), other);
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
                unsafe { $c_func(self.as_raw(), *other) == 0 }
            }
        }
    };
}

macro_rules! impl_part_cmp_c {
    ($t: ty, $c_type: ty, $c_func: ident) => {
        impl PartialOrd<$c_type> for $t {
            fn partial_cmp(&self, other: &$c_type) -> Option<Ordering> {
                Some(int_to_ord!(unsafe { $c_func(self.as_raw(), *other) }))
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
                    $cfunc(res.as_raw_mut(), self.as_raw(), other.as_raw());
                }
                res
            }
        }

    }
}


macro_rules! impl_operator_w_ref {
    ($tr: ident, $t: ty, $method: ident, $cfunc: ident, $to: ty) => {
        impl<'a, 'b> $tr<&'b $to> for &'a $t {
            type Output = $t;
            fn $method(self, other: &$to) -> $t {
                let mut res: $t = Default::default();
                unsafe{
                    $cfunc(res.as_raw_mut(), self.as_raw(), other.as_raw());
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
                    $cfunc(a.as_raw_mut(), self.as_raw());
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
                    $cfunc(res.as_raw_mut(), self.as_raw(), other);
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
                $c_func(self.as_raw(), $(__ref_or_val!($t, $x)),*) as $rtyp
            }
        }
    };
}

macro_rules! impl_mut_c_wrapper_w_rtype {
    ($meth: ident, $c_func: ident, $rtyp: ty, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) -> $rtyp {
            unsafe {
                $c_func(self.as_raw_mut(), $(__ref_or_val!($t, $x)),*) as $rtyp
            }
        }
    };
}

macro_rules! impl_mut_c_wrapper {
    ($meth: ident, $c_func: ident, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) {
            unsafe {
                $c_func(self.as_raw_mut(), $(__ref_or_val!($t, $x)),*);
            }
        }
    };
}

macro_rules! __ann_type {
    (SelfRef) => {&Self};
    (SelfRefMut) => {&mut Self};
    (FmpzRef) => {&Fmpz};
    (FmpzRefMut) => {&mut Fmpz};
    (FmpqRef) => {&Fmpq};
    (FmpqRefMut) => {&mut Fmpq};
    (Si) => {c_long};
    (Ui) => {c_ulong};
    (fmpzref) => {&fmpz};
    (fmpzrefmut) => {&mut fmpz};
    (fmpqref) => {&fmpq};
    (fmpqrefmut) => {&mut fmpq};
    ($t: ident) => {$t};
}

macro_rules! __ref_or_val {
    (Si, $val: expr) => {$val};
    (Ui, $val: expr) => {$val};
    (SelfRef, $val: expr) => {$val.as_raw()};
    (SelfRefMut, $val: expr) => {$val.as_raw_mut()};
    (FmpzRef, $val: expr) => {$val.as_raw()};
    (FmpzRefMut, $val: expr) => {$val.as_raw_mut()};
    (FmpqRef, $val: expr) => {$val.as_raw()};
    (FmpqRefMut, $val: expr) => {$val.as_raw_mut()};
    ($t: ident, $val: expr) =>  {$val};
}

macro_rules! impl_self_mut_call_c {
    ($meth: ident, $c_func: ident, ($($x:ident: $t:ident),*), $($m: meta),*) => {
        $(#[$m])*
        pub fn $meth(&mut self, $($x: __ann_type!($t)),*) {
            unsafe {
                $c_func(self.as_raw_mut(), self.as_raw(), $(__ref_or_val!($t, $x)),*);
            }
        }
    }
}
