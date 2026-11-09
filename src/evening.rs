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

pub fn count_even(mas: [i32; 6]) -> i32 {
    let mut count = 0;
    for element in mas {
        if element % 2 == 0 {
            count += 1;
        }
    }
    count
}

pub fn mu(sum: [i32; 6]) -> i32 {
    let mut count = 0;
    for element in sum {
        if element >= 200 {
            count * 2;
        }
    }
    count 
}

pub fn get_first_half(numbers: &[i32]) -> &[i32] {
    let half = numbers.len() / 2;
    &numbers[0..half]
}

pub fn get_last_half(numbers: &[i32]) -> &[i32] {
    let half = numbers.len() / 2;
    &numbers[half..]
}


pub fn manage_numbers() -> Vec<i32> {
    let mut num: Vec<i32> = Vec::new();
    num.push(10);
    num.push(20);
    num.push(30);
    num.push(40);
    num.pop();
    num.push(50);
    num
}


pub fn analyze_numbers(numbers: Vec<i32>) -> usize {
    if numbers.is_empty() {
        0
    } else {
        numbers.len()
    }
} 


pub fn get_element(numbers: &[i32], index: usize) -> Option<&i32> {
    numbers.get(index)
}


pub fn sum_numbers(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for number in numbers { 
        total += *number;
    }
    total
}


pub fn double_numbers(numbers: &mut [i32]) {
    for number in numbers.iter_mut() { 
        *number *= 2;
    }
}

pub fn count_even(numbers: &[i32]) -> usize {
    let mut count = 0;
    for element in numbers {
        if element % 2 == 0 {
            count += 1; 
        }
    }
    count
}


pub fn count_positive(numbers: &[i32]) -> usize {
    let mut tot = 0;
    for element in numbers {
        if element > 0 {
            tot += 1;
        }
    }
    tot
}


pub fn find_max(numbers: &[i32]) -> Option<i32> {
    let Some(mut max) = numbers.first() else {
    return None;
};
    for element in numbers {
        if element > max {
            max = element;
        }
    }
    Some
}

pub fn mn() {
    let a = String::from("Hello");
    let b = String::from("azusagawosh");
    let c = format!("{a}, {b}");
    let d = a + " " + &b;
    println!("{c} | {d}");
}
