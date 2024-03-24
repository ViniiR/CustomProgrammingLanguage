#[derive(Debug)]
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
    Function, // Func
    Variable, // Define Type
    If,       // If
    ElseIf,   // Elseif
    Else,     // Else
    While,    // While
    For,      // For
    Return,   // Ret
    Break,    // Brk
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

    // Unknown
    UNKNOWN, // any token that doesnt match anything

    // File
    EOF, // end of file
}

#[derive(Debug)]
pub struct Token {
    pub token_value: String,
    pub token_type: TokenTypes,
    pub column_number: u32,
    pub line_number: u32,
}
