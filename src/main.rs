use std::io;

fn main() {
    println!("This is an interactive loop!");

    println!(
        "Write a function from this list:
        [first, last, longest, count, trim, preview x, squeeze, reverse, quit]
        + a String"
    );

    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("First word: {}", first_word(&input));

        // match &str {
        //     "first" => get_first(),
        //     "last" => get_last(),
        //     "longest" => get_longest(),
        //     "count" => count(),
        //     "trim" => trim(),
        //     "preview" => preview(), //the only one with 2 parameters
        //     "squeeze" => squeeze(),
        //     "reverse" => reverse(),
        //     "quit" => break,
        //     _ => println!("Retry!"),
        // }
        input = String::from("");
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
