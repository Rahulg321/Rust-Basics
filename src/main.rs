fn main() {
    immutable_borrowing();
    mutable_borrowing();
}

pub fn ownership_concept() {
    let x = String::from("hello");
    let y = x; // x is no longer valid, y is the new owner
    println!("y is {}", y)
}

pub fn immutable_borrowing() {
    // borrowing allows to use a value without modifying it

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}", s1);
    println!("s2 is {}", s2)
}

pub fn mutable_borrowing() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1; // s2 borrows s1 mutably
    s2.push_str(", world"); // modify the value through s2
    println!("{}", s1); // s1 is now "hello, world"
}

pub fn stack_fn() {
    let a = 10;
    let b = 10;
    let c = a + b;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!(
        "Capacity: {}, length: {}, Pointer p {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    // Append some text to the string
    s.push_str(" and some additional text");
    println!(
        "Capacity: {}, length: {}, Pointer p {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    println!("After update: {}", s);
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

fn simple_loop(n: i32) {
    for i in 1..n {
        println!("hello");
    }
}

// a public function that returns true if the number is even
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
