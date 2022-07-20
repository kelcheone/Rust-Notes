fn main() {
    /*
    let s = String::from("hello");
    // s.push_str(", world");
    let mut s1 = s.clone();
    s1.push_str(", string");
    println!("{} {}", s, s1);

    let mut s3 = String::from("data");

    // References.
    calc_len(&mut s3);
    dangle();
    println!("{}", first_word(&s1));
    println!("smt value: {}", smt(false, true));
    */
    let rect = Rectangle {
        length: 32,
        width: 12,
    };
    println!(
        "The area of the reactangle is {} in square pixels",
        rect.area()
    );
    if rect.width() {
        println!("The rectangle has a nonzero width of {} ", rect.width)
    }
    value_in_cents(Coin::Quarter(UsStates::Alaska));

    println!("five {:?}", plus_one(Some(5)));
    println!("None {:?}", plus_one(None));

    let config_max = Some(5);
    match config_max {
        Some(max) => println!("The max Config is {}", max),
        _ => (),
    }
}

fn calc_len(s: &mut String) {
    s.push_str(", new");
    println!("{}", s);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn smt(a: bool, b: bool) -> bool {
    let out = (!b) || b;
    let n_out = out || a;
    let nn_out = !out;
    let f_out = nn_out || n_out;
    f_out
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
