#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self)->u32{
        self.width * self.height
    }
    fn width(&self)->bool{
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn rectangles(){
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 10,
        height: 20
    };

    println!("rectangle is {:?}", rect);

    println!("the area is {} square pixels", rect.area() );

    if rect.width(){
        println!("rectangle has a width, is is {}", rect.width)
    }

    println!("{}", rect.can_hold(&rect2))
}

