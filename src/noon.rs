pub fn go_to(time: &str, place: &str) {
    println!("вышел из дома в: {}, и поехал в: {}", time, place);
}

pub fn go_home(time: &str, place: &str) {
    println!("вышел с колледжа в: {}, и поехал в: {}", time, place);
}

pub fn daily_plan() {
    go_to("12:00", "college");
    go_to("14:30", "library");
    travel("12:00", "college", 35, true);
    travel("14:30", "library", 20, false);
    study_session("Rust", 90, true);
    study_session("higher Math", 30, false)
}

pub fn travel(time: &str, place: &str, duration: i32, status: bool) {
    println!("time: {}, place: {}, duration: {}, status: {}", time, place, duration, status);
}

pub fn study_session(subject: &str,duration: i32, completed: bool) {
    println!("subject: {}, duration: {}, completed? {}", subject, duration, completed);
}

pub fn study_duration(hours: i32) -> i32 {
    let mut minutes = 30;
    if minutes == 30 {
        minutes += 10;
        let minutes = minutes - 20;
        println!("{minutes}");
    }
    println!("{minutes}");
    if hours <= 0 {
        0
    } else {
        hours * 60
    }
}
/*
pub fn calculate(a: i32, b: i32) -> i32 {
    a - b
}
*/

pub fn calculate_pay(hours: i32, rate: i32) -> i32 {
    if hours <= 0 || rate <= 0 {
        0
    } else {
        hours * rate
    }
}


