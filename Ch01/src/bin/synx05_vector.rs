fn main() {
    // Створюємо вектор з деякими елементами
    let fruits = vec!["яблуко", "помідор", "груша"];
    // Вектор не може бути безпосередньо роздрукований
    // Але ми можемо побачити його в режимі відладки
    println!("Фрукти: {:?}", fruits);

    // Створюємо порожній вектор і заповнюємо його
    let mut fruits = Vec::new();
    fruits.push("яблуко");
    fruits.push("помідор");
    fruits.push("груша");
    println!("Фрукти: {:?}", fruits);

    // Видаляємо останній елемент
    let last = fruits.pop();
    if let Some(last) = last {
        println!("Видалено {} з {:?}", last, fruits);
    }

    // Вставляємо елемент всередину вектора
    fruits.insert(1, "виноград");
    println!("Фрукти після вставки: {:?}", fruits);

    // Поміняємо місцями два елементи
    fruits.swap(0, 1);
    println!("Фрукти після заміни місцями: {:?}", fruits);

    // Доступ до першого та останнього елементів
    let first = fruits.first();
    if let Some(first) = first {
        println!("Перший фрукт: {}", first);
    }
    let last = fruits.last();
    if let Some(last) = last {
        println!("Останній фрукт: {}", last);
    }

    // Доступ до певних елементів
    let second = fruits.get(1); // Option(Some|None)
    if let Some(second) = second {
        println!("Другий фрукт: {}", second);
    }
    // Доступ до певних елементів без перевірки межі елементу
    let second = fruits[1];
    println!("Другий фрукт: {}", second);



    // Ініціалізуємо вектор з значенням
    // Тут ми заповнюємо наш вектор п'ятьма нулями
    let bunch_of_zeroes = vec![0; 5];
    println!("bunch_of_zeroes: {:?}", bunch_of_zeroes);

    // Видаляємо деякий елемент і зміщаємо всі елементи, що 
    // йдуть за ним на його місце
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Видаляємо {} з {:?}", second_num, nums);

    // Зміна значень елементів вектору
    let mut v = vec![100, 32, 57];
    println!("Значення елементів вектору {:?}", v);
    for i in &mut v {
        print!("{} -> ", i);
        *i += 50;
        println!("{}", i);
    }
    println!("Значення елементів після зміни вектору {:?}", v);

    // Фільтруємо вектор на місці
    let mut names = vec!["Арон", "Фелісія", "Алекс", "Даніель"];
    // Залишаємо імена, що починаються на 'A'
    names.retain(|name| name.starts_with('А'));
    println!("Імена, що починаються на А: {:?}", names);

    // Перевіряємо, чи вектор містить елемент
    println!("Вектор 'names' містить \"Алекс\"? {}", names.contains(&"Алекс"));



    // Видаляємо послідовні (!) дублікати
    let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
    nums.dedup();
    println!("Виведено попередньо відсортований nums: {:?}", nums);

    // Будьте обережні, якщо ваші дані не відсортовані!
    let mut nums = vec![2, 1, 4, 2, 3, 5, 1, 2];
    nums.dedup();
    // Не надрукує те, що можна б було очікувати
    println!("Виведен невідсортований nums: {:?}", nums);

    // Сортуємо вектор
    nums.sort();
    println!("Ручне відсортування nums: {:?}", nums);
    nums.dedup();
    println!("Виведемо, відсортування nums: {:?}", nums);

    // Вектор в зворотньому порядку
    nums.reverse();
    println!("nums після застосування зворотнього порядку: {:?}", nums);

    // Створимо споживаючий ітератор над певним діапазоном вектора
    let mut alphabet = vec!['а', 'б', 'в'];
    print!("Перші дві літери алфавіту: ");
    for letter in alphabet.drain(..2) {
        print!("{} ", letter);
    }
    println!();
    // Злиті елементи більше не знаходяться у векторі
    println!("Алфавіт після зливання елементів: {:?}", alphabet);


    // Перевіримо чи вектор є порожнім
    let mut fridge = vec!["Пиво", "Рештки їжі", "Майонез"];
    println!("Холодильник пороній? {}", fridge.is_empty());
    // Вилаляємо всі елементи
    fridge.clear();
    println!("Зараз холодильник порожній? {}", fridge.is_empty());

    // Розділення вектору на дві частини
    let mut colors = vec!["червоний", "зелений", "синій", "жовтий"];
    println!("Кольори перед розділенням: {:?}", colors);
    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);
    println!("Кольори після розділення: {:?}", colors);
    println!("second_half: {:?}", second_half);

    // Об'єднуємо два вектори разом
    colors.append(&mut second_half);
    println!("Кольори : {:?}", colors);
    // Другий вектор став порожнім
    println!("second_half після з'єднання: {:?}", second_half);

    // Зрощення векторів
    // Ви могли запам'ятати це з JavaScript
    let mut stuff = vec!["1", "2", "3", "4", "5", "6"];
    println!("Оригінальний вектор: {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];
    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();
    println!("Зрощений вектор: {:?}", stuff);
    println!("Видалені елементи з вектора: {:?}", removed_stuff);


    // Оптимізація:
    // Ініціалізуємо вектор з певною ємкістю
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec після створення:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // Звужуємо вектор як найближче до його довжини
    large_vec.shrink_to_fit();
    println!("large_vec після звуження:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // Видаляємо деякий елемент, змінюючи його з останнім
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.swap_remove(1);
    // Це зміннює порядок, але працює в O(1)
    println!("Видаляємо {} з {:?}", second_num, nums);

    // Визначення enum для зберігання різних типів даних
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Вектор з збереженими значеннями enum: {:?}", row);

    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => println!("{}", int),
            SpreadsheetCell::Text(str) => println!("{}", str),
            SpreadsheetCell::Float(fl) => println!("{}", fl),
        }
    }
}
