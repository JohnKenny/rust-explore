// An attribute to hide warnings for unused code.
#![allow(dead_code)]

fn main() {
    
    let temp = 17;
    // if expressions
    let consejo = if temp < 20 { "Lleva la chaqueta" } else {"Es calido"};
    println!("{}", consejo);
    
    // for loop 
    for count in 0..7 {
        print!("{},", count);
    }

    println!();
    // ..= to include last index
    for count in 0..=7 {
        print!("{},", count);
    }

    // counntdown 
    println!();
    for count in (0..7).rev() {
        print!("{} ", count);
    }

    
    // while loop mut var needed
    println!();
    let mut count = 0;

    while count < 11 {
        count += 2;
        print!("{} ", count);
    }

    // loop keyword
    println!();
    count = 0;
    loop {
        count += 1;

        if count > 7 {
            break;
        }
        if count == 3 {
            continue;
        }

        print!("{} ", count);

    }

    // Loop labels
    println!();
    'outer: for x in 0..4 {
        for y in 0..4 {
            print!("{},{} ", x, y);

            if x * y == 6 {
                break 'outer;
            }
        }
        println!();
    }


}
