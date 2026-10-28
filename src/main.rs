mod morning;
mod evening;
mod noon;
mod night;

fn main() {
    morning::wake_up();
    morning::coffee();
    noon::go_to("12:00", "колледж");
    noon::go_home("18:00", "библиотека");
    noon::daily_plan();
    let minutes = noon::study_duration(4);
    println!("{minutes}");
}

