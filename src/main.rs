use std::io;

fn main() {
    let mut i = 0;
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    let num_of_test_cases = text.trim().parse().expect("Input not an integer");

    while i < num_of_test_cases {
        text.clear();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let num: i32 = text.trim().parse().expect("Input not an integer");
        let is_prime_number = is_prime(num);

        if is_prime_number == true {
            println!("Yes");
        } else {
            println!("No");
        }

        i += 1;
    }
}

fn is_prime(num: i32) -> bool {
    let upper_bound = f64::sqrt(num as f64) as i32 + 1;
    if num == 1 {
        return false;
    }
    for i in 2..upper_bound {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
