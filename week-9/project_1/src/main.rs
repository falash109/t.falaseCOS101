use std::fs::File;
use std::io::Write;

fn main() {
    println!("\nNigerian Breweries beer category list");
    
    let larger_drinks: Vec<&str> = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout_drinks: Vec<&str> = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic_drinks: Vec<&str> = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("product.txt").expect("create failed");

    writeln!(file, "Larger").expect("write failed");
    for drink in larger_drinks {

        writeln!(file, "{}", drink).expect("write failed");
    }

    writeln!(file, "Stout").expect("write failed");
    for drink in stout_drinks {
        writeln!(file, "{}", drink).expect("write failed");
    }

    writeln!(file, "Non-Alcoholic").expect("write failed");
    for drink in non_alcoholic_drinks {
        writeln!(file, "{}", drink).expect("write failed");
    }
    println!("Data successfully written to file");
}