fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x += 6;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");   

    let y = 5;

    let y = y * 2;

    {
        let y = y * 3;

        println!("The value of y is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Not Allowed
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");

}