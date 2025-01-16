fn number_to_binary_mod6(number: u32) -> String {
    let binary = format!("{:b}", number); // Convert to binary string
    let padding_length = (6 - binary.len() % 6) % 6; // Calculate padding
    let padded_binary = format!("{:0>width$}", binary, width = binary.len() + padding_length); // Pad with leading zeros
    padded_binary
}

fn binary_to_chunks_and_numbers(num: u32) -> Vec<usize> {
    let binary = number_to_binary_mod6(num);
    let chunks: Vec<String> = binary
        .as_bytes()
        .chunks(6)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect();

    // Convert each 6-bit chunk to its decimal value
    let numbers: Vec<usize> = chunks
        .iter()
        .map(|chunk| usize::from_str_radix(chunk, 2).unwrap())
        .collect();

    numbers
}

pub fn character_mapper(num: i32) -> String {
    println!("Index generated is {:?}", num);
    let indexes: Vec<usize> = binary_to_chunks_and_numbers(num as u32);
    let mut map: Vec<char> = ('a'..='z') // Lowercase letters
        .chain('A'..='Z')
        .chain('0'..='9')
        .collect();
    map.push('*');
    map.push('-');
    let mut res = "".to_string();
    for index in indexes {
        res.push(*map.get(index).unwrap());
    }
    res
}
