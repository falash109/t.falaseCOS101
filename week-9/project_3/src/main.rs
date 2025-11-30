use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

fn main() {
    let initial_data = 
        "Student Name: Tunde, Age: 20\n\
         Student Name: Funke, Age: 22\n";

    let mut file = File::create("students.txt")
        .expect("Failed to create the students.txt file.");

    file.write_all(initial_data.as_bytes())
        .expect("Failed to write initial data to students.txt.");

    let mut file = OpenOptions::new().append(true).create(true).open("students.txt")
        .expect("Failed to append file");

    file.write_all(b"\nStudent Name: Obasanjo").expect("Write failed");

    println!("Data appended succesfully");
}