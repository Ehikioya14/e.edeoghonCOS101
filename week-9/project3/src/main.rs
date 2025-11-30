use std::io::Write;

fn main() {
    //Step 1: store data in a vector of tuples
    let ministers = vec![
        ("1.","Aigbogun Alamba Daudu","Internal Affairs","South West"),
        ("2.","Murtala Afeez Bendu","Justice","North East"),
        ("3.","Okorocha Calistus Ogbona","Defense","South South"),
        ("4.","Adewale Jimoh Akanbi","Power & Steel","South West"),
        ("5.","Osazuwa Faith Etieye","Petroleum","South East"),
    ];
    //Step 2: Display header
    println!("\nConvicted Ministers from different geopolitical zones in the country");
    println!("\nS/N | Name Of Commisioner | Ministry | Geopolitical Zone");

    //Step 3: using for loop display the convicted ministers data
    for minister in &ministers {
        println!("\n{} | {} | {} | {}",minister.0 , minister.1 , minister.2 , minister.3);
    }

    //Step 4: create a file
    let mut file = std::fs::File::create("File of Convicted Ministers.txt").expect("create failed");

    // Step 5: Display header into file
    writeln!(file,"\nConvicted Ministers from different geopolitical zones in the country");
    writeln!(file,"\nS/N | Name Of Commisioner | Ministry | Geopolitical Zone");

    // Step 6: Display ministers in the file
    for mini in &ministers {
       let min = format!("\n{} | {} | {} | {}",mini.0 , mini.1 ,mini.2 , mini.3);
        file.write_all(min.as_bytes()).expect("write failed");
    }

    println!("Data successfully saved in convicted ministers file");










































    
}
