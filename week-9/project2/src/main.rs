
use std::io::Write;

fn main() {
    // step 1: Store student data into a vector of tuples
    let students = vec! [
        ("Oluchi Mordi","ACC10211111","Accounting","300"),
        ("Adams Aliyu","ECO10110101","Economics","100"),
        ("Shania Bolade","CSC10328828","Computer","200"),
        ("Adekunle Gold","EEE11020202","Electrical","200"),
        ("Blanca Edemoh","MEE10202001","Mechanical","100"),
    ];

    // step 2: Display header
    println!("\nPAU Student Management Information System (PAU-SIMS)");
    println!("\nStudent Name | Matric Number | Department | Level");

    //step 3: Display student details using for loop
    for student in &students {
        println!("\n{} | {} | {} | {}",student.0,student.1,student.2,student.3);
    }

    //step 4: Create a file
    let mut file = std::fs::File::create("students.txt").expect("create failed");

    //step 5: display header in file
    file.write_all("PAU STUDENT MANAGEMENT INFORMATION SYSTEM\n".as_bytes()).expect("write failed");
    file.write_all("Student Name | Matric Number | Department | Level\n".as_bytes()).expect("write failed");

    // step 6: Write student into file
    for student in &students {
        let stu =  format!("{} | {} | {} | {}\n",student.0, student.1, student.2, student.3);
         file.write_all(stu.as_bytes()).expect("write failed");
    }

   

    println!("\nStudent data successfuly saved in students.txt");























}
