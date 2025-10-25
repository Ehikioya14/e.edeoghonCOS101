use std::io;

fn main() {
   // Step 1: input values a, b and c from the keyboard
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();


    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of b:" );
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Please anter a valid number");

    // Step 2: Find the discriminant (b*b - 4ac)
    let discriminant = b.powf(2.00) - 4.00 * a * c;

    // Step 3:Determine the nature of the roots
    if discriminant > 0.00 {
        let root1 = (-b + discriminant.sqrt()) / (2.00 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.00 * a);
        println!("The equation has two distinct real roots");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.00 {
        let root = -b / (2.00 * a);
        println!("The equation has one real root");
        println!("Root = {}", root);
    } else if discriminant < 0.00 {
        let real_part = -b / (2.00 * a);
        let imaginary_part = -discriminant.sqrt() / (2.00 * a);
        println!("The equation has no real roots");
        println!("complex roots are: ");
        println!("Root 1 = {} + {}i", real_part, imaginary_part);
        println!("Root 2 = {} + {}i", real_part, imaginary_part);
    }
}
