use crate::{
    error,
    types::{Expression, Start, Statement, Token, TokenTypes, VarDeclarationKind},
};
use std::{iter::Peekable, process::exit, str::EncodeUtf16};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    abstract_syntax_tree: Statement,
}

#[derive(Debug)]
enum VarDecMutateOptions {
    Name,
    Value,
}

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
            Statement::VariableDeclaration {
                name,
                // r#type,
                // value,
                ..
            } => match option {
                VarDecMutateOptions::Name => *name = Some(self.current().token_value.to_owned()),
                VarDecMutateOptions::Value => {}
            },
            _ => exit(1),
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
                        dbg!(&var_dec);
                        return var_dec;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        // peek expects so current is Type
        self.expected_or_error(&TokenTypes::Assign, "=");
        // advances to =
        self.advance();
        // advances to Expression
        self.advance();

        // TODO:
        // expects current to be Expression, errors if not
        self.parse_expr();
        //

        self.expected_or_error(&TokenTypes::Semicolon, ";");

        dbg!(&var_dec);

        var_dec
    }

    fn expected_or_error(&mut self, expected: &TokenTypes, expected_name: &str) {
        if !self.peek_expect(expected) {
            let def = self.current().to_owned();
            let peek = self.peek().unwrap_or_else(|| &def).to_owned();
            self.expected_error(expected_name, &peek);
            exit(1)
        }
    }

    fn determine_var_type(&mut self, var_dec: &mut Statement) {
        match var_dec {
            Statement::VariableDeclaration { r#type, .. } => match self.peek_type() {
                TokenTypes::Int => {
                    *r#type = Some(TokenTypes::Int);
                }
                TokenTypes::Str => {
                    *r#type = Some(TokenTypes::Str);
                }
                TokenTypes::Boo => {
                    *r#type = Some(TokenTypes::Boo);
                }
                TokenTypes::Nul => {
                    *r#type = Some(TokenTypes::Nul);
                }
                TokenTypes::Flo => {
                    *r#type = Some(TokenTypes::Flo);
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

    fn parse_binary_expr(&mut self) -> Expression {
        let mut left: Expression = Expression::Literal(self.current().token_value.to_owned());

        //

        left
    }

    fn determine_call(&mut self) -> Expression {
        let identifier = self.current().to_owned();
        self.advance();
        let mut left_paren_count = 1;
        let mut right_paren_count = 0;

        let mut arg_list: Vec<Expression> = Vec::new();

        while left_paren_count > right_paren_count {
            self.advance();
            match self.current_type() {
                TokenTypes::LeftParenthesis => {
                    left_paren_count += 1;
                    // arg_list.push(self.current().to_owned())
                }
                TokenTypes::RightParenthesis => {
                    right_paren_count += 1;
                    // arg_list.push(self.current().to_owned())
                }
                TokenTypes::Semicolon | TokenTypes::EOF => {
                    self.advance();
                    break;
                }
                _ => arg_list.push(self.parse_expr()),
            }
        }

        dbg!(&arg_list);

        Expression::Call {
            name: identifier.token_value.to_owned(),
            arguments: Box::new(arg_list),
        }
    }

    fn parse_identifier(&mut self) -> Expression {
        dbg!(&self.current());
        match self.current_type() {
            TokenTypes::Identifier => match self.peek_type() {
                TokenTypes::LeftParenthesis => self.determine_call(),
                _ => Expression::Identifier(self.current().token_value.to_owned()),
            },
            _ => {
                self.expected_error("Identifier", self.current());
                exit(1);
            }
        }
    }

    fn parse_expr(&mut self) -> Expression {
        match self.current_type() {
            TokenTypes::Identifier => self.parse_identifier(),
            TokenTypes::NumberLiteral => Expression::Literal(self.current().token_value.to_owned()),
            TokenTypes::StringLiteral => Expression::Literal(self.current().token_value.to_owned()),
            TokenTypes::LogicalNot => match self.peek_type() {
                TokenTypes::True | TokenTypes::False => {
                    let operator = self.current();
                    Expression::Unary {
                        operator: operator.token_type.to_owned(),
                        operand: Box::new(Expression::Literal(
                            self.current().token_value.to_owned(),
                        )),
                    }
                }
                TokenTypes::Identifier => {
                    let operator = self.current().to_owned();
                    self.advance();
                    let operand = self.parse_identifier();

                    Expression::Unary {
                        operator: operator.token_type,
                        operand: Box::new(operand),
                    }
                }
                _ => {
                    self.expected_error("Boo Expression", self.current());
                    exit(1)
                }
            },
            _ => {
                self.expected_error("Expression", self.current());
                exit(1)
            }
        }
    }

    fn is_binary_operator(&mut self, token: &Token) -> bool {
        match token.token_type {
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
