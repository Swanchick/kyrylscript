use std::fmt;

const SYMBOLS: &str = "()[]{}<>;:=+-*/!.,^&|/?";

pub const COMMENT: &str = "//";

pub fn get_token(text: &str) -> Option<Token> {
    match text {
        "let" => Some(Token::Let),
        "function" => Some(Token::Function),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "while" => Some(Token::While),
        "for" => Some(Token::For),
        "return" => Some(Token::Return),
        "int" => Some(Token::Int),
        "float" => Some(Token::Float),
        "string" => Some(Token::String),
        "bool" => Some(Token::Bool),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "void" => Some(Token::Void),
        "null" => Some(Token::Null),
        "struct" => Some(Token::Struct),
        "enum" => Some(Token::Enum),
        "in" => Some(Token::In),
        "use" => Some(Token::Use),
        "pub" => Some(Token::Pub),
        "root" => Some(Token::Root),

        "(" => Some(Token::LeftParenthesis),
        ")" => Some(Token::RightParenthesis),
        "{" => Some(Token::LeftBrace),
        "}" => Some(Token::RightBrace),
        "[" => Some(Token::LeftSquareBracket),
        "]" => Some(Token::RightSquareBracket),
        ";" => Some(Token::Semicolon),
        ":" => Some(Token::Colon),
        "=" => Some(Token::Equal),
        "+" => Some(Token::Plus),
        "+=" => Some(Token::PlusEqual),
        "++" => Some(Token::PlusPlus),
        "-" => Some(Token::Minus),
        "-=" => Some(Token::MinusEqual),
        "--" => Some(Token::MinusMinus),
        "*" => Some(Token::Multiply),
        "/" => Some(Token::Divide),
        "<" => Some(Token::LessThan),
        ">" => Some(Token::GreaterThan),
        "!" => Some(Token::Not),
        "," => Some(Token::Comma),
        "^" => Some(Token::Power),
        "==" => Some(Token::EqualEqual),
        "!=" => Some(Token::NotEqual),
        "<=" => Some(Token::LessEqual),
        ">=" => Some(Token::GreaterEqual),
        "&&" => Some(Token::And),
        "||" => Some(Token::Or),
        "?" => Some(Token::Question),
        "." => Some(Token::Dot),
        "::" => Some(Token::ColonColon),
        "->" => Some(Token::Arrow),
        _ => None,
    }
}

pub fn is_symbol(c: char) -> bool {
    SYMBOLS.contains(c)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i32),
    FloatLiteral(f64),

    // Keywords
    Let,
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Int,
    Float,
    String,
    Bool,
    True,
    False,
    Void,
    Null,
    Struct,
    Enum,
    In,
    Use,
    Pub,
    Root,

    // Symbols
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftBrace,          // {
    RightBrace,         // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Semicolon,          // ;
    Colon,              // :
    Comma,              // ,
    Equal,              // =
    Plus,               // +
    PlusEqual,          // +=
    PlusPlus,           // ++
    Minus,              // -
    MinusEqual,         // -=
    MinusMinus,         // --
    Multiply,           // *
    Divide,             // /
    LessThan,           // <
    GreaterThan,        // >
    Not,                // !
    Power,              // ^
    EqualEqual,         // ==
    NotEqual,           // ~=
    LessEqual,          // <=
    GreaterEqual,       // >=
    And,                // &&
    Or,                 // ||
    Question,           // ?
    Dot,                // .
    ColonColon,         // ::
    Arrow,              // ->
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "identifier ({})", name),
            Token::StringLiteral(string_literal) => {
                write!(f, "string literal ({})", string_literal)
            }
            Token::IntegerLiteral(number) => write!(f, "integer literal ({})", number),
            Token::FloatLiteral(number) => write!(f, "float literal ({})", number),

            Token::Let => write!(f, "let"),
            Token::Function => write!(f, "function"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::Return => write!(f, "return"),
            Token::Int => write!(f, "int"),
            Token::Float => write!(f, "float"),
            Token::String => write!(f, "string"),
            Token::Bool => write!(f, "bool"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Void => write!(f, "void"),
            Token::Null => write!(f, "null"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::In => write!(f, "in"),
            Token::Use => write!(f, "use"),
            Token::Pub => write!(f, "pub"),
            Token::Root => write!(f, "root"),

            Token::RightParenthesis => write!(f, ")"),
            Token::LeftParenthesis => write!(f, "("),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightSquareBracket => write!(f, "["),
            Token::LeftSquareBracket => write!(f, "]"),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Equal => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::PlusEqual => write!(f, "+="),
            Token::PlusPlus => write!(f, "++"),
            Token::Minus => write!(f, "--"),
            Token::MinusEqual => write!(f, "-="),
            Token::MinusMinus => write!(f, "--"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::Not => write!(f, "!"),
            Token::Power => write!(f, "^"),
            Token::EqualEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Question => write!(f, "?"),
            Token::Dot => write!(f, "."),
            Token::ColonColon => write!(f, "::"),
            Token::Arrow => write!(f, "->"),
        }
    }
}
