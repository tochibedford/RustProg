

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i +1)
        }
    }

    
    
    let five = dbg!(Some(5));
    let six = dbg!(plus_one(five));
    let none = dbg!(plus_one(None));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the max is {}", max);
    }

}
