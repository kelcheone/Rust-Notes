fn main() {
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
