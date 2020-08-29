/*

Rust не має конструкторів в структурах даних, проте використовується
конвенція, яка застосована в стандартній бібліотеці і передбачає
шаблон для створення конструктора

*/

use std::borrow::Cow;

fn main() {
    // Ми не повинні думати про внутрішню структуру NameLength
    // Замість цього ми можемо просто викликати конструктор
    let name_length = NameLength::new("John");
    name_length.print(); // -> "The name 'John' is '4' characters long"

let name_length2 = NameLength2::new("John2");
    name_length2.print(); // -> "The name 'John' is '4' characters long"
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    // Користувачу не потрібно нашатовуваи length
    // Ми зробимо це за нього!
    fn new(name: &str) -> Self {
        NameLength {
            length: name.len(),
            name: name.to_string(),
        }
    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name,
            self.length
        );
    }
}

struct NameLength2<'a> {
    name: Cow<'a, str>,
    length: usize,
}


impl<'a> NameLength2<'a> {
    // 
    fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let name: Cow<'a, str> = name.into();
        NameLength2 {
            length: name.len(),
            name,
        }

    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name,
            self.length
        );
    }

}