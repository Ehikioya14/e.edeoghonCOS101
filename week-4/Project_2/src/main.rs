use std::io;

fn main() {
    // Step 1: Get the user's input
    println!("Calculator for Annual Incentives");
    let mut experience_input = String::new();
    let mut age_input = String::new();
    
    println!("Does the employee have experience (yes/no):");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience = experience_input.trim().to_lowercase();


    
    if experience == "yes" {
    println!("Enter the age of the employee");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age:u32 = age_input.trim().parse().expect("Please enter a valid age");

    // Step 2: use decision making to determine the incentives
   
        if age >= 40 {
            println!("The incentive of the employee is N1,560,000");
        } else if age >= 30 && age < 40 {
            println!("The incentive of the employee is N1,480,000");
        } else if age < 28 {
            println!("The incentive of the employee is N1,300,000");
        }
    } else if experience == "no"{
        println!("The incentive of the employee is N100,000");
    }
}
