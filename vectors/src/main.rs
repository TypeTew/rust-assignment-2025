// Crabby's Challenge: Organize & Manage a
// Growing Treasure Hoard

// Crabby needs you to help manage his inventory by adding
// treasures, removing old ones, and keeping track of the total
// treasures in Vectors.

fn main() {
    let mut items = vec!["Gold", "Silver", "Emerald"];

    items.push("Diamond");

    items.remove(1);
    println!("Items: {:?}", items);
    println!("Total items: {}", items.len());
    print!("Item Capacity: {}", items.capacity())
}
