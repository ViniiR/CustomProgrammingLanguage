use crate::{
    error,
    frontend::types::{
        ArrayAccess, Expression, FuncParam, LiteralTypes, Start, Statement, Token, TokenTypes,
        VarDeclarationKind, VariableTypes,
    },
    report,
};
use std::{iter::Peekable, process::exit};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    pub abstract_syntax_tree: Statement,
}

#[derive(Debug)]
enum VarDecMutateOptions {
    Name,
    Value(Expression),
}

#[derive(Debug)]
enum Loop {
    Yes,
    No,
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

    pub fn parse_tokens(&mut self) {
        while !self.current_type().eq(&TokenTypes::EOF) {
            let ast_node = match &self.current_type() {
                TokenTypes::Function => self.parse_function_statement(),
                TokenTypes::Semicolon => {
                    self.unexpected_token_error(self.current());
                    exit(1)
                }
                TokenTypes::Comment => {
                    self.advance();
                    continue;
                }
                TokenTypes::ConstantVariable | TokenTypes::MutableVariable => {
                    self.custom_error_current("Variables cannot be defined outside of a function");
                    exit(1)
                }
                TokenTypes::If => {
                    self.custom_error_current(
                        "If statements cannot be defined outside of a function",
                    );
                    exit(1)
                }
                TokenTypes::ElseIf => {
                    self.custom_error_current(
                        "ElseIf statements cannot be defined outside of a function",
                    );
                    exit(1)
                }
                TokenTypes::Else => {
                    self.custom_error_current(
                        "Else statements cannot be defined outside of a function",
                    );
                    exit(1)
                }
                TokenTypes::While | TokenTypes::For => {
                    self.custom_error_current("Loops cannot be defined outside of a function");
                    exit(1)
                }
                TokenTypes::Return => {
                    self.custom_error_current(
                        "Return statements cannot be used outside of a function",
                    );
                    exit(1)
                }
                TokenTypes::Continue | TokenTypes::Break => {
                    self.custom_error_current("Loop controls cannot be used outside of a loop");
                    exit(1)
                }
                _ => {
                    if self.is_expr() {
                        self.custom_error_current("Expressions cannot be standalone statements");
                        exit(1)
                    }
                    self.custom_error_current("Only functions can be defined at the global scope");
                    exit(1)
                }
            };
            self.push_statement(ast_node);
            self.advance();
        }
    }

    /// parse { ... }
    fn parse_block(&mut self, is_loop: &Loop) -> Option<Vec<Statement>> {
        // current {
        self.advance();
        // inside block
        let mut block_stmts: Vec<Statement> = Vec::new();

        let mut stmt: Statement;

        while !self.current_type().eq(&TokenTypes::EOF) {
            if self.current_type().eq(&TokenTypes::RightCurlyBrace) {
                break;
            }

            stmt = match self.current_type() {
                TokenTypes::ConstantVariable | TokenTypes::MutableVariable => {
                    let dec = self.parse_var_declaration();
                    self.advance();
                    dec
                }
                TokenTypes::Identifier => {
                    let peek = self.peek().unwrap().to_owned();
                    if self.peek_expect(&TokenTypes::LeftParenthesis) {
                        let stmt = self.parse_call_statement();
                        self.advance();
                        stmt
                    } else if self.is_assign_operator(&peek.token_type) {
                        self.parse_var_mutation()
                    } else {
                        self.unexpected_token_error(&self.current());
                        exit(1)
                    }
                }
                TokenTypes::Function => {
                    report(
                        self.current().line_number,
                        self.current().column_number,
                        String::from("Functions cannot be defined inside functions"),
                    );
                    exit(1)
                }
                TokenTypes::Continue | TokenTypes::Break => match is_loop {
                    Loop::Yes => self.parse_loop_controls(),
                    _ => {
                        report(
                            self.current().line_number,
                            self.current().column_number,
                            String::from("Loop controls cannot be used outside of loops"),
                        );
                        exit(1)
                    }
                },
                TokenTypes::Return => self.parse_func_return(),
                TokenTypes::If => self.parse_if_stmt(is_loop),
                TokenTypes::ElseIf => {
                    report(
                        self.current().line_number,
                        self.current().column_number,
                        String::from("Standalone elseif statement"),
                    );
                    exit(1)
                }
                TokenTypes::Else => {
                    report(
                        self.current().line_number,
                        self.current().column_number,
                        String::from("Standalone else statement"),
                    );
                    exit(1)
                }
                TokenTypes::While => self.parse_while_loop(&Loop::Yes),
                TokenTypes::For => self.parse_for_loop(&Loop::Yes),
                TokenTypes::Semicolon => {
                    self.unexpected_token_error(self.current());
                    exit(1)
                }
                TokenTypes::Comment => {
                    self.advance();
                    continue;
                }
                _ => {
                    if self.is_expr() {
                        self.custom_error_current(
                            "Only function calls can be standalone statements",
                        );
                        exit(1)
                    }
                    self.unknown_error(&self.current_token);
                    exit(1)
                }
            };

            block_stmts.push(stmt);
        }
        // current }

        if !self.current_type().eq(&TokenTypes::RightCurlyBrace) {
            report(
                self.current().line_number,
                self.current().column_number,
                String::from("Unclosed block"),
            );
            exit(1)
        }

        if block_stmts.len() == 0 {
            None
        } else {
            Some(block_stmts)
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
        // self.advance();

        match &var_dec {
            Statement::VariableDeclaration { kind, .. } => match kind {
                VarDeclarationKind::Mutable => match self.current_type() {
                    TokenTypes::Semicolon => {
                        return var_dec;
                    }
                    TokenTypes::Assign => {}
                    _ => {
                        self.unexpected_token_error(self.current());
                        exit(1)
                    }
                },
                _ => {}
            },
            _ => {}
        }

        if !self.current_type().eq(&TokenTypes::Assign) {
            self.expected_error("=", self.current());
            exit(1)
        }
        // self.expected_or_error(&TokenTypes::Assign, "=");
        // self.advance();

        self.expect_expr_or_error();
        self.advance();

        let expr = self.parse_expr();
        self.mutate_var_declaration(&mut var_dec, VarDecMutateOptions::Value(expr));

        self.expected_or_error(&TokenTypes::Semicolon, ";");
        self.advance();

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
        let mut left = self.parse_unary_expr();

        while self.peek_type().eq(&TokenTypes::BinaryMultiply)
            || self.peek_type().eq(&TokenTypes::BinaryDivision)
            || self.peek_type().eq(&TokenTypes::BinaryRest)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_primary_expr();

            left = Expression::Binary {
                left: Box::new(left),
                operator: operator.token_type,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_unary_expr(&mut self) -> Expression {
        match self.current_type() {
            TokenTypes::BinaryPlus => {
                self.advance();
                self.parse_unary_expr()
            }
            TokenTypes::BinaryMinus => {
                self.advance();
                Expression::Unary {
                    operator: TokenTypes::BinaryMinus,
                    operand: Box::new(self.parse_unary_expr()),
                }
            }
            TokenTypes::LogicalNot => {
                self.advance();
                Expression::Unary {
                    operator: TokenTypes::LogicalNot,
                    operand: Box::new(self.parse_unary_expr()),
                }
            }
            _ => self.parse_primary_expr(),
        }
    }

    fn parse_func_call(&mut self) -> Expression {
        let name = self.current().to_owned();
        // (
        self.advance();
        // arg 1 | )
        self.advance();

        let mut arg_vec: Vec<Expression> = Vec::new();
        let mut expr: Expression;

        while !self.current_type().eq(&TokenTypes::RightParenthesis) {
            if self.current_type().eq(&TokenTypes::EOF)
                || self.current_type().eq(&TokenTypes::Semicolon)
            {
                break;
            }

            if self.current_type().eq(&TokenTypes::Comma) {
                if self.peek_expect(&TokenTypes::Comma) {
                    let peek = self.peek().unwrap().to_owned();
                    self.unexpected_token_error(&peek);
                    exit(1)
                }
                self.advance();
                continue;
            }

            expr = self.parse_expr();
            if self.peek_expect(&TokenTypes::Comma)
                || self.peek_expect(&TokenTypes::RightParenthesis)
            {
                arg_vec.push(expr);
                self.advance();
            } else {
                self.advance();
                self.expected_error(",", self.current());
                exit(1)
            }
        }

        if !self.current_type().eq(&TokenTypes::RightParenthesis) {
            self.expected_error(")", self.current());
        }
        if arg_vec.len() == 0 {
            return Expression::Call {
                name: name.token_value,
                arguments: None,
            };
        }

        Expression::Call {
            name: name.token_value,
            arguments: Some(Box::new(arg_vec)),
        }
    }

    fn parse_square_brackets(&mut self) -> Expression {
        self.advance();

        let mut expr_vec: Vec<Expression> = Vec::new();
        let mut expr: Expression;

        while !self.current_type().eq(&TokenTypes::RightSquareBracket) {
            if self.current_type().eq(&TokenTypes::EOF)
                || self.current_type().eq(&TokenTypes::Semicolon)
            {
                break;
            }

            if self.current_type().eq(&TokenTypes::Comma) {
                if self.peek_expect(&TokenTypes::Comma) {
                    let peek = self.peek().unwrap().to_owned();
                    self.unexpected_token_error(&peek);
                    exit(1)
                }
                self.advance();
                continue;
            }

            expr = self.parse_expr();
            if self.peek_expect(&TokenTypes::Comma)
                || self.peek_expect(&TokenTypes::RightSquareBracket)
            {
                expr_vec.push(expr);
                self.advance();
            } else {
                self.advance();
                self.expected_error(",", self.current());
                exit(1)
            }
        }

        if !self.current_type().eq(&TokenTypes::RightSquareBracket) {
            self.expected_error("]", self.current());
        }
        if expr_vec.len() == 0 {
            return Expression::ArrayLiteral { elements: None };
        }

        Expression::ArrayLiteral {
            elements: Some(Box::new(expr_vec)),
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
    }

    fn parse_array_access(&mut self) -> Expression {
        let name = self.current().token_value.to_owned();
        let mut index: Expression;

        let mut arr_access: Option<ArrayAccess> = None;

        while self.peek_expect(&TokenTypes::LeftSquareBracket) {
            self.advance();

            self.expect_expr_or_error();
            self.advance();
            index = self.parse_expr();
            self.advance();

            if !self.current_type().eq(&TokenTypes::RightSquareBracket) {
                self.expected_error("]", self.current());
                exit(1)
            }

            arr_access = match arr_access {
                Some(acc) => Some(ArrayAccess::NestedAccess {
                    access: Box::new(acc),
                    index: Box::new(index),
                }),
                None => Some(ArrayAccess::Access {
                    name: name.to_owned(),
                    index: Box::new(index),
                }),
            };
        }

        match arr_access {
            Some(acc) => Expression::ArrayAccess(acc),
            None => {
                eprintln!("Expected Array access token, found internal error");
                exit(1)
            }
        }
    }

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
            TokenTypes::Null => Expression::Literal {
                r#type: LiteralTypes::Null,
                value: self.current().token_value.to_owned(),
            },
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

    fn make_array_type(&mut self) -> VariableTypes {
        self.advance();
        match self.current_type() {
            TokenTypes::Int => VariableTypes::Int,
            TokenTypes::Str => VariableTypes::Str,
            TokenTypes::Boo => VariableTypes::Boo,
            TokenTypes::Null => VariableTypes::Nul,
            TokenTypes::Flo => VariableTypes::Flo,
            TokenTypes::Arr => {
                if !self.peek_expect(&TokenTypes::LogicalSmallerThan) {
                    report(
                        self.current().line_number,
                        self.current().column_number,
                        String::from("Arr type must be generic, Arr<Type>"),
                    );
                    exit(1)
                }

                self.advance();
                let r#type = self.make_array_type();
                self.expected_or_error(&TokenTypes::LogicalGreaterThan, ">");
                self.advance();
                VariableTypes::Arr(Box::new(r#type))
            }
            _ => {
                self.unexpected_token_error(self.current());
                exit(1)
            }
        }
    }

    fn get_generic_type(&mut self) -> VariableTypes {
        // Type
        // peek < | unknown
        match self.current().token_type {
            TokenTypes::Int => VariableTypes::Int,
            TokenTypes::Str => VariableTypes::Str,
            TokenTypes::Boo => VariableTypes::Boo,
            TokenTypes::Null => VariableTypes::Nul,
            TokenTypes::Flo => VariableTypes::Flo,
            TokenTypes::Arr => {
                if !self.peek_expect(&TokenTypes::LogicalSmallerThan) {
                    report(
                        self.current().line_number,
                        self.current().column_number,
                        String::from("Arr type must be generic, Arr<Type>"),
                    );
                    exit(1)
                }
                self.advance();
                self.advance();

                let r#type = self.get_generic_type();

                self.expected_or_error(&TokenTypes::LogicalGreaterThan, ">");
                self.advance();

                VariableTypes::Arr(Box::new(r#type))
            }
            _ => {
                self.unexpected_token_error(self.current());
                exit(1)
            }
        }
    }

    fn get_type(&mut self) -> VariableTypes {
        match self.current_type() {
            TokenTypes::Int => VariableTypes::Int,
            TokenTypes::Str => VariableTypes::Str,
            TokenTypes::Boo => VariableTypes::Boo,
            TokenTypes::Null => VariableTypes::Nul,
            TokenTypes::Flo => VariableTypes::Flo,
            TokenTypes::Arr => {
                self.expected_or_error(&TokenTypes::LogicalSmallerThan, "<");
                self.advance();
                self.advance();
                let r#type = self.get_generic_type();
                self.expected_or_error(&TokenTypes::LogicalGreaterThan, ">");
                self.advance();
                VariableTypes::Arr(Box::new(r#type))
            }
            _ => {
                dbg!(self.current());
                self.expected_error("Type", self.current());
                exit(1)
            }
        }
    }

    fn determine_var_type(&mut self, var_dec: &mut Statement) {
        match var_dec {
            Statement::VariableDeclaration { r#type, .. } => match self.peek_type() {
                TokenTypes::Int => {
                    self.advance();
                    self.advance();
                    *r#type = Some(VariableTypes::Int);
                }
                TokenTypes::Str => {
                    self.advance();
                    self.advance();
                    *r#type = Some(VariableTypes::Str);
                }
                TokenTypes::Boo => {
                    self.advance();
                    self.advance();
                    *r#type = Some(VariableTypes::Boo);
                }
                TokenTypes::Null => {
                    self.advance();
                    self.advance();
                    *r#type = Some(VariableTypes::Nul);
                }
                TokenTypes::Flo => {
                    self.advance();
                    self.advance();
                    *r#type = Some(VariableTypes::Flo);
                }
                TokenTypes::Arr => {
                    *r#type = Some(self.make_array_type());
                    self.advance();
                }
                _ => {
                    let def = &self.current().to_owned();
                    let peek = &self.peek().unwrap_or_else(|| &def).to_owned();
                    self.expected_error("Type", peek);
                    exit(1)
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
                TokenTypes::LeftSquareBracket => self.parse_array_access(),
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
        let call = self.parse_func_call();

        if !self.peek_expect(&TokenTypes::Semicolon) {
            let peek = self.peek().unwrap().to_owned();
            self.expected_error(";", &peek);
            exit(1)
        }

        self.advance();

        Statement::FunctionCall(call)
    }

    fn parse_var_mutation(&mut self) -> Statement {
        let name = self.current().token_value.to_owned();
        self.advance();

        let operator = self.current().to_owned();
        self.advance();

        let expr = self.parse_expr();

        if !self.peek_expect(&TokenTypes::Semicolon) {
            let peek = self.peek().unwrap().to_owned();
            self.expected_error(";", &peek);
            exit(1)
        }
        self.advance();
        self.advance();

        Statement::VariableAlteration {
            name,
            value: expr,
            operator: operator.token_type,
        }
    }

    fn parse_params(&mut self) -> Vec<FuncParam> {
        // (
        self.advance();
        // foo: Int,
        let mut params: Vec<FuncParam> = Vec::new();

        let mut param: FuncParam;

        if self.current_type().eq(&TokenTypes::RightParenthesis) {
            return params;
        }

        while !self.peek_expect(&TokenTypes::RightParenthesis)
            || !self.peek_expect(&TokenTypes::Semicolon)
            || !self.peek_expect(&TokenTypes::EOF)
        {
            if !self.current_type().eq(&TokenTypes::Identifier) {
                self.expected_error("Identifier", self.current());
                exit(1)
            }

            let name = self.current().to_owned().token_value;

            self.expected_or_error(&TokenTypes::Colon, ":");
            self.advance();
            self.advance();

            let r#type: VariableTypes = self.get_type();
            self.advance();
            // after closing >, likely a , or )

            param = FuncParam { name, r#type };

            if self.current_type().eq(&TokenTypes::RightParenthesis) {
                params.push(param.to_owned());
                break;
            } else if self.current_type().eq(&TokenTypes::Comma) {
                params.push(param.to_owned());
                self.advance();
            } else {
                self.expected_error(", or )", self.current());
                exit(1)
            }
        }

        if !self.current_type().eq(&TokenTypes::RightParenthesis) {
            self.expected_error(")", self.current());
            exit(1)
        }

        params
    }

    fn parse_function_statement(&mut self) -> Statement {
        let func_tk = self.current().to_owned();

        self.expected_or_error(&TokenTypes::Identifier, "Identifier");
        self.advance();

        let name = self.current().to_owned();

        self.expected_or_error(&TokenTypes::LeftParenthesis, "(");
        self.advance();
        // current (
        let param_vec: Vec<FuncParam> = self.parse_params();
        // current )

        self.expected_or_error(&TokenTypes::Colon, ":");
        // current  )
        self.advance();
        // current  :
        self.advance();

        // current == Type | unknown
        let r#type: VariableTypes = self.get_type();

        self.expected_or_error(&TokenTypes::LeftCurlyBrace, "{");
        self.advance();

        let body_block: Option<Vec<Statement>> = self.parse_block(&Loop::No);

        self.expected_or_error(&TokenTypes::Semicolon, ";");
        self.advance();

        Statement::FunctionDeclaration {
            start: Start {
                line: func_tk.line_number,
                column: func_tk.column_number,
            },
            name: name.token_value,
            params: if param_vec.len() > 0 {
                Some(param_vec)
            } else {
                None
            },
            r#type,
            body: match body_block {
                Some(b) => Some(Box::new(b)),
                _ => None,
            },
        }
    }

    fn parse_if_stmt(&mut self, is_loop: &Loop) -> Statement {
        let first = self.current().to_owned();

        self.expect_expr_or_error();
        self.advance();

        let expression = self.parse_expr();

        self.expected_or_error(&TokenTypes::LeftCurlyBrace, "{");
        self.advance();

        // curr {
        let block = self.parse_block(is_loop);
        // curr }

        match self.peek_type() {
            TokenTypes::Semicolon => {
                self.advance();
                self.advance();
                Statement::If {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: None,
                }
            }
            TokenTypes::ElseIf => {
                self.advance();
                Statement::If {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: Some(Box::new(self.parse_elseif_stmt(is_loop))),
                }
            }
            TokenTypes::Else => {
                self.advance();
                Statement::If {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: Some(Box::new(self.parse_else_stmt(is_loop))),
                }
            }
            _ => {
                let peek = self.peek().unwrap().to_owned();
                self.expected_error("; or elseif or else", &peek);
                exit(1)
            }
        }
    }

    fn parse_elseif_stmt(&mut self, is_loop: &Loop) -> Statement {
        let first = self.current().to_owned();

        self.expect_expr_or_error();
        self.advance();

        let expression = self.parse_expr();

        self.expected_or_error(&TokenTypes::LeftCurlyBrace, "{");
        self.advance();

        // curr {
        let block = self.parse_block(is_loop);
        // curr }

        match self.peek_type() {
            TokenTypes::Semicolon => {
                self.advance();
                self.advance();
                Statement::ElseIf {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: None,
                }
            }
            TokenTypes::ElseIf => {
                self.advance();
                Statement::ElseIf {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: Some(Box::new(self.parse_elseif_stmt(is_loop))),
                }
            }
            TokenTypes::Else => {
                self.advance();
                Statement::ElseIf {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    condition: expression,
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                    alternate: Some(Box::new(self.parse_else_stmt(is_loop))),
                }
            }
            _ => {
                let peek = self.peek().unwrap().to_owned();
                self.expected_error("; or elseif or else", &peek);
                exit(1)
            }
        }
    }

    fn parse_else_stmt(&mut self, is_loop: &Loop) -> Statement {
        let first = self.current().to_owned();

        self.expected_or_error(&TokenTypes::LeftCurlyBrace, "{");
        self.advance();

        let block = self.parse_block(is_loop);

        match self.peek_type() {
            TokenTypes::Semicolon => {
                self.advance();
                self.advance();
                Statement::Else {
                    start: Start {
                        line: first.line_number,
                        column: first.column_number,
                    },
                    block: match block {
                        Some(b) => Some(Box::new(b)),
                        None => None,
                    },
                }
            }
            TokenTypes::If => {
                report(
                    self.current().line_number,
                    self.current().column_number,
                    String::from("If statements cannot go after else"),
                );
                exit(1)
            }
            TokenTypes::ElseIf => {
                report(
                    self.current().line_number,
                    self.current().column_number,
                    String::from("ElseIf statements cannot go after else"),
                );
                exit(1)
            }
            _ => {
                let peek = self.peek().unwrap().to_owned();
                self.expected_error(";", &peek);
                exit(1)
            }
        }
    }

    fn parse_while_loop(&mut self, is_loop: &Loop) -> Statement {
        let initial = self.current().to_owned();

        self.expect_expr_or_error();
        self.advance();

        let test = self.parse_expr();

        self.expected_or_error(&TokenTypes::LeftCurlyBrace, "{");
        self.advance();

        let block = self.parse_block(is_loop);
        // curr }

        self.expected_or_error(&TokenTypes::Semicolon, ";");
        self.advance();
        self.advance();

        Statement::While {
            start: Start {
                line: initial.line_number,
                column: initial.column_number,
            },
            test,
            block: match block {
                Some(b) => Some(Box::new(b)),
                None => None,
            },
        }
    }

    /// if self.current if valid expr return true
    fn is_expr(&mut self) -> bool {
        match self.current_type() {
            TokenTypes::Identifier
            | TokenTypes::NumberLiteral
            | TokenTypes::StringLiteral
            | TokenTypes::BinaryPlus
            | TokenTypes::BinaryMinus
            | TokenTypes::LogicalNot
            | TokenTypes::True
            | TokenTypes::False
            | TokenTypes::Null
            | TokenTypes::LeftSquareBracket
            | TokenTypes::LeftParenthesis => true,
            _ => false,
        }
    }

    /// if self.peek is valid expr return true
    fn peek_is_expr(&mut self) -> bool {
        match self.peek_type() {
            TokenTypes::Identifier
            | TokenTypes::NumberLiteral
            | TokenTypes::StringLiteral
            | TokenTypes::BinaryPlus
            | TokenTypes::BinaryMinus
            | TokenTypes::LogicalNot
            | TokenTypes::True
            | TokenTypes::False
            | TokenTypes::Null
            | TokenTypes::LeftSquareBracket
            | TokenTypes::LeftParenthesis => true,
            _ => false,
        }
    }

    fn parse_func_return(&mut self) -> Statement {
        let start = self.current().to_owned();
        if self.peek_is_expr() {
            self.advance();
            let expr = self.parse_expr();
            self.expected_or_error(&TokenTypes::Semicolon, ";");
            self.advance();
            self.advance();
            Statement::Return {
                start: Start {
                    line: start.line_number,
                    column: start.column_number,
                },
                expression: Some(expr),
            }
        } else if self.peek_expect(&TokenTypes::Semicolon) {
            self.advance();
            self.advance();
            Statement::Return {
                start: Start {
                    line: start.line_number,
                    column: start.column_number,
                },
                expression: None,
            }
        } else {
            let peek = self.peek().unwrap().to_owned();
            self.expected_error("Expression or ;", &peek);
            exit(1)
        }
    }

    fn parse_for_loop(&mut self, is_loop: &Loop) -> Statement {
        let start = Start {
            line: self.current().line_number,
            column: self.current().column_number,
        };

        if self.peek_expect(&TokenTypes::ConstantVariable) {
            report(
                self.current().line_number,
                self.current().column_number,
                String::from("Immutable variables cannot be used inside a loop variablechange 'let' to 'mut'")
            );
            exit(1);
        }
        self.advance();

        let variable: Option<Statement>;
        if self.current_type().eq(&TokenTypes::MutableVariable) {
            variable = Some(self.parse_var_declaration());
        } else if self.current_type().eq(&TokenTypes::Semicolon) {
            self.advance();
            variable = None;
        } else {
            self.unexpected_token_error(self.current());
            exit(1)
        }
        // curr ;

        let test: Option<Expression>;

        if self.peek_is_expr() {
            self.advance();
            test = Some(self.parse_expr());
        } else if self.current_type().eq(&TokenTypes::Semicolon) {
            test = None;
        } else {
            self.unexpected_token_error(self.current());
            exit(1)
        }
        self.advance();
        // curr test; >i += 1<;

        let variable_update: Option<Statement>;

        if self.peek_type().eq(&TokenTypes::Identifier) {
            self.advance();
            variable_update = Some(self.parse_var_mutation());
        } else if self.current_type().eq(&TokenTypes::Semicolon) {
            self.advance();
            if self.current_type().eq(&TokenTypes::Semicolon) {
                self.advance();
            }
            variable_update = None;
        } else {
            self.unexpected_token_error(self.current());
            exit(1)
        }

        if !self.current_type().eq(&TokenTypes::LeftCurlyBrace) {
            self.expected_error("{", self.current());
            exit(1)
        }

        let block = self.parse_block(is_loop);
        self.expected_or_error(&TokenTypes::Semicolon, ";");
        self.advance();
        self.advance();

        Statement::For {
            start,
            variable: match variable {
                Some(v) => Some(Box::new(v)),
                None => None,
            },
            test: match test {
                Some(v) => Some(v),
                None => None,
            },
            variable_update: match variable_update {
                Some(v) => Some(Box::new(v)),
                None => None,
            },
            block: match block {
                Some(b) => Some(Box::new(b)),
                None => None,
            },
        }
    }

    fn parse_loop_controls(&mut self) -> Statement {
        match self.peek_type() {
            TokenTypes::Semicolon => {}
            _ => {
                let peek = self.peek().unwrap().to_owned();
                self.expected_error(";", &peek);
                exit(1)
            }
        }
        match self.current_type() {
            TokenTypes::Break => {
                let stmt = Statement::Break {
                    start: Start {
                        line: self.current().line_number,
                        column: self.current().column_number,
                    },
                };
                self.advance();
                self.advance();
                stmt
            }
            TokenTypes::Continue => {
                let stmt = Statement::Continue {
                    start: Start {
                        line: self.current().line_number,
                        column: self.current().column_number,
                    },
                };
                self.advance();
                self.advance();
                stmt
            }
            _ => {
                self.expected_error("brk or cnt", self.current());
                exit(1)
            }
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

    /// peeks next token and exits if its not a valid expr
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
            | TokenTypes::Null
            | TokenTypes::LeftSquareBracket
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

    /// peeks next token and exit if its not of given type
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

    /// eprints a custom error related to self.current_token
    /// does not exit program
    fn custom_error_current(&self, msg: &str) {
        report(
            self.current().line_number,
            self.current().column_number,
            format!("{msg}, remove this '{}'", self.current().token_type),
        );
    }

    fn unexpected_token_error(&self, token: &Token) {
        report(
            token.line_number,
            token.column_number,
            format!("Unexpected token '{}'", token.token_type),
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

    /// if peek token is of given type, return true
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
