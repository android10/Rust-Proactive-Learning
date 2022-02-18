pub fn intermediate_rust() {
    references();
    vectors();
    structs();

    let odd_number = Number { odd: true, value: 2 };
    let even_number = Number { odd: false, value: 5 }; 
    print_number_version_one(&odd_number);
    print_number_version_one(&even_number);
    print_number_version_two(&odd_number);
    print_number_version_two(&even_number);
    print_number_value(&odd_number);
    print_number_value(&even_number);
    print_positive_number();
}

fn references() {
    println!("--------------------------------------------------------------");
    println!("REFERENCES INTERMEDIATE RUST:");
    println!("--------------------------------------------------------------");
    println!("https://fasterthanli.me/articles/a-half-hour-to-learn-rust");
    println!("--------------------------------------------------------------");
}

fn vectors() {
    let mut my_first_vec: Vec<i32> = Vec::new();
    my_first_vec.push(1);
    my_first_vec.push(2);
    my_first_vec.push(3);
    my_first_vec.push(4);

    let mut my_second_vec = vec![5, 6, 7];
    my_second_vec.push(8);
}

struct Vec2 {
    x: f64,
    y: f64,
}

struct Point {
    x_coordinate: f64,
    y_coordinate: f64,
}

fn structs() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 14.3, y: 10.5 };
    let v3 = Vec2 {
        x: 12.3,
        ..v2
    };
    let v4 = Vec2 { ..v3 };

    let my_point = Point { x_coordinate: 2.0, y_coordinate: 2.0 };
    let Point { x_coordinate, y_coordinate } = my_point;
    let Point { x_coordinate, .. } = my_point;
}

struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

trait Signed {
    fn is_strictly_negative(self) -> bool;
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool { 
        self.value < 0 
    }
}

impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0 
    }
}

// the `Neg` trait is used to overload `-`, the
// unary minus operator.
impl std::ops::Neg for Number {
    type Output = Number;
    
    fn neg(self) -> Number { 
        Number {
            value: -self.value,
            odd: self.odd,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Self;
    
    fn neg(self) -> Self { 
        Point {
            x_coordinate: -self.x_coordinate,
            y_coordinate: -self.y_coordinate,
        } 
    }
}

fn print_number_version_one(number: &Number) {
    if let Number { odd: true, value } = number {
        println!("The number {} is odd.", value);
    } else if let Number { odd: false, value } = number {
        println!("The number {} is even.", value);
    }
}

fn print_number_version_two(number: &Number) {
    match number {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

fn print_number_value(number: &Number) {
    match number.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", number.value)
    }
}

fn print_positive_number() {
    let minus_two = Number { odd: false, value: -2 };
    println!("Positive? {}", minus_two.is_strictly_positive());

    let minus_four = Number { odd: false, value: -4 };
    println!("Negative? {}", minus_four.is_strictly_negative());

    let one = 1;
    println!("Negative? {}", one.is_strictly_negative());

    let another_number = Number { odd: true, value: 987 };
    let another_number_negative = -another_number;
    println!("{}", another_number_negative.value);
}