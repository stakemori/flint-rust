pub trait SetSelf<T> {
    fn set(&mut self, x: T);
}

pub trait SetCoefficient<T> {
    fn set_coefficient(&mut self, n: i64, x: T);
}

pub trait SetEntry<T> {
    fn set_entry(&mut self, i: isize, j: isize, x: T);
}
