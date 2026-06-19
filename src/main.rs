fn main() {
    println!("{:?}", lex("12 + 3"))
}

#[derive(Debug, PartialEq, Clone)]
enum Token{
    Number(f64),
    Plus, 
    Minus, 
    Star, 
    Slash, 
    LParen, 
    RParen, 
    Eof
}
fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            _ if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num_str.push(chars[i]);
                    i += 1;
                }
                let value: f64 = num_str.parse().unwrap();
                tokens.push(Token::Number(value));
            }
            _ => { i += 1; }//this is for anything esle such as whitespaces
        }
    }

    tokens.push(Token::Eof);
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_numbers_and_operators() {
        let result = lex("12 + 3");
        assert_eq!(result, vec![
            Token::Number(12.0),
            Token::Plus,
            Token::Number(3.0),
            Token::Eof,
        ]);
    }
}