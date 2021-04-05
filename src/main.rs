use std::io;

fn main() {
    let mut input = String::new();

    println!("Input temperature with C/F at the end");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.pop();
    let temperature_type = input.pop().unwrap();
    let mut result_type = String::new();
    println!("value is {}", input);
    let mut result:f32;

    if "C".contains(temperature_type) {
        println!("input is in celcius");
        result_type = "F".to_string();
        result = (input.parse::<f32>().unwrap() * 1.8) + 32.0;
    } else if "F".contains(temperature_type) {
        println!("input is in farenheit");
        result_type = "C".to_string();
        result = (input.parse::<f32>().unwrap() - 32.0) * (5.0 / 9.0);
    } else {
        panic!("Incorrect input");
    };

    println!("Temperature is {value}{type}", 
        value = result,
        type = result_type)
}
