// Rust program to read the height of a person
// and then print if person is tall, dwarf, 
// or average height person 

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter yor height in centimetres: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        print!("You are a person of average height");
    }
    else if height > 170.0 && height <= 195.0
    {
        print!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        print!("You are dwarf");
    }
    else {
        print!("Abnormal height");
    }
}
