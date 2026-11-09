fn main() {
    // immutable variable
    let x = 12;
    // shadows x
    // let x = 7;

    let mut y = 4;
    println!("Value of y = {y}");
    y = y + 3;
    println!("Value of y = {y}");

    println!("Value of x = {x}");

    // specify type after :
    let val1:i8 = -14;
    println!("value 1 = {val1}");

    let val2:i16 = 14;
    let val3:i32 = 14;
    let val4:i64 = 14;
    let val5:i128 = 14;
    let val6:bool = true;




}
