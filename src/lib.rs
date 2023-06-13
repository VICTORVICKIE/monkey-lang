pub mod lexer;
pub use crate::lexer::*;

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::lexer::*;

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LCurly,
            TokenType::RCurly,
            TokenType::Comma,
            TokenType::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token.token_type);
        }

        return Ok(());
    }

    #[test]
    fn get_next_complete() -> Result<()> {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            TokenType::Let,
            TokenType::Identifier(String::from("five")),
            TokenType::Assign,
            TokenType::Number(String::from("5")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("ten")),
            TokenType::Assign,
            TokenType::Number(String::from("10")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("add")),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Identifier(String::from("x")),
            TokenType::Comma,
            TokenType::Identifier(String::from("y")),
            TokenType::RParen,
            TokenType::LCurly,
            TokenType::Identifier(String::from("x")),
            TokenType::Plus,
            TokenType::Identifier(String::from("y")),
            TokenType::Semicolon,
            TokenType::RCurly,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier(String::from("result")),
            TokenType::Assign,
            TokenType::Identifier(String::from("add")),
            TokenType::LParen,
            TokenType::Identifier(String::from("five")),
            TokenType::Comma,
            TokenType::Identifier(String::from("ten")),
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::FSlash,
            TokenType::Asterisk,
            TokenType::Number(String::from("5")),
            TokenType::Semicolon,
            TokenType::Number(String::from("5")),
            TokenType::LessThan,
            TokenType::Number(String::from("10")),
            TokenType::GreaterThan,
            TokenType::Number(String::from("5")),
            TokenType::Semicolon,
            TokenType::If,
            TokenType::LParen,
            TokenType::Number(String::from("5")),
            TokenType::LessThan,
            TokenType::Number(String::from("10")),
            TokenType::RParen,
            TokenType::LCurly,
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::RCurly,
            TokenType::Else,
            TokenType::LCurly,
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolon,
            TokenType::RCurly,
            TokenType::Number(String::from("10")),
            TokenType::Equal,
            TokenType::Number(String::from("10")),
            TokenType::Semicolon,
            TokenType::Number(String::from("10")),
            TokenType::NotEqual,
            TokenType::Number(String::from("9")),
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        for token in tokens {
            let next_token = lex.next_token()?;
            println!(
                "expected: {:?}, received {:?}",
                token, next_token.token_type
            );
            assert_eq!(token, next_token.token_type);
        }

        return Ok(());
    }

    #[test]
    fn get_all_token() -> Result<()> {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let lex = Lexer::new(input.into());
        let tokens: Vec<Token> = lex.into();
        println!("{:?}", tokens);

        return Ok(());
    }
}
