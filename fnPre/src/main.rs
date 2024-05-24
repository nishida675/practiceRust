use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f64 = input.trim().parse()
        .expect("Please type a valid integer!");

    let x = fahrenheitConverter(input);
    println!("華氏 is: {}", x);
    // another_function(5);

    //print_labeled_measurement(5, 'h');

    //let x = plus_one(5);

    //println!("The value of x is: {}", x);
}

fn fahrenheitConverter(x: f64) -> f64{
   x * 1.8 + 32.0
}
/* fn plus_one(x: i32) -> i32 {
    x + 1
} */

/*
fn another_function() {
    println!("Another function.");  // 別の関数
} 

fn another_function(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
} */

/* fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
} */