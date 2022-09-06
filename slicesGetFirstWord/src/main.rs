fn main() {
    let s = String::from("hello world");
    let result = first_word2(&s);
    println!("{result}");
}

fn first_word(s: &str) -> usize {
    // my version
    // let mut count = 0;

    // for letter in s.chars(){
    //     if letter == ' '{
    //         break;
    //     }
    //     count += 1;
    // }
    // count

    // rust book version
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}