use crate::{
    error,
    types::{Token, TokenTypes as TType},
};
use std::process::exit;

#[derive(Debug)]
pub struct Lexer<'a> {
    source_code_iter: std::str::Chars<'a>,
    current_char: char,
    current_line: u32,
    current_column: u32,
    is_end_of_file: bool,
    pub token_list: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Lexer {
            source_code_iter: source_code.chars(),
            token_list: Vec::new(),
            is_end_of_file: false,
            current_char: '\0',
            current_line: 1,
            current_column: 0,
        }
    }

    pub fn scan_source_code(&mut self) {
        // initializes the current_char to Ln 1 Col 1 char of the file
        self.move_to_next_char();
        while !self.is_end_of_file {
            if self.is_whitespace() {
                self.move_to_next_char();
            } else {
                if self.is_valid_initial_identifier() {
                    let token = self.determine_alphabetic_token();
                    self.add_token_to_list(token);
                } else if self.is_valid_number_literal_initializer() {
                    let token = self.determine_number_literal();
                    self.add_token_to_list(token);
                } else if self.is_valid_string_literal() {
                    let token = self.determine_string_literal();
                    self.add_token_to_list(token);
                } else {
                    // this function moves to the char after the current one to check for
                    // 2 char long operators
                    let token = self.determine_token();
                    match token.token_type {
                        TType::Comment => {
                            self.add_token_to_list(token);
                            self.ignore_current_line();
                        }
                        _ => {
                            self.add_token_to_list(token);
                        }
                    }
                }
            }
        }
    }

    fn ignore_current_line(&mut self) {
        while !self.is_new_line() {
            self.move_to_next_char();
        }
    }

    fn is_valid_string_literal(&self) -> bool {
        self.current_char == '\'' || self.current_char == '\"'
    }

    fn add_token_to_list(&mut self, token: Token) {
        self.token_list.push(token)
    }

    fn is_valid_initial_identifier(&self) -> bool {
        self.is_alphabetic() || self.current_char == '_'
    }

    fn is_valid_following_identifier(&self) -> bool {
        self.is_alphabetic() || self.current_char == '_' || self.is_number_digit()
    }

    fn is_whitespace(&self) -> bool {
        self.current_char.is_ascii_whitespace()
    }

    fn is_new_line(&self) -> bool {
        self.current_char == '\n' || self.current_char == '\r'
    }

    fn is_number_digit(&self) -> bool {
        self.current_char.is_ascii_digit()
    }

    fn is_alphabetic(&self) -> bool {
        self.current_char.is_ascii_alphabetic()
    }

    fn is_valid_number_literal_initializer(&self) -> bool {
        self.is_number_digit() || self.current_char == '.'
    }

    fn determine_string_literal(&mut self) -> Token {
        let initial_column = self.current_column;
        let initial_line = self.current_line;
        let literal_initializer = self.current_char;
        self.move_to_next_char();
        let mut prev_char = self.current_char;
        self.move_to_next_char();
        let mut current_char = self.current_char;

        let mut string_literal: Vec<char> = vec![literal_initializer];

        loop {
            if self.is_new_line() {
                if prev_char == literal_initializer {
                    error(
                        initial_line,
                        initial_column,
                        format!("Missing ';' at the end of String literal"),
                    );
                    exit(1)
                }
                error(
                    initial_line,
                    initial_column,
                    format!(
                        "String literal '{}' must be terminated within the same line",
                        literal_initializer
                    ),
                );
                exit(1)
            }
            if self.is_end_of_file {
                error(
                    initial_line,
                    initial_column,
                    format!(
                        "String literal '{}' not terminated before the end of file",
                        literal_initializer
                    ),
                );
                exit(1)
            }

            if prev_char == '\\' {
                match get_escaped_char(current_char) {
                    Ok(character) => {
                        string_literal.push(character);
                        self.move_to_next_char();
                        prev_char = self.current_char;
                        self.move_to_next_char();
                        current_char = self.current_char;
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            } else if prev_char == literal_initializer {
                string_literal.push(prev_char);
                break;
            }
            string_literal.push(prev_char);
            prev_char = current_char;
            self.move_to_next_char();
            current_char = self.current_char;
        }

        // removes the surrounding quotes from the string
        string_literal.remove(0);
        string_literal.pop();

        Token {
            token_value: string_literal.into_iter().collect(),
            token_type: TType::StringLiteral,
            column_number: initial_column,
            line_number: initial_line,
        }
    }

    fn determine_number_literal(&mut self) -> Token {
        let initial_column = self.current_column;
        let initial_line = self.current_line;
        let mut number_literal: Vec<char> = vec![];

        let mut has_dot = false;
        let mut previous_char: Option<char> = None;

        let mut previous_column = self.current_column;
        let mut previous_line = self.current_line;

        loop {
            if self.is_end_of_file {
                break;
            }
            if has_dot && self.current_char == '.' {
                error(
                    self.current_line,
                    self.current_column,
                    String::from("Cannot have multiple '.' in a number literal"),
                );
                exit(1)
            }
            if self.current_char == '.' {
                has_dot = true;
            }
            if let Some(c) = previous_char {
                if c == '_' && self.current_char == '_' {
                    error(
                        self.current_line,
                        self.current_column,
                        String::from("Cannot have multiple adjacent '_'"),
                    );
                    exit(1)
                } else if !c.is_ascii_digit() && !self.is_number_digit() {
                    // if its a whitespace or linebreak, ignore it
                    if self.is_whitespace() {
                        self.move_to_next_char();
                        continue;
                    }
                    if c == '_' {
                        error(
                            previous_line,
                            previous_column,
                            String::from("'_' can only appear between digits"),
                        );
                    } else if c == '.' {
                        error(
                            previous_line,
                            previous_column,
                            String::from(
                                "'.' can only appear between or on the start of numeric literals",
                            ),
                        );
                    }
                    exit(1)
                }
            }
            if is_valid_number_literal(&self.current_char) {
                previous_char = Some(self.current_char);
                number_literal.push(self.current_char);
                previous_column = self.current_column;
                previous_line = self.current_line;
                self.move_to_next_char();
            } else if self.is_whitespace() {
                //TODO: remove else if
                break;
            } else {
                break;
            }
        }

        let number_literal: String = number_literal.into_iter().collect();
        Token {
            token_value: number_literal,
            token_type: TType::NumberLiteral,
            line_number: initial_line,
            column_number: initial_column,
        }
    }

    fn move_to_next_char(&mut self) {
        if let Some(next_char) = self.source_code_iter.next() {
            self.current_char = next_char;
            if self.is_new_line() {
                self.move_to_next_line();
            } else {
                self.current_column += 1;
            }
        } else {
            self.token_list.push(Token {
                token_value: String::new(),
                token_type: TType::EOF,
                line_number: self.current_line,
                column_number: self.current_column,
            });
            self.is_end_of_file = true;
        }
    }

    fn move_to_next_line(&mut self) {
        self.current_line += 1;
        self.current_column = 0;
    }

    fn determine_token(&mut self) -> Token {
        let first_char = self.current_char;
        let first_char_line = self.current_line;
        let first_char_column = self.current_column;
        self.move_to_next_char();
        let token_type = self.determine_operator(first_char, self.current_char);
        match token_type {
            TType::UNKNOWN => {
                error(
                    first_char_line,
                    first_char_column,
                    format!("Unknown Token '{}'", &first_char),
                );
                exit(1)
            }
            _ => {
                let token_value = format!("{}{}", first_char, self.current_char);
                if is_valid_multi_char(&token_value) {
                    let current_line = self.current_line;
                    self.move_to_next_char();
                    Token {
                        token_type,
                        token_value,
                        line_number: current_line,
                        column_number: first_char_column,
                    }
                } else {
                    Token {
                        token_type,
                        token_value: first_char.to_string(),
                        line_number: first_char_line,
                        column_number: first_char_column,
                    }
                }
            }
        }
    }

    fn determine_alphabetic_token(&mut self) -> Token {
        let initial_line = self.current_line;
        let initial_column = self.current_column;
        // stores the current character
        let mut alphabetic_token: Vec<char> = vec![self.current_char];
        // ignores the current character for the loop and checks only the next one
        self.move_to_next_char();
        while self.is_valid_following_identifier() {
            alphabetic_token.push(self.current_char);
            self.move_to_next_char();
        }
        let alphabetic_token: String = alphabetic_token.into_iter().collect();
        let token_type = determine_alphabetic_token_type(&alphabetic_token);
        Token {
            token_type,
            line_number: initial_line,
            column_number: initial_column,
            token_value: alphabetic_token.to_string(),
        }
    }

    fn determine_operator(&mut self, first_char: char, next_char: char) -> TType {
        match first_char {
            '=' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::LogicalEquals,
                _ => TType::Assign,
            },
            '+' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::AssignPlus,
                _ => TType::BinaryPlus,
            },
            '-' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::AssignMinus,
                _ => TType::BinaryMinus,
            },
            '/' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::AssignDivision,
                TType::BinaryDivision => TType::Comment,
                _ => TType::BinaryDivision,
            },
            '*' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::AssignMultiply,
                _ => TType::BinaryMultiply,
            },
            '%' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::AssignRest,
                _ => TType::BinaryRest,
            },
            '<' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::LogicalSmallerOrEqualsThan,
                _ => TType::LogicalSmallerThan,
            },
            '>' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::LogicalGreaterOrEqualsThan,
                _ => TType::LogicalGreaterThan,
            },
            '!' => match is_valid_long_operator(next_char) {
                TType::Assign => TType::LogicalDifferent,
                _ => TType::LogicalNot,
            },
            '&' => TType::LogicalAnd,
            '|' => TType::LogicalOr,
            '\"' => TType::DoubleQuotes,
            '\'' => TType::SingleQuotes,
            '(' => TType::LeftParenthesis,
            ')' => TType::RightParenthesis,
            '[' => TType::LeftSquareBracket,
            ']' => TType::RightSquareBracket,
            '{' => TType::LeftCurlyBrace,
            '}' => TType::RightCurlyBrace,
            '.' => TType::Dot,
            ':' => TType::Colon,
            ';' => TType::Semicolon,
            ',' => TType::Comma,
            _ => TType::UNKNOWN,
        }
    }
}

fn get_escaped_char(character: char) -> Result<char, ()> {
    match character {
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        '\\' => Ok('\\'),
        '\'' => Ok('\''),
        '\"' => Ok('\"'),
        _ => Err(()),
    }
}

fn is_valid_number_literal(digit: &char) -> bool {
    digit.is_ascii_digit() || digit == &'_' || digit == &'.'
}

fn is_valid_long_operator(character: char) -> TType {
    match character {
        '=' => TType::Assign,
        // '+' => TType::BinaryPlus,
        // '-' => TType::BinaryMinus,
        '/' => TType::BinaryDivision,
        // '*' => TType::BinaryMultiply,
        // '%' => TType::BinaryRest,
        // '&' => TType::LogicalAnd,
        // '|' => TType::LogicalOr,
        '<' => TType::LogicalSmallerThan,
        '>' => TType::LogicalGreaterThan,
        // '!' => TType::LogicalNot,
        // '\"' => TType::DoubleQuotes,
        // '\'' => TType::SingleQuotes,
        _ => TType::UNKNOWN,
    }
}

fn is_valid_multi_char(string: &str) -> bool {
    match string {
        "==" => true,
        "+=" => true,
        "-=" => true,
        "/=" => true,
        "*=" => true,
        "%=" => true,
        // "++" => true,
        // "--" => true,
        "<=" => true,
        ">=" => true,
        "!=" => true,
        "//" => true,
        _ => false,
    }
}

fn determine_alphabetic_token_type(token: &str) -> TType {
    match token {
        "true" => TType::True,
        "false" => TType::False,
        "null" => TType::Null,
        "Boo" => TType::Boo,
        "Int" => TType::Int,
        "Flo" => TType::Flo,
        "Str" => TType::Str,
        "Nul" => TType::Nul,
        "Arr" => TType::Arr,
        "let" => TType::ConstantVariable,
        "mut" => TType::MutableVariable,
        "func" => TType::Function,
        "while" => TType::While,
        "if" => TType::If,
        "elseif" => TType::ElseIf,
        "else" => TType::Else,
        "brk" => TType::Break,
        "cnt" => TType::Continue,
        "ret" => TType::Return,
        "for" => TType::For,
        _ => TType::Identifier,
        // "Obj" => TType::Object,
        // "use" => TType::Use,
        // "from" => TType::From,
        // "switch" => TType::Switch,
        // "case" => TType::Case,
        // "casenot" => TType::CaseNot,
    }
}
