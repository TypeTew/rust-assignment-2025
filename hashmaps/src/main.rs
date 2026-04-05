use std::collections::HashMap;

fn main() {
    let mut treasures : HashMap<&str, i32> = HashMap::new();

    treasures.insert("Silver Coins", 100);
    treasures.insert("Gold Coins", 2);
    treasures.insert("Ruby", 3);

    // Loop hashmap and print key and value
    for (key, value) in &treasures {
        println!("{}: {}", key, value);
    }

    if let Some(ruby) = treasures.get_mut("Ruby") {
        *ruby += 5;
    }
    println!("Treasure Map: {:?}", treasures);
}
