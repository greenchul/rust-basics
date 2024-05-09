pub fn add_numbers(a:i32, b:i32)->i32{
    a+b
}

pub fn subtract_numbers(a:i32, b:i32)->i32{
    a-b
}

pub fn power(a:i32, b:u32)->i32{
    i32::pow(a, b)
}

pub fn round(a: f32)->f32{
    a.round()
}

pub fn odd_or_even(num: i32)->String{
    if num%2==0{
        "even".to_string()
    }
    else {
        "odd".to_string()
    }
}