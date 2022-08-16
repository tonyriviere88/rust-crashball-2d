use bevy::prelude::Vec3;

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn left(self: &Self) -> f32 {
        self.x
    }

    pub fn right(self: &Self) -> f32 {
        self.x + self.w
    }

    pub fn top(self: &Self) -> f32 {
        self.y + self.h
    }

    pub fn bottom(self: &Self) -> f32 {
        self.y
    }

    pub fn top_left(self: &Self) -> Vec3 {
        Vec3 {
            x: self.left(),
            y: self.top(),
            z: 0.0,
        }
    }

    pub fn bottom_left(self: &Self) -> Vec3 {
        Vec3 {
            x: self.left(),
            y: self.bottom(),
            z: 0.0,
        }
    }

    pub fn top_right(self: &Self) -> Vec3 {
        Vec3 {
            x: self.right(),
            y: self.top(),
            z: 0.0,
        }
    }

    pub fn bottom_right(self: &Self) -> Vec3 {
        Vec3 {
            x: self.right(),
            y: self.bottom(),
            z: 0.0,
        }
    }

    pub fn top_middle(self: &Self) -> Vec3 {
        Vec3 {
            x: (self.left() + self.right()) / 2.0,
            y: self.top(),
            z: 0.0,
        }
    }

    pub fn bottom_middle(self: &Self) -> Vec3 {
        Vec3 {
            x: (self.left() + self.right()) / 2.0,
            y: self.bottom(),
            z: 0.0,
        }
    }

    pub fn left_middle(self: &Self) -> Vec3 {
        Vec3 {
            x: self.left(),
            y: (self.top() + self.bottom()) / 2.0,
            z: 0.0,
        }
    }

    pub fn right_middle(self: &Self) -> Vec3 {
        Vec3 {
            x: self.right(),
            y: (self.top() + self.bottom()) / 2.0,
            z: 0.0,
        }
    }
}

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
