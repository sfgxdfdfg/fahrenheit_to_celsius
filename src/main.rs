use std::io::stdin;

// Ez pipa
fn main() {
    let mut fahrenheit = String::new();
    stdin().read_line(&mut fahrenheit).expect("Failed!");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Faield!");

    let mut celsius = String::new();
    stdin().read_line(&mut celsius).expect("Failed!");
    let celsius: f64 = celsius.trim().parse().expect("Faield!");

    let mut operation = String::new();
    stdin().read_line(&mut operation).expect("Failed!");

    let result: f64 = match operation.trim() {
      "f_c" => fahrenheit - 32.0,
      "c_f" => celsius * 1.8,
      _=> panic!(),
    };

    println!("The result is: {}", result / 1.8);
    println!("The result is: {}", result + 32.0);

    // let eredmeny = fahrenheit - 32.0;
    // println!("Celsius-ban: {}", eredmeny / 1.8);
}
