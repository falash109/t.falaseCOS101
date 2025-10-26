use std::io;

fn main() {
    let mut age = String::new();
    let mut level = String::new();

    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("invalid string");
    let age:i32 = age.trim().parse().expect("Invalid entry");

    println!("Are you experienced?");
    io::stdin().read_line(&mut level).expect("Invalid answer");
    let level:String = level.trim().parse().expect("invalid");

    if level == "Yes"{
        if age >= 40{
           let incentive = 1_560_000;
           println!("Your annual incentive is {}",incentive);
        }
        else if 40 > age && age > 30{
            let incentive = 1_480_000;
            println!("Your annual incentive is {}",incentive);
        }
        else if age < 28{
            let incentive = 1_300_000;
            println!("Your annual incentive is {}",incentive);
        }
        
    }
    else{
            let incentive = 100_000;
            println!("Your annual incentive is {}",incentive);
        }
    }