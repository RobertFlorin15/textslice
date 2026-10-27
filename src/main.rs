use std::{io};

fn main() {
    println!("This is an interactive loop!");

    println!(
        "Write a function from this list:
        [first, last, longest, count, trim, preview x, squeeze, reverse, quit]
        + a String"
    );

    let mut input = String::new();

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (word1, dimension) = first_word(&input);
        let input_without_first_word = &input[dimension + 1..input.len() - 1];

        match word1 {
            "first" => println!("{}", get_first(input_without_first_word)),
            "last" => println!("{}", get_last(input_without_first_word)),
            // "longest" => get_longest(),
            // "count" => count(),
            // "trim" => trim(),
            // "preview" => preview(), //the only one with 2 parameters
            // "squeeze" => squeeze(),
            // "reverse" => reverse(),
            "quit\n" => break,
            _ => println!("Retry!")
        }

    }
}

fn first_word(s: &str) -> (&str, usize) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], i);
        }
    }
    (&s[..], s.len())
}

fn get_first(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

fn get_last(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..]
        }
    }
    &s[..]

}

