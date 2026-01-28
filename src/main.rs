fn main() {
    // phase to test/print (set to 0 for all)
    let phase: u8 = 3;

    // check if `phase` is equal to given number or 0
    let phs = |p: u8| -> bool {
        if phase == p || phase == 0 {
            return true;
        }
        false
    }; // I don't actually understand how this works, Rust Rover just converted my broken function into this for me so that I could access `phase`

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
    }
    if phs(3) {
        println!("====== Phase 3: Data Structures ======");
        println!();
        println!("=== Vectors ===");
        vectors();
        print_line();
        println!("=== Structs ===");
        structs();
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
