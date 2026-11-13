mod morning;
mod evening;
mod noon;
mod night;
mod study;
mod report;
mod pract;

fn main() {
    /*
    morning::wake_up();
    morning::coffee();
    noon::go_to("12:00", "колледж");
    noon::go_home("18:00", "библиотека");
    noon::daily_plan();
    let minutes = noon::study_duration(4);
    println!("{minutes}");
    study::study("Rust", 90, true);
    let total = report::total_duration("Rust", 90, true, "Math", 30, false);
    println!("{total}");
    let tot = calc(120, 200);
    println!("{tot}");
    let progress = calculate_progress();
    println!("{progress}");
    let pro = weekly_progress(30, 60, 90);
    println!("{pro}");
*/
    pract::clculator();
}



fn calc(a: i32, b: i32) -> i32 {
    let result = a + b;
    result
} 

fn calculate_progress() -> i32 {
    let mut total = 0;
    total += 30;
    total += 60;
    total += 90;
    total 
}

fn weekly_progress(a: i32, b: i32, c: i32) -> i32 {
    let mut result = a + b + c + 100; 

    if result <= 280 {
        
        result *= 2; 
    }
    
    result 
}
