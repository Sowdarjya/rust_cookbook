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

    let shape1 = Shape::Rectangle(4.0, 6.0);
    println!("{}", print_area(shape1));
    let shape2 = Shape::Circle(5.0);
    println!("{}", print_area(shape2));
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