use std::collections::HashMap;

pub struct MorseDecoder {
    pub morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    pub fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                (".-", "A"),
                ("-...", "B"),
                ("-.-.", "C"),
                ("-..", "D"),
                (".", "E"),
                ("..-.", "F"),
                ("--.", "G"),
                ("....", "H"),
                ("..", "I"),
                (".---", "J"),
                ("-.-", "K"),
                (".-..", "L"),
                ("--", "M"),
                ("-.", "N"),
                ("---", "O"),
                (".--.", "P"),
                ("--.-", "Q"),
                (".-.", "R"),
                ("...", "S"),
                ("-", "T"),
                ("..-", "U"),
                ("...-", "V"),
                (".--", "W"),
                ("-..-", "X"),
                ("-.--", "Y"),
                ("--..", "Z"),
                ("-----", "0"),
                (".----", "1"),
                ("..---", "2"),
                ("...--", "3"),
                ("....-", "4"),
                (".....", "5"),
                ("-....", "6"),
                ("--...", "7"),
                ("---..", "8"),
                ("----.", "9"),
                (".-.-.-", "."),
                ("--..--", ","),
                ("..--..", "?"),
                (".----.", "\'"),
                ("-.-.--", "!"),
                ("-..-.", "/"),
                ("-.--.", "("),
                ("-.--.-", ")"),
                (".-...", "&"),
                ("---...", ","),
                ("-.-.-.", ";"),
                ("-...-", "="),
                (".-.-.", "+"),
                ("-....-", "-"),
                ("..--.-", "_"),
                (".-..-.", "\""),
                ("...-..-", "$"),
                (".--.-.", "@"),
                ("...---...", "SOS"),
            ]
            .iter()
            .map(|&(from, to)| (from.to_string(), to.to_string()))
            .collect(),
        }
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .split(' ')
            .map(|x| {
                self.morse_code
                    .get(x)
                    .unwrap_or(&" ".to_owned())
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test00_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }

    #[test]
    fn test01_basic() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(""), "");
        assert_eq!(decoder.decode_morse(".-"), "A");
        assert_eq!(decoder.decode_morse("."), "E");
        assert_eq!(decoder.decode_morse(".."), "I");
        assert_eq!(decoder.decode_morse(". ."), "EE");
        assert_eq!(decoder.decode_morse(".   ."), "E E");
        assert_eq!(decoder.decode_morse("...---..."), "SOS");
        assert_eq!(decoder.decode_morse("... --- ..."), "SOS");
        assert_eq!(decoder.decode_morse("...   ---   ..."), "S O S");
    }

    #[test]
    fn test02_extra_space() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("   "), "");
        assert_eq!(decoder.decode_morse(" . "), "E");
        assert_eq!(decoder.decode_morse("   .   . "), "E E");
        assert_eq!(decoder.decode_morse(".   .   "), "E E");
    }

    #[test]
    fn test03_complex() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse("      ...---... -.-.--   - .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   ..-. --- -..-   .--- ..- -- .--. ...   --- ...- . .-.   - .... .   .-.. .- --.. -.--   -.. --- --. .-.-.-     "), "SOS! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.");
    }
}
