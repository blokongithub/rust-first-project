use std::num::ParseIntError;

pub mod calc;

fn main() {
    let mut playing: bool = true;
    let mut option = String::new();
    const lastoption: u8  = 5;
    while playing == true{
        print!("Would you like to \n 1) add 2 numbers \n 2) subtract 2 numbers \n 3) multiply 2 numbers \n 4) divide 1 number by the other \n 5) exit \n");
        std::io::stdin().read_line(&mut option).unwrap();
        let intoption = option.trim().parse::<u8>();
        match intoption {
            Ok(op) => {
                match op {
                    5 => playing = false,
                    4 => calc::divide(),
                    3 => calc::multiply(),
                    2 => calc::subtract(),
                    1 => calc::add(),
                    _ => println!("invalid options")
                }
            }
            Err(op) => {
                println!("please enter an integer below 255")
            }
        }
        option.clear();

    }
}