fn add_numbers(a:i32, b:i32)->i32{
    a+b
}

fn subtract_numbers(a:i32, b:i32)->i32{
    a-b
}

fn power(a:i32, b:u32)->i32{
    i32::pow(a, b)
}

fn round(a: f32)->f32{
    a.round()
}

fn odd_or_even(num: i32)->String{
    if num%2==0{
        "even".to_string()
    }
    else {
        "odd".to_string()
    }
}

pub fn numbers(){
    println!("{}", add_numbers(2, 3));
    println!("{}", subtract_numbers(24, 8));
    println!("{}", power(2, 2));
    println!("{}", round(4.9));
    println!("{}", odd_or_even(3));
}