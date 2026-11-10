fn main() {

    const PI:f32 = 3.141592;
    println!("PI = : {PI}");

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

    let _val2:i16 = 14;
    let _val3:i32 = 14;
    let _val4:i64 = 14;
    let _val5:i128 = 14;
    let _val6:bool = true;
    let _val7:char = 'c';

    // intentionally unused variables prefixed with _
    let _val8:isize = 44;

    // floating point types
    let _val9:f32 = 4.3;
    let _val10:f64 = 4.1;

    // unsigned versions use u prefix
    let _val11:u32 = 12;
    let _val12:usize = 24;




}
