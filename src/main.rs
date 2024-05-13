use::std::io;

fn main() {
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    transform_to_pig_latin(&guess);
}

fn transform_to_pig_latin(text: &String) {

    for word in text.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            if ["a","u","i","e","o"].contains(&first_char.to_string().as_str()) {
                println!("{}-hay", word)
            } else {
                print!("{}-{}ay", &word[1..], first_char)
            }
        }
    }
}
