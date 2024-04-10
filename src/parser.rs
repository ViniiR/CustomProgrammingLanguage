use crate::{
    error,
    types::{Expression, Start, Statement, Token, TokenTypes, VarDeclarationKind, VariableTypes},
};
use std::{iter::Peekable, process::exit};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    abstract_syntax_tree: Statement,
}

#[derive(Debug)]
enum VarDecMutateOptions {
    Name,
    Value(Expression),
}

// operator precedence
// 12 ()
//
// 11 [] Arr[0]
// 11 func()
//
// 10 !Boo
//
// 9 +
// 9 -
//
// 8 Bin * Bin
// 8 Bin / Bin
// 8 Bin % Bin
//
// 7 Bin + Bin
// 7 Bin - Bin
// 7 Str + Str
//
// 6 Boo < Boo
// 6 Boo <= Boo
// 6 Boo > Boo
// 6 Boo >= Boo
//
// 5 Boo == Boo
// 5 Boo != Boo
//
// 4 Boo & Boo
//
// 3 Boo | Boo
//
// 2 = assignment
// 2 += assignment
// 2 -= assignment
// 2 *= assignment
// 2 /= assignment
// 2 %= assignment
//
// 1 , separator (10,10+10)

impl Parser {
    pub fn new(token_vector: Vec<Token>) -> Self {
        let mut iterator = token_vector.into_iter().peekable();
        Self {
            current_token: iterator.next().unwrap(),
            tokens: iterator,
            abstract_syntax_tree: Statement::Program {
                start: Start { line: 1, column: 0 },
                body: Box::new(Vec::new()),
            },
        }
    }

    fn mutate_var_declaration(&mut self, var_dec: &mut Statement, option: VarDecMutateOptions) {
        match var_dec {
            Statement::VariableDeclaration { name, value, .. } => match option {
                VarDecMutateOptions::Name => *name = Some(self.current().token_value.to_owned()),
                VarDecMutateOptions::Value(val) => *value = Some(val),
            },
            _ => exit(1),
        }
    }

    // error fixme
    fn expect_expr_or_error(&mut self) {
        match self.peek_type() {
            TokenTypes::Identifier
            | TokenTypes::NumberLiteral
            | TokenTypes::StringLiteral
            | TokenTypes::BinaryPlus
            | TokenTypes::BinaryMinus
            | TokenTypes::LogicalNot
            | TokenTypes::True
            | TokenTypes::False
            | TokenTypes::LeftParenthesis => {}
            TokenTypes::EOF => {
                let current = self.current().to_owned();
                if self.is_binary_operator(&current.token_type) {
                    let peek = self.peek().unwrap().to_owned();
                    self.expected_error("Expression", &peek);
                }
                let peek = self.peek().unwrap().to_owned();
                self.expected_error(";", &peek);
            }
            _ => {
                let peek = self.peek().unwrap().to_owned();
                self.expected_error("Expression", &peek);
            }
        }
    }

    fn expected_or_error(&mut self, expected: &TokenTypes, expected_name: &str) {
        if !self.peek_expect(expected) {
            let def = self.current().to_owned();
            let peek = self.peek().unwrap_or_else(|| &def).to_owned();
            self.expected_error(expected_name, &peek);
            exit(1)
        }
    }

    fn mutate_or_error(
        &mut self,
        expected: &str,
        expected_type: &TokenTypes,
        var_dec: &mut Statement,
        option: VarDecMutateOptions,
    ) {
        if self.peek_expect(expected_type) {
            self.advance();
            self.mutate_var_declaration(var_dec, option);
        } else {
            self.advance();
            self.expected_error(expected, &self.current());
        }
    }

    fn parse_var_declaration(&mut self) -> Statement {
        let mut var_dec = Statement::VariableDeclaration {
            start: Start {
                line: self.current().line_number,
                column: self.current().column_number,
            },
            name: None,
            kind: match self.current().token_type {
                TokenTypes::ConstantVariable => VarDeclarationKind::Immutable,
                TokenTypes::MutableVariable => VarDeclarationKind::Mutable,
                _ => exit(1),
            },
            r#type: None,
            value: None,
        };

        self.mutate_or_error(
            "Identifier",
            &TokenTypes::Identifier,
            &mut var_dec,
            VarDecMutateOptions::Name,
        );

        self.expected_or_error(&TokenTypes::Colon, ":");
        self.advance();

        self.determine_var_type(&mut var_dec);
        self.advance();

        match &var_dec {
            Statement::VariableDeclaration { kind, .. } => match kind {
                VarDeclarationKind::Mutable => match self.peek_type() {
                    TokenTypes::Semicolon => {
                        return var_dec;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        self.expected_or_error(&TokenTypes::Assign, "=");
        self.advance();

        self.expect_expr_or_error();
        self.advance();

        let expr = self.parse_compound_expr();
        self.mutate_var_declaration(&mut var_dec, VarDecMutateOptions::Value(expr));

        self.expected_or_error(&TokenTypes::Semicolon, ";");

        dbg!(&var_dec);

        var_dec
    }

    fn parse_parentheses(&mut self) -> Expression {
        // TODO: fix
        while !self.current_type().eq(&TokenTypes::RightParenthesis) {
            self.advance();
            if self.current_type().eq(&TokenTypes::Semicolon)
                || self.current_type().eq(&TokenTypes::EOF)
            {
                self.expected_error("Expression", self.current());
            }
            // parse primary | compound?
            return self.parse_compound_expr();
        }

        self.expected_error(")", self.current());
        exit(1)
    }

    fn parse_boolean_expr(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_string_expr(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_func_call(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_square_brackets(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_binary_expr(&mut self) -> Expression {
        // allow accept , inside func() or arr[];
        // allow_comma: bool;
        let mut left: Expression = self.parse_primary_expr();

        loop {
            let updated_left = left;
            self.advance();

            let operator = self.current().to_owned();
            if !self.is_binary_operator(&operator.token_type) {
                self.expected_error("Operator or ;", &self.current());
                exit(1)
            }
            self.advance();

            match self.peek() {
                None => {
                    self.expected_error("Numeric literal", &self.current());
                    exit(1)
                }
                _ => {}
            }
            let right = self.parse_primary_expr();

            left = Expression::Binary {
                left: Box::new(updated_left),
                operator: operator.token_type,
                right: Box::new(right),
            };

            if self.peek_expect(&TokenTypes::Semicolon) {
                break;
            }
        }

        left
    }

    fn parse_compound_expr(&mut self) -> Expression {
        let token = self.current().to_owned();
        let peek = self.peek().unwrap().to_owned();

        if self.is_binary_operator(&peek.token_type) {
            self.parse_binary_expr()
        } else if token.token_type.eq(&TokenTypes::LeftParenthesis) {
            // self.advance();
            self.parse_parentheses()
        } else {
            self.parse_primary_expr()
        }
    }

    fn parse_primary_expr(&mut self) -> Expression {
        let token = self.current().to_owned();

        match token.token_type {
            TokenTypes::Identifier => self.parse_identifier(),
            TokenTypes::NumberLiteral => Expression::Literal(token.token_value),
            TokenTypes::StringLiteral => self.parse_string_expr(),
            TokenTypes::True | TokenTypes::False => Expression::Literal(token.token_value),
            TokenTypes::LeftParenthesis => self.parse_parentheses(),
            TokenTypes::LeftSquareBracket => self.parse_square_brackets(),
            TokenTypes::BinaryPlus => {
                self.advance();
                Expression::Literal(token.token_value)
            }
            TokenTypes::BinaryMinus => Expression::Unary {
                operator: token.token_type,
                operand: Box::new(Expression::Literal(token.token_value)),
            },
            TokenTypes::LogicalNot => self.parse_boolean_expr(),
            TokenTypes::EOF => {
                self.expected_error(";", &token);
                exit(1)
            }
            _ => {
                self.expected_error("Expression", &token);
                exit(1)
            }
        }
    }

    fn determine_var_type(&mut self, var_dec: &mut Statement) {
        match var_dec {
            Statement::VariableDeclaration { r#type, .. } => match self.peek_type() {
                TokenTypes::Int => {
                    *r#type = Some(VariableTypes::Int);
                }
                TokenTypes::Str => {
                    *r#type = Some(VariableTypes::Str);
                }
                TokenTypes::Boo => {
                    *r#type = Some(VariableTypes::Boo);
                }
                TokenTypes::Nul => {
                    *r#type = Some(VariableTypes::Nul);
                }
                TokenTypes::Flo => {
                    *r#type = Some(VariableTypes::Flo);
                }
                TokenTypes::Arr => {
                    // *r#type = Some(TokenTypes::Int);
                    // handle generics
                }
                _ => {
                    let def = &self.current().to_owned();
                    let peek = &self.peek().unwrap_or_else(|| &def).to_owned();
                    self.expected_error("Type", peek);
                }
            },
            _ => {
                eprintln!("Unknown error at variable declaration");
                exit(1)
            }
        }
    }

    fn parse_identifier(&mut self) -> Expression {
        match self.current_type() {
            TokenTypes::Identifier => match self.peek_type() {
                TokenTypes::LeftParenthesis => self.parse_func_call(),
                _ => Expression::Identifier(self.current().token_value.to_owned()),
            },
            _ => {
                self.expected_error("Identifier", self.current());
                exit(1);
            }
        }
    }

    fn is_binary_operator(&self, token_type: &TokenTypes) -> bool {
        match token_type {
            TokenTypes::BinaryPlus
            | TokenTypes::BinaryMinus
            | TokenTypes::BinaryDivision
            | TokenTypes::BinaryMultiply
            | TokenTypes::BinaryRest => true,
            _ => false,
        }
    }

    pub fn parse(&mut self) {
        while !self.current_type().eq(&TokenTypes::EOF) {
            let ast_node = match &self.current_type() {
                TokenTypes::ConstantVariable | TokenTypes::MutableVariable => {
                    self.parse_var_declaration()
                }
                //
                TokenTypes::Semicolon => {
                    self.advance();
                    continue;
                }
                _ => {
                    self.unknown_error(&self.current_token);
                    exit(1)
                }
            };
            self.push_statement(ast_node);
            self.advance();
        }
    }

    fn current_type(&mut self) -> &TokenTypes {
        &self.current().token_type
    }

    fn current(&self) -> &Token {
        &self.current_token
    }

    fn advance(&mut self) {
        match self.current().token_type {
            TokenTypes::EOF => {}
            _ => self.current_token = self.tokens.next().unwrap(),
        }
    }

    fn peek_type(&mut self) -> &TokenTypes {
        match self.tokens.peek() {
            Some(t) => &t.token_type,
            None => &TokenTypes::EOF,
        }
    }

    fn peek_expect(&mut self, expected: &TokenTypes) -> bool {
        self.peek_type().eq(expected)
    }

    fn expected_error(&self, expected: &str, found: &Token) {
        error(
            found.line_number,
            found.column_number,
            format!("Expected '{}', found '{}'", expected, found.token_type),
        );
        exit(1)
    }

    fn unknown_error(&self, token: &Token) {
        error(
            token.line_number,
            token.column_number,
            format!("Unknown token '{}'", token.token_value),
        );
        exit(1);
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn push_statement(&mut self, stmt: Statement) {
        match &mut self.abstract_syntax_tree {
            Statement::Program { body, .. } => {
                body.push(stmt);
            }
            _ => {}
        }
    }
}
