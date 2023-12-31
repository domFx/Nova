pub fn convert_vec_to_string(vec: &Vec<i8>) -> String {
    let null_idx = vec
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(vec.len());

    let char_slice: &[i8] = &vec[0..null_idx];

    let device_name: String = char_slice
        .iter()
        .map(|&c| c as u8 as char)
        .collect();

    device_name
}