use std::io;

fn main() {
    'main_loop: loop {
        println!("Enter a temperature in Fahrenheit:");

        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        match input_temp.as_str() {
            "Exit" | "exit" => {
                println!("Exiting program.");
                break 'main_loop
            },
            &_ => ()
        }

        let input_temp: f64 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input!");
                return;
            }
        };

        println!("You entered {input_temp} degrees Fahrenheit, which is {} degrees Celcius.", degf_to_c(input_temp));
    }
}

fn degf_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}