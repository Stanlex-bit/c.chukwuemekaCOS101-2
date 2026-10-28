
struct Laptop {
    brand: String,
    price: u64,
    stock: u32,
}

impl Laptop {
    fn cost_for(&self, qty: u32) -> u64 {
        let actual = qty.min(self.stock) as u64;
        actual * self.price
    }
}

fn main() {
    let inventory = vec![
        Laptop { brand: "HP".to_string(),     price: 650_000, stock: 10 },
        Laptop { brand: "IBM".to_string(),    price: 755_000, stock: 6  },
        Laptop { brand: "Toshiba".to_string(),price: 550_000, stock: 10 },
        Laptop { brand: "Dell".to_string(),   price: 850_000, stock: 4  },
    ];

    let buy_each_brand: u32 = 3;

    let mut total: u64 = 0;
    for l in &inventory {
        let c = l.cost_for(buy_each_brand);
        println!("{} x {} each => cost: {}", buy_each_brand.min(l.stock) , l.brand, c);
        total += c;
    }

    println!("Total cost for buying {} from each brand: {}", buy_each_brand, total);
}

