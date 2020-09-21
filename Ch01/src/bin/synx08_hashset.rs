use std::collections::HashSet;

fn main() {
    // Більшість інтерфейсу HashSet аналогічний HashMap,
    // тільки без методів, що обробляють значення
    let mut books = HashSet::new();
    books.insert("Harry Potter and the Philosopher's Stone");
    books.insert("The Name of the Wind");
    books.insert("A Game of Thrones");

    // HashSet ігнорує дублікати сутностей, але повертає,
    // якщо сутність є новою чи ні
    let is_new = books.insert("The Lies of Locke Lamora");
    if is_new {
        println!("We've just added a new book!");
    }

    let is_new = books.insert("A Game of Thrones");
    if !is_new {
        println!("Sorry, we already had that book in store");
    }

    // Перевіряємо чи колекція містить ключ
    if !books.contains("The Doors of Stone") {
        println!("We sadly don't have that book yet");
    }

    // Видаляємо сутність
    let was_removed = books.remove("The Darkness that comes before");
    if !was_removed {
        println!("Couldn't remove book; We didn't have it to begin with");
    }
    let was_removed = books.remove("Harry Potter and the Philosopher's Stone");
    if was_removed {
        println!("Oops, we lost a book");
    }

    // Порівнюємо два HashSets

    let one_to_five: HashSet<_> = (1..6).collect();
    let five_to_ten: HashSet<_> = (5..11).collect();
    let one_to_ten: HashSet<_> = (1..11).collect();
    let three_to_eight: HashSet<_> = (3..9).collect();

    // Перевіряємо чи два HashSets немають загальних елементів
    let is_disjoint = one_to_five.is_disjoint(&five_to_ten);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five,
        five_to_ten,
        is_disjoint
    );
    let is_disjoint = one_to_five.is_disjoint(&three_to_eight);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five,
        three_to_eight,
        is_disjoint
    );

    // Перевіряємо, якщо HashSet повністю міститься в іншому
    let is_subset = one_to_five.is_subset(&five_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five,
        five_to_ten,
        is_subset
    );
    let is_subset = one_to_five.is_subset(&one_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five,
        one_to_ten,
        is_subset
    );

    // Переіряємо, якщо HashSet повністю містить інший
    let is_superset = three_to_eight.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        three_to_eight,
        five_to_ten,
        is_superset
    );
    let is_superset = one_to_ten.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        one_to_ten,
        five_to_ten,
        is_superset
    );

    // З'єднуємо два HashSets різними способами

    // Отримуємо значення, що містяться в першому HashSet, але немає в другому
    let difference = one_to_five.difference(&three_to_eight);
    println!(
        "The difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        difference
    );

    // Отримуємо значення, що є в кожному HashSets, але не в обох
    let symmetric_difference = one_to_five.symmetric_difference(&three_to_eight);
    println!(
        "The symmetric difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        symmetric_difference
    );

    // Отримуємо значення, що є в обох HashSets
    let intersection = one_to_five.intersection(&three_to_eight);
    println!(
        "The intersection difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        intersection
    );

    // Отримуємо всі значення, що є в обох HashSets
    let union = one_to_five.union(&three_to_eight);
    println!(
        "The union difference between {:?} and {:?} is {:?}",
        one_to_five,
        three_to_eight,
        union
    );
}
