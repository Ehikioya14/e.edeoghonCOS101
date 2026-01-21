use std::fs;
use std::io;

fn main() {

    let mut role = String::new();
    println!("WELCOME!!!");
    println!("\nThese are the available roles");
    println!("\n1. Administrator
              \n2. Project Manager
              \n3. Employee
              \n4. Customer
              \n5. Vendor 
               ");
     println!("\nPick a Role :");
    io::stdin().read_line(&mut role).expect("Failed to read input");

    let file = match role.trim() {
        "1" => "globacom_dbase.sql",
        "2" => "project_tb.sql",
        "3" => "staff_yb.sql",
        "4" => "customer_tb.sql",
        "5" => "dataplan_tb.sql",
        _=> { return println!("Not an option."); }
    };

    match fs::read_to_string(file) {
        Ok(content) => println!("\n Data for {}    \n{}",file, content),
        Err(_) => println!("Error: File {} not found.",file),
    }

}
