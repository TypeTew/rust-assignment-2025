// Crabby's Challenge: The Lifetime
// Treasure Hunt

// Crabby is on a treasure hunt and has two old treasure maps. He
// wants to borrow one that lasts longer because he's going to
// need it until the very end of his adventure.

// ความท้าทายของ Crabby: การล่าสมบัติแห่งอายุขัย

// Crabby กำลังออกล่าสมบัติและมีแผนที่สมบัติเก่าสองแผ่น เขา
// ต้องการยืมแผ่นที่อยู่ได้นานกว่า เพราะเขาจะต้องใช้มัน
// จนถึงจุดจบของการผจญภัย

fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}
fn main() {
    let map1 = "Ancient Map of the Sea";
    let map2 = "Map to Hidden Gold";

    let chosen_map = longest_map(map1, map2);

    println!("Crabby's longest map: {}", chosen_map);
}