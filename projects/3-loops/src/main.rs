fn main() {
    let loop_with_array: [i32; 5] = [0, 1, 2, 3, 4];
    for item in loop_with_array.iter() {
        println!("For loop with arrays: {}", item);
    }

    let loop_with_range = 0..5;

    for item in loop_with_range.into_iter() {
        println!("For loop with range:  {}", item);
    }

    let loop_with_vector = vec![0, 1, 2, 3, 4];

    for item in loop_with_vector {
        println!("For loop with vector: {}", item);
    }
}
