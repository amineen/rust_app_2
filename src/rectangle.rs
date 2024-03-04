#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn compute_area(&self) -> f32 {
        let area = self.width * self.height;
        area
    }

    pub fn compute_perimeter(&self) -> f32 {
        let perimeter = 2.0 * self.width + 2.0 * self.height;
        perimeter
    }
}

pub fn square(size: f32) -> Rectangle {
    Rectangle {
        width: size,
        height: size,
    }
}
