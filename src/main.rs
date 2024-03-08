#![allow(unused)]
mod file_services;
mod hash_map_service;
mod point;
mod power_consumption;
mod rectangle;
mod services;

use std::collections::HashMap;

use point::Point;
use rectangle::{square, Rectangle};
use services::{print_bytes, print_character};

use hash_map_service::{
    count_words_in_text, get_value, insert_value, min_max_scores, print_hash_map, print_scores,
};

use file_services::{file_exists, read_file_content, write_text_to_file};

use power_consumption::create_power_consumptions;

struct PowerConsumption {
    datetime: String,
    active_power: f32,
    reactive_power: f32,
    power_factor: f32,
}

fn main() {
    // create_point();
    // create_rectangle();
    // print_characters();
    // create_hash_map_services();

    let file_name = "power_consumption.csv";

    //create an array of power consumption data
    let power_consumption_data = create_power_consumptions("2023-01-02", 12.0);

    let mut power_consumption_text = String::new();

    //check if the file exists
    if !file_exists(file_name) {
        let header = "datetime,active_power,energy_used,power_factor\n";
        power_consumption_text.push_str(header);
    }

    for data in power_consumption_data {
        let line = format!(
            "{},{},{},{}\n",
            data.datetime, data.active_power, data.energy_used, data.power_factor
        );
        power_consumption_text.push_str(&line);
    }

    write_text_to_file(file_name, &power_consumption_text);
}

fn create_point() {
    let p1 = Point { x: 3, y: 4 };

    let p2 = Point { x: 4, y: 8 };

    let distance: f32 = p1.compute_distance(&p2);

    println!(
        "The distance between {:?} and {:?} is {:#?}",
        p1, p2, distance
    );
}

fn create_rectangle() {
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
}

fn print_characters() {
    let name = "Aaron Mineen";
    print_character(name);
    print_bytes(name);
}

fn create_hash_map_services() {
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
