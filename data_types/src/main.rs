/*
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.
*/
fn main() {
    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("hep{guess}");
    /*
        Scalar Types

    A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
         */

    /*Table 3-1: Integer Types in Rust

    Length	Signed	Unsigned
    8-bit	i8	u8
    16-bit	i16	u16
    32-bit	i32	u32
    64-bit	i64	u64
    128-bit	i128	u128
    arch	isize	usize
    */
    /*
    Table 3-2: Integer Literals in Rust
    Number literals	Example
    Decimal	98_222
    Hex	0xff
    Octal	0o77
    Binary	0b1111_0000
    Byte (u8 only)	b'A'

         */
    /*
    //floating point types
    let x = 2.0;
    let y: f32 = 3.5;
    println!("{x}");
    println!("{y}");
    //Numeric Operations

    // addition
    let sum: i32 = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product: i32 = 4 * 30;
    println!("{product}");

    // division
    let quotient: f64 = 56.7 / 32.2;
    println!("{quotient}");

    let truncated: i32 = -5 / 3; // Results in -1
    println!("{truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
    let c: char = 'z';
    println!("{c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("{z}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}")
    */

    /*Compound Types

    Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. */
    /*The Tuple Type

    A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example: */
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //println!("{tup:#?}");
    //destructuring
    // let (_x, y, _z) = tup;
    // println!(" The value of y is  {y}");
    // let five_hundred = tup.0;
    // println!("0 index value is {five_hundred}");
    // let flo = tup.1;
    // println!("index value of 1 will be {flo}");
    // let two = tup.2;
    // println!("{two}")
    /*
        This program creates the tuple x and then accesses each element of the tuple using their respective indices. As with most programming languages, the first index in a tuple is 0.

    The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
         */

    //The Array Type

    // let a: [i8; 7] = [1, 2, 3, 4, 5, 6, 7];
    // println!("{a:#?}");

    //Months
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{months:#?}");
}
