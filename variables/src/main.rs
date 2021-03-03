const MAX_POINTS: u32 = 100_000; // readability

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant
    println!("The maximum number of points is: {}", MAX_POINTS);

    // shadowing
    let x = x + 5;
    println!("The value of x is: {}", x);
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    let spaces = "   ";
    println!("spaces: [{}]", spaces);
    let spaces = spaces.len();
    println!("spaces: [{}]", spaces);

    // Shadowing is different from marking a variable as mut
    // let mut spaces = "   ";
    // spaces = spaces.len(); // rustc --explain E0308
}
