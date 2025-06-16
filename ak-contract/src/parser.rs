use std::cell::Cell;

use anyhow::{bail, Result};

use crate::ast::{ExprTree, Node};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AndAnd,
    OrOr,
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        match c {
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value: f64 = num.parse()?;
                tokens.push(Token::Number(value));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Caret);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '=' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::BangEqual);
                } else {
                    bail!("Unexpected '!'");
                }
            }
            '>' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '<' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '&' => {
                chars.next();
                if let Some('&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::AndAnd);
                } else {
                    bail!("Unexpected '&'");
                }
            }
            '|' => {
                chars.next();
                if let Some('|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::OrOr);
                } else {
                    bail!("Unexpected '|'");
                }
            }
            _ => bail!("Unexpected character: {}", c),
        }
    }

    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn expect(&mut self, token: Token) -> Result<()> {
        let t = self.advance().ok_or_else(|| anyhow::anyhow!("unexpected end"))?;
        if t == token {
            Ok(())
        } else {
            bail!("expected {:?}, found {:?}", token, t);
        }
    }

    fn parse(&mut self) -> Result<ExprTree> {
        let expr = self.parse_assignment()?;
        if self.pos != self.tokens.len() {
            bail!("Unexpected tokens remaining");
        }
        Ok(expr)
    }

    fn parse_assignment(&mut self) -> Result<ExprTree> {
        if let Some(Token::Ident(name)) = self.peek().cloned() {
            if matches!(self.tokens.get(self.pos + 1), Some(Token::Equal)) {
                self.advance();
                self.advance();
                let expr = self.parse_assignment()?;
                return Ok(Box::new(Node::Assign(name, expr)));
            }
        }
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_logical_and()?;
        while matches!(self.peek(), Some(Token::OrOr)) {
            self.advance();
            let rhs = self.parse_logical_and()?;
            node = Box::new(Node::Or(node, rhs));
        }
        Ok(node)
    }

    fn parse_logical_and(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_equality()?;
        while matches!(self.peek(), Some(Token::AndAnd)) {
            self.advance();
            let rhs = self.parse_equality()?;
            node = Box::new(Node::And(node, rhs));
        }
        Ok(node)
    }

    fn parse_equality(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_comparison()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::EqualEqual => {
                    self.advance();
                    let rhs = self.parse_comparison()?;
                    node = Box::new(Node::Equal(node, rhs));
                }
                Token::BangEqual => {
                    self.advance();
                    let rhs = self.parse_comparison()?;
                    node = Box::new(Node::Different(node, rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Greater) => {
                    self.advance();
                    let rhs = self.parse_term()?;
                    node = Box::new(Node::Superior(node, rhs));
                }
                Some(Token::GreaterEqual) => {
                    self.advance();
                    let rhs = self.parse_term()?;
                    node = Box::new(Node::SuperiorEqual(node, rhs));
                }
                Some(Token::Less) => {
                    self.advance();
                    let rhs = self.parse_term()?;
                    node = Box::new(Node::Inferior(node, rhs));
                }
                Some(Token::LessEqual) => {
                    self.advance();
                    let rhs = self.parse_term()?;
                    node = Box::new(Node::InferiorEqual(node, rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_term(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    let rhs = self.parse_factor()?;
                    node = Box::new(Node::Add(node, rhs));
                }
                Some(Token::Minus) => {
                    self.advance();
                    let rhs = self.parse_factor()?;
                    node = Box::new(Node::Sub(node, rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_exponent()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    let rhs = self.parse_exponent()?;
                    node = Box::new(Node::Mul(node, rhs));
                }
                Some(Token::Slash) => {
                    self.advance();
                    let rhs = self.parse_exponent()?;
                    node = Box::new(Node::Div(node, rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_exponent(&mut self) -> Result<ExprTree> {
        let mut node = self.parse_unary()?;
        if matches!(self.peek(), Some(Token::Caret)) {
            self.advance();
            let rhs = self.parse_exponent()?; // right associative
            node = Box::new(Node::Pow(node, rhs));
        }
        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<ExprTree> {
        match self.peek() {
            Some(Token::Plus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Box::new(Node::Uplus(expr)))
            }
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Box::new(Node::Uminus(expr)))
            }
            Some(Token::Ident(ident)) if ident == "log" => {
                self.advance();
                self.expect(Token::LParen)?;
                let inner = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(Box::new(Node::Log(inner)))
            }
            Some(Token::Ident(ident)) if ident == "sqrt" => {
                self.advance();
                self.expect(Token::LParen)?;
                let inner = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(Box::new(Node::Sqrt(inner)))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<ExprTree> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.advance();
                Ok(Box::new(Node::Constant(val)))
            }
            Some(Token::Ident(ident)) if ident == "max" => {
                self.advance();
                self.expect(Token::LParen)?;
                let a = self.parse_assignment()?;
                self.expect(Token::Comma)?;
                let b = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(Box::new(Node::Max(a, b)))
            }
            Some(Token::Ident(ident)) if ident == "min" => {
                self.advance();
                self.expect(Token::LParen)?;
                let a = self.parse_assignment()?;
                self.expect(Token::Comma)?;
                let b = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(Box::new(Node::Min(a, b)))
            }
            Some(Token::Ident(ident)) if ident == "spot" => {
                self.advance();
                self.expect(Token::LParen)?;
                let inner = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(Box::new(Node::Spot(inner)))
            }
            Some(Token::Ident(name)) => {
                let id = name.clone();
                self.advance();
                Ok(Box::new(Node::Variable(id, Cell::new(None))))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_assignment()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => bail!("Unexpected token: {:?}", self.peek()),
        }
    }
}

pub fn parse_expression(text: &str) -> Result<ExprTree> {
    let tokens = tokenize(text)?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;

    fn boxed(n: Node) -> ExprTree {
        Box::new(n)
    }

    #[test]
    fn simple_add_precedence() {
        let expr = parse_expression("1 + 2 * 3").unwrap();
        assert_eq!(expr,
            boxed(Node::Add(
                boxed(Node::Constant(1.0)),
                boxed(Node::Mul(boxed(Node::Constant(2.0)), boxed(Node::Constant(3.0))))
            )));
    }

    #[test]
    fn parentheses() {
        let expr = parse_expression("(1 + 2) * 3").unwrap();
        assert_eq!(expr,
            boxed(Node::Mul(
                boxed(Node::Add(boxed(Node::Constant(1.0)), boxed(Node::Constant(2.0)))),
                boxed(Node::Constant(3.0))
            )));
    }

    #[test]
    fn assignment_parse() {
        let expr = parse_expression("x = 5").unwrap();
        assert_eq!(expr,
            boxed(Node::Assign(
                "x".into(),
                boxed(Node::Constant(5.0))
            )));
    }

    #[test]
    fn unary_and_functions() {
        let expr = parse_expression("-sqrt(4) + log(1)").unwrap();
        assert_eq!(expr,
            boxed(Node::Add(
                boxed(Node::Uminus(boxed(Node::Sqrt(boxed(Node::Constant(4.0)))))),
                boxed(Node::Log(boxed(Node::Constant(1.0))))
            )));
    }

    #[test]
    fn max_min() {
        let expr = parse_expression("max(2, min(3, 4))").unwrap();
        assert_eq!(expr,
            boxed(Node::Max(
                boxed(Node::Constant(2.0)),
                boxed(Node::Min(boxed(Node::Constant(3.0)), boxed(Node::Constant(4.0))))
            )));
    }

    #[test]
    fn comparisons_and_logical() {
        let expr = parse_expression("a < b + 5 && c != d").unwrap();
        assert_eq!(expr,
            boxed(Node::And(
                boxed(Node::Inferior(
                    boxed(Node::Variable("a".into(), Cell::new(None))),
                    boxed(Node::Add(
                        boxed(Node::Variable("b".into(), Cell::new(None))),
                        boxed(Node::Constant(5.0))
                    ))
                )),
                boxed(Node::Different(
                    boxed(Node::Variable("c".into(), Cell::new(None))),
                    boxed(Node::Variable("d".into(), Cell::new(None)))
                ))
            )));
    }

    #[test]
    fn exponent_right_assoc() {
        let expr = parse_expression("2 ^ 3 ^ 2").unwrap();
        assert_eq!(expr,
            boxed(Node::Pow(
                boxed(Node::Constant(2.0)),
                boxed(Node::Pow(boxed(Node::Constant(3.0)), boxed(Node::Constant(2.0))))
            )));
    }
}

