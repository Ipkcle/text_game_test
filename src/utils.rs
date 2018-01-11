#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl Vec2<i32> {
    pub fn add(&mut self, vec: Vec2<i32>) {
        self.x += vec.x;
        self.y += vec.y;
    }

    pub fn set(&mut self, x: i32, y:i32) {
        self.x = x;
        self.y = y;
    }
}
