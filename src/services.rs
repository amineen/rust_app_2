pub fn print_character(str_value: &str) {
    for c in str_value.chars() {
        println!("{}", c);
    }
}

pub fn print_bytes(str_value: &str) {
    for b in str_value.bytes() {
        println!("{}", b);
    }
}
