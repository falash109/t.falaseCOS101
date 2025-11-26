use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output";
    let dept = "Department of Data Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    
    writeln!(file, "Welcome to Rust Programming").expect("write failed");
    writeln!(file, "{}", announce).expect("write failed");
    writeln!(file, "{}", dept).expect("write failed");
    
    println!("\nData written to file.");
}