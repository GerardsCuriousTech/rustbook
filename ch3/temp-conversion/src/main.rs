// Celsius to Fahrenheit Conversion
// F = C * 1.8 + 32
// C = (F - 32)/1.8

use std::io;

fn main() {
    let mut response = String::new();
    let mut input_temp = String::new();
    let mut to_celsius = false;

    println!("Would you like to convert Celsius to Fahrenheit?");
    println!("Yes(Y) or No(N)");

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    println!("Recorded Response {}", response);
    response = response.trim().to_lowercase();

    if response != "yes" && response != "y" {
        println!("Converting from Fahrenheit to Celsius");
        to_celsius = true;
    }
    println!("Please enter the temperature");
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    let response: f32 = input_temp.trim().parse().expect("Please type a number!");
    if to_celsius {
        println!("Converting from Fahrenheit to Celsius {} - {}", response, f_to_c(response));
    } else {
        println!("Converting from Celsius to Fahrenheit {} - {}", response, c_to_f(response));
    }
}


fn c_to_f(c: f32) -> f32 {
    return c * 1.8 + 32.0
}

fn f_to_c(f: f32) -> f32 {
    return (f-32.0)/1.8
}
