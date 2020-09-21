use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name і field_value не дійсні в цій точці

    // Зшивання колекції з двох векторів
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    println!("Зшита колекція: {:?}", scores);

    // HashMap може перетворювати будь-які типи hashable до будь-якого іншого
    // Перший тип називається "ключ", другий - "значення"
    let mut tv_ratings = HashMap::new();
    // приклад, де ми призначаємо &str до i32
    tv_ratings.insert("The IT Crowd", 8);
    tv_ratings.insert("13 Reasons Why", 7);
    tv_ratings.insert("House of Cards", 9);
    tv_ratings.insert("Stranger Things", 8);
    tv_ratings.insert("Breaking Bad", 10);

    // Перевірка на існування ключа
    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("Чи призначили ми ключ House of Cards? {}", contains_tv_show);
    let contains_tv_show = tv_ratings.contains_key("House");
    println!("Чи призначили ми ключ House? {}", contains_tv_show);

    // Доступ до значення
    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {} out of 10", rating);
    }

    // Якщо ми вставимо значення двічі, ми його перепишемо
    let old_rating = tv_ratings.insert("13 Reasons Why", 9);
    if let Some(old_rating) = old_rating {
        println!("13 Reasons Why's old rating was {} out of 10", old_rating);
    }
    if let Some(rating) = tv_ratings.get("13 Reasons Why") {
        println!("But I changed my mind, it's now {} out of 10", rating);
    }

    // Видалення ключа і його значення
    let removed_value = tv_ratings.remove("The IT Crowd");
    if let Some(removed_value) = removed_value {
        println!("The removed series had a rating of {}", removed_value);
    }

    // Ітерація по всіх доступних ключах та значеннях
    println!("All ratings:");
    for (key, value) in &tv_ratings {
        println!("{}\t: {}", key, value);
    }

    // Ми можемо провести мутовану ітерацію
    println!("All ratings with 100 as a maximum:");
    for (key, value) in &mut tv_ratings {
        *value *= 10;
        println!("{}\t: {}", key, value);
    }

    // Ітерація без посилань, переміщує вміст HashMap
    for _ in tv_ratings {}
    // tv_ratings більше не використовується

    // Подібно до інших колекцій, ви можете встановити виділений розмір для 
    // отримання деякої продуктивності
    let mut age = HashMap::with_capacity(10);
    age.insert("Dory", 8);
    age.insert("Nemo", 3);
    age.insert("Merlin", 10);
    age.insert("Bruce", 9);

    // Ітерація по всіх ключах
    println!("All names:");
    for name in age.keys() {
        println!("{}", name);
    }

    // Ітерація по всіх значеннях
    println!("All ages:");
    for age in age.values() {
        println!("{}", age);
    }

    // Ітерація по всім значенням і їх зміна
    println!("All ages in 10 years");
    for age in age.values_mut() {
        *age += 10;
        println!("{}", age);
    }

    // Використовуємо entry API для призначення значень по-замовчуванню,
    // яких ще не має в HashMap
    {
        let age_of_coral = age.entry("coral").or_insert(11);
        println!("age_of_coral: {}", age_of_coral);
    }
    let age_of_coral = age.entry("coral").or_insert(15);
    println!("age_of_coral: {}", age_of_coral);

    //
}
