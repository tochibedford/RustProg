fn main() {
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str= &s[6..11];

    println!("{hello}");
}
