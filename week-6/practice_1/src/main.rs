fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("  Name: {}", name)
    println!(" University: {}", uni)

let ddpatment:&'static str = "Computer Science";
let school:&'static str = "School of Science and Technilogy";
println!("Department: {}, \nSchool: {}", department,school);
}