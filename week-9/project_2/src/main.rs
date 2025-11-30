use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main(){
    println!("Student Information System");

    let student_name: Vec<&str> = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let mac_number: Vec<&str> = vec!["ACC", "ECO", "CSC", "EEE", "MEE"];
    let dept: Vec<&str> = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level: Vec<&str> = vec!["300", "100", "200", "200", "100"];

    let mut file = File::create("student_info.txt").expect("create failed");
    
    writeln!(file, "Name").expect("write failed");
    for info in student_name {
        writeln!(file, "{}", info).expect("write failed");
    }

    writeln!(file, "Matric Number").expect("write failed");
    for info in mac_number {
        writeln!(file, "{}", info).expect("write failed");
    }

    writeln!(file, "Department").expect("write failed");
    for info in dept {
        writeln!(file, "{}", info).expect("write failed");
    }

    writeln!(file, "Student Level").expect("write failed");
    for info in level {
        writeln!(file, "{}", info).expect("write failed");
    }

    println!("Data written top file successfully");

    let mut file_read = File::open("student_info.txt").expect("Error, student_info.txt does not exist.");

    let mut read_contents = String::new();
    file_read.read_to_string(&mut read_contents).expect("Failed to read content");

    println!("\nReading file contents....");
    println!("{}", read_contents);
}