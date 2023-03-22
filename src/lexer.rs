use crate::util::remove_whitespace;

pub struct Token {
    pub index: i32,
    pub value: String,
}

pub enum TokenType {
    Text,
    StartTag,
    EndTag,
    EmptyTag,
    Attribute,
    Comment,
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn tokenize(source: String) -> Vec<Token> {
        let tokens: Vec<Token> = Vec::new();

        let source = remove_whitespace(source);
        let char_array: Vec<char> = source.chars().collect();

        let mut head = 0;
        loop {
            println!("{}", char_array[head]);
            head += 1;
            if head == char_array.len() {
                break;
            }
        }

        return tokens;
    }
}

#[cfg(test)]
mod util {
    use super::Lexer;

    #[test]
    fn tokenizer_test() {
        let xml_raw = include_str!("../test.xml");
        let lexer = Lexer::tokenize(xml_raw.to_string());

        assert_eq!(true, true);
    }
}
