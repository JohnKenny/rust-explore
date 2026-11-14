fn main() {
    /* Compound types: arrays and tuples */
    
    // Arrays:
    // must use mut keyword to change array values
    let mut nums = [1,2,3,4,5,6,7];
    println!("{:?}", nums);

    nums[0]= 9;
    println!("{:?}", nums);

    // looping
    for i in nums {
        println!("{i}");
    }

    let mut nums:[u32; 4]= [4,3,2,1];
    println!("{:?}", nums);

    // creates array of four threes.
    let nums = [3; 4];
    println!("{:?}", nums);

    // Tuples:
    let mut values = ("Ola", 4);
    println!("{:?}", values);

    // accessing values
    let x = values.0;
    let y = values.1;

    println!("{x}, {y}");

    // changing value
    values.1 = 5;
    println!("{:?}", values);

    // initialises an empty array.
    let mut values:(u32, f64);
    values = (4, 3.7);
    println!("{:?}", values);
    values.0 = 7;
    values.1 = 4.4;
    println!("{:?}", values);




}
 