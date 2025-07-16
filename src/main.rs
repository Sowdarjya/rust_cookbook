
fn main() {
    let name = String::from("Sowdarjya");
    println!("{}", is_even(-21));
    println!("{}", fibo(4));
    println!("{}", get_str_length(name));
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