use std::io;

fn main() {
    let mut value1 = String::new();
    let mut value2 = String::new();

    println!("Enter distance of the car ");
    io::stdin().read_line(&mut value1).expect("Not a valid string");
    let d:f32 = value1.trim().parse().expect("Not a valid number");

    println!("Enter the time spent during");
    io::stdin().read_line(&mut value2).expect("Not a valid string");
    let t:f32 = value2.trim().parse().expect("Not a valid number");

    let speed:f32 = (d*1.609) / t;
    println!("The speed of the car is: {}km/hr",speed );
}
