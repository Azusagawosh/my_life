pub fn day_1() {
    let name = String::from("Azusagawosh");
    greet(&name);
    let nam = String::from("azusagawosh");
    analyze_name(&nam);
    println!("{nam}");
    let na = String::from("azusa");
    let prof = String::from("rust backend developer");
    user_info(&na, &prof);
    println!("{na}");
    println!("{prof}");
    let mut n = String::from("azu");
    change_name(&mut n);
}


pub fn day_2() {
    let mut ch = String::from("azusagawosh");
    ch.push_str(" rust developer");
    ch.push('!');
    println!("{ch}");
    println!("{}", ch.len());
    println!("{}", ch.is_empty());
}

pub fn greet(name: &str) {
    println!("hello, {name}");
}

fn analyze_name(name: &str) {
    println!("{}", name);
    println!("{}", name.len());
    println!("{}", name.is_empty());
}

pub fn user_info(name: &str, profession: &str) {
    println!("name: {name}, profession: {profession}");
}

pub fn change_name(name: &mut String) {
    println!("{name}");
    name.push_str(" Rust");
}


pub fn mass() {
    let num = [10, 20, 30, 40, 50, 60];
    let kor = ("azusagawosh", 19, true);
    let name = kor.0;
    let age = kor.1;
    let status = kor.2;
    for element in num {
        if element == 30 {
            println!("тридцать");
        }
    }
}

pub fn call(mass: [i32; 6]) -> i32 {
    let mut total = 0;
    for element in mass {
        total += element;
    }
    total
}
