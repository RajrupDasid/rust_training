/*
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.
*/
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("hep{guess}");
    /*
        Scalar Types

    A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
         */
}
