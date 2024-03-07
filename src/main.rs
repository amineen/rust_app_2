#![allow(unused)]
mod hash_map_service;
mod point;
mod rectangle;
mod services;

use std::collections::HashMap;

use point::Point;
use rectangle::{square, Rectangle};
// use services::{print_bytes, print_character};

use hash_map_service::{
    count_words_in_text, get_value, insert_value, min_max_scores, print_hash_map, print_scores,
};

fn main() {
    // let p1 = Point { x: 3, y: 4 };

    // let p2 = Point { x: 4, y: 8 };

    // let distance: f32 = p1.compute_distance(&p2);

    // println!(
    //     "The distance between {:?} and {:?} is {:#?}",
    //     p1, p2, distance
    // );

    // let rect1 = Rectangle {
    //     height: 20.0,
    //     width: 30.0,
    // };
    // let rect2 = Rectangle {
    //     height: 10.0,
    //     width: 40.0,
    // };

    // let area: f32 = rect1.compute_area();

    // println!("The area of {:?} is {:?}", rect1, area);

    // let perimeter: f32 = rect1.compute_perimeter();

    // println!("The perimeter of {:?} is {:?}", rect1, perimeter);

    // let sqr: Rectangle = square(10.0);
    // println!("{:?} is a Square", sqr);

    // let name = "Aaron Mineen";
    // print_character(name);
    // print_bytes(name);

    let mut grades: HashMap<String, i32> = HashMap::new();

    insert_value("Aaron", 98, &mut grades);
    insert_value("Alex", 87, &mut grades);
    insert_value("Jenkins", 90, &mut grades);
    insert_value("Eric", 92, &mut grades);

    let aaron_score = get_value("Aaron", &grades);
    println!("Aaron's score is {}", aaron_score);

    let (min, max) = min_max_scores(&grades);

    println!(
        "The minimum score is {} and the maximum score is {}",
        min, max
    );

    print_scores(&grades);

    for i in 0..5 {
        println!("↓");
    }

    let text =
        String::from("My name is Aaron Mineen I live in Zambia I am a Liberian Zambia is great");
    let word_map = count_words_in_text(&text);
    print_hash_map(&word_map);
}
