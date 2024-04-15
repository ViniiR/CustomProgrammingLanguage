use crate::{
    error, report,
    types::{
        Expression, LiteralTypes, Start, Statement, Token, TokenTypes, VarDeclarationKind,
        VariableTypes,
    },
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
// 9 + Num
// 9 - Num
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
    const HAS_MASTER_FUNC: bool = false;

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

        let expr = self.parse_expr();
        self.mutate_var_declaration(&mut var_dec, VarDecMutateOptions::Value(expr));

        self.expected_or_error(&TokenTypes::Semicolon, ";");

        dbg!(&var_dec);

        var_dec
    }

    fn parse_expr(&mut self) -> Expression {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Expression {
        let mut left = self.parse_and_expr();

        while self.peek_type().eq(&TokenTypes::LogicalOr) {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_and_expr();

            left = Expression::Logical {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_and_expr(&mut self) -> Expression {
        let mut left = self.parse_comparison_expr();

        while self.peek_type().eq(&TokenTypes::LogicalAnd) {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_comparison_expr();

            left = Expression::Logical {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_comparison_expr(&mut self) -> Expression {
        let mut left = self.parse_greater_smaller_expr();

        while self.peek_type().eq(&TokenTypes::LogicalEquals)
            || self.peek_type().eq(&TokenTypes::LogicalDifferent)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_greater_smaller_expr();

            left = Expression::Logical {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_greater_smaller_expr(&mut self) -> Expression {
        let mut left = self.parse_additive_expr();

        while self.peek_type().eq(&TokenTypes::LogicalSmallerThan)
            || self.peek_type().eq(&TokenTypes::LogicalSmallerOrEqualsThan)
            || self.peek_type().eq(&TokenTypes::LogicalGreaterThan)
            || self.peek_type().eq(&TokenTypes::LogicalGreaterOrEqualsThan)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_additive_expr();

            left = Expression::Logical {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_additive_expr(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expr();

        while self.peek_type().eq(&TokenTypes::BinaryMinus)
            || self.peek_type().eq(&TokenTypes::BinaryPlus)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_multiplicative_expr();

            left = Expression::Binary {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> Expression {
        let mut left = self.parse_primary_expr();

        while self.peek_type().eq(&TokenTypes::BinaryMultiply)
            || self.peek_type().eq(&TokenTypes::BinaryDivision)
            || self.peek_type().eq(&TokenTypes::BinaryRest)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            // TODO: change this
            dbg!(self.current());
            let right = self.parse_primary_expr();

            left = Expression::Binary {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_boolean_expr(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_func_call(&mut self) -> Expression {
        unimplemented!()
    }

    fn parse_square_brackets(&mut self) -> Expression {
        unimplemented!()
    }

    // fn oldparse_binary_expr(&mut self) -> Expression {
    //     // allow accept , inside func() or arr[];
    //     // allow_comma: bool;
    //     let mut left: Expression = self.parse_primary_expr();
    //
    //     let is_parenthesis = self.current_type().eq(&TokenTypes::LeftParenthesis);
    //
    //     loop {
    //         let updated_left = left;
    //         self.advance();
    //
    //         let operator = self.current().to_owned();
    //         if !self.is_binary_operator(&operator.token_type)
    //             && (!is_parenthesis && !self.current_type().eq(&TokenTypes::RightParenthesis))
    //         {
    //             self.expected_error("Operator or ;", &self.current());
    //             exit(1)
    //         }
    //         self.advance();
    //
    //         match self.peek() {
    //             None => {
    //                 self.expected_error("Numeric literal", &self.current());
    //                 exit(1)
    //             }
    //             _ => {}
    //         }
    //         dbg!(self.current());
    //         let mut right = self.parse_primary_expr();
    //
    //         let peek_type = self.peek_type().to_owned();
    //         if self.get_prec(&peek_type) > self.get_prec(&operator.token_type)
    //             || (is_parenthesis && self.get_prec(&peek_type) > 11)
    //         {
    //             dbg!(self.current());
    //             right = self.parse_binary_expr()
    //         };
    //
    //         if operator.token_type.eq(&TokenTypes::LogicalOr)
    //             || operator.token_type.eq(&TokenTypes::LogicalAnd)
    //         {
    //             left = Expression::Logical {
    //                 left: Box::new(updated_left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         } else {
    //             left = Expression::Binary {
    //                 left: Box::new(updated_left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         }
    //
    //         if is_parenthesis && self.peek_expect(&TokenTypes::RightParenthesis) {
    //             self.advance();
    //             if self.peek_expect(&TokenTypes::Semicolon) {
    //                 break;
    //             }
    //         } else if self.peek_expect(&TokenTypes::Semicolon) {
    //             break;
    //         }
    //     }
    //     dbg!(self.current_type());
    //
    //     left
    // }

    // fn parse_compound_expr(&mut self) -> Expression {
    //     let token = self.current().to_owned();
    //     let peek = self.peek().unwrap().to_owned();
    //
    //     if self.is_binary_operator(&peek.token_type) {
    //         let expr = self.parse_primary_expr();
    //         self.parse_binary_expr(self.get_prec(&peek.token_type), expr)
    //     } else {
    //         self.parse_primary_expr()
    //     }
    // }
    //
    // fn parse_binary_expr(&mut self, prec: u8, left_expr: Expression) -> Expression {
    //     let is_paren = prec == 12;
    //
    //     let mut expr = left_expr;
    //
    //     loop {
    //         let left = expr;
    //         self.advance();
    //
    //         let operator = self.current().to_owned();
    //
    //         if !self.is_binary_operator(&operator.token_type) {
    //             self.expected_error("Binary Operator", &operator);
    //             exit(1)
    //         }
    //
    //         let right: Expression;
    //
    //         let peek = self.peek_type().to_owned();
    //         if self.get_prec(&peek) > prec {
    //             let prec = self.get_prec(&peek);
    //             right = self.parse_binary_expr(prec, left.to_owned());
    //         } else {
    //             right = self.parse_primary_expr();
    //         }
    //
    //         if operator.token_type.eq(&TokenTypes::LogicalOr)
    //             || operator.token_type.eq(&TokenTypes::LogicalAnd)
    //         {
    //             expr = Expression::Logical {
    //                 left: Box::new(left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         } else {
    //             expr = Expression::Binary {
    //                 left: Box::new(left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         }
    //         break;
    //     }
    //
    //     expr
    // }

    // fn oldoldparse_binary_expr(&mut self, prec: u8, left_expr: Option<Expression>) -> Expression {
    //     let is_paren = prec == 12;
    //     let is_array = self.current_type().eq(&TokenTypes::LeftSquareBracket);
    //
    //     // if is_paren | is_array {}
    //
    //     let mut left = if let Some(l) = left_expr.to_owned() {
    //         l
    //     } else {
    //         self.parse_primary_expr()
    //     };
    //
    //     loop {
    //         let new_left = left;
    //         match left_expr {
    //             Some(_) => {}
    //             None => {
    //                 self.advance();
    //             }
    //         }
    //
    //         if self.current_type().eq(&TokenTypes::Semicolon) {
    //             left = new_left;
    //             break;
    //         }
    //         dbg!(self.current());
    //         let mut operator = self.current().to_owned();
    //         self.advance();
    //
    //         if !self.is_binary_operator(&operator.token_type) {
    //             if operator.token_type.eq(&TokenTypes::RightParenthesis) {
    //                 self.advance();
    //                 operator = self.current().to_owned();
    //             } else if operator.token_type.eq(&TokenTypes::Semicolon) {
    //                 dbg!(&new_left);
    //             } else {
    //                 self.expected_error("Operator", &operator);
    //             }
    //         }
    //
    //         let mut right = self.parse_primary_expr();
    //         let peek_token = self.peek().unwrap().to_owned();
    //         if self.get_prec(&peek_token.token_type) > prec {
    //             right = self.parse_binary_expr(self.get_prec(&peek_token.token_type), None);
    //         }
    //
    //         if operator.token_type.eq(&TokenTypes::LogicalOr)
    //             || operator.token_type.eq(&TokenTypes::LogicalAnd)
    //         {
    //             left = Expression::Logical {
    //                 left: Box::new(new_left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         } else {
    //             left = Expression::Binary {
    //                 left: Box::new(new_left),
    //                 operator: operator.token_type,
    //                 right: Box::new(right),
    //             };
    //         }
    //
    //         dbg!(self.current());
    //         if is_paren && self.peek_expect(&TokenTypes::RightSquareBracket)
    //             || (is_array && self.peek_expect(&TokenTypes::RightParenthesis))
    //         {
    //             self.advance();
    //         }
    //
    //         // remove
    //         if !is_array && !is_paren && self.peek_expect(&TokenTypes::Semicolon) {
    //             break;
    //         }
    //     }
    //
    //     dbg!(self.current());
    //     left
    // }

    fn get_prec(&self, operator: &TokenTypes) -> u8 {
        match operator {
            TokenTypes::Assign
            | TokenTypes::AssignPlus
            | TokenTypes::AssignMinus
            | TokenTypes::AssignMultiply
            | TokenTypes::AssignDivision
            | TokenTypes::AssignRest => 2,
            TokenTypes::LogicalOr => 3,
            TokenTypes::LogicalAnd => 4,
            TokenTypes::LogicalEquals | TokenTypes::LogicalDifferent => 5,
            TokenTypes::LogicalSmallerOrEqualsThan
            | TokenTypes::LogicalSmallerThan
            | TokenTypes::LogicalGreaterThan
            | TokenTypes::LogicalGreaterOrEqualsThan => 6,
            TokenTypes::BinaryPlus | TokenTypes::BinaryMinus => 7,
            TokenTypes::BinaryMultiply | TokenTypes::BinaryDivision | TokenTypes::BinaryRest => 8,
            TokenTypes::LogicalNot => 10,
            TokenTypes::LeftParenthesis => 12,
            _ => 0,
        }
    }

    fn parse_parentheses(&mut self) -> Expression {
        self.advance();
        let expr = self.parse_expr();
        self.advance();
        let current = self.current().to_owned();
        if !self.current_type().eq(&TokenTypes::RightParenthesis) {
            self.expected_error(")", &current);
        }
        expr
        // self.advance();
        // let peek = self.peek_type().to_owned();
        // let mut expr: Expression;
        // if self.is_binary_operator(&peek) {
        //     dbg!(self.current());
        //     unimplemented!();
        //     // expr = self.parse_binary_expr(12);
        // } else {
        //     expr = self.parse_primary_expr();
        // }
        // self.expected_or_error(&TokenTypes::RightParenthesis, ")");
        // self.advance();
        // let peek_type = self.peek_type().to_owned();
        // if !peek_type.eq(&TokenTypes::Semicolon) && self.is_binary_operator(&peek_type) {
        //     self.advance();
        //     let current = self.current_type().to_owned();
        //     let left_expr = self.parse_primary_expr();
        //     expr = self.parse_binary_expr(self.get_prec(&current), left_expr)
        // }
        //
        // expr
    }

    // i ain't got no fucking idea of how to do this shit, goodluck
    fn parse_primary_expr(&mut self) -> Expression {
        let token = self.current().to_owned();

        match token.token_type {
            TokenTypes::Identifier => self.parse_identifier(),
            TokenTypes::NumberLiteral => Expression::Literal {
                r#type: LiteralTypes::Numeric,
                value: self.current().token_value.to_owned(),
            },
            TokenTypes::StringLiteral => Expression::Literal {
                r#type: LiteralTypes::String,
                value: self.current().token_value.to_owned(),
            },
            TokenTypes::True | TokenTypes::False => Expression::Literal {
                r#type: LiteralTypes::Boolean,
                value: self.current().token_value.to_owned(),
            },
            TokenTypes::LeftParenthesis => self.parse_parentheses(),
            TokenTypes::LeftSquareBracket => self.parse_square_brackets(),
            TokenTypes::BinaryPlus => {
                self.advance();
                self.parse_expr()
            }
            TokenTypes::BinaryMinus => {
                self.advance();

                Expression::Unary {
                    operator: token.token_type,
                    operand: Box::new(self.parse_expr()),
                }
            }
            TokenTypes::Null => Expression::Literal {
                r#type: LiteralTypes::Null,
                value: self.current().token_value.to_owned(),
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
            | TokenTypes::BinaryRest
            | TokenTypes::LogicalOr
            | TokenTypes::LogicalAnd
            | TokenTypes::LogicalEquals
            | TokenTypes::LogicalDifferent => true,
            _ => false,
        }
    }

    fn parse_call_statement(&mut self) -> Statement {
        unimplemented!()
    }

    fn parse_var_mutation(&mut self) -> Statement {
        unimplemented!()
    }

    pub fn parse(&mut self) {
        while !self.current_type().eq(&TokenTypes::EOF) {
            let ast_node = match &self.current_type() {
                TokenTypes::ConstantVariable | TokenTypes::MutableVariable => {
                    self.parse_var_declaration()
                }
                TokenTypes::Identifier => {
                    let peek = self.peek().unwrap().to_owned();
                    if self.peek_expect(&TokenTypes::LeftParenthesis) {
                        self.parse_call_statement()
                    } else if self.is_assign_operator(&peek.token_type) {
                        self.parse_var_mutation()
                    } else {
                        self.unexpected_token_error(&peek);
                        exit(1)
                    }
                }
                TokenTypes::Semicolon | TokenTypes::Comment => {
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

    fn is_assign_operator(&self, operator: &TokenTypes) -> bool {
        match operator {
            TokenTypes::Assign
            | TokenTypes::AssignPlus
            | TokenTypes::AssignMinus
            | TokenTypes::AssignMultiply
            | TokenTypes::AssignDivision
            | TokenTypes::AssignRest => true,
            _ => false,
        }
    }

    fn unexpected_token_error(&self, token: &Token) {
        report(
            token.line_number,
            token.column_number,
            String::from("Unexpected token"),
        );
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
        exit(1);
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
