use std::{char, io};

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
            "first" => println!("{}", get_first(trim(input_without_first_word))),
            "last" => println!("{}", get_last(trim(input_without_first_word))),
            "longest" => println!("{}", longest(trim(input_without_first_word))),
            "count" => println!("{}", count(trim(input_without_first_word))),
            "trim" => println!("{}", trim(input_without_first_word)),
            "preview" => {
                let (string, bool) = preview(input_without_first_word);
                if bool == true {
                    println!("{string}...")
                }
                else {
                    continue;
                }
            }, //the only one with 2 parameters
            "squeeze" => println!("{}", squeeze(trim(input_without_first_word))),
            "reverse" => println!("{}", reverse(trim(input_without_first_word))),
            "quit\n" => break,
            _ => println!("Retry!"),
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
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn get_last(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}

fn longest(s: &str) -> &str {
    let bytes = s.as_bytes();
    // 0 -> first letter | 1 -> last letter
    let mut word: (usize, usize) = (0, 0);
    let mut counter = 0;
    let mut first_letter = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' && item != b'\n' {
            if counter == 0 {
                first_letter = i;
            }
            counter += 1;
        } else {
            if (word.1 - word.0) < counter {
                word = (first_letter, first_letter + counter);
            }
            counter = 0;
        }
    }

    if (word.1 - word.0) < counter {
        word = (first_letter, first_letter + counter);
    }

    &s[word.0..word.1]
}

fn count(s: &str) -> usize {
    let mut number = 0;

    let bytes = s.as_bytes();

    for (_i, &item) in bytes.iter().enumerate() {
        if (item == b' ' && item + 1 != b' ') || (item + 1 == b' ' && item != b' ') {
            number += 1;
        }
    }
    //trim added
    number + 1
}

fn trim(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate().rev() {
        if item != b' '  {
            return &s[..i+1];
        }
    }
    ""
}

fn number_from_char(s: u8) -> usize{
    (s - '0' as u8) as usize
}

fn preview(s: &str) -> (&str, bool) {
    let bytes = s.as_bytes();
    let mut number: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item as char >= '0' && item as char <= '9' {
            number = number * 10 + number_from_char(item);
        }

        else if item == b' ' && number != 0 {
            return (&s[i + 1..number + i + 1], true)
        }

        else {
            println!("Sintaxa gresita: preview numar string");
            break;
        }

    }

    (&s, false)

}

fn squeeze(s: &str) -> String {
    let mut s2: String = String::from("");
    let bytes = s.as_bytes();

    for (_i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            s2.push(item as char);
        }
    }

    s2
}

fn reverse(s: &str) -> String {
    let mut s2: String = String::from("");
    let bytes = s.as_bytes();

    for (_i, &item) in bytes.iter().enumerate().rev() {
        s2.push(item as char);
    }

    s2
}
