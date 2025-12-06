// defining the brand of laptops
struct Price {
    hp:u32, ibm:u32, tosh:u32, dell:u32
}

//logic to calculate total cot of laptops if a customer purchases 3 of each brand
impl Price{
    fn sum(&self)->u32 {
        (self.hp * 3) + (self.ibm * 3) + (self.tosh * 3) + (self.dell * 3)
    }
}


fn main() {
    //instantiate it
    let laptops = Price {
        hp:650_000,
        ibm:755_000,
        tosh:550_000,
        dell:850_000
    };
    // print the sum
    println!("\nHey!");
    println!("The price of an hp laptop is {} \n 
        The price of an ibm laptop is {} \n The price of a Toshiba laptop is {} \n 
        The price of a dell laptop is {} \n The total cost if a customer buys 3 of each brand is {} ",laptops.hp,laptops.ibm,laptops.tosh,laptops.dell,laptops.sum());

}
