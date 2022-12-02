fn main() {
    println!("Input a number.");
    println!("End your number with a specific letter to specify notation:");
    println!("b = binary; o/q = octal; d = decimal(optional); h = hexadecimal;");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().is_empty() {
            println!("Input cannot be empty. Try again.");
            continue;
        }
        let test_number = input.trim().to_owned();
        match NumberNotation::detect_notation(test_number.chars().last().unwrap()) {
            Ok(notation) => {
                if test_number.len() == 1 {
                    let number: Vec<char> = test_number.chars().collect();
                    if !number[0].is_ascii_digit() {
                        println!("Input a number and notation. Try again.");
                        continue;
                    }
                }
                println!("Notation detected: {:?}", notation);
                match notation.validate(&test_number) {
                    Ok(number_as_text) => {
                        println!("To Binary: {}", notation.to_binary(number_as_text));
                        println!("To Octal: {}", notation.to_octal(number_as_text));
                        println!("To Decimal: {}", notation.to_decimal(number_as_text));
                        println!("To Hexadecimal: {}", notation.to_hexadecimal(number_as_text));
                        let decimal = notation.to_decimal(number_as_text).parse::<u64>().unwrap();
                        println!("\nIn rust as a u64 decimal using display formatting (for comparison):");
                        println!("{:b}\n{:o}\n{}\n{:x}", decimal, decimal, decimal, decimal);
                    },
                    Err(e) => {println!("{e}"); continue;},
                }
            },
            Err(e) => {println!("{e}"); continue;},
        }
        break;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumberNotation {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumberNotation {
    // Check ending to determine number notation
    fn detect_notation(ending: char) -> Result<Self, NotationError> {
        match ending {
            'b' | 'B' => Ok(Self::Binary),
            'o' | 'O' | 'q' | 'Q' => Ok(Self::Octal),
            'd' | 'D' => Ok(Self::Decimal),
            'h' | 'H' => Ok(Self::Hexadecimal),
            _ => if ending.is_ascii_digit() {Ok(Self::Decimal)} else {Err(NotationError::InvalidNotation(ending))}
        }
    }
    // Check if number is valid for selected notation
    fn validate<'a>(&'a self, mut text: &'a str) -> Result<&str, NotationError> {
        text = if text.chars().last().unwrap().is_ascii_digit() {text}
        else {text.strip_suffix(text.chars().last().unwrap()).unwrap()};
        text = strip_leading_zeros(text);
        if text.is_empty() {return Err(NotationError::Zero)}
        for character in text.chars() {char_to_number(character, *self)?;}
        Ok(text)
    }
    // Helper function for char_to_number
    fn above_binary(self) -> bool {
        self != Self::Binary
    }
    // Helper function for char_to_number
    fn above_octal(self) -> bool {
        self != Self::Binary && self != Self::Octal
    }
    // Helper function for char_to_number
    fn above_decimal(self) -> bool {
        self == Self::Hexadecimal
    }
    // Make a binary string out of the input string
    fn to_binary(self, input: &str) -> String {
        match self {
            Self::Binary => input.to_string(),
            Self::Octal | Self::Hexadecimal => strip_leading_zeros(&self.binary_string(input)).to_string(),
            Self::Decimal => Self::decimal_from(input, Self::Binary),
        }
    }
    // Make an octal string out of the input string
    fn to_octal(self, input: &str) -> String {
        match self {
            Self::Binary => Self::binary_to(input, Self::Octal),
            Self::Octal => input.to_owned(),
            Self::Decimal => Self::decimal_from(input, Self::Octal),
            Self::Hexadecimal => Self::binary_to(strip_leading_zeros(&self.binary_string(input)), Self::Octal),
        }
    }
    // Make a decimal string out of the input string
    // Needs input to be parsed as numbers to perform mathematical operations
    fn to_decimal(self, input: &str) -> String {
        if self == Self::Decimal {return input.to_string()}
        let from = match self {
            Self::Binary => 2_u32,
            Self::Octal => 8_u32,
            Self::Decimal => unreachable!(),
            Self::Hexadecimal => 16_u32,
        };
        let mut output = 0;
        let mut length = (input.len() - 1) as u32;
        for character in input.chars() {
            if let Ok(num) = char_to_number(character, self) {
                output += from.pow(length) * num;
                length = length.saturating_sub(1);
            }
        }
        output.to_string()
    }
    // Make a hexadecimal string out of the input string
    fn to_hexadecimal(self, input: &str) -> String {
        match self {
            Self::Binary => Self::binary_to(input, Self::Hexadecimal),
            Self::Octal => Self::binary_to(strip_leading_zeros(&self.binary_string(input)), Self::Hexadecimal),
            Self::Decimal => Self::decimal_from(input, Self::Hexadecimal),
            Self::Hexadecimal => {input.to_string()},
        }
    }
    // Take a whole binary string and convert it to an octal string or a hexadecimal string
    // This starts from the leftmost digit working rightwards
    // The leftmost part can be less than a 3 or 4 digit binary string
    // The subsequent parts are always a 3 or 4 digit binary string with leading zeros
    fn binary_to(input: &str, notation: Self) -> String {
        let divisor = match notation {
            Self::Octal => 3,
            Self::Hexadecimal => 4,
            _ => unreachable!(),
        };
        let remainder = input.len() % divisor;
        let first: Option<String> = if remainder == 0 {None}
        else {Some(input.chars().take(remainder).collect())};
        let mut input = input;
        let mut num_string = String::new();
        if let Some(start) = first {
            num_string.push(binary_str_to_char(&start));
            input = input.strip_prefix(&start).unwrap();
        }
        while !input.is_empty() {
            let binary_str: String = input.chars().take(divisor).collect();
            num_string.push(binary_str_to_char(&binary_str));
            input = input.strip_prefix(&binary_str).unwrap();
        }
        num_string
    }
    // Take any string and convert it to a decimal string
    // This starts with the whole number and divides it and uses the remainder to construct the decimal string
    // The string needs to be reversed because the remainder starts rightmost working leftwards
    // Needs input to be parsed as numbers to perform mathematical operations
    fn decimal_from(input: &str, from: Self) -> String {
        let divisor = match from {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Hexadecimal => 16,
            Self::Decimal => unreachable!(),
        };
        let mut string_vec = Vec::new();
        let mut quotient = input.parse::<u64>().unwrap();
        while quotient != 0 {
            let remainder = quotient % divisor;
            quotient /= divisor;
            string_vec.push(if divisor == 16 {number_to_char(remainder).to_string()} else {remainder.to_string()});
        }
        let mut num_string = String::new();
        string_vec.reverse();
        string_vec.into_iter().for_each(|bit| num_string.push_str(&bit));
        num_string
    }
    // Make a binary string from octal or hexadecimal char digits
    fn binary_string(self, input: &str) -> String {
        let mut num_string = String::new();
        for character in input.chars() {
            num_string.push_str(char_to_binary_str(character, self));
        }
        num_string
    }
}

// Take a char and convert it to a u32
// Used to validate the input number and for decimal calculations
fn char_to_number(input: char, notation: NumberNotation) -> Result<u32, NotationError> {
    match input {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' if notation.above_binary() => Ok(2),
        '3' if notation.above_binary() => Ok(3),
        '4' if notation.above_binary() => Ok(4),
        '5' if notation.above_binary() => Ok(5),
        '6' if notation.above_binary() => Ok(6),
        '7' if notation.above_binary() => Ok(7),
        '8' if notation.above_octal() => Ok(8),
        '9' if notation.above_octal() => Ok(9),
        'a' | 'A' if notation.above_decimal() => Ok(10),
        'b' | 'B' if notation.above_decimal() => Ok(11),
        'c' | 'C' if notation.above_decimal() => Ok(12),
        'd' | 'D' if notation.above_decimal() => Ok(13),
        'e' | 'E' if notation.above_decimal() => Ok(14),
        'f' | 'F' if notation.above_decimal() => Ok(15),
        _ => Err(NotationError::InvalidDigit(input)),
    }
}

// Take a number and convert it to a hexadecimal digit char
// Used for constructing a number in the form of a string
fn number_to_char(input: u64) -> &'static str {
    match input {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "a",
        11 => "b",
        12 => "c",
        13 => "d",
        14 => "e",
        15 => "f",
        _ => unreachable!(),
    }
}

// Take an octal digit char and return a 3 digit binary string or
// Take a hexadecimal digit char and return a 4 digit binary string
// Used for constructing a binary string
fn char_to_binary_str(input: char, notation: NumberNotation) -> &'static str {
    match input {
        '0' => if notation == NumberNotation::Octal {"000"} else {"0000"},
        '1' => if notation == NumberNotation::Octal {"001"} else {"0001"},
        '2' => if notation == NumberNotation::Octal {"010"} else {"0010"},
        '3' => if notation == NumberNotation::Octal {"011"} else {"0011"},
        '4' => if notation == NumberNotation::Octal {"100"} else {"0100"},
        '5' => if notation == NumberNotation::Octal {"101"} else {"0101"},
        '6' => if notation == NumberNotation::Octal {"110"} else {"0110"},
        '7' => if notation == NumberNotation::Octal {"111"} else {"0111"},
        '8' => "1000",
        '9' => "1001",
        'a' | 'A' => "1010",
        'b' | 'B' => "1011",
        'c' | 'C' => "1100",
        'd' | 'D' => "1101",
        'e' | 'E' => "1110",
        'f' | 'F' => "1111",
        _ => unreachable!(),
    }
}

// Take a binary digit string and convert it to a char that represents a number
// Used for constructing a number in the form of a string
fn binary_str_to_char(input: &str) -> char {
    match input {
        "0000" | "000" | "00" | "0" => '0',
        "0001" | "001" | "01" | "1" => '1',
        "0010" | "010" | "10" => '2',
        "0011" | "011" | "11" => '3',
        "0100" | "100" => '4',
        "0101" | "101" => '5',
        "0110" | "110" => '6',
        "0111" | "111" => '7',
        "1000" => '8',
        "1001" => '9',
        "1010" => 'a',
        "1011" => 'b',
        "1100" => 'c',
        "1101" => 'd',
        "1110" => 'e',
        "1111" => 'f',
        _ => unreachable!(),
    }
}

// Remove any leading zeros from the string if present
fn strip_leading_zeros(input: &str) -> &str {
    let mut zero_string = String::new();
    for character in input.chars() {if character == '0' {zero_string.push('0')} else {break;}}
    if zero_string.is_empty() {input} else {input.strip_prefix(&zero_string).unwrap()}
}

enum NotationError {
    InvalidNotation(char),
    InvalidDigit(char),
    Zero,
}

impl std::fmt::Display for NotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNotation(n) => write!(f, "Invalid notation ending: {n}. Try again."),
            Self::InvalidDigit(d) => write!(f, "Invalid digit in number: {d}. Try again."),
            Self::Zero => write!(f, "0 in all notations is 0. Enter a non-zero number."),
        }
    }
}
