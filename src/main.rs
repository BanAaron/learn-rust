use std::io;

fn main() {
    // we will store the user input in `input`
    let mut input: String = String::new();

    // prompt for a number
    println!("Enter a number");

    // capture the input
    io::stdin().read_line(&mut input).expect("Invalid input");

    // cast to a i32
    let input_number: i32 = input.trim().parse().expect("Not a number");

    // call fizz_buzz function using the converted user input
    println!("{:?}", fizz_buzz(input_number));
}

fn fizz_buzz(number: i32) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();

    // for x in range(number + 1)
    for x in 1..=number {
        // init a mutable result variable as a new string
        let mut result: String = String::new();

        // check our conditions
        if x % 3 == 0 {
            result = result + "Fizz";
        }
        if x % 5 == 0 {
            result = result + "Buzz";
        }

        // check if we have any results
        if result.len() > 0 {
            answer.push(result);
        } else {
            let result = x.to_string();
            answer.push(result);
        }
    }
    return answer;
}
