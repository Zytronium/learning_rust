use std::io;

fn main() {
    // phase to test/print (set to 0 for all)
    let phase: u8 = 5;

    // check if `phase` is equal to given number or 0
    let phs = |p: u8| phase == p || phase == 0;

    if phs(1) {
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
    }
    if phs(2) {
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
        println!();
    }
    if phs(3) {
        println!("====== Phase 3: Data Structures ======");
        println!();
        println!("=== VECTORS ===");
        vectors();
        print_line();
        println!("=== STRUCTS ===");
        structs();
        print_line();
        println!("=== ENUMS ===");
        enums();
        print_line();
        println!();
    }
    if phs(4) {
        println!("====== Phase 4: Error Handling ======");
        println!();
        println!("=== OPTIONS ===");
        options();
        print_line();
        results();
        print_line();
        println!();
    }
    if phs(5) {
        println!("====== Phase 5: Practical Projects ======");
        println!();
        println!("=== TEMPERATURE CONVERTER ===");
        temperature();
        print_line();
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

// -------------------------------

fn vectors() {
    let mut numbers: Vec<i32> = Vec::new();
    // push 10, 20, 30, 40, and 50 to the vector
    for i in 1..=5 {
        numbers.push(i * 10);
    }

    println!("{:?}", numbers);
    println!("{}", numbers[2]);

    numbers[1] = 25;
    println!("{:?}", numbers);

    for number in &numbers {
        println!("{number}");
    }

    for number in numbers.iter_mut() {
        *number *= 2;
    }

    println!("{:?}", numbers);

    let last = numbers.pop();
    println!("{:?}", last); // .unwrap() to get rid of some()
    println!("{:?}", numbers);
}

struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Person {
        // Added validation because I remember how to do it and wanted to test my memory
        validate_name(&name);
        validate_age(age);
        validate_email(&email);

        fn validate_name(name: &String) {
            if name.len() <= 1 {
                panic!("Name must be at least 2 bytes long");
            }
            if name.len() > 64 {
                panic!("Name cannot be longer than 64 bytes");
            }
        }

        fn validate_age(age: u32) {
            if age > 150 {
                panic!("Age cannot be older than 150 years")
            }
            // 0 is still a valid age and u32 can't be negative,
            // so we won't set a minimum age
        }

        fn validate_email(email: &String) {
            // I'm not gonna bother with complex email regex. We'll do something simple
            if !email.contains('@') {
                panic!("Email must contain an '@' character")
            }
            if !email.contains('.') {
                panic!("Email must contain a '.' character")
            }
            if email.len() < 6 { // a@b.co is the shortest possible valid email
                panic!("Email must be at least 6 bytes")
            }
            // I'm aware this is not perfect validation, as something like ".@abc!"
            // would pass this
        }

        Person {
            name,
            age,
            email
        }

    }

    fn display(&self) {
        println!("{} ({} years old) - {}", self.name, self.age, self.email)
    }
}

fn structs() {
    let john_doe: Person = Person::new(
        String::from("John Doe"),
        23,
        String::from("johndoe@example.com")
    );

    john_doe.display();

    println!();

    let jane_doe: Person = Person::new(
        String::from("Jane Doe"),
        22,
        String::from("janedoe@example.com"),
    );
    let jack_smith: Person = Person::new(
        String::from("Jack Smith"),
        28,
        String::from("jack_smith123@example.com"),
    );

    let team: Vec<Person> = vec![john_doe, jane_doe, jack_smith];

    for member in team {
        member.display();
    }

    // attempt to create an invalid person
/*    let invalid: Person = Person::new(
      String::from(""),
      2048,
      String::from("&@dotcom"),
    );*/ // tested: all 3 values trigger a panic correctly

}

enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // three sides' lengths
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14159 * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn shape_name(shape: &Shape) -> String {
    match shape {
        Shape::Circle(..) => String::from("Circle"),
        Shape::Rectangle(..) => String::from("Rectangle"),
        Shape::Triangle(..) => String::from("Triangle"),
    }
}

fn enums() {
    let tri_size = 2.69354737417719669;
    let circle = Shape::Circle(5.0);
    let rect = Shape::Rectangle(4.0, 6.0);
    let triangle = Shape::Triangle(tri_size, tri_size, tri_size);

    let shapes = vec![circle, rect, triangle];

    for shape in &shapes {
        let area = calculate_area(shape);
        println!("{} area: {}", shape_name(shape), area);
    }
}

// -------------------------------

fn options() {
    let numbs: Vec<i32> = vec![10, 20, 30, 40, 50];

    // search for 30 using match
    let result = find_number(&numbs, 30);
    match result {
        Some(i) => println!("Found at index {i}"),
        None => println!("Not found"),
    }

    // search for 25 using if let Some() = ...
    if let Some(i) = find_number(&numbs, 25) {
        println!("Found at index {i}");
    }

    // Search for 100 using .unwrap_or(999)
    println!("Index: {}", find_number(&numbs, 100).unwrap_or(999));

    fn find_number(numbers: &Vec<i32>, target: i32) -> Option<usize> {
        for (index, number) in numbers.iter().enumerate() {
            if *number == target {
                return Some(index);
            }
        }
        None
    }
}

fn results() {
    let result0 = safe_divide(10.0, 2.0);
    match result0.is_ok() {
        true => println!("{}", result0.unwrap()),
        _ => eprintln!("Error: {}", result0.unwrap_err()),
    }

    let result1 = calculate(5.0);
    match result1.is_ok() {
        true => println!("{}", result1.unwrap()),
        _ => eprintln!("Error: {}", result1.unwrap_err()),
    }

    let result2 = calculate(0.0);
    match result2.is_ok() {
        true => println!("{}", result2.unwrap()),
        _ => eprintln!("Error: {}", result2.unwrap_err()),
    }

    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero!"))
        } else {
            Ok(a/b)
        }
    }

    fn calculate(x: f64) -> Result<f64, String> {
        let result = safe_divide(100.0, x)?;

        safe_divide(result, 2.0)
    }
}

// -------------------------------

fn temperature() {
    println!("Temperature Converter");
    println!("Enter Temperature: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let temp: f64 = temp.trim().parse().unwrap();

    println!("Enter unit (C or F): ");

    let mut unit = String::new();
    io::stdin().read_line(&mut unit).unwrap();
    let unit: String = unit.trim().to_uppercase();

    match unit.as_str() {
        "C" => println!("{temp}°C = {}°F", celsius_to_fahrenheit(temp)),
        "F" => println!("{temp}°F = {}°C", fahrenheit_to_celsius(temp)),
        _ => println!("Invalid unit"),
    }

    fn celsius_to_fahrenheit(deg: f64) -> f64 {
        (deg * 9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celsius(deg: f64) -> f64 {
        (deg - 32.0) * 5.0 / 9.0
    }

}
