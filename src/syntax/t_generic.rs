pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<usize> {
    pub fn usize_x(&self) -> &usize {
        &self.x
    }
}
