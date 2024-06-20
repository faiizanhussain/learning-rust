// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    /* let word = String::from("green"); and let word = "green"; are different. The first one is a String type and the second one is a &str type. The function is_a_color_word() takes a &str type as an argument. So, we need to convert the String type to a &str type. We can do this by using the as_str() method. The as_str() method converts a String to a &str type. So, we can use the as_str() method to convert the String type to a &str type. */

    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

/*

fn main() {
    /* let word = String::from("green"); and let word = "green"; are different. The first one is a String type and the second one is a &str type. The function is_a_color_word() takes a &str type as an argument. So, we need to convert the String type to a &str type. We can do this by using the as_str() method. The as_str() method converts a String to a &str type. So, we can use the as_str() method to convert the String type to a &str type. */

    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word.as_str()) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

*/
