pub fn basic_rust() {
    references();
    variables_basic();
    variables_shadow();
    tuples();
    fair_random_numbers();
    blocks();
    conditionals();
    match_expression();
    directives();
}

fn references() {
    println!("--------------------------------------------------------------");
    println!("REFERENCES BASIC RUST:");
    println!("--------------------------------------------------------------");
    println!("https://fasterthanli.me/articles/a-half-hour-to-learn-rust");
    println!("--------------------------------------------------------------");
}

fn variables_basic() {
    let my_name = "Fernando";
    let my_age: i32 = 41;

    println!("My name is {}.", my_name);
    println!("I am {} years old.", my_age);
}

fn variables_shadow() {
    let x = 10;
    let x = x + 5;

    println!("Shawoding variables: {}", x);
}

fn tuples() {
    let name_and_age = ("Fernando", 41);
    let birthday_and_nationality: (String, String) = ("06.11.1980".to_string(), "Argentina".to_string());
    let (birthday, nationality) = birthday_and_nationality;

    println!("My name is {} and I am {} years old.", name_and_age.0, name_and_age.1);
    println!("My birthday is {}.", birthday);
    println!("I come from {}.", nationality);
}

fn fair_random_numbers() -> (i32, i32) {
    let random_number_one = 5;
    let random_number_two = 8;
    
    (random_number_one, random_number_two)
}

fn blocks() {
    let my_variable = "Outer Variable";
    {
        let my_variable = "Inner variable";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);

    let result = {
        let a: i32 = 10;
        let b: i32 = 15;
        a + b
    };

    println!("Result: {}", result);
}

fn conditionals() -> i32 {
    if feeling_lucky() {
        4
    } else {
        6
    }
}

fn feeling_lucky() -> bool {
    true
}

fn match_expression() -> String {
    
    match feeling_lucky() {
        true => "I am lucky today".to_string(),
        false => "".to_string()
    }   
}

fn directives() {
    let minimum_value = std::cmp::min(80, 40);
    let maximum_value = std::cmp::max(80, 40);
    
    println!("Min: {}", minimum_value);
    println!("Max: {}", maximum_value);
}