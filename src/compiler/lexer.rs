// SigmaOS Compiler — Lexer
// Tokenizes Sigma source language into a stream of typed tokens.
// Supports identifiers, keywords, literals (int, float, string, char),
// operators, delimiters, and comments.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

// ─── Token Types ────────────────────────────────────────────────────────────

/// Source location for error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}

impl SourceSpan {
    pub fn new(file: &str, line: usize, column: usize, offset: usize, length: usize) -> Self {
        Self {
            file: String::from(file),
            line,
            column,
            offset,
            length,
        }
    }

    pub fn synthetic() -> Self {
        Self::new("<synthetic>", 0, 0, 0, 0)
    }
}

/// All keyword variants recognised by the Sigma language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Struct,
    Enum,
    Impl,
    Trait,
    Pub,
    Use,
    Mod,
    Const,
    Static,
    Match,
    Type,
    True,
    False,
    As,
    Break,
    Continue,
    Loop,
    Extern,
    Unsafe,
    Self_,
    Super,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "fn" => Some(Keyword::Fn),
            "let" => Some(Keyword::Let),
            "mut" => Some(Keyword::Mut),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "in" => Some(Keyword::In),
            "return" => Some(Keyword::Return),
            "struct" => Some(Keyword::Struct),
            "enum" => Some(Keyword::Enum),
            "impl" => Some(Keyword::Impl),
            "trait" => Some(Keyword::Trait),
            "pub" => Some(Keyword::Pub),
            "use" => Some(Keyword::Use),
            "mod" => Some(Keyword::Mod),
            "const" => Some(Keyword::Const),
            "static" => Some(Keyword::Static),
            "match" => Some(Keyword::Match),
            "type" => Some(Keyword::Type),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "as" => Some(Keyword::As),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "loop" => Some(Keyword::Loop),
            "extern" => Some(Keyword::Extern),
            "unsafe" => Some(Keyword::Unsafe),
            "self" => Some(Keyword::Self_),
            "super" => Some(Keyword::Super),
            _ => None,
        }
    }
}

/// All token types produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntegerLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    // Identifiers & keywords
    Identifier(String),
    Keyword(Keyword),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Eq,
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AmpAmp,
    PipePipe,
    LtLt,
    GtGt,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    Arrow,       // ->
    FatArrow,    // =>
    DotDot,      // ..
    DotDotEq,    // ..=
    ColonColon,  // ::

    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    At,
    Hash,
    Question,

    // Special
    Eof,
    Newline,
}

/// A single token with its kind and source location.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
    pub lexeme: String,
}

// ─── Lexer ──────────────────────────────────────────────────────────────────

/// Error produced during lexing.
#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub span: SourceSpan,
}

/// The Sigma language lexer. Converts source text to a token stream.
pub struct Lexer {
    source: Vec<char>,
    file: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str, file: &str) -> Self {
        Self {
            source: source.chars().collect(),
            file: String::from(file),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire source into a Vec<Token>, or return errors.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<LexError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            if self.is_eof() {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: self.span(self.pos, 0),
                    lexeme: String::new(),
                });
                break;
            }

            match self.next_token() {
                Ok(tok) => tokens.push(tok),
                Err(e) => {
                    errors.push(e);
                    self.advance(); // skip bad char
                }
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let ch = self.peek().unwrap();

        // Newline
        if ch == '\n' {
            self.advance();
            return Ok(Token {
                kind: TokenKind::Newline,
                span: self.span(start, 1),
                lexeme: String::from("\n"),
            });
        }

        // Numbers
        if ch.is_ascii_digit() {
            return self.lex_number(start);
        }

        // Identifiers / keywords
        if ch == '_' || ch.is_ascii_alphabetic() {
            return Ok(self.lex_identifier(start));
        }

        // Strings
        if ch == '"' {
            return self.lex_string(start);
        }

        // Char literals
        if ch == '\'' {
            return self.lex_char(start);
        }

        // Multi-char operators / single-char tokens
        self.lex_operator_or_delimiter(start, start_line, start_col)
    }

    fn lex_number(&mut self, start: usize) -> Result<Token, LexError> {
        let mut is_float = false;

        // Handle 0x, 0b, 0o prefixes
        if self.peek() == Some('0') && self.pos + 1 < self.source.len() {
            let next = self.source[self.pos + 1];
            if next == 'x' || next == 'X' {
                self.advance();
                self.advance();
                while self.peek().map_or(false, |c| c.is_ascii_hexdigit() || c == '_') {
                    self.advance();
                }
                let lexeme: String = self.source[start..self.pos].iter().collect();
                let cleaned: String = lexeme.replace('_', "");
                let val = u64::from_str_radix(&cleaned[2..], 16).unwrap_or(0);
                return Ok(Token {
                    kind: TokenKind::IntegerLiteral(val),
                    span: self.span(start, self.pos - start),
                    lexeme,
                });
            }
            if next == 'b' || next == 'B' {
                self.advance();
                self.advance();
                while self.peek().map_or(false, |c| c == '0' || c == '1' || c == '_') {
                    self.advance();
                }
                let lexeme: String = self.source[start..self.pos].iter().collect();
                let cleaned: String = lexeme.replace('_', "");
                let val = u64::from_str_radix(&cleaned[2..], 2).unwrap_or(0);
                return Ok(Token {
                    kind: TokenKind::IntegerLiteral(val),
                    span: self.span(start, self.pos - start),
                    lexeme,
                });
            }
        }

        while self.peek().map_or(false, |c| c.is_ascii_digit() || c == '_') {
            self.advance();
        }

        // Check for decimal point
        if self.peek() == Some('.') && self.pos + 1 < self.source.len()
            && self.source[self.pos + 1].is_ascii_digit()
        {
            is_float = true;
            self.advance(); // consume '.'
            while self.peek().map_or(false, |c| c.is_ascii_digit() || c == '_') {
                self.advance();
            }
        }

        // Scientific notation
        if self.peek().map_or(false, |c| c == 'e' || c == 'E') {
            is_float = true;
            self.advance();
            if self.peek().map_or(false, |c| c == '+' || c == '-') {
                self.advance();
            }
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let lexeme: String = self.source[start..self.pos].iter().collect();
        let cleaned: String = lexeme.replace('_', "");

        if is_float {
            let val: f64 = cleaned.parse().unwrap_or(0.0);
            Ok(Token {
                kind: TokenKind::FloatLiteral(val),
                span: self.span(start, self.pos - start),
                lexeme,
            })
        } else {
            let val: u64 = cleaned.parse().unwrap_or(0);
            Ok(Token {
                kind: TokenKind::IntegerLiteral(val),
                span: self.span(start, self.pos - start),
                lexeme,
            })
        }
    }

    fn lex_identifier(&mut self, start: usize) -> Token {
        while self.peek().map_or(false, |c| c == '_' || c.is_ascii_alphanumeric()) {
            self.advance();
        }
        let lexeme: String = self.source[start..self.pos].iter().collect();
        let kind = if let Some(kw) = Keyword::from_str(&lexeme) {
            TokenKind::Keyword(kw)
        } else {
            TokenKind::Identifier(lexeme.clone())
        };
        Token {
            kind,
            span: self.span(start, self.pos - start),
            lexeme,
        }
    }

    fn lex_string(&mut self, start: usize) -> Result<Token, LexError> {
        self.advance(); // skip opening "
        let mut value = String::new();
        loop {
            match self.peek() {
                None | Some('\n') => {
                    return Err(LexError {
                        message: String::from("unterminated string literal"),
                        span: self.span(start, self.pos - start),
                    });
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.peek() {
                        Some('n') => { value.push('\n'); self.advance(); }
                        Some('t') => { value.push('\t'); self.advance(); }
                        Some('r') => { value.push('\r'); self.advance(); }
                        Some('\\') => { value.push('\\'); self.advance(); }
                        Some('"') => { value.push('"'); self.advance(); }
                        Some('0') => { value.push('\0'); self.advance(); }
                        _ => {
                            return Err(LexError {
                                message: String::from("invalid escape sequence"),
                                span: self.span(self.pos - 1, 2),
                            });
                        }
                    }
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }
        let lexeme: String = self.source[start..self.pos].iter().collect();
        Ok(Token {
            kind: TokenKind::StringLiteral(value),
            span: self.span(start, self.pos - start),
            lexeme,
        })
    }

    fn lex_char(&mut self, start: usize) -> Result<Token, LexError> {
        self.advance(); // skip '
        let ch = match self.peek() {
            Some('\\') => {
                self.advance();
                match self.peek() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\\') => '\\',
                    Some('\'') => '\'',
                    Some('0') => '\0',
                    _ => {
                        return Err(LexError {
                            message: String::from("invalid char escape"),
                            span: self.span(start, 3),
                        });
                    }
                }
            }
            Some(c) if c != '\'' => c,
            _ => {
                return Err(LexError {
                    message: String::from("empty char literal"),
                    span: self.span(start, 1),
                });
            }
        };
        self.advance();
        if self.peek() != Some('\'') {
            return Err(LexError {
                message: String::from("unterminated char literal"),
                span: self.span(start, self.pos - start),
            });
        }
        self.advance();
        let lexeme: String = self.source[start..self.pos].iter().collect();
        Ok(Token {
            kind: TokenKind::CharLiteral(ch),
            span: self.span(start, self.pos - start),
            lexeme,
        })
    }

    fn lex_operator_or_delimiter(
        &mut self,
        start: usize,
        _start_line: usize,
        _start_col: usize,
    ) -> Result<Token, LexError> {
        let ch = self.advance().unwrap();
        let next = self.peek();

        let (kind, extra) = match (ch, next) {
            ('+', Some('=')) => { self.advance(); (TokenKind::PlusEq, true) }
            ('-', Some('>')) => { self.advance(); (TokenKind::Arrow, true) }
            ('-', Some('=')) => { self.advance(); (TokenKind::MinusEq, true) }
            ('*', Some('=')) => { self.advance(); (TokenKind::StarEq, true) }
            ('/', Some('=')) => { self.advance(); (TokenKind::SlashEq, true) }
            ('=', Some('>')) => { self.advance(); (TokenKind::FatArrow, true) }
            ('=', Some('=')) => { self.advance(); (TokenKind::EqEq, true) }
            ('!', Some('=')) => { self.advance(); (TokenKind::BangEq, true) }
            ('<', Some('=')) => { self.advance(); (TokenKind::LtEq, true) }
            ('<', Some('<')) => { self.advance(); (TokenKind::LtLt, true) }
            ('>', Some('=')) => { self.advance(); (TokenKind::GtEq, true) }
            ('>', Some('>')) => { self.advance(); (TokenKind::GtGt, true) }
            ('&', Some('&')) => { self.advance(); (TokenKind::AmpAmp, true) }
            ('|', Some('|')) => { self.advance(); (TokenKind::PipePipe, true) }
            (':', Some(':')) => { self.advance(); (TokenKind::ColonColon, true) }
            ('.', Some('.')) => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    (TokenKind::DotDotEq, true)
                } else {
                    (TokenKind::DotDot, true)
                }
            }
            ('+', _) => (TokenKind::Plus, false),
            ('-', _) => (TokenKind::Minus, false),
            ('*', _) => (TokenKind::Star, false),
            ('/', _) => (TokenKind::Slash, false),
            ('%', _) => (TokenKind::Percent, false),
            ('&', _) => (TokenKind::Ampersand, false),
            ('|', _) => (TokenKind::Pipe, false),
            ('^', _) => (TokenKind::Caret, false),
            ('~', _) => (TokenKind::Tilde, false),
            ('!', _) => (TokenKind::Bang, false),
            ('=', _) => (TokenKind::Eq, false),
            ('<', _) => (TokenKind::Lt, false),
            ('>', _) => (TokenKind::Gt, false),
            ('(', _) => (TokenKind::LParen, false),
            (')', _) => (TokenKind::RParen, false),
            ('[', _) => (TokenKind::LBracket, false),
            (']', _) => (TokenKind::RBracket, false),
            ('{', _) => (TokenKind::LBrace, false),
            ('}', _) => (TokenKind::RBrace, false),
            (',', _) => (TokenKind::Comma, false),
            (';', _) => (TokenKind::Semicolon, false),
            (':', _) => (TokenKind::Colon, false),
            ('.', _) => (TokenKind::Dot, false),
            ('@', _) => (TokenKind::At, false),
            ('#', _) => (TokenKind::Hash, false),
            ('?', _) => (TokenKind::Question, false),
            _ => {
                return Err(LexError {
                    message: alloc::format!("unexpected character: '{}'", ch),
                    span: self.span(start, 1),
                });
            }
        };

        let len = if extra { self.pos - start } else { 1 };
        let lexeme: String = self.source[start..self.pos].iter().collect();
        Ok(Token {
            kind,
            span: self.span(start, len),
            lexeme,
        })
    }

    // ── Helpers ──────────────────────────────────────────────────────────

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.source.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace (except newlines — those are tokens)
            while self.peek().map_or(false, |c| c == ' ' || c == '\t' || c == '\r') {
                self.advance();
            }
            // Line comments
            if self.pos + 1 < self.source.len()
                && self.source[self.pos] == '/'
                && self.source[self.pos + 1] == '/'
            {
                while self.peek().map_or(false, |c| c != '\n') {
                    self.advance();
                }
                continue;
            }
            // Block comments
            if self.pos + 1 < self.source.len()
                && self.source[self.pos] == '/'
                && self.source[self.pos + 1] == '*'
            {
                self.advance();
                self.advance();
                let mut depth = 1u32;
                while depth > 0 && !self.is_eof() {
                    if self.pos + 1 < self.source.len()
                        && self.source[self.pos] == '/'
                        && self.source[self.pos + 1] == '*'
                    {
                        depth += 1;
                        self.advance();
                        self.advance();
                    } else if self.pos + 1 < self.source.len()
                        && self.source[self.pos] == '*'
                        && self.source[self.pos + 1] == '/'
                    {
                        depth -= 1;
                        self.advance();
                        self.advance();
                    } else {
                        self.advance();
                    }
                }
                continue;
            }
            break;
        }
    }

    fn span(&self, offset: usize, length: usize) -> SourceSpan {
        SourceSpan::new(&self.file, self.line, self.column, offset, length)
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_keywords() {
        let mut lexer = Lexer::new("fn let if else return", "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Keyword(Keyword::Fn)));
        assert!(matches!(tokens[1].kind, TokenKind::Keyword(Keyword::Let)));
        assert!(matches!(tokens[2].kind, TokenKind::Keyword(Keyword::If)));
        assert!(matches!(tokens[3].kind, TokenKind::Keyword(Keyword::Else)));
        assert!(matches!(tokens[4].kind, TokenKind::Keyword(Keyword::Return)));
    }

    #[test]
    fn test_lex_integers() {
        let mut lexer = Lexer::new("42 0xFF 0b1010 1_000_000", "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral(42));
        assert_eq!(tokens[1].kind, TokenKind::IntegerLiteral(255));
        assert_eq!(tokens[2].kind, TokenKind::IntegerLiteral(10));
        assert_eq!(tokens[3].kind, TokenKind::IntegerLiteral(1_000_000));
    }

    #[test]
    fn test_lex_float() {
        let mut lexer = Lexer::new("3.14 2.0e10", "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::FloatLiteral(_)));
        assert!(matches!(tokens[1].kind, TokenKind::FloatLiteral(_)));
    }

    #[test]
    fn test_lex_string() {
        let mut lexer = Lexer::new(r#""hello\nworld""#, "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].kind,
            TokenKind::StringLiteral(String::from("hello\nworld"))
        );
    }

    #[test]
    fn test_lex_operators() {
        let mut lexer = Lexer::new("-> => :: .. ..= && ||", "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Arrow);
        assert_eq!(tokens[1].kind, TokenKind::FatArrow);
        assert_eq!(tokens[2].kind, TokenKind::ColonColon);
        assert_eq!(tokens[3].kind, TokenKind::DotDot);
        assert_eq!(tokens[4].kind, TokenKind::DotDotEq);
        assert_eq!(tokens[5].kind, TokenKind::AmpAmp);
        assert_eq!(tokens[6].kind, TokenKind::PipePipe);
    }

    #[test]
    fn test_nested_block_comments() {
        let mut lexer = Lexer::new("/* outer /* inner */ still comment */ 42", "test.sg");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral(42));
    }
}
