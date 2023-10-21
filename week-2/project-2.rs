fn main() {
	let t = 450_000;
	let m = 1_500_000;
	let h = 750_000;
	let d = 2_850_000;
	let n = 250_000;
    
    // sum
    let s = n + m + (t * 2) + (h * 3) + (d * 3);
    println!("Sum is {}", s);

    //Average
    let a = s / 10;
    println!("Average is {}", a); 

}