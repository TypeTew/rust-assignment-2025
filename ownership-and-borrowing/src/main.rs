// Crabby's Challenge: Protect & Lend the
// Treasure

// Crabby carefully guards his treasure, but he wants to lend it to
// some friends. However, he is careful! He'll only allow you to
// borrow the treasure immutably unless you know Crabby well
// enough to change it.


// Crabby เฝ้าสมบัติของเขาอย่างระมัดระวัง แต่เขาต้องการให้เพื่อนๆ 
// ยืมไปใช้ อย่างไรก็ตาม เขาระมัดระวังมาก! เขาจะอนุญาตให้คุณ
// ยืมสมบัติแบบไม่สามารถเปลี่ยนแปลงได้เท่านั้น 
// เว้นแต่ว่าคุณจะรู้จัก Crabby ดีพอที่จะเปลี่ยนแปลงมันได้

//1. ป้องกัน Memory Leaks 🛡️
// 2. ป้องกัน Dangling Pointers ⚠️
// 3. ป้องกัน Data Races 🏃‍♂️

fn main() {
    let mut treasure = String::from("gold coins");
    // Multiple friends borrow immutably!
    // code here ...

    let friend1 = &treasure;
    let friend2 = &treasure;

    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);

    // Trusted friend borrows mutably
    // code here ...

    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" and silver coins");
    println!("Trusted friend updates: {}", trusted_friend);
}
