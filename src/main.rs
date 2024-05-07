mod numbers;



// takes in a string slice, a reference to part of a string
// returns a String type. This lives on the heap
fn say_hello(name: &str)->String{
    let mut greeting = String::from("hello ");
    greeting.push_str(name);
    greeting
}

fn greet_person(person: &str, hour: i8)->String{
    let mut greeting: String; 
    if hour > 17 {
        greeting = "Good evening ".to_string();
    }
    else if hour >12 {
        greeting = "Good afternoon ".to_string();
    }
    else {
        greeting = "Good morning ".to_string();
    }
    greeting.push_str(person);
    greeting
}

fn count_chars(text: &str)->usize{
    text.len()
}

fn upper_case(text:&str)->String{
    text.to_uppercase()
}

fn first_character(text: &str)->char{
    let first: char = text.chars().next().unwrap();
    first
}

fn n_character(text: &str, n: usize)->char{
    let mut chars: std::str::Chars = text.chars();
    let nth_char: Option<char> = chars.nth(n-1);
    match nth_char{
        Some(x)=> x,
        None => panic!("argh")
    }
}




fn main() {
    println!("*** Strings ***");
    let name: String = String::from("rachel");
    println!("{}", say_hello(&name));
    println!("{}", greet_person(&name, 13));
    println!("{}", count_chars(&name));
    println!("{}", upper_case(&name));
    println!("{}", first_character(&name));
    println!("{}",  n_character(&name, 2) );
    
    println!("*** Numbers ***");
    println!("{}", numbers::add_numbers(2, 3));
    
   
    
}




