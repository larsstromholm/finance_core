pub trait Reset {
    fn reset(&mut self);
}

pub trait Period {
    fn period(&self) -> usize;
}

pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

pub trait Open {
    fn open(&self) -> f64;
}

pub trait High {
    fn high(&self) -> f64;
}

pub trait Low {
    fn low(&self) -> f64;
}

pub trait Close {
    fn close(&self) -> f64;
}

pub trait Volume {
    fn volume(&self) -> usize;
}