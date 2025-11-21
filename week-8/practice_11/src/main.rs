fn main() {
    // Creating a mutable vector of i32
    let mut v = vec![1, 2, 3, 4, 5];

    // Iterating over the elements of the vector immutably
    for i in &v {
        println!("i is {}", i);
    }

    // Iterating over the elements of the vector mutably
    for i in &mut v {
        *i += 10; // Dereference and add 10 to each element
    }
    
    // Printing the modified vector
    println!("Modified Vector: {:?}", v);
}