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
    // Nul, // Null

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
    Continue,         // Cnt
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
                write!(f, "-=")
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
                write!(f, "i32")
            }
            TokenTypes::Flo => {
                write!(f, "f64")
            }
            TokenTypes::Str => {
                write!(f, "str")
            }
            TokenTypes::Arr => {
                write!(f, "vec")
            }
            TokenTypes::Boo => {
                write!(f, "bool")
            }
            // TokenTypes::Nul => {
            //     write!(f, "Keyword Nul")
            // }
            TokenTypes::Comment => {
                write!(f, "Comment")
            }
            TokenTypes::Function => {
                write!(f, "func")
            }
            TokenTypes::ConstantVariable => {
                write!(f, "let")
            }
            TokenTypes::MutableVariable => {
                write!(f, "mut")
            }
            TokenTypes::If => {
                write!(f, "if")
            }
            TokenTypes::ElseIf => {
                write!(f, "elseif")
            }
            TokenTypes::Else => {
                write!(f, "else")
            }
            TokenTypes::While => {
                write!(f, "while")
            }
            TokenTypes::For => {
                write!(f, "for")
            }
            TokenTypes::Return => {
                write!(f, "ret")
            }
            TokenTypes::Break => {
                write!(f, "brk")
            }
            TokenTypes::Continue => {
                write!(f, "cnt")
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
            // TokenTypes::ArrayLiteral => {
            //     write!(f, "ArrayLiteral")
            // }
            TokenTypes::True => {
                write!(f, "true")
            }
            TokenTypes::False => {
                write!(f, "false")
            }
            TokenTypes::Null => {
                write!(f, "null")
            }
            TokenTypes::UNKNOWN => {
                write!(f, "Unknown Token")
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

#[derive(Debug, Clone)]
pub enum VarDeclarationKind {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub struct Start {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Binary {
        operator: TokenTypes,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Logical {
        operator: TokenTypes,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: TokenTypes,
        operand: Box<Expression>,
    },
    Literal {
        r#type: LiteralTypes,
        value: String,
    },
    ArrayLiteral {
        elements: Option<Box<Vec<Expression>>>,
    },
    ArrayAccess(ArrayAccess),
    Call {
        name: String,
        arguments: Option<Box<Vec<Expression>>>,
    },
}

#[derive(Debug, Clone)]
pub enum ArrayAccess {
    Access {
        name: String,
        index: Box<Expression>,
    },
    NestedAccess {
        access: Box<ArrayAccess>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralTypes {
    Numeric,
    String,
    // Array,
    Boolean,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableTypes {
    Int,
    Flo,
    Str,
    Nul,
    Boo,
    Arr(Box<VariableTypes>),
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub r#type: VariableTypes,
}

#[derive(Debug, Clone)]
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
    FunctionDeclaration {
        start: Start,
        name: String,
        r#type: VariableTypes,
        params: Option<Vec<FuncParam>>,
        body: Option<Box<Vec<Statement>>>,
    },
    If {
        start: Start,
        condition: Expression,
        block: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Statement>>,
    },
    ElseIf {
        start: Start,
        condition: Expression,
        block: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Statement>>,
    },
    Else {
        start: Start,
        block: Option<Box<Vec<Statement>>>,
    },
    While {
        start: Start,
        test: Expression,
        block: Option<Box<Vec<Statement>>>,
    },
    For {
        start: Start,
        variable: Option<Box<Statement>>,
        test: Option<Expression>,
        variable_update: Option<Box<Statement>>,
        block: Option<Box<Vec<Statement>>>,
    },
    Break {
        start: Start,
    },
    Continue {
        start: Start,
    },
    Return {
        start: Start,
        expression: Option<Expression>,
    },
    VariableAlteration {
        name: String,
        operator: TokenTypes,
        value: Expression,
    },
    FunctionCall(Expression),
}
