fn main() {
    run(3, 4.7);
    let value = 4;
    let result = add_five(value);
    println!("{value}, {result}");

    // creates a mutable String
    let s = String::from("Hola ");
    let s = greet(s);
    println!("{s}");
    
}

fn greet(mut s: String) -> String{
    s.push_str("ahi");
    return s;

}

// default return type of function is called unit -> ()
fn run(x:i32, y:f64) -> (){
    println!("{x}, {y}");
}

fn add_five(i:i32) -> i32 {
    // returns result from if expression
    if i < 0 {
        0
    }
    else {
        i + 5
    }
}