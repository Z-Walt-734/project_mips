use std::fs;

pub fn read(a_one: &str, a_two: &str) -> String {
    println!("Read data activated");
    let cache_contents =
        fs::read_to_string("src/cache.memr").expect("Error when reading cache memory");

    let mut reg_one_pos: usize = cache_contents.find(a_one).unwrap() + 6 as usize;

    let mut reg_two_pos: usize = cache_contents.find(a_two).unwrap() + 6 as usize;

    let mut reg_one = String::from("");
    let mut reg_two = String::from("");

    //find reg 1
    loop {
        if cache_contents.chars().nth(reg_one_pos).unwrap() == ',' {
            break;
        }
        reg_one.push(cache_contents.chars().nth(reg_one_pos).unwrap());
        reg_one_pos += 1;
    }

    //find reg 2
    loop {
        if cache_contents.chars().nth(reg_two_pos).unwrap() == ',' {
            break;
        }
        reg_two.push(cache_contents.chars().nth(reg_two_pos).unwrap());
        reg_two_pos += 1;
    }

    println!("Fetching data {} and {}", reg_one, reg_two);

    format!("{},{}", reg_one, reg_two)
}

pub fn write(data: i32, a_thr: &str) {
    println!("Write data activated");

    if a_thr == "00000" {
        println!("Error: unable to write to R0. Exiting write function");
        return;
    }

    let mut cache_contents =
        fs::read_to_string("src/cache.memr").expect("Error when reading cache memory");

    let reg_one_pos: usize = cache_contents.find(a_thr).unwrap() + 6 as usize;

    loop {
        if cache_contents.chars().nth(reg_one_pos).unwrap() == ',' {
            break;
        }
        cache_contents.remove(reg_one_pos);
    }

    cache_contents.insert_str(reg_one_pos, &data.to_string()[..]);

    fs::write("src/cache.memr", cache_contents).expect("Data Write to cache failed");

    println!("{} Written to {}", data, a_thr);

}
