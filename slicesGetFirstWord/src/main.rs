fn main() {
    let s = String::from("hello");
    let result = first_word(&s);
    println!("{result}");
}

fn first_word(s: &String) -> usize {
    // my version
    let mut count = 0;

    for letter in s.chars(){
        if letter == ' '{
            break;
        }
        count += 1;
    }
    count
}