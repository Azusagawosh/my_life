pub fn study(item: &str, minutes: i32, status: bool) {
    println!("{}", item);
    println!("{}", minutes);
    println!("{}", status);
}

pub fn daily_progress() {
    let session = [30, 90, 45, 60, 120];
    let total = calculate_total(session);
    println!("{total}");
    let long = count_long_sessions(session);
    println!("{long}");

    if total < 60 {
        println!("keep going");
    } else if total >= 60 && total <= 120 {
        println!("good work");
    } else if total > 120 {
        println!("excellent");
    } else {
        println!("err")
    }
} 

pub fn calculate_total(session: [i32; 5]) -> i32 {
    let mut total = 0;
    for element in session {
        total += element;
    }
    total
}
pub fn count_long_sessions(session: [i32; 5]) -> i32 {
    let mut count = 0;
    for element in session {
        if element >= 60 {
            count += 1;
        }
    }
    count
}
