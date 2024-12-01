use std::io::Read;

fn main() {
    println!("Hello, world!");

    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut pointer = 0;
    const MEMORY_SIZE: usize = 30000;
    let mut memory = vec![0; MEMORY_SIZE];

    for c in code.chars() {
        match c {
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '.' => print!("{}", char::from(memory[pointer])),
            ',' => memory[pointer] = std::io::stdin().bytes().next().unwrap().unwrap() as u8,
            '[' => {
                if memory[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pointer += 1;
                        match code.chars().nth(pointer).expect("string index overflow") {
                            '[' => open_brackets += 1,
                            ']' => open_brackets -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pointer -= 1;
                        match code.chars().nth(pointer).expect("string index overflow") {
                            '[' => open_brackets -= 1,
                            ']' => open_brackets += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
