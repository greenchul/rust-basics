struct Person{
    age: u32,
    name: String
}

pub fn generate_person(name: String, age: u32)->Person{
    let person = Person{
        age,
        name
    };
    person
}

pub fn generate_person_mut(name: String, age: u32)->Person{
    let mut person = Person{
        age,
        name
    };
    person.name = String::from("bob");
    person
}

struct Colour(i32, i32, i32);

pub fn colours(){
    let black = Colour(0,0,0);
}