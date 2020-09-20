fn main() {
    // Тип String є типом vector, ви можете конструювати його аналогічним чином
    let mut s = String::new();
    s.push('H');
    s.push('i');
    println!("s: {}", s);

    // String можна зконфігурувати з рядкового зрізу (&str)
    // Наступні два способи еквівалентні
    let s = "Hello".to_string();
    println!("s: {}", s);
    let s = String::from("Hello");
    println!("s: {}", s);

    // String в Rust завжди в кодуванні UTF-8
    let s = "汉语 한글 Þjóðhildur 😉 🍺".to_string();
    println!("s: {}", s);
    
    // Додавання рядків один до одного
    let mut s = "Hello ".to_string();
    s.push_str("World");

    // Ітерація на символами, де "символ" визначається як Unicode Scalar Value
    for ch in "Tubular".chars() {
        print!("{}.", ch);
    }
    println!();

    // Хоча будьте обережні "символ" не завжди те, що ви можете очікувати
    for ch in "y̆".chars() {
        // Це не надрукує y̆
        print!("{} ", ch);
    }
    println!();

     for ch in "Здравствуйте".chars() {
        // Це не надрукує y̆
        print!("{} ", ch);
    }
    println!();

    // Розділення рядка різними способами

    // Розділення рядка поділом на дві половинки
    let str_for_split = "HelloThere";
    let (first, second) = str_for_split.split_at(5);
    println!("first: {}, second: {}", first, second);

    // Поділ по ідивідуальним лініям
    let haiku = "\
        she watches\n\
        satisfied after love\n\
        he lies\n\
        looking up at nothing\n\
    ";
    for line in haiku.lines() {
        println!("\t{}.", line);
    }

    // Поділ на підрядки
    for s in "Never;Give;Up".split(';') {
        println!("{}", s);
    }
    // Коли розділення рядку відбувається на початку чи в кінці, в результаті можуть бути порожні рядки
    let s: Vec<_> = "::Hi::There::".split("::").collect();
    println!("{:?}", s);
    
    // Ви можете усунути порожній ряки вкінці, використовуючи split_termitor
    let s: Vec<_> = "Mr. T.".split_terminator('.').collect();
    println!("{:?}", s);

    // char має деякі методи, що ви можете використовувати для розділення
    for s in "I'm2fast4you".split(char::is_numeric) {
        println!("{}", s);
    }

    // Розділяєте тільки певну кількість разів
    for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
        println!("{}", s);
    }

    // Get only the substrings that match a pattern
    // This is the opposite of splitting
    for c in "The Dark Knight rises".matches(char::is_uppercase) {
        println!("{}", c);
    }

    // Check if a string starts with something
    let saying = "The early bird gets the worm";
    let starts_with_the = saying.starts_with("The");
    println!("Does \"{}\" start with \"The\"?: {}", saying, starts_with_the);
    let starts_with_bird = saying.starts_with("bird");
    println!("Does \"{}\" start with \"bird\"?: {}", saying, starts_with_bird);

    // Check if a string ends with something
    let ends_with_worm = saying.ends_with("worm");
    println!("Does \"{}\" end with \"worm\"?: {}", saying, ends_with_worm);

    // Check if the string contains something somewhere
    let contains_bird = saying.contains("bird");
    println!("Does \"{}\" contain \"bird\"?: {}", saying, contains_bird);


    // Remove whitespace

    // Splitting on whitespace might not result in what you expect
    let a_lot_of_whitespace = "    I   love spaaace     ";
    let s: Vec<_> = a_lot_of_whitespace.split(' ').collect();
    println!("{:?}", s);
    // Use split_whitespace instead
    let s: Vec<_> = a_lot_of_whitespace.split_whitespace().collect();
    println!("{:?}", s);

    // Remove leading and trailing whitespace
    let username = "   P3ngu1n\n".trim();
    println!("{}", username);
    // Remove only leading whitespace
    let username = "   P3ngu1n\n".trim_start();
    println!("{}", username);
    // Remove only trailing whitespace
    let username = "   P3ngu1n\n".trim_end();
    println!("{}", username);


    // Parse a string into another data type
    // This requires type annotation
    let num = "12".parse::<i32>();
    if let Ok(num) = num {
        println!("{} * {} = {}", num, num, num * num);
    }

    // Modify the string

    // Replace all occurences of a pattern
    let s = "My dad is the best dad";
    let new_s = s.replace("dad", "mom");
    println!("new_s: {}", new_s);

    // Replace all characters with their lowercase
    let lowercase = s.to_lowercase();
    println!("lowercase: {}", lowercase);

    // Replace all characters with their uppercase
    let uppercase = s.to_uppercase();
    println!("uppercase: {}", uppercase);

    // These also work with other languages
    let greek = "ὈΔΥΣΣΕΎΣ";
    println!("lowercase greek: {}", greek.to_lowercase());

    // Repeat a string
    let hello = "Hello! ";
    println!("Three times hello: {}", hello.repeat(3));
}
