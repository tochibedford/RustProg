fn other_function() -> i32 {
    let number = 3;

    let y = if number == 3 {4} else {5};

    y
}

fn main() {
    let y = other_function();
    
    println!("{y}");
}
