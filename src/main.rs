fn main() {
    // simple_loop();
    // basic_rust();

    let sentence = String::from("Decentralized applications are the future");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut answer = String::from("");
    for char in sentence.chars() {
        answer.push_str(char.to_string().as_str());
        if (char == ' ') {
            break;
        }
    }

    return answer;
}

fn simple_loop(n:i32) {
    for i in 1..n {
        println!("hello");
    }
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn basic_rust() {
    println!("Hello, world!");
    let mut x: i8 = 1;
    let y: i32 = -222;
    // define a floating point variable
    let z: f32 = 222.3131;

    let name = "rahul";
    println!("");

    let crush_name = String::from("juliaane hough");
    let main_goal = String::from("live a good life that has meaning");
    println!("{}", crush_name);
    println!("{}", main_goal);

    let char = crush_name.chars().nth(0);

    match char {
        Some(c) => println!("character is {}", c),
        None => println!("No character found"),
    }

    let is_male = false;
    let is_married = true;
}

fn add_two_numbers() {
    let x: i32 = 1;
    let y: i32 = 2;
    let sum = x + y;
    println!("Sum is {}", sum);
}

fn overflow_variables() {
    // it will overflow the variable
    // as it does not contain enough memory

    let mut x: i8 = 1;
    for i in 1..1000 {
        x = x + 10;
    }

    println!("x is {}", x);
}
