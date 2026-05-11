use std::collections::HashMap;
use std::io::{self, Write,};

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

fn main() {
    // (A Vec works for Stage 1, but HashMap shines in Stages 2 & 3)
    let mut bills: HashMap<String, Bill> = HashMap::new();

    loop {
        print_menu();
        match get_menu_choice() {
            Some(1) => add_bill(&mut bills),
            Some(2) => view_bills(&bills),
            Some(3) => remove_bill(&mut bills),
            Some(4) => edit_bill(&mut bills),
            Some(5) => {
                println!("\n👋 Exiting. Goodbye!");
                break;
            }
            Some(_) => println!("⚠️  Invalid option. Please choose 1-5."),
            None => println!("⚠️  Input error. Try again."),
        }
    }
}

fn print_menu() {
    println!("\n========== 💰 Bill Manager ==========");
    println!("1. Add a bill");
    println!("2. View all bills");
    println!("3. Remove a bill");
    println!("4. Edit a bill");
    println!("5. Exit");
    println!("Enter your choice (1-5): ");
}

fn get_menu_choice() -> Option<u8> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None, 
        Ok(_) => input.trim().parse().ok(),
        Err(_) => None,
    }
}

// **************** STAGE 1 ***************
fn add_bill(bills: &mut HashMap<String, Bill>) {
    print!("Enter bill name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    if io::stdin().read_line(&mut name).is_err() || name.trim().is_empty() {
        println!("⚠️  Invalid name. Returning to menu.");
        return;
    }
    let name = name.trim().to_string();

    print!("Enter amount owed: ");
    io::stdout().flush().unwrap();
    let mut amount_str = String::new();
    if io::stdin().read_line(&mut amount_str).is_err() {
        println!("⚠️  Input error. Returning to menu.");
        return;
    }

    match amount_str.trim().parse::<f64>() {
        Ok(amount) if amount >= 0.0 => {
            bills.insert(name.clone(), Bill { name, amount });
            println!("✅ Bill added successfully!");
        }
        _ => println!("⚠️  Invalid amount. Please enter a positive number."),
    }
}

fn view_bills(bills: &HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("📭 No bills registered yet.");
        return;
    }

    // Sort for consistent, readable output
    let mut sorted: Vec<&Bill> = bills.values().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    println!("\n📊 Current Bills:");
    let total: f64 = sorted.iter().map(|b| b.amount).sum();
    for (i, bill) in sorted.iter().enumerate() {
        println!("{}. {} - ${:.2}", i + 1, bill.name, bill.amount);
    }
    println!("💰 Total: ${:.2}", total);
}

// *************** STAGE 2 *****************
fn remove_bill(bills: &mut HashMap<String, Bill>) {
    print!("Enter bill name to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();


    if bills.remove(input).is_some() {
        println!("✅ Bill '{}' removed.", input);
    } else {
        println!("⚠️ Bill '{}' not found.", input);
    }
}

// ***************** STAGE 3 *****************
fn edit_bill(bills: &mut HashMap<String, Bill>) {
    print!("Enter bill name to edit: ");
    io::stdout().flush().unwrap();
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name_input = name_input.trim();


    if let Some(bill) = bills.get_mut(name_input) {
        println!("Current amount: ${:.2}", bill.amount);
        print!("Enter new amount: ");
        io::stdout().flush().unwrap();
        let mut amount_input = String::new();
        io::stdin().read_line(&mut amount_input).unwrap();
        let amount_input = amount_input.trim();

        match amount_input.parse::<f64>() {
            Ok(new_amount) if new_amount >= 0.0 => {
                bill.amount = new_amount;
                println!("✅ Bill '{}' updated to ${:.2}", name_input, new_amount);
            }
            _ => println!("⚠️ Invalid amount. Edit aborted."),
        }
    } else {
        println!("⚠️ Bill '{}' not found.", name_input);
    }
}