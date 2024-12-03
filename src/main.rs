use std::io::Read;

fn main() {
    let code = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++++++++++++++.------------.>+++++++++++++++.>.+++.------.--------.<<+.>+++++++++++.";
    let mut pointer = 0;
    const MEMORY_SIZE: usize = 30000;
    let mut memory = vec![0u8; MEMORY_SIZE];
    let code_chars: Vec<char> = code.chars().collect();
    let mut c = 0;
    let mut stack = Vec::new();

    while c < code_chars.len() {
        match code_chars[c] {
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '>' => {
                pointer += 1;
                if pointer >= MEMORY_SIZE {
                    panic!("pointer overflow");
                }
            }
            '<' => {
                if pointer == 0 {
                    panic!("pointer underflow");
                }
                pointer -= 1;
            }
            '.' => {
                print!("{}", char::from(memory[pointer]));
            }
            ',' => memory[pointer] = std::io::stdin().bytes().next().unwrap().unwrap() as u8,
            '[' => {
                if memory[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        c += 1;
                        if c >= code_chars.len() {
                            panic!("no matching bracket");
                        }
                        match code_chars[c] {
                            '[' => open_brackets += 1,
                            ']' => open_brackets -= 1,
                            _ => {}
                        }
                    }
                } else {
                    stack.push(c);
                }
            }
            ']' => {
                if let Some(last_c) = stack.pop() {
                    if memory[pointer] != 0 {
                        c = last_c - 1;
                        stack.push(last_c)
                    }
                } else {
                    panic!("no matching bracket");
                }
            }
            _ => {}
        }
        c += 1;
    }
}
