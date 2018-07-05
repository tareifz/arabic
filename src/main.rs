extern crate arabic;


fn main() {
    println!("is this char 'أ' diacritic? -> {}", arabic::is_diacritic(&'أ'));
    println!("Phrase: {}\nCleaned: {}", "السَّلامُ عَليْكُمْ", arabic::remove_diacritics("السَّلامُ عَليْكُمْ"));

    let text = "ألسّلآم عليْكُم";

    println!("text before filtering: {},\n after filtering: {}", text, arabic::remove(text, arabic::Filter {
        diacritics: false,
        hamzah: true,
    }));

    println!("text before filtering: {},\n after filtering: {}", text, arabic::remove(text, arabic::Filter {
        diacritics: true,
        hamzah: true,
    }));
}
