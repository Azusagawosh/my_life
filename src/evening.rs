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
