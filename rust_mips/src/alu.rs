pub fn alu(a: i32, b: i32, alu_control: &str) -> i32 {
    // perform not on b
    let mut code: String = alu_control[29..].to_string();
    //not b with alucontrol bit 2
    let foo = code.remove(0).to_digit(10);

    let mut sum = b;

    if foo == Some(1) {
        sum = sum ^ 1;
    }
    let result = match code.as_str() {
        "00" => a & sum, //and
        "01" => a | sum, //or
        "10" => a + sum, //sum
        "11" => panic!("This alu_control signal not supported yet"),
        _ => panic!("Invalid alu_control signal"),
    };
    result
}
