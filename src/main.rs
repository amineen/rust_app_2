#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn compute_area(&self) -> f32 {
        let area = self.width * self.height;
        area
    }

    fn compute_perimeter(&self) -> f32 {
        let perimeter = 2.0 * self.width + 2.0 * self.height;
        perimeter
    }

    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Point {
    fn compute_distance(&self, p2: &Point) -> f32 {
        let delta_x = p2.x - self.x;
        let delta_y = p2.y - self.y;

        let squared = delta_x.pow(2) + delta_y.pow(2);
        let distance = f32::sqrt(squared as f32);
        // let distance = (squared as f64).sqrt();
        distance
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let p1 = Point { x: 3, y: 4 };

    let p2 = Point { x: 4, y: 8 };

    let distance = p1.compute_distance(&p2);

    println!(
        "The distance between {:?} and {:?} is {:#?}",
        p1, p2, distance
    );

    let rect1 = Rectangle {
        height: 20.0,
        width: 30.0,
    };

    let area = rect1.compute_area();

    println!("The area of {:?} is {:?}", rect1, area);

    let perimeter = rect1.compute_perimeter();

    println!("The perimeter of {:?} is {:?}", rect1, perimeter);

    let square = Rectangle::square(10.0);
    println!("{:?} is a Square", square)
}
