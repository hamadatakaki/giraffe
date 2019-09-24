pub fn parse_to_u32_from_vec_u8(vec: &mut Vec<u8>) -> u32 {
    vec.reverse();
    let mut i = 0;
    let mut sum = 0;
    for v in vec {
        sum += (*v as u32) * 8u32.pow(i);
        i += 1;
    }
    sum
}

pub fn convert_to_hex(u: u8) -> String {
    format!("{:>02}", format!("{:x}", u)).replace(" ", "0")
}

pub fn name_length_of_entry(name_length: usize) -> Vec<u8> {
    let mut vec = Vec::new();
    vec.push((name_length / 16) as u8);
    vec.push((name_length % 16) as u8);
    vec
}

pub fn zero_filled_of_entry(name_len: usize) -> Vec<u8> {
    let pad_num = 8 - (6+name_len)%8;
    let mut vec = Vec::new();
    vec.resize(pad_num, 0);
    vec
}
