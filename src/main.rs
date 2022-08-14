pub mod lib;

fn main() {
    let s = "réve léve".to_string();
    println!("Forward words: {s:?}");
    let s = lib::reverse_words(s);
    println!("Back words: {s:?}");
    let s = lib::reverse_words(s); // Reset

    let s = lib::reverse_chars(s);
    println!("Back chars: {s:?}");
}
