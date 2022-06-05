/*
  This below is a function.
  It's the main point of the program.
  This function will start executing when we run the program.
*/
fn main() {
    our_own_function("Hello");
}

/*

  We can declare a function with the 'fn' keyword, then our name to the function
  Example:
*/
fn our_own_function(/* Here we declare the inputs we want => */ string: &str) -> &str /* <- Here we declare the output type */
/* Declare the scope -> */ {
    println!("{}", string);
    return string;
} /* <- End the scope */
