use std::io::stdin;

// Ez pipa
fn main() {
    let mut number = String::new();
    stdin().read_line(&mut number).expect("Failed!");
    let number: f64 = number.trim().parse().expect("Faield!");

    let mut operation = String::new();
    stdin().read_line(&mut operation).expect("Failed!");

    let result: f64 = match operation.trim() {
      "f_c" => number - 32.0,
      "c_f" => number * 1.8,
      _=> panic!(),
    };

    println!("The result is: {}", result / 1.8);
    println!("The result is: {}", result + 32.0);

    // let eredmeny = fahrenheit - 32.0;
    // println!("Celsius-ban: {}", eredmeny / 1.8);
}
