#![allow(unused)]
mod point;
mod rectangle;
mod services;

use point::Point;
use rectangle::{square, Rectangle};
use services::{print_bytes, print_character};

fn main() {
    let p1 = Point { x: 3, y: 4 };

    let p2 = Point { x: 4, y: 8 };

    let distance: f32 = p1.compute_distance(&p2);

    println!(
        "The distance between {:?} and {:?} is {:#?}",
        p1, p2, distance
    );

    let rect1 = Rectangle {
        height: 20.0,
        width: 30.0,
    };
    let rect2 = Rectangle {
        height: 10.0,
        width: 40.0,
    };

    let area: f32 = rect1.compute_area();

    println!("The area of {:?} is {:?}", rect1, area);

    let perimeter: f32 = rect1.compute_perimeter();

    println!("The perimeter of {:?} is {:?}", rect1, perimeter);

    let sqr: Rectangle = square(10.0);
    println!("{:?} is a Square", sqr);

    let name = "Aaron Mineen";
    print_character(name);
    print_bytes(name);
}
