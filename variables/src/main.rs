fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x now is: {x}");
    const HOURS_IN_DAY: u32 = 24;
    println!("There are {HOURS_IN_DAY} hours in a day.");

    // Shadowing creates multiple copies
    let y = 2;
    let y = y + 2;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}.");
    }

    println!("The value of y is {y}.");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("spaces value: {spaces}.");

    const PI: f64 = 3.14;

    println!("The value of PI is {PI}.");

    let difference = 10.5 - 5.2;
    println!("diff: {difference}");

    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.23 = {quotient}");

    let t = true;
    println!("T: {t}");

    let _z: char = 'z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a, _b, _c) = tup;
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("first in tuple: {first}, second: {second}, third: {third}");

    let arr = [1,2,3,4,5];
    let f = arr[0];
    println!("First elem in arr: {f}")
}
