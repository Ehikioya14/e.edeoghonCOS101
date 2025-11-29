use std::io::Write;

fn main() {
    // Rust programme that saves thr categories of drinks from Nigerian breweries plc
    let lager = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = std::fs::File::create("Nigerian Breweries Plc.txt").expect("create failed");
    file.write_all("Nigerian Breweries Plc collection of drinks\n\n".as_bytes()).expect("write failed");

    file.write_all("\n\nlager:\n".as_bytes()).expect("write failed");
    file.write_all(lager.join("\n").as_bytes()).expect("write failed");

    file.write_all("\n\nstout:\n".as_bytes()).expect("write failed");
    file.write_all(stout.join("\n").as_bytes()).expect("write failed");
        
    file.write_all("\n\nnon_alcoholic:\n".as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.join("\n").as_bytes()).expect("write failed");
    

    println!("Data successfully written");

}
