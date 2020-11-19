//mod alu;
//mod controller;
//mod dataPath;
mod alu;
mod register;

use std::io;
//use controller::controller;
//use dataPath::dataPath;

fn hex_convert(mut hex: &str) -> String {
    let mut out = String::new();
    if hex.to_uppercase().find('X') != None {
        hex = &hex[1..];
    }
    for digit in hex.to_uppercase().chars() {
        let bin_digit = match digit {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        out.push_str(bin_digit);
    }
    out
}

fn address_convert(address: &str) -> &str {
    match address.trim().to_uppercase().as_ref() {
        "R0" => "00000",
        "R1" => "00001",
        "R2" => "00010",
        "R3" => "00011",
        "R4" => "00100",
        "R5" => "00101",
        "R6" => "00110",
        "R7" => "00111",
        "R8" => "01000",
        "R9" => "01001",
        "R10" => "01010",
        "R11" => "01011",
        "R12" => "01100",
        "R13" => "01101",
        "R14" => "01110",
        "R15" => "01111",
        "R16" => "10000",
        "R17" => "10001",
        "R18" => "10010",
        "R19" => "10011",
        "R20" => "10100",
        "R21" => "10101",
        "R22" => "10110",
        "R23" => "10111",
        "R24" => "11000",
        "R25" => "11001",
        "R26" => "11010",
        "R27" => "11011",
        "R28" => "11100",
        "R29" => "11101",
        "R30" => "11110",
        "R31" => "11111",
        _ => panic!("Error parsing address {}", address),
    }
}

fn main() {
    println!("Welcome to Rust based mips simulator.\nA basic operator for simulating and monitoring the cache memory operation of a MIPS processor.\nGeneral opeartion guidlines can be found in the README");
    loop {
        println!("Enter a standard mips command separated by spaces:\n");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error : User Input Readline Failed.");

        if input.trim().is_empty() {
            break;
        }

        println!("User Input marked as {}", input);

        let parsed_input: Vec<&str> = input.split(' ').collect();

        let instruction = match parsed_input[0].to_uppercase().as_ref() {
            "ADD" => format!(
                "{}{}{}{}{}{}",
                "000000",
                address_convert(parsed_input[2]),
                address_convert(parsed_input[3]),
                address_convert(parsed_input[1]),
                "00000",
                "100000"
            ),
            "SUB" => format!(
                "{}{}{}{}{}{}",
                "000000",
                address_convert(parsed_input[2]),
                address_convert(parsed_input[3]),
                address_convert(parsed_input[1]),
                "00000",
                "100010"
            ),
            "AND" => format!(
                "{}{}{}{}{}{}",
                "000000",
                address_convert(parsed_input[2]),
                address_convert(parsed_input[3]),
                address_convert(parsed_input[1]),
                "00000",
                "100100"
            ),
            "OR" => format!(
                "{}{}{}{}{}{}",
                "000000",
                address_convert(parsed_input[2]),
                address_convert(parsed_input[3]),
                address_convert(parsed_input[1]),
                "00000",
                "100101"
            ),
            "ADDI" => {
                let immd: i16 = parsed_input[3].trim().parse().unwrap();
                let bin_immd = format!("{:016b}", immd);
                format!(
                    "{}{}{}{}",
                    "001000",
                    address_convert(parsed_input[2]),
                    address_convert(parsed_input[1]),
                    bin_immd
                )
            }
            "J" => format!("{}{}", "000010", hex_convert(parsed_input[1])),
            _ => panic!("Error in parsing input"),
        };
        // a_one = instruction[25 : 21]
        // a_two = instruction[20 : 16]
        // a_thr = write_reg(a_two, instruction[15 : 11]) this will be a 5 bit out
        // sign_extend(instruction[15 : 0])
        //
        //
        println!("Current Binary instruction: {}", instruction);

        let ans: i32;
        let a_one = &instruction[6..11];
        let a_two = &instruction[11..16];
        let a_thr = &instruction[16..21];
        let data = register::read(a_one, a_two);

        let data_vec: Vec<&str> = data.split(',').collect();

        println!("Stored address: {} {} {}", a_one, a_two, a_thr);
        if &instruction[2..3] == "1" {
            ans = alu::alu(
                data_vec[0].parse().unwrap(),
                parsed_input[3].trim().parse().unwrap(),
                "00000000000000000000000000000010",
            );
            register::write(ans, a_two);
        } else {
            ans = alu::alu(
                data_vec[0].parse().unwrap(),
                data_vec[1].parse().unwrap(),
                &instruction,
            );
            register::write(ans, a_thr);
        }
        println!("Calculated output: {}", ans);
    }
    println!("EOL")
}
