use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nAre you experienced?, TRUE(yes) OR FALSE(no):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience:bool = input1.trim().to_lowercase().parse().expect("You can only type true or false");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experience {
         if age >= 40 
    {
        println!("incentive of employee is 1,560,000");
    }
            else if experience == true && age >= 30 
    {
        println!("incentive of employee is 1,480,000");
    }
            else 
    {
        println!("incentive of employee is 1,300,000");
    }
        
    }else if experience == false 
    {
        println!("incentive of employee is 100,000");
    }

}
    




    

