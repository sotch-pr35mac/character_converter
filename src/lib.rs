// @author	:: Preston Wang-Stosur-Bassett <p.wanstobas@gmail.com>
// @date	:: May 4, 2020
// @description :: Turn Traditional Chinese characters into Simplified Chinese characters and vice versa.

//! ### About
//! Turn Traditional Chinese script to Simplified Chinese script and vice-versa. Check string script to determine if string is Traditional or Simplified Chinese.
//!
//! ### Usage
//! ```
//! extern crate character_converter;
//!
//! use character_converter::*;
//!
//!
//! let traditional_text = "歐洲";
//! let simplified_text = "欧洲";
//!
//! // Check script
//! assert!(is_traditional(traditional_text));
//!
//! assert!(!is_simplified(traditional_text));
//!
//! // Convert script
//! let result_three = traditional_to_simplified(traditional_text);
//! assert!(result_three == simplified_text);
//!
//! let result_four = simplified_to_traditional(simplified_text);
//! assert!(result_four == traditional_text);
//! ```
#![cfg_attr(feature = "bench", feature(test))]

extern crate bincode;

use std::collections::HashMap;

use bincode::deserialize_from;
use once_cell::sync::Lazy;

static T2S: Lazy<HashMap<String, String>> =
    Lazy::new(|| deserialize_from(&include_bytes!("../data/t2s.profile")[..]).unwrap());
static S2T: Lazy<HashMap<String, String>> =
    Lazy::new(|| deserialize_from(&include_bytes!("../data/s2t.profile")[..]).unwrap());

fn is_script(
    raw: &str,
    mapping: &HashMap<String, String>,
    backup: &HashMap<String, String>,
) -> bool {
    for character in raw.chars() {
        if mapping.get(&character.to_string()).is_none() {
            if backup.get(&character.to_string()).is_some() {
                return false;
            }
        }
    }

    return true;
}

pub fn is_traditional(raw: &str) -> bool {
    is_script(raw, &T2S, &S2T)
}

pub fn is_simplified(raw: &str) -> bool {
    is_script(raw, &S2T, &T2S)
}

fn convert_script(raw: &str, mapping: &HashMap<String, String>, max: usize) -> String {
    let mut converted_characters: String = String::new();
    let default_take = raw.chars().take(max).count();
    let mut skip_bytes = 0;
    let mut take = default_take;

    while skip_bytes < raw.len() {
        let (index, last_char) = raw[skip_bytes..].char_indices().take(take).last().unwrap();
        let end_index = index + last_char.len_utf8();
        let substr = &raw[skip_bytes..][..end_index];

        match mapping.get(substr) {
            Some(mapped_char) => {
                converted_characters.push_str(mapped_char);
                skip_bytes += substr.len();
                take = default_take;
            }
            None => {
                if take > 1 {
                    take -= 1;
                } else {
                    converted_characters.push_str(substr);
                    skip_bytes += substr.len();
                    take = default_take;
                }
            }
        }
    }

    converted_characters
}

pub fn traditional_to_simplified(raw: &str) -> String {
    convert_script(raw, &T2S, 20)
}

pub fn simplified_to_traditional(raw: &str) -> String {
    convert_script(raw, &S2T, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_traditional() {
        let simplified = "欧洲";
        let traditional = "歐洲";
        assert_eq!(true, is_traditional(traditional));
        assert_eq!(false, is_traditional(simplified));
    }

    #[test]
    fn test_is_simplified() {
        let simplified = "欧洲";
        let traditional = "歐洲";
        assert_eq!(true, is_simplified(simplified));
        assert_eq!(false, is_simplified(traditional));
    }

    #[test]
    fn test_traditional_to_simplified() {
        let simplified = "欧洲";
        let traditional = "歐洲";
        assert_eq!(simplified, traditional_to_simplified(traditional));
    }

    #[test]
    fn test_simplified_to_traditional() {
        let simplified = "欧洲";
        let traditional = "歐洲";
        assert_eq!(traditional, simplified_to_traditional(simplified));
    }
}

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate test;
    use test::Bencher;

    use super::*;

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_traditional_to_simplified(b: &mut Bencher) {
        let traditional = "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";
        b.iter(|| traditional_to_simplified(traditional));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_simplified_to_traditional(b: &mut Bencher) {
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
        b.iter(|| simplified_to_traditional(simplified));
    }
}
