fn main() {
    // 1. Correct the println! macro to use debug formatting ({:?})
    let b: (i32, bool, f64) = (30, true, 4.9);
    println!("The tuple 'b' is: {:?}", b); // Fix: Added format string "{:?}"

    // 2. Call the 'print' function with the tuple 'b'
    print(b); 
}

fn print(x: (i32, bool, f64)) {
    // Correct the println! syntax by closing the format string with double quotes
    println!("Inside print method"); // Fix: Added missing closing quote and colon

    let (age, is_male, cgpa) = x;
    println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}