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
//! assert_eq!(result_three, simplified_text);
//!
//! let result_four = simplified_to_traditional(simplified_text);
//! assert_eq!(result_four, traditional_text);
//! ```
#![cfg_attr(feature = "bench", feature(test))]

extern crate bincode;

use std::borrow::Cow;
use std::collections::HashMap;

use bincode::deserialize_from;
use fst::raw::{Fst, Output};
use once_cell::sync::Lazy;

static T2S: Lazy<HashMap<String, String>> =
    Lazy::new(|| deserialize_from(&include_bytes!("../data/t2s.profile")[..]).unwrap());
static S2T: Lazy<HashMap<String, String>> =
    Lazy::new(|| deserialize_from(&include_bytes!("../data/s2t.profile")[..]).unwrap());

// create an fst containing all the keys
static T2S_FST: Lazy<Fst<Vec<u8>>> = Lazy::new(|| {
    let mut keys: Vec<_> = T2S.keys().collect();
    keys.sort_unstable();
    Fst::from_iter_set(keys).unwrap()
});

// create an fst containing all the keys
static S2T_FST: Lazy<Fst<Vec<u8>>> = Lazy::new(|| {
    let mut keys: Vec<_> = S2T.keys().collect();
    keys.sort_unstable();
    Fst::from_iter_set(keys).unwrap()
});

fn is_script(
    raw: &str,
    mapping: &HashMap<String, String>,
    backup: &HashMap<String, String>,
) -> bool {
    let mut buffer = [0; 4];
    for character in raw.chars() {
        let c = character.encode_utf8(&mut buffer);
        if mapping.get(c).is_none() {
            if backup.get(c).is_some() {
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

fn convert_script<'a>(
    raw: &'a str,
    mapping: &HashMap<String, String>,
    fst: &Fst<Vec<u8>>,
) -> Cow<'a, str> {
    let mut converted_characters: Option<String> = None;
    let mut skip_bytes = 0;

    while skip_bytes < raw.len() {
        let tailstr = &raw[skip_bytes..];

        match find_longest_prefix(fst, tailstr.as_bytes()) {
            Some((_, length)) => {
                let tailstr = &tailstr[..length];
                let mapped = mapping.get(tailstr).unwrap();
                if let Some(converted_characters) = converted_characters.as_mut() {
                    converted_characters.push_str(mapped);
                } else if tailstr != mapped {
                    converted_characters
                        .get_or_insert_with(|| raw[..skip_bytes].to_string())
                        .push_str(mapped);
                }
                skip_bytes += tailstr.len();
            }
            None => {
                let first = tailstr.chars().next().unwrap();
                if let Some(converted_characters) = converted_characters.as_mut() {
                    converted_characters.push(first);
                }

                skip_bytes += first.len_utf8();
            }
        }
    }

    match converted_characters {
        Some(s) => Cow::Owned(s),
        None => Cow::Borrowed(raw),
    }
}

pub fn traditional_to_simplified(raw: &str) -> Cow<str> {
    convert_script(raw, &T2S, &T2S_FST)
}

pub fn simplified_to_traditional(raw: &str) -> Cow<str> {
    convert_script(raw, &S2T, &S2T_FST)
}

/// Thanks to @llogiq for this function
/// https://github.com/BurntSushi/fst/pull/104/files
///
/// find the longest key that is prefix of the given value.
///
/// If the key exists, then `Some((value, key_len))` is returned, where
/// `value` is the value associated with the key, and `key_len` is the
/// length of the found key. Otherwise `None` is returned.
///
/// This can be used to e.g. build tokenizing functions.
#[inline]
fn find_longest_prefix(fst: &Fst<Vec<u8>>, value: &[u8]) -> Option<(u64, usize)> {
    let mut node = fst.root();
    let mut out = Output::zero();
    let mut last_match = None;
    for (i, &b) in value.iter().enumerate() {
        if let Some(trans_index) = node.find_input(b) {
            let t = node.transition(trans_index);
            node = fst.node(t.addr);
            out = out.cat(t.out);
            if node.is_final() {
                last_match = Some((out.cat(node.final_output()).value(), i + 1));
            }
        } else {
            return last_match;
        }
    }
    last_match
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

        let traditional = "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
        assert_eq!(simplified, traditional_to_simplified(traditional));
    }

    #[test]
    fn test_simplified_to_traditional() {
        let simplified = "欧洲";
        let traditional = "歐洲";
        assert_eq!(traditional, simplified_to_traditional(simplified));

        let traditional = "人人生而自由﹐在尊嚴咊權利上一律平等。他們賦有理性咊良心﹐並應㕥兄弟關係的精神互相對待。";
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
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

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_traditional_to_traditional(b: &mut Bencher) {
        let traditional = "人人生而自由﹐在尊嚴咊權利上一律平等。他們賦有理性咊良心﹐並應㕥兄弟關係的精神互相對待。";
        b.iter(|| simplified_to_traditional(traditional));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_simplified_to_simplified(b: &mut Bencher) {
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
        b.iter(|| traditional_to_simplified(simplified));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_traditional_is_traditional(b: &mut Bencher) {
        let traditional = "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";
        b.iter(|| is_traditional(traditional));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_simplified_is_simplified(b: &mut Bencher) {
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
        b.iter(|| is_simplified(simplified));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_traditional_is_simplified(b: &mut Bencher) {
        let traditional = "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";
        b.iter(|| !is_simplified(traditional));
    }

    #[bench]
    #[cfg(feature = "bench")]
    fn bench_simplified_is_traditional(b: &mut Bencher) {
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
        b.iter(|| !is_traditional(simplified));
    }
}
