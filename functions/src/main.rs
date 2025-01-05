fn main() {
    println!("Hello, world!");
    another_function();
    printnum(24);
    let y = 5;
    print_labeled_measurement(y, 'h');
    let ret = five();
    println!("Return value for five(): {ret}.");
}

fn another_function() {
    println!("Another function.")
}

fn printnum(x: i32) {
    println!("The value of x is: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}.")
}

fn five() -> i32 {
    // returns 5 implicitly
    return 5;
}
