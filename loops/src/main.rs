fn main() {
    let treasures = ["gold", "silver", "diamonds"];
    let mut energy = 100;

    //iter() เป็นการยืมค่าทีตัวได้
    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are out of energy. Game over!");
            break;
        } else if treasure == &"Ruby Gem" {
            println!("You found the Ruby Gem! You win!");
            break;
        }

        energy -= 1;
    }

    println!("Energy left: {}", energy);
}
