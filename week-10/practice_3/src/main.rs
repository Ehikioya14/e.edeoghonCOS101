fn main() {

    let v = vec![20, 40, 60 , 80];
    //vector v owns the object in heap

    let v2 = v.clone();
    let v2_return = display(v2);
    println!("In main {:?}",v);
    // this code will give an error because ownership of v has been given to v2 and v2 has been passed to the parameter of function display. so to correct this error use .clone() on line 6
}

fn display(v:Vec<i32>)->Vec<i32> {
    // returning same vector
    println!("Inside display {:?}",v);
    return v;
}