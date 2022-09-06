fn main() {
    let mut s1 = String::from("tochukwu");
    let length = calculate_length(&s1);
    println!(" Length of string {s1} is  {length}");
    
    change(&mut s1);
    println!(" string is {s1}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}