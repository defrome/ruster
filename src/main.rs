fn check_variable(x: i32) {
    if x == 0 {
        println!("Переменная равна 0");
    } else {
        println!("1");
    }
}

fn main() {
    println!("Hello, world!");
    
    // Примеры использования функции
    check_variable(0);  // Выведет: "Переменная равна 0"
    check_variable(5);  // Выведет: "1"
    check_variable(-3); // Выведет: "1"
}
