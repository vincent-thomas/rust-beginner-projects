fn main() {
    /*
        This variable below is a string that has the 'String' type.
    */
    let string = String::from("hello world");

    /*

        We can create a variable that refers to this variable above.

    */
    #[warn(unused_variables)]
    let string_pointer = &string;

    // We are required to use these in certain situations. for example when we pass variables to functions
    print_string(string_pointer);

    /* If we don't do this, it will unvalidate the variable. The rust compiler will bring the variable out of this scope.
    If you would like to see the error, uncomment the 2 lines below. */
    // print_string_without_pointers(string);
    // print_string_without_pointers(string);

    // I will explain why this happens in another project.
}

fn print_string(string: &String) {
    println!("{}", string);
}
#[warn(dead_code)]
fn print_string_without_pointers(string: String) {
    println!("{}", string);
}
