const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_POINT
}

fn main() {
    // starting temp in F
    let mut fahrenheit: f64 = 32.0; 

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);

    for _i in 1..=5 {
        fahrenheit += 1.0;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
    }
}

// RICK BALDERAS

fn main() {
    let arr1 = [1, 2, 3, 4, 5, 10, 20, 30, 40, 17];

    for &num in arr1.iter() {
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // sum of all numbers 
    let mut sum = 0;
    let mut index = 0;
    while index < arr1.len() {
        sum += arr1[index];
        index += 1;
    }
    println!("sum of array elements: {}", sum);

    // largest number 
    let mut max = arr1[0];
    for &num in arr1.iter() {
        if num > max {
            max = num;
        }
    }
    println!("largest number in the array: {}", max);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
// RICK BALDERAS

fN check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 
    } else if guess > secret {
        1 
    } else {
        -1 
    }
}

fn main() {
    let secret_number = 42; 
    let mut guess_count = 0; 

    loop {
        let guess = 50; 

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("you guessed the correct number: {}", secret_number);
            break;
        } else if result == 1 {
            println!("your guess {} is too high.", guess);
            guess_count -= 1;
        } else {
            println!("your guess {} is too low.", guess);
            guess_count += 1;
        }

        
    }

    println!("number of guesses: {}", guess_count);
}
// RICK BALDERAS
