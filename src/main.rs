mod decoder;

fn main() {
    let mut sequence = String::new();

    println!("Please, enter a sequence for decoding:");

    match std::io::stdin().read_line(&mut sequence) {
        Ok(n) => {
            println!("Entered {n} bytes, decoding...");

            match decoder::decode(&sequence) {
                Ok(result) => {
                    println!("Decoded succesfully, the result is: {result}");
                }
                Err(err) => {
                    println!("An error occurred while decoding the sequence: {err}");
                }
            }
        }
        Err(err) => {
            println!("An error occurred while entering the sequence: {err}");
        }
    }
}
