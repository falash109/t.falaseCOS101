// Rust program to convert temperature from Celsius to Farenheit and Kelvin

use std::io;

fn main() {

    // Input the temperature in Celsius
    let mut input1 = String::new;
    println!("n/Input Temperature in celsius: {}", input1);
    io::stdin().read_line(&mut String).expect("Not a valid string!");
    let celc:f32 = input1().trim().parse().expect("Temperature Invalid")

    if celc < 0.0 {
         println!("It's FREEZING!");
    }
    
    else if celc > 30.0 {
        println!("It is HOT!");
    }
    else celc >= 0.0 && celc <= 30.0
    {
        println! ("The Temperature is Normal.");
    }

    // Conversion from Celsius to Farenheit
    let mut input2 = String::new;
    println!("Temperature in Farenheit: {}", input2);
    io::stdin().read_line(&mut String).expect("Temperature Invalid");
    let farh:f32 = input2().trim().parse().expect("Temperature Invalid")


    let farh: f32 = (9.0/5.0) * (celc + 32.0);
    println!("New Temperature in Farenheit: {}", );

    // Conversion from Celsius to Kelvin
    let mut input3 = String::new;
    println!("Temperature in Kelvin: {}", kelv);
    io::stdin().read_line(&mut String).expect("Temperature Invalid");
    let temperature_k:f32 = input3().trim().parse().expect("Temperature Invalid");

    let kelv: f:32 = celc + 273.15
    println!("New Temperature in Kelvin: {}", kelv);
}
    
