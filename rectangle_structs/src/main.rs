#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 =30;
    let height1 = 50;
    let rect1 = (width1, height1);

    let rect2 = Rectangle {
        width: width1,
        height: height1
    };

    println!(
        "The area of the rectangle {:#?} is {} square pixels. ",
        rect2,
        area(&rect2)
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//refactoring to use tuples

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//refactoring with structs
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}