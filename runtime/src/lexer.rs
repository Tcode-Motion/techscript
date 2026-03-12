// ── TechScript Lexer ─────────────────────────────────────────────────
// Port of lexer.py — converts raw source text into a stream of tokens.

use crate::token::{Token, TokenType, is_keyword};
use crate::error::{TechError, TechResult};

pub struct Lexer {
    source: Vec<char>,
    filename: String,
    pos: usize,
    line: usize,
    col: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str, filename: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            filename: filename.to_string(),
            pos: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
        }
    }

    fn peek(&self, offset: usize) -> char {
        if self.pos + offset < self.source.len() {
            self.source[self.pos + offset]
        } else {
            '\0'
        }
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn advance(&mut self) -> char {
        let ch = self.current();
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn add(&mut self, tt: TokenType, value: impl Into<String>, line: usize, col: usize) {
        self.tokens.push(Token::new(tt, value, line, col));
    }

    fn error(&self, msg: impl Into<String>) -> TechError {
        TechError::lexer(msg, self.line, self.col, &self.filename)
    }

    pub fn tokenize(mut self) -> TechResult<Vec<Token>> {
        while self.pos < self.source.len() {
            let ch = self.current();

            // Skip spaces (but not tabs)
            if ch == ' ' || ch == '\r' {
                self.advance();
                continue;
            }

            // Tab rejection
            if ch == '\t' {
                return Err(self.error("Tabs are not allowed in TechScript. Use spaces instead."));
            }

            // Newline
            if ch == '\n' {
                let line = self.line;
                let col = self.col;
                self.advance();
                // Collapse consecutive newlines
                if self.tokens.last().map_or(true, |t| t.token_type != TokenType::Newline) {
                    self.add(TokenType::Newline, "\\n", line, col);
                }
                continue;
            }

            // Comments
            if ch == '#' {
                if self.peek(1) == '[' {
                    self.skip_block_comment()?;
                } else {
                    self.skip_line_comment();
                }
                continue;
            }

            // Numbers
            if ch.is_ascii_digit() {
                self.read_number()?;
                continue;
            }

            // Strings
            if ch == '"' || ch == '\'' {
                self.read_string(ch)?;
                continue;
            }

            // f-strings
            if ch == 'f' && (self.peek(1) == '"' || self.peek(1) == '\'') {
                let quote = self.peek(1);
                let line = self.line;
                let col = self.col;
                self.advance(); // skip 'f'
                self.advance(); // skip quote
                let s = self.read_string_content(quote)?;
                self.add(TokenType::FString, s, line, col);
                continue;
            }

            // Raw strings
            if ch == 'r' && (self.peek(1) == '"' || self.peek(1) == '\'') {
                let quote = self.peek(1);
                let line = self.line;
                let col = self.col;
                self.advance(); // skip 'r'
                self.advance(); // skip quote
                let s = self.read_raw_string_content(quote)?;
                self.add(TokenType::String, s, line, col);
                continue;
            }

            // Identifiers and keywords
            if ch.is_alphabetic() || ch == '_' {
                self.read_identifier();
                continue;
            }

            // Symbols and operators
            self.read_symbol()?;
        }

        self.add(TokenType::Eof, "", self.line, self.col);
        Ok(self.tokens)
    }

    fn skip_line_comment(&mut self) {
        while self.pos < self.source.len() && self.current() != '\n' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> TechResult<()> {
        let start_line = self.line;
        self.advance(); // #
        self.advance(); // [
        while self.pos < self.source.len() {
            if self.current() == ']' && self.peek(1) == '#' {
                self.advance();
                self.advance();
                return Ok(());
            }
            self.advance();
        }
        Err(TechError::lexer("Unterminated block comment", start_line, 1, &self.filename))
    }

    fn read_number(&mut self) -> TechResult<()> {
        let line = self.line;
        let col = self.col;
        let mut num_str = String::new();
        let mut is_float = false;

        // Hex, binary, octal prefix
        if self.current() == '0' && self.pos + 1 < self.source.len() {
            let next = self.peek(1);
            if next == 'x' || next == 'X' {
                num_str.push(self.advance());
                num_str.push(self.advance());
                while self.pos < self.source.len() && (self.current().is_ascii_hexdigit() || self.current() == '_') {
                    let ch = self.advance();
                    if ch != '_' { num_str.push(ch); }
                }
                self.add(TokenType::NumberInt, num_str, line, col);
                return Ok(());
            }
            if next == 'b' || next == 'B' {
                num_str.push(self.advance());
                num_str.push(self.advance());
                while self.pos < self.source.len() && (self.current() == '0' || self.current() == '1' || self.current() == '_') {
                    let ch = self.advance();
                    if ch != '_' { num_str.push(ch); }
                }
                self.add(TokenType::NumberInt, num_str, line, col);
                return Ok(());
            }
            if next == 'o' || next == 'O' {
                num_str.push(self.advance());
                num_str.push(self.advance());
                while self.pos < self.source.len() && ((self.current() >= '0' && self.current() <= '7') || self.current() == '_') {
                    let ch = self.advance();
                    if ch != '_' { num_str.push(ch); }
                }
                self.add(TokenType::NumberInt, num_str, line, col);
                return Ok(());
            }
        }

        // Regular number
        while self.pos < self.source.len() && (self.current().is_ascii_digit() || self.current() == '_') {
            let ch = self.advance();
            if ch != '_' { num_str.push(ch); }
        }

        // Decimal point
        if self.pos < self.source.len() && self.current() == '.' && self.peek(1) != '.' {
            is_float = true;
            num_str.push(self.advance());
            while self.pos < self.source.len() && (self.current().is_ascii_digit() || self.current() == '_') {
                let ch = self.advance();
                if ch != '_' { num_str.push(ch); }
            }
        }

        // Scientific notation
        if self.pos < self.source.len() && (self.current() == 'e' || self.current() == 'E') {
            is_float = true;
            num_str.push(self.advance());
            if self.pos < self.source.len() && (self.current() == '+' || self.current() == '-') {
                num_str.push(self.advance());
            }
            while self.pos < self.source.len() && self.current().is_ascii_digit() {
                num_str.push(self.advance());
            }
        }

        let tt = if is_float { TokenType::NumberFloat } else { TokenType::NumberInt };
        self.add(tt, num_str, line, col);
        Ok(())
    }

    fn read_string(&mut self, quote: char) -> TechResult<()> {
        let line = self.line;
        let col = self.col;
        self.advance(); // skip opening quote

        // Triple-quoted string
        if self.current() == quote && self.peek(1) == quote {
            self.advance();
            self.advance();
            let s = self.read_triple_string_content(quote)?;
            self.add(TokenType::String, s, line, col);
            return Ok(());
        }

        let s = self.read_string_content(quote)?;
        self.add(TokenType::String, s, line, col);
        Ok(())
    }

    fn read_string_content(&mut self, quote: char) -> TechResult<String> {
        let mut result = String::new();
        while self.pos < self.source.len() && self.current() != quote {
            if self.current() == '\n' {
                return Err(self.error("Unterminated string literal"));
            }
            if self.current() == '\\' {
                self.advance();
                result.push(self.read_escape());
            } else {
                result.push(self.advance());
            }
        }
        if self.pos >= self.source.len() {
            return Err(self.error("Unterminated string literal"));
        }
        self.advance(); // closing quote
        Ok(result)
    }

    fn read_raw_string_content(&mut self, quote: char) -> TechResult<String> {
        let mut result = String::new();
        while self.pos < self.source.len() && self.current() != quote {
            if self.current() == '\n' {
                return Err(self.error("Unterminated raw string literal"));
            }
            result.push(self.advance());
        }
        if self.pos >= self.source.len() {
            return Err(self.error("Unterminated raw string literal"));
        }
        self.advance();
        Ok(result)
    }

    fn read_triple_string_content(&mut self, quote: char) -> TechResult<String> {
        let mut result = String::new();
        loop {
            if self.pos >= self.source.len() {
                return Err(self.error("Unterminated triple-quoted string"));
            }
            if self.current() == quote && self.peek(1) == quote && self.peek(2) == quote {
                self.advance();
                self.advance();
                self.advance();
                return Ok(result);
            }
            if self.current() == '\\' {
                self.advance();
                result.push(self.read_escape());
            } else {
                result.push(self.advance());
            }
        }
    }

    fn read_escape(&mut self) -> char {
        if self.pos >= self.source.len() {
            return '\\';
        }
        let ch = self.advance();
        match ch {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '\\' => '\\',
            '\'' => '\'',
            '"' => '"',
            '0' => '\0',
            _ => ch,
        }
    }

    fn read_identifier(&mut self) {
        let line = self.line;
        let col = self.col;
        let mut ident = String::new();
        while self.pos < self.source.len() && (self.current().is_alphanumeric() || self.current() == '_') {
            ident.push(self.advance());
        }

        let tt = match ident.as_str() {
            "true" => TokenType::BoolTrue,
            "false" => TokenType::BoolFalse,
            "none" => TokenType::None,
            _ if is_keyword(&ident) => TokenType::Keyword,
            _ => TokenType::Identifier,
        };
        self.add(tt, ident, line, col);
    }

    fn read_symbol(&mut self) -> TechResult<()> {
        let line = self.line;
        let col = self.col;
        let ch = self.advance();

        match ch {
            '+' => {
                if self.match_char('=') {
                    self.add(TokenType::PlusAssign, "+=", line, col);
                } else {
                    self.add(TokenType::Plus, "+", line, col);
                }
            }
            '-' => {
                if self.match_char('=') {
                    self.add(TokenType::MinusAssign, "-=", line, col);
                } else {
                    self.add(TokenType::Minus, "-", line, col);
                }
            }
            '*' => {
                if self.match_char('*') {
                    self.add(TokenType::Power, "**", line, col);
                } else if self.match_char('=') {
                    self.add(TokenType::StarAssign, "*=", line, col);
                } else {
                    self.add(TokenType::Star, "*", line, col);
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.add(TokenType::DoubleSlash, "//", line, col);
                } else if self.match_char('=') {
                    self.add(TokenType::SlashAssign, "/=", line, col);
                } else {
                    self.add(TokenType::Slash, "/", line, col);
                }
            }
            '%' => self.add(TokenType::Percent, "%", line, col),
            '=' => {
                if self.match_char('=') {
                    self.add(TokenType::Equal, "==", line, col);
                } else if self.match_char('>') {
                    self.add(TokenType::Arrow, "=>", line, col);
                } else {
                    self.add(TokenType::Assign, "=", line, col);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add(TokenType::NotEqual, "!=", line, col);
                } else {
                    return Err(TechError::lexer(
                        "Unexpected character '!'. Did you mean '!=' or 'not'?",
                        line, col, &self.filename
                    ));
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add(TokenType::LessEqual, "<=", line, col);
                } else {
                    self.add(TokenType::Less, "<", line, col);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add(TokenType::GreaterEqual, ">=", line, col);
                } else {
                    self.add(TokenType::Greater, ">", line, col);
                }
            }
            '.' => {
                if self.match_char('.') {
                    if self.match_char('=') {
                        self.add(TokenType::DotDotEqual, "..=", line, col);
                    } else if self.match_char('.') {
                        self.add(TokenType::Spread, "...", line, col);
                    } else {
                        self.add(TokenType::DotDot, "..", line, col);
                    }
                } else {
                    self.add(TokenType::Dot, ".", line, col);
                }
            }
            '?' => {
                if self.match_char('?') {
                    self.add(TokenType::Nullish, "??", line, col);
                } else if self.match_char('.') {
                    self.add(TokenType::OptionalChain, "?.", line, col);
                } else {
                    self.add(TokenType::Question, "?", line, col);
                }
            }
            '|' => {
                if self.match_char('>') {
                    self.add(TokenType::Pipe, "|>", line, col);
                } else {
                    return Err(TechError::lexer(
                        "Unexpected character '|'. Did you mean '|>'?",
                        line, col, &self.filename
                    ));
                }
            }
            '@' => self.add(TokenType::At, "@", line, col),
            '(' => self.add(TokenType::LParen, "(", line, col),
            ')' => self.add(TokenType::RParen, ")", line, col),
            '[' => self.add(TokenType::LBracket, "[", line, col),
            ']' => self.add(TokenType::RBracket, "]", line, col),
            '{' => self.add(TokenType::LBrace, "{", line, col),
            '}' => self.add(TokenType::RBrace, "}", line, col),
            ',' => self.add(TokenType::Comma, ",", line, col),
            ':' => self.add(TokenType::Colon, ":", line, col),
            _ => {
                return Err(TechError::lexer(
                    format!("Unexpected character '{}'", ch),
                    line, col, &self.filename
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<Token> {
        Lexer::new(src, "<test>").tokenize().unwrap()
    }

    #[test]
    fn test_hello_world() {
        let tokens = lex("say \"Hello, World!\"");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].value, "say");
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "Hello, World!");
    }

    #[test]
    fn test_numbers() {
        let tokens = lex("42 3.14 0xFF 0b1010 1_000");
        assert_eq!(tokens[0].token_type, TokenType::NumberInt);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[1].token_type, TokenType::NumberFloat);
        assert_eq!(tokens[1].value, "3.14");
        assert_eq!(tokens[2].token_type, TokenType::NumberInt);
        assert_eq!(tokens[2].value, "0xFF");
        assert_eq!(tokens[3].token_type, TokenType::NumberInt);
        assert_eq!(tokens[3].value, "0b1010");
        assert_eq!(tokens[4].token_type, TokenType::NumberInt);
        assert_eq!(tokens[4].value, "1000");
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("make x = 10");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].value, "make");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "x");
        assert_eq!(tokens[2].token_type, TokenType::Assign);
    }

    #[test]
    fn test_fstring() {
        let tokens = lex("f\"Hello {name}\"");
        assert_eq!(tokens[0].token_type, TokenType::FString);
        assert_eq!(tokens[0].value, "Hello {name}");
    }

    #[test]
    fn test_operators() {
        let tokens = lex("+ - * / ** // % == != <= >= = += -= *= /= => |> .. ..= ...");
        let types: Vec<_> = tokens.iter().map(|t| &t.token_type).collect();
        assert_eq!(types[0], &TokenType::Plus);
        assert_eq!(types[1], &TokenType::Minus);
        assert_eq!(types[2], &TokenType::Star);
        assert_eq!(types[3], &TokenType::Slash);
        assert_eq!(types[4], &TokenType::Power);
        assert_eq!(types[5], &TokenType::DoubleSlash);
        assert_eq!(types[6], &TokenType::Percent);
        assert_eq!(types[7], &TokenType::Equal);
        assert_eq!(types[8], &TokenType::NotEqual);
    }
}
