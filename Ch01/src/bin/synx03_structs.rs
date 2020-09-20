fn main() {
    // Аннотація (УВАГА символ ! на рівні коробки, над структурами не потрібний)
    // дозволяє подавити значення, які в демонстраційному коді не використовуються
    // Проте в продакшині цю аннотацію не потрібно використовувати, щоб не залишалось
    // застарілого чи мертвого коду, який не використовується
    // https://stackoverflow.com/questions/25877285
    #![allow(dead_code)]
    // Аннотація (УВАГА символ ! на рівні коробки, над змінними не потрібний)
    // дозволяє подавляти оголошенні змінні, які не використовуються в коді
    // Проте в продакшині цю аннотацію не потрібно використовувати, щоб не залишати
    // невикористаних змінних, які забруднюють код
    #![allow(unused_variables)]
    
    println!(">>> Структури");
    // Приклад оголошення структури для зберігання користувацьких даних
    // Аннотація дозволяє відображати поля структури в println! при відладці,
    // або реалізувати типаж std::fmt::Debug для відображення потрібних полів
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // Приклад екземпляру структури з заповненими даними
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    // варіант зміни поля email екземпляра структури
    user1.email = String::from("anotheremail@example.com");
    // функція, яка повертає екземпляр структури User з вказанням адреси та імені.
    // Поле active отримує значення true, а поле sign_in_count отримує значення 1.
    // Функція використовує скорочену ініціалізацію полів
    //  email: email  = email
    //  username: username = username
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    // приклад створення екземпляра структури з іншого екземпляра стуктрури
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);
    // приклад створення екземпляра структури з використанням полів, які
    // не вказані з іншого екземпляра структури
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    // варіант більш кращого форматування при виводі
    println!("{:#?}", user2);
    
    println!("=======");
    
    println!(">>> Створення кортежних структур");
    // Приклад створення двох кортежних структур (структури не мають назв полів)
    // Аннотація дозволяє відображати поля структури в println! при відладці,
    // або реалізувати типаж std::fmt::Debug для відображення потрібних полів
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    // Аннотація дозволяє відображати поля структури в println! при відладці,
    // або реалізувати типаж std::fmt::Debug для відображення потрібних полів
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);
    
    println!("=======");
    
    println!(">>> Створення прожньої структури");
    // Приклад створення порожньої структури без полів
    // Аннотація дозволяє відображати поля структури в println! при відладці,
    // або реалізувати типаж std::fmt::Debug для відображення потрібних полів
    #[derive(Debug)]
    struct Empty();
    
    let empty = Empty();
    println!("{:?}", empty);
    println!("=======");
    
    println!(">>> Використання структур");
    // Приклад розрахунку площі прямокутника
    let width1 = 30;
    let height1 = 50;

    println!(
        "Площа прямокутника {} квадратних пікселів",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    
    println!("=======");
    // Змінимо попередній приклад на кортеж, оскільки ширина і висота логічно зв'язані
    let rect1 = (30, 50);

    println!(
        "Площа прямокутника {} квадратних пікселів",
        area2(rect1)
    );

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    
    println!("=======");
    // Змінимо попередній приклад на структуру для більшої точності встанолення полів ширини і висоти
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "Площа прямокутника {} квадратних пікселів",
        area3(&rect1)
    );

    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    
    println!("=======");
    // Покращимо функцію area, логічно приєднавши його до структури
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    // Rust дозволяє додавати кілька реалізацій до одної і тої ж структури
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    // Rust підтримує асоційовані методи (в ООП це статичні методи класів),
    // такі методи не використовують аргумент &self чи &mut self
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    println!(
        "Площа прямокутника {} квадратних пікселів",
        rect1.area()
    );
    Rectangle::square(3);
    println!("=======");
}
