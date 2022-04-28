// @author		:: Preston Wang-Stosur-Bassett <p.wanstobas@gmail.com>
// @date		:: January 26, 2020
// @description		:: This package converters traditional chinese characters to simplified chinese characters and vice versa

use std::collections::HashMap;

use bincode::deserialize_from;

static T2S: &'static [u8] = include_bytes!("../data/t2s.profile");
static S2T: &'static [u8] = include_bytes!("../data/s2t.profile");

pub struct Converter {
    traditional2simplified_map: HashMap<String, String>,
    simplified2traditional_map: HashMap<String, String>,
}

impl Converter {
    pub fn new() -> Converter {
        Converter {
            traditional2simplified_map: deserialize_from(T2S).unwrap(),
            simplified2traditional_map: deserialize_from(S2T).unwrap(),
        }
    }

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

    pub fn is_traditional(&self, raw: &str) -> bool {
        Converter::is_script(
            raw,
            &self.traditional2simplified_map,
            &self.simplified2traditional_map,
        )
    }

    pub fn is_simplified(&self, raw: &str) -> bool {
        Converter::is_script(
            raw,
            &self.simplified2traditional_map,
            &self.traditional2simplified_map,
        )
    }

    fn convert_script(raw: &str, mapping: &HashMap<String, String>) -> String {
        let mut converted_characters: String = String::new();
        let default_take = raw.chars().take(20).count();
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

    pub fn traditional_to_simplified(&self, raw: &str) -> String {
        Converter::convert_script(raw, &self.traditional2simplified_map)
    }

    pub fn simplified_to_traditional(&self, raw: &str) -> String {
        Converter::convert_script(raw, &self.simplified2traditional_map)
    }
}
