use std::fs::read_to_string;
use chrono::{Local, Utc};

struct User {
    first_name: String,
    last_name: String,
    age: i32
}
struct Rect {
    width: i32,
    height: i32
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64)
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self, num: i32) -> i32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    // let name = String::from("Sowdarjya");
    // println!("{}", is_even(-21));
    // println!("{}", fibo(4));
    // println!("{}", get_str_length(name));

    let  user = User {
        first_name: String::from("Sowdarjya"),
        last_name: String::from("Kolay"),
        age: 20
    };

    // println!("{}", user.first_name);
    // println!("{}", user.last_name);
    // println!("{}", user.age);

    let rect = Rect {
        width: 10,
        height: 20
    };
    
    // println!("{}", rect.area());
    // println!("{}", rect.perimeter(1));
    // println!("{}", Rect::debug());

    // let shape1 = Shape::Rectangle(4.0, 6.0);
    // println!("{}", print_area(shape1));
    // let shape2 = Shape::Circle(5.0);
    // println!("{}", print_area(shape2));

    // let  index = find_first_a(String::from("Sowdarjya"));
    // let  index = find_first_a(String::from("Preet"));

    // match index {
    //     Some(value) => println!("Found 'a' at index: {}", value),
    //     None => println!("No 'a' found in the string"),
    // }

    // let res = read_to_string("a.txt");
    // match res {
    //     Ok(data) => println!("{}", data),
    //     Err(err) => println!("Error reading file: {}", err),
    // }
    // let ans = read_from_file(String::from("a.txt"));
    // match ans {
    //     Ok(data) => println!("{}", data),
    //     Err(err) => println!("Error: {}", err),
    // }

    let now = Local::now();
    println!("current time is {}", now);

    let utc_now = Utc::now();
    println!("current UTC time is {}", utc_now);

    let mut a = 69;
    a = 70;
    println!("a is {}", a);

    let mut a1 = String::from("Sowdarjya");
    do_something(&mut a1);
    let a2 = &a1;
    println!("{}", a2);
    println!("{}", a1);

    // Have either one mutable reference or multiple immutable references
    let mut s1 = String::from("Sowdarjya");
    let s2 = &s1; // Immutable reference
    let s3 = &s1; // Another immutable reference    
    println!("s2: {}, s3: {}", s2, s3);

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("{:?}", vec);
    println!("Even numbers: {:?}", even_filter(&vec));
    remove_odd(&mut vec);
    println!("After removing odd numbers: {:?}", vec);

}

fn remove_odd(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return new_vec ;
}

fn do_something(s2: &mut String) {
    s2.push_str(" Kolay");
    println!("{}", s2);
}

fn find_first_a(a: String) -> Option<i32> {
    for (index, char) in a.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

fn read_from_file(file_path: String) -> Result<String, String>{
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not read")),
    }
}

fn print_area(shape: Shape) -> f64{
    match  shape {
        Shape::Rectangle(a, b ) => a * b,
        Shape::Circle(a) => 3.14 * a * a,
    }
}

fn is_even(num: i32) -> bool{
    if num % 2 == 0 {
        return  true;
    }
    return  false;
}

fn fibo(num: u32) -> u32 {
    let mut  first = 0;
    let mut  second = 1;

    if num == 0 {
        return  0;
    } 

    if num == 1 {
        return  1;
    }

    println!("{} {}", first, second); 
    for _ in 0..(num - 1) {
        let temp = second;
        second = first + second;
        println!("{}", second);
        first = temp;
    }

    return  second;
}

fn get_str_length(str: String) -> usize {
    str.chars().count()
}