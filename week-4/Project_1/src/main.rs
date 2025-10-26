use std::io;

fn main() {
    println!("\nProgram to calculate the roots of a quadratic equation");
   // Step 1: input values a, b and c from the keyboard
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();


    println!("\nEnter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the value of b:" );
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("\nEnter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    // Step 2: Find the discriminant (b*b - 4ac)
    let discriminant = b.powf(2.00) - 4.00 * a * c;

    // Step 3:Determine the nature of the roots
    if discriminant > 0.00 {
        let root1 = (-b + discriminant.sqrt()) / (2.00 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.00 * a);
        println!("\nThe equation has two distinct real roots");
        println!("\nRoot 1 = {}", root1);
        println!("\nRoot 2 = {}", root2);
    } else if discriminant == 0.00 {
        let root = -b / (2.00 * a);
        println!("\nThe equation has one real root");
        println!("\nRoot = {}", root);
    } else if discriminant < 0.00 {
        let real_part = -b / (2.00 * a);
        let imaginary_part = -discriminant.sqrt() / (2.00 * a);
        println!("\nThe equation has no real roots");
        println!("\ncomplex roots are: ");
        println!("\nRoot 1 = {} + {}i", real_part, imaginary_part);
        println!("\nRoot 2 = {} + {}i", real_part, imaginary_part);
    }
}
