use std::io;

fn main() {
    let mut food_choice = String::new();
    let mut quantity = String::new();

    loop {
        food_choice.clear();
        quantity.clear();

        println!("
        
        
        MENU                          PRICE
        P = Poundo Yam/Edinkaiko Soup -N3,200
        F = Fried Rice & Chicken      -N3,000
        A = Amala & Ewedu Soup        -N2,500
        E = Eba & Egusi Soup          -N2,000
        W = White Rice & Stew         -N2,500
        Enter a letter to select a meal (or press Ctrl+C to exit)");

        io::stdin().read_line(&mut food_choice).expect("not a valid string");
        let food_choice_char = food_choice.trim().to_uppercase().chars().next();

        if food_choice_char.is_none() || !matches!(food_choice_char.unwrap(), 'P' | 'F' | 'A' | 'E' | 'W') {
            println!("Invalid meal selection. Please try again.");
            continue;
        }


        println!("How many portions do you want? (5 max)");
        io::stdin().read_line(&mut quantity).expect("Not a valid string");

        let quantity: f32 = match quantity.trim().parse() {
            Ok(num) => {
                if num > 5.0 || num < 1.0 {
                    println!("Invalid quantity. Please enter a number between 1 and 5.");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Invalid number entered for quantity. Please try again.");
                continue;
            }
        };

        let price_info = match food_choice_char {
            Some('P') => (3200.0, "Poundo Yam/Edinkaiko Soup"),
            Some('F') => (3000.0, "Fried Rice & Chicken"),
            Some('A') => (2500.0, "Amala & Ewedu Soup"),
            Some('E') => (2000.0, "Eba & Egusi Soup"),
            Some('W') => (2500.0, "White Rice & Stew"),
            _ => continue,
        };
        
        let price = price_info.0;
        let meal_name = price_info.1;

        let total_charges = quantity * price;

        if total_charges > 10000.0 {
            let discount_rate = 0.05;
            let discount_amount = total_charges * discount_rate;
            let new_charges = total_charges - discount_amount;
            
            println!("\n✅ Order Summary:");
            println!("Meal: {} (x{})", meal_name, quantity);
            println!("Original Total: N{:.2}", total_charges);
            println!("🎉 Congratulations! You qualify for a 5% discount (N{:.2}).", discount_amount);
            println!("💰 Your discounted total charges is N{:.2}\n", new_charges);

        } else if total_charges <= 10000.0 {
            println!("\n✅ Order Summary:");
            println!("Meal: {} (x{})", meal_name, quantity);
            println!("Your total charges is N{:.2}\n", total_charges);
        }
    }
}