# character_converter
##### v0.1.1
---

Just to trigger a change

### About
Turn Traditional Chinese script to Simplified Chinese script and vice-versa. Check string script to determine if string is Traditional or Simplified Chinese Characters.

### Usage
```rust
extern crate character_converter;

use character_converter::CharacterConverter;

let converter: CharacterConverter = CharacterConverter::new();

let traditional_text = String::from("復雜");
let simplified_text = String::from("复杂");

// Check Script
let result_one: bool = converter.is_traditional(traditional_text);
println!("{}", result_one); // --> true

let result_two: bool = converter.is_simplified(traditional_text);
println!("{}", result_two); // --> false

// Convert Script
let result_three: String = converter.traditional_to_simplified(traditional_text);
println!("{}", result_three == simplified_text); // --> true

let result_four: String = converter.simplified_to_traditional(simplified_text);
println!("{}", result_four == traditional_text); // --> true
```

### Contributors
- [Preston Wang-Stosur-Bassett](http://stosur.info)

### License
[MIT](https://github.com/sotch-pr35mac/character_converter/blob/master/LICENSE)
