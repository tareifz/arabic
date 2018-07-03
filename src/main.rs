extern crate arabic;


fn main() {
    println!("is this char 'أ' diacritic? -> {}", arabic::is_diacritic(&'أ'));
    println!("Phrase: {}\nCleaned: {}", "السَّلامُ عَليْكُمْ", arabic::remove_diacritics("السَّلامُ عَليْكُمْ"));
}
