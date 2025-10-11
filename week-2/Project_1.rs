fn main(){
	println!("PROGRAM USED TO FIND THE COMPOUND INTEREST FOR 5 YRS AT 10% PER ANNNUM COMPOUNDED ANNUALLY");
let p :f64 = 520000.0;
let r :f64 = 10.0;
let t :f64 = 5.0;

// simple interest
let a = p * (1.0 + (r/100.0)) * t;
println!("Amount is {}", a);

//compound interest
let ci = a - p;
println!("Compound interest is {}", ci);
}