pub fn good_night() {
    let age = 19;
    let mut score = 0;

if age >= 18 {
    score = score + 90;
    println!("{age}");   
    println!("{score}"); 
}

println!("{age}");  
println!("{score}");
}

pub fn night() {
    let score = 50;
    let age = 19;
    if age >= 18 {
        let score = score - 10;
        println!("{score}");
    }
    println!("{score}");
}

pub fn pract() {

    let mut level = 1;

    {
        level += 5;
        let level = level + 2;
        println!("{level}");
    }

    println!("{level}");

}


