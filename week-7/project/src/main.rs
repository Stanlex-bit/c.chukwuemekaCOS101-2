use std::io;

fn main() {
    println!("=== The Shape Calculator ===");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    
    print!("Choose a shape (1-5): ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    if choice == 1 {
        calc_trapezium();
    } else if choice == 2 {
        calc_rhombus();
    } else if choice == 3  {
        calc_parallelogram();
    } else if choice == 4 {
        calc_cube();
    } else if choice == 5 {
        calc_cylinder();
    } else {
        println!("Invalid choice.");
    }
}

// 1. Trapezium, area = height / 2 * (base1 + base2)
fn calc_trapezium() {
    let height = read_number("Enter height: ");
    let base1 = read_number("Enter base1: ");
    let base2 = read_number("Enter base2: ");

    let area = height / 2.0 * (base1 + base2);
    println!("Area: {}", area);
}

// 2. Rhombus, area = 1/2 * diagonal1 * diagonal2
fn calc_rhombus() {
    let diagonal1 = read_number("Enter diagonal1: ");
    let diagonal2 = read_number("Enter diagonal2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area: {}", area);
}

// 3. Parallelogram, area = base * altitude[cite: 1]
fn calc_parallelogram() {
    let base = read_number("Enter base: ");
    let altitude = read_number("Enter altitude: ");

    let area = base * altitude;
    println!("Area: {}", area);
}

// 4. Cube, surface area = 6 * side * side[cite: 1]
fn calc_cube() {
    let side = read_number("Enter side: ");

    let surface_area = 6.0 * side * side;
    println!("Surface Area: {}", surface_area);
}

// 5. Cylinder, volume = pi * radius * radius * height[cite: 1]
fn calc_cylinder() {
    let radius = read_number("Enter radius: ");
    let height = read_number("Enter height: ");
    let pi = 3.14159;

    let volume = pi * radius * radius * height;
    println!("Volume: {}", volume);
}

// Simple helper to read numbers
fn read_number(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
