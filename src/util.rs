pub trait New {
    fn new() -> Self;
}

pub trait New1<T> {
    fn new(t: T) -> Self;
}

pub fn inject<T>() -> Box<T> where T: New {
    Box::new(T::new())
}

pub fn inject1<T, U>(t: U) -> Box<T> where T: New1<U> {
    Box::new(T::new(t))
}