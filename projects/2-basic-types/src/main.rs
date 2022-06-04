#[warn(unused_variables)]

fn main() {
    /*
        Theres two types of string types in Rust:
        1. String: This is a string that is stored in the heap. It is a contiguous sequence of bytes. It is mutable.
        2. &str: This is a string slice. It is a reference to a string stored in the heap. It is immutable.
    */
    let string1: String = String::from("Hello, world!");
    let string2: &str = "Hello, world!";

    /*
    Rust has many built-in types for integers, these include:
        i8, i16, i32, i64, isize
        u8, u16, u32, u64, usize

        i8 can store from -128 to 127
        u8 stores from 0 to 255

        i = signed
        u = unsigned

        signed integers are more common than unsigned integers
        they can store negative numbers when unsigned integers can't

        u8 < 255
        u16 < 65535
        u32 < 4294967295
        u64 < 18446744073709551615
        u128 < 340282366920938463463374607431768211455

        i8 can store from -128 to 127
        i16 can store from -32768 to 32767
        i32 can store from -2147483648 to 2147483647
        i64 can store from -9223372036854775808 to 9223372036854775807
        i128 can store from -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727

    */
    let int_u8: u8; // = number_here
    let int_u16: u16;
    let int_u32: u32;
    let int_u64: u64;
    let int_u128: u128;

    let int_i8: i8;
    let int_i16: i16;
    let int_i32: i32;
    let int_i64: i64;
    let int_i128: i128;

    /*
    Rust also has array-like types, the most used are:
        Vector: A growable array. It is a collection of elements of the same type.
            This is mutable, and can be resized. Size does not need to be known at compile time.

        Array: A fixed-size array. It is a collection of elements of the same type.

        Examples:
    */

    // Using Array type, immutable, fixed-size through the type declaration
    let array1: [i32; 5] = [1, 2, 3, 4, 5];

    // Using Vector type, mutable, growable
    // To make a variable mutable we need to add the 'mut' keyword to the variable declaration
    let mut vector1: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Notice we can remove and add items from the vector

    // Pushes a new variable to the end of the vector
    vector1.push(6);

    // Removes the item from the index specified, this returns a variable of the item removed
    // One could store it in a variable like this: 'let removed_item = vector1.remove(0);'
    vector1.remove(0);

    println!("This {:?} should equal [2, 3, 4, 5, 6]", vector1)
}
