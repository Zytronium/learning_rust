fn main() {
    let phase: u8 = 2;
    if phase == 1 {
        println!("====== Phase 1: Syntax Fundamentals ======");
        println!();
        println!("=== VARIABLES ===");
        variables();
        print_line();
        println!("=== CONDITIONS ===");
        conditions();
        print_line();
        println!("=== LOOPS ===");
        loops();
        print_line();
        println!("=== FUNCTIONS ===");
        functions();
        print_line();
        println!();
    } else if phase == 2 {
        println!("====== Phase 2: Ownership Basics ======");
        println!();
        println!("=== STRINGS ===");
        strings();
        print_line();
        println!("=== OWNERSHIP ==="); // The most important part of Rust
        ownership();
        print_line();
        println!("=== BORROWING ==="); // Also pretty important
        borrowing();
        print_line();
    } else {
        println!("Phase {phase} does not exist.");
    }
}

fn print_line() {
    println!("--------------------------------");
}

fn variables() {
    let x: i32 = 42;
    let mut y: i32 = 10;
    let pi: f64 = 3.14159265358979323846264338;
    let is_rust_fun: bool = true;
    let language: &str = "Rust";
    let greeting: String = String::from("Hello World!");

    // silence the warning about y never being used before its first update
    if y == 10 {
        y = 20;
    }

    println!("{}", x);
    println!("{}", y);
    println!("{}", pi);
    println!("{}", is_rust_fun);
    println!("{}", language);
    println!("{}", greeting);
    println!("x plus y equals {}", x + y)
}

fn conditions() {
    let number: i16 = -15;

    if number > 0 {
        println!("Positive");
    } else if number < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    if number != 0 {
        if number % 2 == 0 {
            println!("Even");
        } else {
            println!("Odd");
        }
    }

    let score: i32 = 87;
    let letter_grade: &str;

    if score >= 100 {
        letter_grade = "A+";
    } else if score >= 90 {
        letter_grade = "A";
    } else if score >= 80 {
        letter_grade = "B";
    } else if score >= 70 {
        letter_grade = "C";
    } else if score >= 60 {
        letter_grade = "D";
    } else {
        letter_grade = "F";
    }

    println!("Score: {score}, Grade: {letter_grade}");
    if letter_grade == "C" {
        println!("But my grade is Rust, which is a higher grade than C.")
    }
}

fn loops() {
    let mut i: u8 = 0;

    // loop from 0 to 4, then stop
    loop {
        println!("Count: {i}");
        i += 1;
        if i >= 5 {
            break;
        }
    }

    let mut j: u8 = 10;

    println!();

    // count down from 10, then liftoff!
    while j > 0 {
        println!("{j}");
        j -= 1;
        if j == 0 {
            println!("Liftoff!");
        }
    }

    println!();

    // Count up from 1 to 10 using for loop
    for n in 1..=10 {
        println!("{n}");
    }

    println!();

    let fruits = ["apple", "banana", "orange", "grape"];

    // iterate over a list of fruits
    for fruit in fruits.iter() {
        println!("{fruit}");
    }

    println!();

    // again but with indexes
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{index}: {fruit}");
    }
}

fn functions() {
    println!("{}", add(5, 7));

    let (quotient, remainder): (i32, i32) = divide_and_remainder(17, 5);
    println!("17 / 5 = {quotient} remainder {remainder}");
    print_greeting("Rust Learner");

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }

    fn print_greeting(name: &str) {
        println!("Hello, {name}!");
    }
}

// -------------------------------

fn strings() {
    let slice: &str = "Hello";
    let mut owned: String = String::from("World");

    println!("{}", slice);
    println!("{}", owned);

    // slice.push(' ');
    owned.push('!');

    println!("{}", owned);

    let slice_to_string = slice.to_string();
    let string_to_slice: &str = &owned[..];

    println!("{}", slice_to_string);
    println!("{}", string_to_slice);

    println!("slice is type &str, owned is type String");
}

fn ownership() {
    let mystery_item: String = String::from("five nuclear missiles");
    take_ownership(mystery_item);
    // println!("{}", mystery_item);
    // This fails because nuclear bombs are not legal yet, so they were confiscated
    // (by the take_ownership function, which took ownership of the missiles and then
    // dropped them at the end of the function. Wait, THEY DROPPED THE NUCLEAR BOMBS!?
    // THIS MEANS WAR!! CLONE WARS TIME!)

    let trooper: String = String::from("Storm Trooper");
    let trooper = borrow_and_clone(trooper);
    println!("Successfully cloned {trooper}");
    println!("The clone is already dead"); // It got dropped at the end of the function,
                                           // just like me when I was a baby
    fn take_ownership(s: String) {
        println!("I now own {s}");
    }

    fn borrow_and_clone(s: String) -> String {
        let cloned = s.clone();
        println!("I have cloned a {cloned}");
        s
        // cloned gets dropped here
    }
}

fn borrowing() {
    let text: String = String::from("Never Gonna Give You Up!");
    let length = calculate_length(&text);
    println!("\"{text}\" is {length} bytes long.");
    let mut text2: String = String::from("Hello World");
    append_exclamation(&mut text2);
    println!("{text2}");

    fn calculate_length(s: &String) -> usize {
        // borrows `s` by taking a reference to it
        s.len() // returns the length of the string
    }

    fn append_exclamation(s: &mut String) {
        s.push_str("!!!");
    }
}
