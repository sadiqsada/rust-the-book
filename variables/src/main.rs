fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS:u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is {}", x);

    // Shadowing different types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    // Cannot use mut, not allowed to mutate a variable's type

    // let mut spaces = "    ";
    // spaces = spaces.len();

    // This will lead to an error.



}
