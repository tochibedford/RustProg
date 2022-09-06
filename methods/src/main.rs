#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { 
            width: size,
            height: size 
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        (self.width > other_rect.width && self.height > other_rect.height) || (self.width > other_rect.height && self.height > other_rect.width)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 40,
        height: 29
    };

    let sq = Rectangle::square(29);

    println!("The area of this {:#?} is: {}", rect1, rect1.area());

    println!("Can {:#?} hold {:#?}? {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("Can {:#?} hold {:#?}? {}", rect1, sq, rect1.can_hold(&sq));
}