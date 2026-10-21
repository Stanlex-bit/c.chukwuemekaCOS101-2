
use std::io::Write;

fn main(){

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    let content = "Category : lager -    Stout -     Non-Alcoholic
                             33 Export   Legend      Maltina
                             Desperados  Turbo KIng  Amstel Malta
                             Goldberg    Williams    Malta Gold
                             Gulder                  Fayrouz
                             Heineken
                             Star";
                    
   
    file.write_all(content.as_bytes()).expect("write failed");
    println!("\ndrinks.txt file created");
}
    

