fn fahrenheit_to_celcius(fahrenheit_value: i32) -> i32 {
    (fahrenheit_value - 32) * (5/9)
}

fn celcius_to_fahrenheit(celcius_value: i32) -> i32 {
    ((9/5) * celcius_value) + 32
}

fn n_fib(index: i32) -> i32 {
    let mut first = 1;
    let mut second = 1;

    if index == 1 {
        first
    }else if index == 2 {
        second
    } else {
        for _ in 3..index+1 {
            let temp = second;
            second = first + second;
            first = temp;
        }
        second
        
    }
}

fn main() {
    let _x = fahrenheit_to_celcius(32);
    let _y = celcius_to_fahrenheit(0);
    let z = n_fib(6);

    // println!(" value 1: {x}");
    // println!(" value 2: {y}");
    println!(" value: {z}");
}
