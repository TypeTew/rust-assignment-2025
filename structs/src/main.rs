
struct Crabby {
    name: String,
    health: u8, // 100
}

// แยกการนิยาม struct กับ behavior (methods)
// จัดระเบียบโค้ด ให้เป็นหมวดหมู่
// เพิ่ม methods ได้ทีหลัง แม้ struct อยู่คนละไฟล์
// ใช้งานง่าย เหมือน OOP ในภาษาอื่น

impl Crabby {
   fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
        // saturating_sub() = การลบที่ ไม่ให้ติดลบ (underflow protection)
        println!("{} takes {} damage! Health: {}", self.name, damage, self.health);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
        println!("{} heals {} HP! Health: {}", self.name, heal, self.health);

    }
}

fn main() {
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.take_damage(100);
    crabby.take_damage(10);
    println!("Crabby Name: {}", crabby.name);
    println!("Crabby health: {}", crabby.health);

    crabby.healing(60);
    println!("Crabby health: {}", crabby.health);

}
