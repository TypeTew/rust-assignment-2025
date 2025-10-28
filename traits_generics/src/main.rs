// traits
//  สามารถเขียนโค้ดที่ทำงานกับ type ต่างๆ ที่ implement trait เดียวกัน
//  แชร์พฤติกรรมระหว่าง types ต่างๆ
//
//
//Inventory System

//Crabby needs a flexible inventory system that can hold ANY
//item (maps, gold, potions) and can display the details of those
//items.
//Your challenge is to:

//Create a generic Inventory struct that can store any kind of
//item: strings, integers, etc.
//Create DisplayItem trait and Implement the DisplayItem trait
//for the Inventory, so Crabby can display his treasures.
// Try holding both strings (like "Diamonds") and integers (like
// 100 coins).
//

struct Inventory<T> {
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T>
where
    T: std::fmt::Debug,
{
    fn display(&self) {
        println!("{:?}", self.item);
    }
}

fn main() {
    let gold = Inventory { item: 100 };
    gold.display();

    let sword = Inventory { item: "Excalibur" };
    sword.display();
}
