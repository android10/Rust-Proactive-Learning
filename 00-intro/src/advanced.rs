use std::fmt::{Display, Debug};
use std::ops::Neg;
use std::clone::Clone;
use std::marker::Copy;

pub fn advanced_rust() {
    references();
    advanced_functions();
    generics();
}

fn references() {
    println!("--------------------------------------------------------------");
    println!("REFERENCES ADVANCED RUST:");
    println!("--------------------------------------------------------------");
    println!("https://fasterthanli.me/articles/a-half-hour-to-learn-rust");
    println!("--------------------------------------------------------------");
}

struct Point {
    x: i32,
    y: i32,
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self { 
        Point {
            x: -self.x,
            y: -self.y,
        } 
    }
}

impl Clone for Point {
    fn clone(&self) -> Self { 
        Self { ..*self } 
    }
}

impl Copy for Point {}

#[derive(Clone, Copy)]
struct Number {
    _odd: bool,
    _value: i32,
}

fn advanced_functions() {
    let point_one = Point { x: 1, y: 2 };
    print_point(&point_one);

    let point_two = point_one.clone();
    print_point(&point_two);

    let mut point_three = point_two.clone();
    point_three.x = 5;
    point_three.y = 8;
    print_point(&point_three);
    print_point_without_borrow(point_three);

    let point_four = Clone::clone(&point_three);
    print_point(&point_four);
    print_point_without_borrow(point_four);
}

fn print_point(point: &Point) {
    println!("Point (X -> {}) (Y -> {})", point.x, point.y)
}

fn print_point_without_borrow(point: Point) {
    println!("Point (X -> {}) (Y -> {})", point.x, point.y)
}

fn generics() {
    generic_fn("arg");
    print("this is a value using generics");
    compare("tea", "coffee");
    compare("coffee", "coffee");
    type_name();

    let pair_one = Pair { a: true, b: false };
    print_type_name(pair_one);

    let pair_two = Pair { a: 1, b: 2 };
    print_type_name(pair_two);
}

fn generic_fn<T>(arg: T) where T: Display {
    println!("Generic argument of implementing Clone Trait: {}", arg);
}

fn print<T: Display>(value: T) {
    println!("value: {}", value);
}

fn compare<T>(left: T, right: T) where T: Debug + PartialEq {
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn type_name() {
    use std::any::type_name;
    println!("{}", type_name::<i32>());
    println!("{}", type_name::<(i64, char)>());
}

fn print_type_name<T>(_arg: T) {
    println!("Type name of argument: {}", std::any::type_name::<T>());
}

#[derive(Clone, Copy)]
struct Pair<T> {
    a: T,
    b: T,
}

