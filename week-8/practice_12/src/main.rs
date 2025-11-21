fn main() {
    // Array to be sliced
    let a = [10, 20, 30, 40, 50, 60];

    // Slice using the '..' operator for a range
    let slice1 = &a[2..5]; // Elements at index 2, 3, and 4

    // Slice from the start to index 3 (exclusive)
    let slice2 = &a[..3]; // Elements at index 0, 1, and 2

    // Slice from index 3 to the end
    let slice3 = &a[3..]; // Elements at index 3, 4, and 5

    // Slice of the entire array
    let slice4 = &a[..];

    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
    println!("slice3: {:?}", slice3);
    println!("slice4: {:?}", slice4);
}