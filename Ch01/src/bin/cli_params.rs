use std::env;

fn main() {
    // env::args повертає ітератор параметрів
    println!("Отримуємо наступні параметри: ");
    for arg in env::args() {
        println!("- {}", arg);
    }

    // Доступ до певних параметрів, використовуючи ітератор API
    let mut args = env::args();
    if let Some(arg) = args.nth(0) {
        println!("Шлях до цієї програми: {}", arg);
    }
    if let Some(arg) = args.nth(1) {
        println!("Периший параметр: {}", arg);
    }
    if let Some(arg) = args.nth(2) {
        println!("Другий параметр: {}", arg);
    }

    // Чи як вектор
    let args = env::args().collect::<Vec<_>>();
    println!("Шлях до цієї програми: {}", args[0]);
    
    if args.len() > 1 {
        println!("Перший параметр: {}", args[1]);
    }
    if args.len() > 2 {
        println!("Другий параметр: {}", args[2]);
    }
}
