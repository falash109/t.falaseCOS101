fn get_data() -> (i32, f64, char) {
    // Returns a tuple containing an integer, a float, and a character
    (100, 3.14, 'A')
}

fn main() {
    // Call the function and assign the returned tuple to a variable
    let result_tuple = get_data();

    // Print the entire tuple
    println!("The full tuple is: {:?}", result_tuple);

    // Access individual elements of the tuple by index
    println!("Integer element: {}", result_tuple.0);
    println!("Float element: {}", result_tuple.1);
    println!("Character element: {}", result_tuple.2);
}