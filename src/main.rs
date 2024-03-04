#![allow(unused)]
mod point;
mod rectangle;

use point::Point;
use rectangle::Rectangle;

fn main() {
    let p1 = Point { x: 3, y: 4 };

    let p2 = Point { x: 4, y: 8 };

    let distance = p1.compute_distance(&p2);

    let data: &str = "Aaron Mineen";

    let s = data.to_string();

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
