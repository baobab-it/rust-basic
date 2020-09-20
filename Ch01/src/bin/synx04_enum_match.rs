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
    
    println!(">>> Перелічування");
    
    // Прості перелічування
    // Аннотація дозволяє відобразити значення в println! для відладки
    #[derive(Debug)]
    enum IpAddrKind {
         V4,
         V6,
    }
    // призначення до змінних
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // використання як аргументів функцій
    fn route(ip_type: IpAddrKind) {
        println!("{:?}", ip_type);
    }
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    // використання в структурах
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    // Складний варіант структури перелічування
    // дані розміщуємо безпосередньо в перелічуваннях
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    
    // Кожен варіант перелічування може мати свою структуру даних
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // Перелічування також може мати реалізацію своїх власних методів
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // реалізуємо метод
            match self {
                Message::Quit => println!("call: quit"),
                Message::Move{x, y} => println!("call: x = {}, y = {}", x, y),
                Message::Write(str) => println!("call: {}", str),
                Message::ChangeColor(x, y, z) => println!("call: {} {} {}", x, y, z),
            }
        }
    }

    let m1 = Message::Quit;
    m1.call();
    let m2 = Message::Move{x : 10, y : 20};
    m2.call();
    let m3 = Message::Write(String::from("hello"));
    m3.call();
    let m4 = Message::ChangeColor(0, 255, 0);
    m4.call();
    
    // 
    
    println!("=======");
    
    println!(">>> Оператор MATCH");

    println!(">>> Прив'язка частин значення для задоволення шаблону");
    // З 1999 по 2008 рік США виготовляли 25 центів з різним дизайном для кожного з 50-ти штатів
    
    
    // Аннотація дозволяє відобразити значення в println! для відладки
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... тощо
    }
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Штат з четвертака {:?}!", state);
                25
            },
        }
    }
    
    value_in_cents(Coin::Quarter(UsState::Alaska));
    
    println!("=======");
    
    
    println!(">>> Співставлення шаблону в переліченні Option<T>");
    // Перелічення Option<T> використовується для обробки null та не null значень
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    println!("п'ять плюс один рівно {:?}", six);
    let six = plus_one(five);
    let six = match six {
        None => 0,
        Some(i) => i,
    };
    println!("п'ять плюс один рівно {:?}", six);
    // Символ _ перед змінною каже компілятору, щоб не виводим попередження
    // #[warn(unused_variables)], це саме можна досягнути аннотувавши змінну, 
    // структуру аннотацією #[allow(unused_variables)] (див. вище)
    let _none = plus_one(None);
    
    println!("=======");
    
    println!(">>> Заповнювач _");
    // Коли потрібні варіанти оброблені, решту варіантів можна обробити в 
    // одному рукаві, використовуючи заповнювач _
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    println!(">>> Використання конструкції if let");
    let some_u8_value = Some(0u8);
    // Конструкція if let дозволяє викорисовувати один варіант та відкинути всі інші
    // Тобто конструкція виступає в ролі синтаксичного цукру, для зменшення кількості коду
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // Проте ігноруючи всі варіанти ми втрачаємо повну перевірку, і такий варіант є компромісом
    // між коротким кодом і обмежною перевіркою варіантів
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // Також можна використовувати else для обробки всіх інших варіантів (аналогічно _=>())
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", &state),
        _ => count +=1, 
    };
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("{}", count);
}
