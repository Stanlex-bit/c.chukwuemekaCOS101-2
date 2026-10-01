use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid string");

    // Find the discriminant
    let discriminant:f32 = b.powf(2.0) - (4.0 * (a * c));

    //Determine nature of roots
    if discriminant > 0.0
    {
    let root1:f32 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2:f32 = (-b - discriminant.sqrt()) / (2.0 * a);
    println!("There are two distinct roots");
    println!("\nRoot 1 = :{} ", root1);
    println!("\nRoot 2 = :{} ", root2);
    }

    else if discriminant == 0.0
    {
        let root = -b / (2.0 * a);
        println!("There is one root");
        println!("Root= {}", root);
    }

    else {
        println!("There are no real roots");
    }
}
    

