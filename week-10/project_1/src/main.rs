struct LaptopBrand {
    unit_cost: u64,
    stock_quantity: u32,
}

impl LaptopBrand {
    fn calculate_purchase_cost(&self, num_purchased: u32) -> u64 {
        self.unit_cost * (num_purchased as u64)
    }
}

fn main() {
    let customer_purchase_quantity: u32 = 3;

    let hp = LaptopBrand {
        unit_cost: 650_000,
        stock_quantity: 10,
    };

    let ibm = LaptopBrand {
        unit_cost: 755_000,
        stock_quantity: 6,
    };

    let toshiba = LaptopBrand {
        unit_cost: 550_000,
        stock_quantity: 10,
    };

    let dell = LaptopBrand {
        unit_cost: 850_000,
        stock_quantity: 4,
    };

    let hp_cost = hp.calculate_purchase_cost(customer_purchase_quantity);
    let ibm_cost = ibm.calculate_purchase_cost(customer_purchase_quantity);
    let toshiba_cost = toshiba.calculate_purchase_cost(customer_purchase_quantity);
    let dell_cost = dell.calculate_purchase_cost(customer_purchase_quantity);

    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("--- Customer Purchase Summary ---");
    println!("Quantity purchased per brand: {} units", customer_purchase_quantity);
    println!("Cost of 3 HP laptops:      {}", hp_cost);
    println!("Cost of 3 IBM laptops:     {}", ibm_cost);
    println!("Cost of 3 Toshiba laptops: {}", toshiba_cost);
    println!("Cost of 3 Dell laptops:    {}", dell_cost);
    println!("-------------------------------");
    println!("Total Cost of Purchase:    {} Naira", total_cost);
}