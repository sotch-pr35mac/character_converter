/*
 * @author		:: Preston Wang-Stosur-Bassett
 * @date		:: January 26, 2020
 * @description	:: This package converters traditional chinese characters to simplified chinese characters and vice versa
 */

use bincode::deserialize_from;
use std::collections::HashMap;

static T2S: &'static [u8] = include_bytes!("../data/t2s.profile");
static S2T: &'static [u8] = include_bytes!("../data/s2t.profile");

pub struct Converter {
	traditional2simplified_map: HashMap<String, String>,
	simplified2traditional_map: HashMap<String, String>
}

impl Converter {
	pub fn new() -> Converter {
		return Converter {
			traditional2simplified_map: deserialize_from(T2S).unwrap(),
			simplified2traditional_map: deserialize_from(S2T).unwrap()
		}
	}

	fn is_script(raw: &str, mapping: &HashMap<String, String>, backup: &HashMap<String, String>) -> bool {
		for character in raw.chars() {
			if mapping.get(&character.to_string()).is_none() {
				if backup.get(&character.to_string()).is_some() {
					return false;
				}
			}
		}

		return true;
	}

	pub fn is_traditional(&self, raw: String) -> bool {
		return Converter::is_script(&raw, &self.traditional2simplified_map, &self.simplified2traditional_map);
	}

	pub fn is_simplified(&self, raw: String) -> bool {
		return Converter::is_script(&raw, &self.simplified2traditional_map, &self.traditional2simplified_map);
	}

	fn convert_script(raw: &str, mapping: &HashMap<String, String>) -> String {
		let mut converted_characters: String = String::new();

		for character in raw.chars() {
			let mapped_char = mapping.get(&character.to_string());
			if mapped_char.is_some() {
				converted_characters.push_str(mapped_char.unwrap())
			} else {
				converted_characters.push(character);
			}
		}

		return converted_characters;
	}

	pub fn traditional_to_simplified(&self, raw: String) -> String {
		return Converter::convert_script(&raw, &self.traditional2simplified_map);
	}

	pub fn simplified_to_traditional(&self, raw: String) -> String {
		return Converter::convert_script(&raw, &self.simplified2traditional_map);
	}
}