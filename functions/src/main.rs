fn main() {
    let result = crabby_tasks("gathering coins", 12);
    println!("{}", result);


    let result2 = crabby_tasks("building a sandcastle", 30);
    println!("{}", result2);

    let result3 = crabby_tasks("finding a treasure", 60);
    println!("{}", result3);

}


fn crabby_tasks(task: &str, time: i32) -> String {
    format!("Crabby has successfully done {} in {} minutes!", task, time)
}