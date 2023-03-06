use num::bigint::BigInt;

fn main() {
    // Fahrenheit to Celsius
    println!("Fahrenheit to Celsius: ");
    print!("32F : ");
    println!("{}", fahrenheit_to_celsius(32.0));
    print!("212F : ");
    println!("{}", fahrenheit_to_celsius(212.0));
    print!("-40F : ");
    println!("{}", fahrenheit_to_celsius(-40.0));
    // Determine sign
    println!("Determine sign: ");
    print!("-5: ");
    determine_sign(-5);
    print!("10: ");
    determine_sign(10);
    print!("0: " );
    determine_sign(0);
    // Roots of quadratic equation
    println!("Roots of quadratic equation: ");
    print!("a: 2, b: 4, c: 2 = ");
    println!("{:?}", roots_of_quadratic_equation(2.0, 4.0, 2.0));
    print!("a: 1, b: 0, c: 0 = ");
    println!("{:?}", roots_of_quadratic_equation(1.0, 0.0, 0.0));
    print!("a: 4, b: 5, c: 1 = ");
    println!("{:?}", roots_of_quadratic_equation(4.0,5.0,1.0));
    // BMI
    println!("BMI: ");
    print!("Weight: 45, Height: 1.7 = ");
    imc(45.0, 1.7);
    print!("Weight: 55, Height: 1.5 = ");
    imc(55.0, 1.5);
    print!("Weight: 76, Height: 1.7 = ");
    imc(76.0, 1.7);
    print!("Weight: 81, Height: 1.6 = ");
    imc(81.0, 1.6);
    print!("Weight: 120, Height: 1.6 = ");
    imc(120.0, 1.6);
    // Factorial
    println!("Factorial: ");
    print!("Factorial of 0: ");
    println!("{}", factorial(0));
    print!("Factorial of 5: ");
    println!("{}", factorial(5));
    print!("Factorial of 40: ");
    println!("{}", factorial(40));
    // Duplicate list
    println!("Duplicate list: ");
    print!("List: [] = ");
    println!("{:?}", duplicate_list::<String>(vec![]));
    print!("List: [1, 2, 3, 4, 5] = ");
    println!("{:?}", duplicate_list(vec![1, 2, 3, 4, 5]));
    print!("List: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] = ");
    println!("{:?}", duplicate_list(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']));
    // Power of two numbers
    println!("Power of two numbers: ");
    print!("5^0 = ");
    println!("{}", power_of_two_numbers(5, 0));
    print!("-5^3 = ");
    println!("{}", power_of_two_numbers(-5, 3));
    print!("15^12 = ");
    println!("{}", power_of_two_numbers(15, 12));
    // Positive numbers
    println!("Positive numbers: ");
    print!("List: [] = ");
    println!("{:?}", positive_numbers(vec![]));
    print!("List: [12, -4, 3, -1, -10, -13, 6, -5] = ");
    println!("{:?}", positive_numbers(vec![12, -4, 3, -1, -10, -13, 6, -5]));
    print!("List: [-4, -1, -10 -13, -5] = ");
    println!("{:?}", positive_numbers(vec![-4, -1, -10, -13, -5]));
    // Add list
    println!("Add list: ");
    print!("List: [] = ");
    println!("{}", add_list(vec![]));
    print!("List: [2,4,1,3] = ");
    println!("{}", add_list(vec![2,4,1,3]));
    print!("List: [1,2,3,4,5,6,7,8,9,10] = ");
    println!("{}", add_list(vec![1,2,3,4,5,6,7,8,9,10]));

    // Invert pairs
    println!("Invert pairs: ");
    print!("List: [] = ");
    println!("{:?}", invert_pairs(vec![]));
    print!("List: [[a,1],[a,2],[b,1],[b,2]] = ");
    println!("{:?}", invert_pairs(vec![vec!["a".to_string(), "1".to_string()], vec!["a".to_string(), "2".to_string()], vec!["b".to_string(), "1".to_string()], vec!["b".to_string(), "2".to_string()]]));
    print!("List: [[1 January], [2 February], [3 March]= ");
    println!("{:?}", invert_pairs(vec![vec!["January".to_string(), "1".to_string()], vec!["February".to_string(), "2".to_string()], vec!["March".to_string(), "3".to_string()]]));    // Binaries

    // List of symbols
    let lst1: &[String] = &[];
    let lst2: &[String] = &["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
    let lst3: &[String] = &["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "42".to_string()];
    println!("List of symbols: ");
    print!("List: [] = ");
    println!("{:?}", list_of_symbols(lst1));
    print!("List: ['a', 'b', 'c', 'd', 'e'] = ");
    println!("{:?}", list_of_symbols(lst2));
    print!("List: ['a', 'b', 'c', 'd','42' ,'e'] = ");
    println!("{:?}", list_of_symbols(lst3));

    // Swapper
    println!("Swapper: ");
    let a = 1;
    let b = 2;
    println!("{:?}",  swapper(a, b, vec![4, 4, 5, 2, 4, 8, 2, 5, 6, 4, 5, 1, 9, 5, 9, 9, 1, 2, 2, 4].as_slice()));
    println!("{:?}",  swapper(a, b, vec![4,3,4,9,9,3,3,3,9,9,7,9,3,7,8,7,8,4,5,6].as_slice()));
    println!("{:?}", swapper("purr", "kitty", &["soft kitty", "warm kitty", "little ball of fur", "happy kitty", "sleepy kitty", "purr", "purr", "purr"]));

    // Dot product
    println!("Dot product: ");
    print!("List 1: [] = ");
    println!("{}", dot_product(vec![], vec![]));
    print!("List 1: [1,2,3] | List 2: [4,5,6] = ");
    println!("{}", dot_product(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]));
    print!("List 1: [1.3, 3.4, 5.7, 9.5, 10.4] | List 2: [-4.5, 3.0, 4.5, 0.9, 0.0] = ");
    println!("{}", dot_product(vec![1.0, 3.0, 5.0, 9.0, 10.0], vec![-4.0, 3.0, 4.0, 0.0, 0.0]));

    // Average
    println!("Average: ");
    print!("List: [] = ");
    println!("{}", average(vec![]));
    print!("List: [4] = ");
    println!("{}", average(vec![4.0]));
    print!("List: [5,6,1,6,0,1,2] = ");
    println!("{}", average(vec![5.0,6.0,1.0,6.0,0.0,1.0,2.0]));
    print!("List: [1.7,4.5,0,2.0,3.4,5,2.5,2.2,1.2] = ");
    println!("{}", average(vec![1.7,4.5,0.0,2.0,3.4,5.0,2.5,2.2,1.2]));
    // Binary of number
    println!("Binary of number: ");
    print!("0 = ");
    println!("{}", binary_of_number(0));
    print!("30 = ");
    println!("{}", binary_of_number(30));
    print!("45123 = ");
    println!("{}", binary_of_number(45123));

}


fn fahrenheit_to_celsius(f: f64) -> f64 { return (f - 32.0) * 5.0 / 9.0; }

fn determine_sign(number: i32){
    match number {
        0 => println!("Zero"),
        1..=100 => println!("Positive"),
        -100..=-1 => println!("Negative"),
        _ => println!("Number is too big"),
    }
}

fn imc(weight: f32, height: f32) {
    let imc = weight / (height * height);
    let table = [
        (0.0, "Underweight"),
        (20.0, "Normal"),
        (25.0, "Obese1"),
        (30.0, "Obese2"),
        (f32::INFINITY, "Obese3")
    ];
    let (_, status) = table.iter()
        .find(|&&(limit, _)| imc <= limit)
        .unwrap();
    println!("{}", status);
}

fn factorial(number: u128) -> BigInt {
    let mut result = BigInt::from(1);
    for i in 2..=number { result *= i; }
    result
}

fn duplicate_list<T: Clone + Default>(list: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(list.len() * 2);
    for i in list {
        result.push(i.clone());
        result.push(i);
    }
    result
}

fn fibonacci(number: u32) -> u32 {
    return if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}

fn enlist_numbers(number: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 0..number {
        result.push(i);
    }
    return result;
}


fn power_of_two_numbers(number: i64, power: i64) -> i64 {
    let mut result = 1;
    for _ in 0..power { result *= number; }
    return result;
}


fn invert_pairs(list: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for i in list {
        if i.len() >= 2 {
            result.push(vec![i[1].clone(), i[0].clone()]);
        }
    }
    return result;
}

fn roots_of_quadratic_equation(a: f32, b: f32, c: f32) -> (f32, f32) {
    let delta = b.powi(2) - 4.0 * a * c;
    let x1 = (-b + delta.sqrt()) / (2.0 * a);
    let x2 = (-b - delta.sqrt()) / (2.0 * a);
    return (x1, x2);
}

fn dot_product(list1: Vec<f64>, list2: Vec<f64>) -> f64 {
    let mut result = 0.0;
    for i in 0..list1.len() {
        result += list1[i] * list2[i];
    }
    return result;
}

fn average(list: Vec<f64>) -> f64 {
    let mut result = 0.0;
    for &i in &list {
        result += i;
    }
    return result / list.len() as f64;
}



fn list_of_symbols(lst: &[String]) -> bool {
    for sym in lst {
        if !is_symbol(sym) {
            return false;
        }
    }
    true
}

fn is_symbol(sym: &str) -> bool {
    sym.chars().all(|c| c.is_ascii_alphabetic())
}



fn add_list(list: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in list {
        result += i;
    }
    return result;
}

fn swapper<T: PartialEq + Clone>(a: T, b: T, lst: &[T]) -> Vec<T> {
    lst.iter()
        .map(|x| {
            if *x == a {
                b.clone()
            } else if *x == b {
                a.clone()
            } else {
                (*x).clone()
            }
        })
        .collect()
}



fn positive_numbers(list: Vec<i32>) -> Vec<i32> {
    // Using recursion
    let mut result = Vec::new();
    return if list.len() == 0 {
        result
    } else {
        if list[0] > 0 {
            result.push(list[0]);
        }
        result.append(&mut positive_numbers(list[1..].to_vec()));
        result
    }

}

fn expand_list(list: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    // Result should return the list with each element repeated the number of times of the element itself
    for i in list {
        for _j in 0..i {
            result.push(i);
        }
    }
    return result;
}

fn binary_of_number(number: i32) -> String {
    let mut result = String::new();
    let mut number = number;
    while number > 0 {
        result.push_str(&(number % 2).to_string());
        number /= 2;
    }
    return result.chars().rev().collect();
}