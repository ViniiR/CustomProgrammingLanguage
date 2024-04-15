use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenTypes {
    // Assignment Operators
    Assign,         // =
    AssignPlus,     // +=
    AssignMinus,    // -=
    AssignMultiply, // *=
    AssignDivision, // /=
    AssignRest,     // %=

    // Binary Operators
    BinaryPlus,     // +
    BinaryMinus,    // -
    BinaryDivision, // /
    BinaryMultiply, // *
    BinaryRest,     // %
    // BinaryIncrement, // ++
    // BinaryDecrement, // --

    // Logical Operators
    LogicalNot,       // !
    LogicalAnd,       // &
    LogicalOr,        // |
    LogicalEquals,    // ==
    LogicalDifferent, // !=
    // LogicalTernary, // Condition ? True : False
    LogicalSmallerThan,         // <
    LogicalGreaterThan,         // >
    LogicalSmallerOrEqualsThan, // <=
    LogicalGreaterOrEqualsThan, // >=
    // QuestionMark, // ? // Its Purpose Is To Check If A Var Is Empty, Such As Array == [] Or Var == Null
    // E.g. If (?varname) {} Only Runs The Block If The Var Is Not Empty

    // Punctuation
    Dot,                // .
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    LeftCurlyBrace,     // {
    RightCurlyBrace,    // }
    DoubleQuotes,       // "
    SingleQuotes,       // '
    Semicolon,          // ;
    Colon,              // :
    Comma,              // ,
    // SmallFunction,      // =>

    // Types
    Int, // 123456789
    Flo, // 1.0 2.0 3.14
    Str, // "hello, World!"
    // Obj, // { Property = "value" }
    Arr, // [0,1,2]
    Boo, // True | False
    Nul, // Null

    // Special
    Comment, // //

    // Keywords
    Function,         // Func
    ConstantVariable, // Define Type
    MutableVariable,  // mut
    If,               // If
    ElseIf,           // Elseif
    Else,             // Else
    While,            // While
    For,              // For
    Return,           // Ret
    Break,            // Brk
    // Use,      // Use Modname From 'mod/path'
    // From,     // From 'mod/path'
    // Switch,   // Switch
    // Case,     // Incase
    // CaseNot,  //incasenot

    // Literals
    Identifier, // any name
    NumberLiteral,
    StringLiteral,
    True,  // True
    False, // False
    Null,  // Null

    // Unknown
    UNKNOWN, // any token that doesnt match anything

    // File
    EOF, // end of file
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTypes::Assign => {
                write!(f, "=")
            }
            TokenTypes::AssignPlus => {
                write!(f, "+=")
            }
            TokenTypes::AssignMinus => {
                write!(f, "-+")
            }
            TokenTypes::AssignMultiply => {
                write!(f, "*=")
            }
            TokenTypes::AssignDivision => {
                write!(f, "/=")
            }
            TokenTypes::AssignRest => {
                write!(f, "%=")
            }
            TokenTypes::BinaryPlus => {
                write!(f, "+")
            }
            TokenTypes::BinaryMinus => {
                write!(f, "-")
            }
            TokenTypes::BinaryDivision => {
                write!(f, "/")
            }
            TokenTypes::BinaryMultiply => {
                write!(f, "*")
            }
            TokenTypes::BinaryRest => {
                write!(f, "%")
            }
            TokenTypes::LogicalNot => {
                write!(f, "!")
            }
            TokenTypes::LogicalAnd => {
                write!(f, "&")
            }
            TokenTypes::LogicalOr => {
                write!(f, "|")
            }
            TokenTypes::LogicalEquals => {
                write!(f, "==")
            }
            TokenTypes::LogicalDifferent => {
                write!(f, "!=")
            }
            TokenTypes::LogicalSmallerThan => {
                write!(f, "<")
            }
            TokenTypes::LogicalGreaterThan => {
                write!(f, ">")
            }
            TokenTypes::LogicalSmallerOrEqualsThan => {
                write!(f, "<=")
            }
            TokenTypes::LogicalGreaterOrEqualsThan => {
                write!(f, ">=")
            }
            TokenTypes::Dot => {
                write!(f, ".")
            }
            TokenTypes::LeftParenthesis => {
                write!(f, "(")
            }
            TokenTypes::RightParenthesis => {
                write!(f, ")")
            }
            TokenTypes::LeftSquareBracket => {
                write!(f, "[")
            }
            TokenTypes::RightSquareBracket => {
                write!(f, "]")
            }
            TokenTypes::LeftCurlyBrace => {
                write!(f, "{{")
            }
            TokenTypes::RightCurlyBrace => {
                write!(f, "}}")
            }
            TokenTypes::DoubleQuotes => {
                write!(f, "\"")
            }
            TokenTypes::SingleQuotes => {
                write!(f, "\'")
            }
            TokenTypes::Semicolon => {
                write!(f, ";")
            }
            TokenTypes::Colon => {
                write!(f, ":")
            }
            TokenTypes::Comma => {
                write!(f, ",")
            }
            TokenTypes::Int => {
                write!(f, "Keyword Int")
            }
            TokenTypes::Flo => {
                write!(f, "Keyword Flo")
            }
            TokenTypes::Str => {
                write!(f, "Keyword Str")
            }
            TokenTypes::Arr => {
                write!(f, "Keyword Arr")
            }
            TokenTypes::Boo => {
                write!(f, "Keyword Boo")
            }
            TokenTypes::Nul => {
                write!(f, "Keyword Nul")
            }
            TokenTypes::Comment => {
                write!(f, "Comment")
            }
            TokenTypes::Function => {
                write!(f, "Keyword func")
            }
            TokenTypes::ConstantVariable => {
                write!(f, "Keyword let")
            }
            TokenTypes::MutableVariable => {
                write!(f, "Keyword mut")
            }
            TokenTypes::If => {
                write!(f, "Keyword if")
            }
            TokenTypes::ElseIf => {
                write!(f, "Keyword elseif")
            }
            TokenTypes::Else => {
                write!(f, "Keyword else")
            }
            TokenTypes::While => {
                write!(f, "Keyword while")
            }
            TokenTypes::For => {
                write!(f, "Keyword for")
            }
            TokenTypes::Return => {
                write!(f, "Keyword ret")
            }
            TokenTypes::Break => {
                write!(f, "Keyword brk")
            }
            TokenTypes::Identifier => {
                write!(f, "Identifier")
            }
            TokenTypes::NumberLiteral => {
                write!(f, "NumericLiteral")
            }
            TokenTypes::StringLiteral => {
                write!(f, "StringLiteral")
            }
            TokenTypes::True => {
                write!(f, "Keyword true")
            }
            TokenTypes::False => {
                write!(f, "Keyword false")
            }
            TokenTypes::Null => {
                write!(f, "Keyword null")
            }
            TokenTypes::UNKNOWN => {
                write!(f, "Unknown")
            }
            TokenTypes::EOF => {
                write!(f, "EndOfFile")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_value: String,
    pub token_type: TokenTypes,
    pub column_number: u32,
    pub line_number: u32,
}

#[derive(Debug)]
pub enum VarDeclarationKind {
    Mutable,
    Immutable,
}

#[derive(Debug)]
pub struct Start {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: TokenTypes,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: TokenTypes,
        right: Box<Expression>,
    },
    Unary {
        operator: TokenTypes,
        operand: Box<Expression>,
    },
    Literal {
        r#type: LiteralTypes,
        // wrong: this should be expression TODO:
        // i have no idea what ^ is talking about but i wont remove it
        value: String,
    },
    Call {
        name: String,
        arguments: Box<Vec<Expression>>,
    },
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum LiteralTypes {
    Numeric,
    String,
    Array,
    Boolean,
    Null,
}
#[derive(Debug)]
pub enum VariableTypes {
    Int,
    Flo,
    Str,
    Nul,
    Boo,
    Arr(Box<VariableTypes>),
}

#[derive(Debug)]
pub enum Statement {
    Program {
        start: Start,
        body: Box<Vec<Statement>>,
    },
    VariableDeclaration {
        start: Start,
        name: Option<String>,
        kind: VarDeclarationKind,
        r#type: Option<VariableTypes>,
        value: Option<Expression>,
    },
    VariableAlteration {
        start: Start,
        name: String,
        value: Expression,
    },
    If {
        start: Start,
        condition: Expression,
    },
    For {
        start: Start,
        // index should ALWAYS be a VariableDeclaration
        index: Box<Statement>,
        // test should be a logical expression
        test: Expression,
        // index_update should be a VariableAlteration
        index_update: Box<Statement>,
    },
}
