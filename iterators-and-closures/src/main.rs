fn main() {
    let treasures : Vec<i32> = vec![1, 2, 3, 4, 5];

    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();    

    println!("Treasures: {:?}", treasures);
    println!("Doubled Treasures: {:?}", doubled_treasures);
}
