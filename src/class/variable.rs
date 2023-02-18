pub struct Variable<T> {
    contains: T,
}

impl<T> Variable<T> {
    pub fn new(contains: T) -> Self {
        Variable { contains } // this won't work, will figure something else out.
    }
}