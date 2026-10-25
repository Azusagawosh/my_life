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
