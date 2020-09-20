/**
 * Конкатенація рядків
*/

fn main() {
    by_moving(); // Віддаємо перевагу цьому способу, використовує менше памяті, діє прямолінійно
    by_cloning();// коли потрібно залишити дані недоторканими, проте може використовувати багато памяті
    by_mutating(); // варіант, який продуктивно працює як в першому випадку, але мутує змінну. Використовуємо у випадку великої потреби мутації даних чи в представлені стану в дуже малому і керованому контексті.
    
    let my_string = String::from("hello world");
    // first_word працює на зрізі `String`
    let _word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word працює на зрізі літералів string
    let _word = first_word(&my_string_literal[..]);
    // Оскільки літерали string вже є зрізом string,
    // це також працює без синтаксису зрізу!
    let _word = first_word(my_string_literal);
}

fn by_moving() {
    let hello = "hello ".to_string(); // :String
    let world = "world!";             // :&str

    // Переміщуємо hello до нової змінної
    let hello_world = hello + world;
    // hello НЕ МОЖЕ більше використовуватись
    //println!("{}", hello); // panic
    println!("{}", world); // -> "hello"
    println!("{}", hello_world); // -> "hello world!"
}

fn by_cloning() {
    let hello = "hello ".to_string(); // :String
    let world = "world!";             // :&str

    // Створюємо копію hello і переміщуємо в нову змінну
    let hello_world = hello.clone() + world;
    // hello може далі використовуватись
    println!("{}", hello_world); // Друкуємо "hello world!"
}

fn by_mutating() {
    let mut hello = "hello ".to_string();
    let world = "world!";

    // Модифікуємо hello
    hello.push_str(world);
    // hello містить два рядка
    println!("{}", world);
    println!("{}", hello); // Друкуємо "hello world!"
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // отримуємо масив байтів (УВАГА використовуємо тільки для Ascii кодувань)

    for (i, &item) in bytes.iter().enumerate() { // enumerate обгортає ітератор в кортеж (index, &item)
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
