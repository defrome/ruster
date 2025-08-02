// ===== ВАЖНЫЙ СИНТАКСИС RUST =====

// 1. ОБЪЯВЛЕНИЕ ПЕРЕМЕННЫХ И ТИПОВ
fn variables_and_types() {
    // Неизменяемые переменные (по умолчанию)
    let x = 5;
    let y: i32 = 10;
    
    // Изменяемые переменные
    let mut z = 15;
    z = 20;
    
    // Константы
    const MAX_POINTS: u32 = 100_000;
    
    // Статические переменные
    static PROGRAM_NAME: &str = "Rust Syntax Guide";
    
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("MAX_POINTS = {}, PROGRAM_NAME = {}", MAX_POINTS, PROGRAM_NAME);
}

// 2. ОСНОВНЫЕ ТИПЫ ДАННЫХ
fn data_types() {
    // Целые числа
    let a: i8 = 127;      // -128 до 127
    let b: u8 = 255;      // 0 до 255
    let c: i32 = 2147483647;  // 32-битное целое
    let d: u64 = 18446744073709551615;  // 64-битное беззнаковое
    
    // Числа с плавающей точкой
    let e: f32 = 3.14;
    let f: f64 = 3.14159265359;
    
    // Булев тип
    let g: bool = true;
    let h: bool = false;
    
    // Символы
    let i: char = 'A';
    let j: char = '😀';
    
    // Строки
    let k: &str = "Hello";  // строковый срез
    let l: String = String::from("World");  // владеющая строка
    
    println!("Целые: a={}, b={}, c={}, d={}", a, b, c, d);
    println!("Плавающие: e={}, f={}", e, f);
    println!("Булевы: g={}, h={}", g, h);
    println!("Символы: i={}, j={}", i, j);
    println!("Строки: k={}, l={}", k, l);
}

// 3. КОМПОЗИТНЫЕ ТИПЫ
fn composite_types() {
    // Кортежи
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;  // деструктуризация
    println!("Кортеж: x={}, y={}, z={}", x, y, z);
    println!("Доступ по индексу: tuple.0 = {}", tuple.0);
    
    // Массивы
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_same = [3; 5];  // [3, 3, 3, 3, 3]
    println!("Массив: {:?}", arr);
    println!("Массив с одинаковыми элементами: {:?}", arr_same);
}

// 4. ФУНКЦИИ
fn function_with_params(x: i32, y: i32) -> i32 {
    x + y  // возвращаемое значение (без точки с запятой)
}

fn function_without_return(x: i32) {
    println!("Значение: {}", x);
    // неявно возвращает ()
}

// 5. УСЛОВНЫЕ КОНСТРУКЦИИ
fn control_flow() {
    let number = 7;
    
    // if-else
    if number < 5 {
        println!("Число меньше 5");
    } else if number < 10 {
        println!("Число меньше 10");
    } else {
        println!("Число больше или равно 10");
    }
    
    // if как выражение
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("Результат: {}", result);
    
    // match (аналог switch)
    match number {
        1 => println!("Один"),
        2 => println!("Два"),
        3..=5 => println!("От трех до пяти"),
        _ => println!("Другое число"),
    }
    
    // match с привязкой
    match number {
        n @ 1..=5 => println!("Число {} в диапазоне 1-5", n),
        n if n % 2 == 0 => println!("Четное число: {}", n),
        _ => println!("Нечетное число больше 5"),
    }
}

// 6. ЦИКЛЫ
fn loops() {
    // loop (бесконечный цикл)
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;  // выход из цикла
        }
    }
    println!("Выполнено итераций: {}", count);
    
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Значение: {}", element);
    }
    
    // for с диапазоном
    for number in 1..4 {
        println!("{}!", number);
    }
    
    // for с включением верхней границы
    for number in 1..=4 {
        println!("{}!", number);
    }
}

// 7. ВЛАДЕНИЕ И ЗАИМСТВОВАНИЕ
fn ownership_and_borrowing() {
    // Владение
    let s1 = String::from("hello");
    let s2 = s1;  // s1 перемещается в s2, s1 больше недействительна
    // println!("{}", s1);  // Ошибка компиляции!
    println!("{}", s2);
    
    // Клонирование
    let s3 = String::from("world");
    let s4 = s3.clone();  // s3 остается действительной
    println!("s3: {}, s4: {}", s3, s4);
    
    // Заимствование (borrowing)
    let s5 = String::from("hello");
    let len = calculate_length(&s5);  // передача ссылки
    println!("Длина '{}' равна {}", s5, len);
    
    // Изменяемые ссылки
    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("{}", s6);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 8. СТРУКТУРЫ
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);  // кортежная структура
struct AlwaysEqual;  // единичная структура

fn structs() {
    // Создание экземпляра структуры
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    
    // Создание из другой структуры
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  // остальные поля копируются из user1
    };
    
    // Кортежная структура
    let black = Color(0, 0, 0);
    println!("Цвет: R={}, G={}, B={}", black.0, black.1, black.2);
}

// 9. ENUM И PATTERN MATCHING
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Pattern matching с enum
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("Выход"),
        Message::Move { x, y } => println!("Перемещение в x: {}, y: {}", x, y),
        Message::Write(text) => println!("Текст: {}", text),
        Message::ChangeColor(r, g, b) => println!("Цвет: R={}, G={}, B={}", r, g, b),
    }
}

// 10. ОБРАБОТКА ОШИБОК
fn error_handling() {
    // Result
    fn divide(x: f64, y: f64) -> Result<f64, String> {
        if y == 0.0 {
            Err(String::from("Деление на ноль"))
        } else {
            Ok(x / y)
        }
    }
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("Результат: {}", result),
        Err(e) => println!("Ошибка: {}", e),
    }
    
    // unwrap и expect
    let result = divide(10.0, 2.0).unwrap();  // паника при ошибке
    println!("Результат: {}", result);
    
    // ? оператор (только в функциях, возвращающих Result)
    fn process_division() -> Result<f64, String> {
        let result = divide(10.0, 0.0)?;  // возвращает ошибку, если есть
        Ok(result)
    }
    
    // Option
    fn find_item(items: &[i32], target: i32) -> Option<usize> {
        for (index, &item) in items.iter().enumerate() {
            if item == target {
                return Some(index);
            }
        }
        None
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    match find_item(&numbers, 3) {
        Some(index) => println!("Найдено на позиции: {}", index),
        None => println!("Не найдено"),
    }
}

// 11. ТРЕЙТЫ (TRAITS)
trait Summary {
    fn summarize(&self) -> String;
    
    // Реализация по умолчанию
    fn default_summary(&self) -> String {
        String::from("(Читать далее...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn traits() {
    let article = NewsArticle {
        headline: String::from("Пингвины выиграли Кубок Стэнли!"),
        location: String::from("Питтсбург, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("Питтсбургские пингвины снова являются лучшей командой в НХЛ."),
    };
    
    println!("Новая статья! {}", article.summarize());
    println!("{}", article.default_summary());
}

// 12. ЖИЗНЕННЫЕ ЦИКЛЫ (LIFETIMES)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetimes() {
    let string1 = String::from("длинная строка");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("Самая длинная строка: {}", result);
}

// 13. ЗАМЫКАНИЯ (CLOSURES)
fn closures() {
    // Простое замыкание
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // Замыкание с несколькими параметрами
    let add = |x, y| x + y;
    println!("3 + 4 = {}", add(3, 4));
    
    // Замыкание с захватом переменных
    let multiplier = 10;
    let multiply = |x| x * multiplier;
    println!("7 * {} = {}", multiplier, multiply(7));
    
    // Замыкание с явным указанием типов
    let expensive_closure = |num: u32| -> u32 {
        println!("Вычисление...");
        num
    };
    
    expensive_closure(5);
}

// 14. ИТЕРАТОРЫ
fn iterators() {
    let v1 = vec![1, 2, 3];
    
    // Создание итератора
    let v1_iter = v1.iter();
    
    // Использование итератора
    for val in v1_iter {
        println!("Получили: {}", val);
    }
    
    // Методы итераторов
    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("v3: {:?}", v3);
    
    // Фильтрация
    let v4: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v5: Vec<_> = v4.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Четные числа: {:?}", v5);
}

// 15. МАКРОСЫ
fn macros() {
    // println! макрос
    println!("Привет, мир!");
    
    // vec! макрос
    let v = vec![1, 2, 3, 4, 5];
    println!("Вектор: {:?}", v);
    
    // format! макрос
    let s = format!("Привет, {}!", "мир");
    println!("{}", s);
    
    // assert! макрос
    assert!(true, "Это условие всегда истинно");
    
    // assert_eq! макрос
    assert_eq!(2 + 2, 4, "Математика работает!");
}

// 16. МОДУЛИ И ОРГАНИЗАЦИЯ КОДА
mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    
    pub fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }
    
    // Приватная функция
    fn private_function() {
        println!("Эта функция приватная");
    }
}

fn modules() {
    use math::{add, subtract};
    
    println!("5 + 3 = {}", add(5, 3));
    println!("10 - 4 = {}", subtract(10, 4));
}

// 17. GENERICS
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    println!("integer.x = {}", integer.x());
    println!("float.x = {}", float.x());
}

// 18. SMART POINTERS
use std::rc::Rc;
use std::cell::RefCell;

fn smart_pointers() {
    // Box<T> - для размещения данных в куче
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Rc<T> - подсчет ссылок
    let data = Rc::new(5);
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);
    
    println!("data = {}", data);
    println!("data_clone1 = {}", data_clone1);
    println!("data_clone2 = {}", data_clone2);
    println!("Количество ссылок: {}", Rc::strong_count(&data));
    
    // RefCell<T> - внутренняя изменяемость
    let data = RefCell::new(5);
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 10;
    }
    println!("data = {}", data.borrow());
}

// ГЛАВНАЯ ФУНКЦИЯ
fn main() {
    println!("=== ВАЖНЫЙ СИНТАКСИС RUST ===\n");
    
    variables_and_types();
    println!();
    
    data_types();
    println!();
    
    composite_types();
    println!();
    
    println!("Сумма: {}", function_with_params(5, 3));
    function_without_return(10);
    println!();
    
    control_flow();
    println!();
    
    loops();
    println!();
    
    ownership_and_borrowing();
    println!();
    
    structs();
    println!();
    
    enums();
    println!();
    
    error_handling();
    println!();
    
    traits();
    println!();
    
    lifetimes();
    println!();
    
    closures();
    println!();
    
    iterators();
    println!();
    
    macros();
    println!();
    
    modules();
    println!();
    
    generics();
    println!();
    
    smart_pointers();
    println!();
    
    println!("=== КОНЕЦ ОБЗОРА СИНТАКСИСА ===");
}
