pub fn total_duration(subject1: &str, minutes1: i32, completed1: bool, subject2: &str, minutes2: i32, completed2: bool) -> i32 {
    let mut total = 0;
    if completed1 {
        total += minutes1;
    } else {
        println!("false");
    }

    if completed2 {
        total += minutes2;
    } else {
        println!("false");
    }
    total
}
