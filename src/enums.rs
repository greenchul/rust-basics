#[derive(Debug)]
enum Animals{
    Cat(String),
    Dog(String),
    Bee(String)
}

impl Animals{
    fn greet(&self){
        println!("{:?} says hello", &self)
    }
}

fn make_sound(animal: &Animals)->String{
    let sound =match animal {
        Animals::Cat(_name) => String::from("Meow"),
        Animals::Dog(_name) => String::from("Woof"),
        Animals::Bee(_name) => String::from("bzzzzzzz"),
    };
    println!("{}", sound);
    sound
}

fn make_sound_2(animal: Animals){
    if let Animals::Cat(_name) = animal{
        print!("MEOW")
    }
}

fn plus_one(num: Option<i32>)-> Option<i32>{
    match num {
        Some(num)=> Some(num +1),
        None => None
    }
}


pub fn enums(){
    let hilary = Animals::Cat(String::from("Hilary"));
    hilary.greet();
    make_sound(&hilary);
    make_sound_2(hilary);

    let five = Some(5);
    let some_five = plus_one(five).unwrap();
    println!("{:?}", some_five);
}

