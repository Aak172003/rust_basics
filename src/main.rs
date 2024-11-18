fn main() {
    basics_rust();

    // Data Types
    data_types();

    // &str and String
    str_string_literals();

    // Tuples
    learn_about_tuple();

    // Functions
    print_value(50)
}

// ------------------------------- Basics Rust --------------------------------
fn basics_rust() {
    println!("Hello, world!");
    println!("Welcome , to Rust programming language");
    println!("Again run rust code......");
}

// ------------------------------- Data Types --------------------------------
fn data_types() {
    // unsigned integer which is 8 bits
    let num: u16 = 256;
    println!("This is stored in num : {}", num);

    let mut num2: i8 = -44;
    // by default whatever we carete variable in rust , it is immutable
    // to make mutable any variable

    println!("This is stored in num2 : {}", num2);

    num2 = -120;

    println!("This is stored in num2 after perform mutable : {}", num2);
}

// ------------------------------- String Literals --------------------------------

fn str_string_literals() {
    let string_literal: &str = "Hi Rust for string_literal";

    let mut string_literal_2 = String::from("Hi Rust for string_literal_2");

    println!("This is string_literals : {}", string_literal);
    println!("This is string_literal_2 : {}", string_literal_2);

    string_literal_2.push_str(" what's app Rust");

    println!(
        "This is string_literal_2 after push some text : {}",
        string_literal_2
    );
}

// ------------------------------- Tuples --------------------------------

fn learn_about_tuple() {
    let employee_info: (&str, u8) = ("Ramesh", 50);

    println!("employee info : {}", employee_info.1);
    println!(
        "employee name : {} employee age : {}",
        employee_info.0, employee_info.1
    );

    // Destructure
    let (emp_name, emp_age) = employee_info;

    println!("employee name : {} employee age : {}", emp_name, emp_age);
}

// ------------------------------- Functions --------------------------------

fn print_value(item: u8) {
    println!("Hello Function");
    println!("value is : {} ", item);

    let num1: u8 = 10;
    let num2: u8 = 20;

    let result: u8 = add(num1, num2);

    println!("the sum value is : {} ", result);
}

// here when any funstion return any value , so also we need to mention what return type of value
fn add(num1: u8, num2: u8) -> u8 {
    return num1 + num2;
}
