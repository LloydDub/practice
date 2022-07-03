use std::io;

fn main() {
    println!("Please enter c for celsius or f for farenheit");

    let mut c_or_f = String::new();
    io::stdin()
        .read_line(&mut c_or_f)
        .expect("Failed to read line");

    println!("Enter a tempreture as a number to convert it to selected tempreture format");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: f32 = input
        .trim()
        .parse()
        .ok()
        .expect("Program only processes numbers, Enter number");

    let c_temp = (input - 32.0) * 5.0 / 90.0;
    let f_temp = (input * 1.8) + 32.0;

    if c_or_f == "c\n" {
        println!("The tempreture in celsius is: {}", c_temp);
    } else if c_or_f == "f\n" {
        println!("The tempreture in ferenheit is: {}", f_temp);
    }
}

//this application is a learning project to convert tempretures between celsius and farenheit

// F° to C°: (°F - 32) x .5556 = °C

// C° to F°: (°C x 1.8) + 32 = °F

//1. temp = x

//2. just show the conversion to celsius and farenheit based on temp
