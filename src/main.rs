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

        match &str {
            "first" => getFirst(),
            "last" => getLast(),
            "longest" => getLongest(),
            "count" => count(),
            "trim" => trim(),
            "preview" => preview(), //the only one with 2 parameters
            "squeeze" => squeeze(),
            "reverse" => reverse(),
            "quit" => break,
            _ => println!("Retry!"),
        }
    }
}
