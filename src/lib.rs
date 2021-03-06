// @author	:: Preston Wang-Stosur-Bassett <p.wanstobas@gmail.com>
// @date	:: May 4, 2020
// @description :: Turn Traditional Chinese characters into Simplified Chinese characters and vice versa.

//! ### About
//! Turn Traditional Chinese script to Simplified Chinese script and vice-versa. Check string script to determine if string is Traditional or Simplified Chinese.
//!
//! ### Usage
//! ```rust
//! extern crate character_converter;
//!
//! use character_converter::CharacterConverter;
//!
//! let converter: CharacterConverter = CharacterConverter::new();
//!
//! let traditional_text = "欧洲";
//! let simplified_text = "歐洲";
//!
//! // Check script
//! let result_one: bool = converter.is_traditional(traditional_text);
//! println!("{}", result_one); // --> true
//!
//! let result_two: bool = converter.is_simplified(traditional_text);
//! println!("{}", result_two); // --> false
//!
//! // Convert script
//! let result_three: String = converter.traditional_to_simplified(traditional_text);
//! println!("{}", result_three == simplified_text); // --> true
//!
//! let result_four: String = converter.simplified_to_traditional(simplified_text);
//! println!("{}", result_four == traditional_text); // --> true

extern crate bincode;

mod character_converter;
pub use self::character_converter::Converter as CharacterConverter;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_traditional() {
		let converter: CharacterConverter = CharacterConverter::new();
		let simplified = "欧洲";
		let traditional = "歐洲";
		assert_eq!(true, converter.is_traditional(traditional));
		assert_eq!(false, converter.is_traditional(simplified));
	}

	#[test]
	fn is_simplified() {
		let converter: CharacterConverter = CharacterConverter::new();
		let simplified = "欧洲";
		let traditional = "歐洲";
		assert_eq!(true, converter.is_simplified(simplified));
		assert_eq!(false, converter.is_simplified(traditional));
	}

	#[test]
	fn traditional_to_simplified() {
		let converter: CharacterConverter = CharacterConverter::new();
		let simplified = "欧洲";
		let traditional = "歐洲";
		assert_eq!(simplified, converter.traditional_to_simplified(traditional));
	}

	#[test]
	fn simplified_to_traditional() {
		let converter: CharacterConverter = CharacterConverter::new();
		let simplified = "欧洲";
		let traditional = "歐洲";
		assert_eq!(traditional, converter.simplified_to_traditional(simplified));
	}
}
