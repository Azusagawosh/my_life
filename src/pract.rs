use std::io;

pub fn ano() {
    let mut user_inf = String::new();
    io::stdin().read_line(&mut user_inf).expect("Err");
    println!("{}", user_inf);
}


pub fn clculator() {
    let mut num = String::new();
    let mut num1 = String::new();
    println!("enter num: ");
    io::stdin().read_line(&mut num).expect("Err");
    println!("enter num: ");
    io::stdin().read_line(&mut num1).expect("Err");

    let date: i128 = num.trim().parse().expect("err");
    let date1: i128 = num1.trim().parse().expect("err");

    let total = date - date1;
    println!("{total}");
}



pub fn sl() {
    let s = String::from("Hello World");
    let d = &s[0..5];
    let n = &s[6..11];
    println!("{d}, {n}");

    let mass = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let f = &mass[0..=8];
    println!("{:?}", f);
}
