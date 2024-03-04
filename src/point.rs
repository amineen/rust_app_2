#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn compute_distance(&self, p2: &Point) -> f32 {
        let delta_x = p2.x - self.x;
        let delta_y = p2.y - self.y;

        let squared = delta_x.pow(2) + delta_y.pow(2);
        let distance = f32::sqrt(squared as f32);
        distance
    }
}
