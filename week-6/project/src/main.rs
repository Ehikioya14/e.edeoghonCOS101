use std::io;

fn main() {
    // Step 1: Display the menu
    println!("\nWelcome to The cafe");
    println!("\nItem       Menu                    Price
              \nP     Pound Yam/Edinkaiko Soup     N3200
              \nF     Fried Rice & Chicken         N3000
              \nA     Amala & Ewedu Soup           N2500
              \nE     Eba & Egusi Soup             N2000
              \nW     White Rice & Stew            N2500 ");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Choose an item P/F/A/E/W");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let  item = input1.trim().to_lowercase();

    println!("How many do you want to order: ");
    io::stdin().read_line(&mut input2).expect("Coulc not read input");
    let  quantity:f32 = input2.trim().parse().expect("Not a Valid number");

    // Step 2: Use decision making to declare the price and quantity

    let mut price: f32 = 0.0;

    if item == "p" {
        price = 3200.0;
    } else if item == "e" {
        price = 3000.0;
    } else if item == "a" {
         price = 2500.0;
    } else if item == "e" {
         price = 2000.0;
    } else if item == "w" {
         price = 2500.0;
    } else {
        println!("Not a Valid input");
    }

    let total = price * quantity;

    if total > 10_000.0 {
        let discount = total - (total * 0.05);
        println!("Your Total after 5% discount is {}",discount);
    } else {
        println!("Your total is {}",total);
    }

}
