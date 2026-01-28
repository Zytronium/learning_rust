AI generated instructions

## Phase 1: Syntax Fundamentals

### Task 1: Hello World & Variables
**Goal:** Practice variable declaration and printing

1. Create a new file called `variables.rs`
2. In the `main()` function, declare these variables:
    - An immutable integer `x` with value 42
    - A mutable integer `y` with value 10, then change it to 20
    - A floating point number `pi` with value 3.14159
    - A boolean `is_rust_fun` set to true
    - A string slice `language` with value "Rust"
    - A String `greeting` with value "Hello, World!" (use `String::from()`)
3. Print each variable on its own line using `println!`
4. Print a formatted line: "x plus y equals {result}" where you calculate x + y
5. Run with: `rustc variables.rs && ./variables`

**Expected output:**
```
42
20
3.14159
true
Rust
Hello, World!
x plus y equals 62
```

---

### Task 2: Control Flow - If Statements
**Goal:** Master conditional logic

1. Create a file called `conditions.rs`
2. In `main()`, create a variable `number` with value -15
3. Write an if-else statement that prints:
    - "Positive" if number > 0
    - "Negative" if number < 0
    - "Zero" if number == 0
4. Add a second check: if the number is not zero, also print whether it's "Even" or "Odd" (use `number % 2 == 0`)
5. Create a variable `score` with value 87
6. Write an if-else chain that categorizes it:
    - 90-100: "A"
    - 80-89: "B"
    - 70-79: "C"
    - 60-69: "D"
    - Below 60: "F"
7. Print: "Score: {score}, Grade: {grade}"

**Expected output:**
```
Negative
Odd
Score: 87, Grade: B
```

---

### Task 3: Loops
**Goal:** Master all three loop types

1. Create a file called `loops.rs`
2. **Part A - loop:** Create a counter variable starting at 0. Use `loop` to:
    - Print "Count: {counter}"
    - Increment counter
    - Break when counter reaches 5
3. **Part B - while:** Create a countdown variable starting at 10. Use `while` to:
    - Print the current number
    - Decrement it
    - Stop when it reaches 0
    - After the loop, print "Liftoff!"
4. **Part C - for with range:** Use `for n in 1..=10` to print each number
5. **Part D - for with array:** Create an array: `let fruits = ["apple", "banana", "orange", "grape"];`
    - Use `for fruit in fruits.iter()` to print each fruit
    - Then use `for (index, fruit) in fruits.iter().enumerate()` to print: "{index}: {fruit}"

**Expected output:**
```
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
10
9
8
7
6
5
4
3
2
1
Liftoff!
1
2
3
4
5
6
7
8
9
10
apple
banana
orange
grape
0: apple
1: banana
2: orange
3: grape
```

---

### Task 4: Functions
**Goal:** Write functions with different return types

1. Create a file called `functions.rs`
2. Write a function `add` that:
    - Takes two parameters: `a: i32, b: i32`
    - Returns `i32`
    - Returns the sum of a and b (no semicolon on the last line!)
3. Write a function `divide_and_remainder` that:
    - Takes two parameters: `dividend: i32, divisor: i32`
    - Returns `(i32, i32)` (a tuple)
    - Returns `(dividend / divisor, dividend % divisor)`
4. Write a function `print_greeting` that:
    - Takes one parameter: `name: &str`
    - Returns nothing (no return type specified)
    - Prints "Hello, {name}!"
5. In `main()`, call each function:
    - Call `add(5, 7)` and print the result
    - Call `divide_and_remainder(17, 5)` and print "17 / 5 = {quotient} remainder {remainder}"
    - Call `print_greeting("Rust Learner")`

**Expected output:**
```
12
17 / 5 = 3 remainder 2
Hello, Rust Learner!
```

---

## Phase 2: Ownership Basics

### Task 5: String vs &str
**Goal:** Understand the difference between String and string slices

1. Create a file called `strings.rs`
2. Create a string slice: `let slice: &str = "Hello";`
3. Create a String: `let mut owned: String = String::from("World");`
4. Print both
5. Try to push a character to the slice (this will fail - that's the point!)
6. Comment out that line and instead push '!' to the owned String using `owned.push('!');`
7. Convert the slice to a String: `let slice_to_string = slice.to_string();`
8. Get a slice from the String: `let string_to_slice: &str = &owned[..];`
9. Print both conversions
10. Print the types to help understand: `println!("slice is type &str, owned is type String");`

**Expected output:**
```
Hello
World
World!
Hello
World!
slice is type &str, owned is type String
```

---

### Task 6: Ownership Transfer
**Goal:** See ownership in action (and its errors)

1. Create a file called `ownership.rs`
2. Write a function `take_ownership` that:
    - Takes parameter `s: String`
    - Prints "I now own: {s}"
    - Returns nothing
3. Write a function `borrow_and_clone` that:
    - Takes parameter `s: String`
    - Creates a clone: `let cloned = s.clone();`
    - Returns the original: `return s;`
4. In `main()`:
    - Create `let s1 = String::from("hello");`
    - Call `take_ownership(s1);`
    - Try to print s1 (this will fail - ownership was transferred!)
    - Comment out the print statement
    - Create `let s2 = String::from("world");`
    - Call `let s2 = borrow_and_clone(s2);` (reassign s2)
    - Now print s2 (this works because we got it back)

**Expected output:**
```
I now own: hello
world
```

---

### Task 7: References & Borrowing
**Goal:** Use references instead of transferring ownership

1. Create a file called `borrowing.rs`
2. Write a function `calculate_length` that:
    - Takes parameter `s: &String` (immutable reference)
    - Returns `usize`
    - Returns `s.len()`
3. Write a function `append_exclamation` that:
    - Takes parameter `s: &mut String` (mutable reference)
    - Returns nothing
    - Calls `s.push_str("!!!");`
4. In `main()`:
    - Create `let s1 = String::from("hello");`
    - Call `let length = calculate_length(&s1);`
    - Print "'{s1}' has length {length}"
    - Notice s1 is still usable!
    - Create `let mut s2 = String::from("world");`
    - Call `append_exclamation(&mut s2);`
    - Print s2

**Expected output:**
```
'hello' has length 5
world!!!
```

---

## Phase 3: Data Structures

### Task 8: Vectors
**Goal:** Work with dynamic arrays

1. Create a file called `vectors.rs`
2. In `main()`:
    - Create an empty vector: `let mut numbers: Vec<i32> = Vec::new();`
    - Push the numbers 10, 20, 30, 40, 50 to it
    - Print the entire vector using `println!("{:?}", numbers);`
    - Access the third element (index 2) and print it
    - Modify the second element (index 1) to be 25
    - Print the vector again
    - Use a for loop to iterate and print each number
    - Use a for loop with `numbers.iter_mut()` to double each number (multiply by 2)
    - Print the final vector
    - Remove the last element using `numbers.pop();`
    - Print what `pop()` returned (it returns `Option<i32>`)
    - Print the final vector

**Expected output:**
```
[10, 20, 30, 40, 50]
30
[10, 25, 30, 40, 50]
10
25
30
40
50
[20, 50, 60, 80, 100]
Some(100)
[20, 50, 60, 80]
```

---

### Task 9: Structs
**Goal:** Define and use custom data types

1. Create a file called `structs.rs`
2. Before `main()`, define a struct:
```rust
struct Person {
    name: String,
    age: u32,
    email: String,
}
```
3. Add an `impl` block for Person with:
    - A method `display` that takes `&self` and prints: "{name} ({age} years old) - {email}"
    - An associated function `new` that takes `name: String, age: u32, email: String` and returns `Person` (construct the struct)
4. In `main()`:
    - Create a person using `Person::new`
    - Call the `display` method on it
    - Create a vector of at least 3 people
    - Loop through the vector and call `display` on each

**Expected output:**
```
Alice (30 years old) - alice@example.com
Alice (30 years old) - alice@example.com
Bob (25 years old) - bob@example.com
Charlie (35 years old) - charlie@example.com
```

---

### Task 10: Enums & Pattern Matching
**Goal:** Use enums and match expressions

1. Create a file called `enums.rs`
2. Define an enum before `main()`:
```rust
enum Shape {
    Circle(f64),              // radius
    Rectangle(f64, f64),      // width, height
    Triangle(f64, f64, f64),  // three sides
}
```
3. Write a function `calculate_area` that:
    - Takes parameter `shape: &Shape`
    - Returns `f64`
    - Uses `match` to calculate area:
        - Circle: π × r² (use `3.14159`)
        - Rectangle: width × height
        - Triangle: Use Heron's formula or just return 0.0 for simplicity
4. In `main()`:
    - Create a circle with radius 5.0
    - Create a rectangle with width 4.0 and height 6.0
    - Create a triangle (any values)
    - Print the area of each using the function

**Expected output:**
```
Circle area: 78.53975
Rectangle area: 24
Triangle area: 0
```

---

## Phase 4: Error Handling

### Task 11: Option Type
**Goal:** Handle values that might not exist

1. Create a file called `options.rs`
2. Write a function `find_number` that:
    - Takes parameters `numbers: &Vec<i32>, target: i32`
    - Returns `Option<usize>`
    - Uses a for loop with `.iter().enumerate()` to find the index
    - Returns `Some(index)` if found, `None` if not found
3. In `main()`:
    - Create a vector: `vec![10, 20, 30, 40, 50]`
    - Search for 30 using `match` and print "Found at index {i}" or "Not found"
    - Search for 25 using `if let Some(i) = ...` and print if found
    - Search for 100 using `.unwrap_or(999)` and print "Index: {result}"

**Expected output:**
```
Found at index 2
Index: 999
```

---

### Task 12: Result Type
**Goal:** Handle operations that can fail

1. Create a file called `results.rs`
2. Write a function `safe_divide` that:
    - Takes parameters `a: f64, b: f64`
    - Returns `Result<f64, String>`
    - If b is 0.0, return `Err(String::from("Division by zero!"))`
    - Otherwise return `Ok(a / b)`
3. Write a function `calculate` that:
    - Takes parameter `x: f64`
    - Returns `Result<f64, String>`
    - Calls `safe_divide(100.0, x)?` and stores result
    - Calls `safe_divide(result, 2.0)?` and returns it
4. In `main()`:
    - Call `safe_divide(10.0, 2.0)` and use `match` to print result or error
    - Call `calculate(5.0)` and use `match` to print result or error
    - Call `calculate(0.0)` and use `match` to print result or error

**Expected output:**
```
Result: 5
Result: 10
Error: Division by zero!
```

---

## Phase 5: Practical Projects

### Task 13: Temperature Converter
**Goal:** Build a small interactive CLI program

1. Create a file called `temperature.rs`
2. Add at the top: `use std::io;`
3. Write a function `celsius_to_fahrenheit` that converts C to F: `(c * 9.0 / 5.0) + 32.0`
4. Write a function `fahrenheit_to_celsius` that converts F to C: `(f - 32.0) * 5.0 / 9.0`
5. In `main()`:
    - Print "Temperature Converter"
    - Print "Enter temperature:"
    - Read input: `let mut temp = String::new(); io::stdin().read_line(&mut temp).unwrap();`
    - Parse it: `let temp: f64 = temp.trim().parse().unwrap();`
    - Print "Enter unit (C or F):"
    - Read unit the same way
    - Trim and convert to uppercase: `let unit = unit.trim().to_uppercase();`
    - Use `match` or `if` on unit:
        - If "C": convert to F and print
        - If "F": convert to C and print
        - Otherwise: print "Invalid unit"

**Expected interaction:**
```
Temperature Converter
Enter temperature:
100
Enter unit (C or F):
C
100°C = 212°F
```

---

### Task 14: Todo List
**Goal:** Manage data with structs and file I/O

1. Create a file called `todo.rs`
2. Add: `use std::fs; use std::io;`
3. Define a struct:
```rust
struct Task {
    description: String,
    completed: bool,
}
```
4. Write these functions:
    - `add_task(tasks: &mut Vec<Task>, description: String)` - pushes a new task
    - `list_tasks(tasks: &Vec<Task>)` - prints all with index, description, and [x] or [ ]
    - `complete_task(tasks: &mut Vec<Task>, index: usize)` - sets completed to true
    - `save_tasks(tasks: &Vec<Task>)` - writes each task to "tasks.txt" (one per line: "description|completed")
    - `load_tasks() -> Vec<Task>` - reads from "tasks.txt" and parses back to vector (handle file not existing)
5. In `main()`:
    - Load tasks at start
    - Loop forever:
        - Print menu: "1. Add 2. List 3. Complete 4. Quit"
        - Read choice
        - Match on choice and call appropriate function
        - Save after each modification
        - Break on 4

**Expected interaction:**
```
1. Add 2. List 3. Complete 4. Quit
1
Enter task:
Buy groceries
1. Add 2. List 3. Complete 4. Quit
2
0: [ ] Buy groceries
```

---

### Task 15: Word Counter
**Goal:** Process files and use HashMap

1. Create a file called `word_counter.rs`
2. Add: `use std::fs; use std::collections::HashMap;`
3. Create a sample text file called `input.txt` with a few sentences
4. In `main()`:
    - Read the file: `let contents = fs::read_to_string("input.txt").unwrap();`
    - Create a HashMap: `let mut word_count: HashMap<String, u32> = HashMap::new();`
    - Split into words: `for word in contents.split_whitespace()`
    - Convert each word to lowercase and remove punctuation (use `.to_lowercase()` and `.trim_matches()`)
    - Insert into HashMap:
```rust
let counter = word_count.entry(word.to_string()).or_insert(0);
*counter += 1;
```
- Convert HashMap to vector of tuples and sort by count (descending)
- Print top 10 most common words

**Expected output:**
```
the: 15
and: 12
to: 10
...
```
