fn main() {
    // every variable declrearation inside rust is by default immutable means it's value cannot be changd or re assign during runtime
    //section of immutability
    //let x: i32 = 78;
    //println!("The value of x is: {x}");
    //x = 90;
    // println!("The value assigned to x: is:  {x} ");

    //section of mutability
    // let mut x: i8 = 78;
    // println!("The value of current X is : {x}");
    // x = 90;
    // println!("The runtime changed value : {x}");
    // const Y: i8 = 89;
    // println!("The value of current Y is: {Y}");
    // const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    // println!("The value is : {THREE_HOURS_IN_SECONDS}")

    // Shadowing

    // let x: u8 = 5;

    // let x: u8 = x + 1;

    // {
    //     let x: u8 = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");
    let spaces: &str = "    ";
    let spaces: usize = spaces.len();
    println!("{spaces}")
}
