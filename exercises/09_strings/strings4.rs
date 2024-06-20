// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // It is a string literal which is a &str
    string("red".to_string()); // .to_string() converts &str to String
    string(String::from("hi")); // .from() converts &str to String
    string("rust is fun!".to_owned()); // .to_owned() is the same as .to_string() which converts &str to String
    string("nice weather".into()); // .into() converts to String
    string(format!("Interpolation {}", "Station")); // format!() returns a String
    string_slice(&String::from("abc")[0..1]); // &String::from("abc")[0..1] returns a &str
    string_slice("  hello there ".trim()); // .trim() returns a &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // .replace() returns a String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // .to_lowercase() returns a String
}
