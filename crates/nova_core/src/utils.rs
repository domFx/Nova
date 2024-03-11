#![allow(dead_code)]

// Utilities
use std::os::raw::c_char;

// Converts an vec[i8] to string
pub fn convert_vec_to_string(vec: &Vec<i8>) -> String {
    vec.iter().filter(|&i| i > &0).map(|&c| c as u8 as char).collect()
}

fn char_array_to_string(slice: &[c_char; 256]) -> String {
    String::from_iter(slice.iter().take_while(|c| **c != 0).map(|c| *c as u8 as char))
}

#[cfg(test)]
mod utils_tests {
    #[cfg(test)]
    mod convert_vec_to_string_tests {
        use crate::utils::convert_vec_to_string;

        #[test]
        fn convert_vec_to_string_valid_when_empty() {
            let str = convert_vec_to_string(&vec![]);
            assert_eq!(str, "")
        }

        #[test]
        fn convert_vec_to_string_valid() {
            let str = convert_vec_to_string(&vec!['d' as i8, 'o' as i8, 'm' as i8]);
            assert_eq!(str, "dom")
        }

        #[test]
        fn convert_vec_to_string_invalid_array() {
            let str = convert_vec_to_string(&vec![0]);
            assert_eq!(str, "")
        }
    }
}