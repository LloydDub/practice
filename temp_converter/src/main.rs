fn main() {
    let temp = 35.0;
    let c_temp = (temp - 32.0) * 5.0 / 90.0;
    let f_temp = (temp * 1.8) + 32.0;

    println!("The tempreture in celsius is: {}", c_temp);
    println!("The tempreture in ferenheit is: {}", f_temp);
}

//this application is a learning project to convert tempretures between celsius and farenheit

// F° to C°: (°F - 32) x .5556 = °C

// C° to F°: (°C x 1.8) + 32 = °F

//1. temp = x

//2. just show the conversion to celsius and farenheit based on temp
