fn main() {
    // every variable declrearation inside rust is by default immutable means it's value cannot be changd or re assign during runtime
    //section of immutability
    //let x: i32 = 78;
    //println!("The value of x is: {x}");
    //x = 90;
    // println!("The value assigned to x: is:  {x} ");

    //section of mutability
    let mut x: i32 = 78;
    println!("The value of current X is : {x}");
    x = 90;
    println!("The runtime changed value : {x}");
    const y: i128 = 89;
    println!("The value of current Y is: {y}");
    const three_hours_in_seconds: u64 = 60 * 60 * 3;
    println!("The value is : {three_hours_in_seconds}")
}
