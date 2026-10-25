pub fn wake_up() {
    let time = ["9:00", "11:00", "10:30", "12:00"];
    for element in time {
        if element == "9:00" {
            continue;
        }
        if element == "10:30" {
            println!("wake up! its time 10:30");
        }
        if element == "11:00" {
            continue;
        }
        if element == "12:00" {
            println!("ITS TIME!! WAKE UP! BRO time is 12:00");
                break;
        }
    }
}

pub fn coffee() {
    let wake = ("проснулся", "встал и умылся", "взял кружку", "поставил рядом с кофеваркой", "заварил кофе", "посмотрел в холодильнике есть ли яица", "как нашел я приготовил и начал завтракать");
    let up = wake.0;
    let wash = wake.1;
    let take = wake.2;
    let machine = wake.3;
    let coffee = wake.4;
    let see = wake.5;
    let eat = wake.6;
    println!("{} после {}", up, wash);
    println!("{} и {} после {}", take, machine, coffee);
    println!("{} после {}", see, eat);
}
