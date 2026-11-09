fn main() {
    // immutable variable
    let x = 12;
    // shadows x
    // let x = 7;

    let mut y = 4;
    println!("Value of y = {y}");
    y = y + 3;
    println!("Value of y = {y}");

    println!("Value of x = {x}")
}
