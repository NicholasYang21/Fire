/// A basic unit of a program.
///
/// It contains a slice from the source code,
/// and holds a kind for recognizing.
///
/// ### Examples
/// ```text
/// 12 + 23 * 34
/// ```
///
/// The **lexer** will split the string into:
///
/// `14, +, 23, *, 34`
///
/// And then, strings above will be the contents of the tokens to output.
///
/// Lexer will determine the kinds of the tokens:
///
/// `<Number>, <Add>, <Number>, <Mul>, <Number>`
pub struct Token {
    pub metadata: MetaData,
    pub content: String,
    pub kind: TokenKind,
}

/// The metadata of a token.
/// It contains the starting position of the token and the length of the token.
pub struct MetaData {
    start_pos: (u32, u32),
    length: u32,
}

/// The kind of a token.
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum TokenKind {
    /// symbol '+'.
    Add,
    /// symbol '-'.
    Minus,
    /// symbol '*'.
    Star,
    /// symbol '/'.
    Slash,
    /// symbol '%'.
    Percent,
    /// symbol '='.
    Equal,
    /// symbol '^'.
    Caret,
    /// symbol '~' (Bitwise not).
    Tilde,
    /// symbol '&'.
    And,
    /// symbol '|'
    Or,
    /// symbol '!'.
    Exclamation,
    /// symbol '.'.
    Dot,
    /// symbol ':'.
    Colon,
    /// symbol ';'.
    Semicolon,
    /// symbol '<' (Less Than Sign).
    Lts,
    /// symbol '>' (Greater Than Sign).
    Gts,
    /// symbol '('.
    LeftParen,
    /// symbol ')'.
    RightParen,
    /// symbol '['.
    LeftBracket,
    /// symbol ']'.
    RightBracket,
    /// symbol '{'.
    LeftBrace,
    /// symbol '}'.
    RightBrace,
    /// an identifier like `abc` or `abc_foo_123`.
    ///
    /// the lexer will **not** check whether the identifier is **valid**.
    Identifier,
    /// An integer.
    ///
    /// ### Representation
    /// ```text
    /// // without prefixes and suffixes
    /// 1234567
    /// 123_456_789 // You can insert underscores between the digits to split them.
    ///
    /// // with prefixes
    /// 0x7_FFF_FFF
    /// 0o777777
    /// 0b1110_0100_1011_0110
    ///
    /// // with suffixes
    /// 0x7_FFF_FFF_i32 // the suffix of this number is `i32 (int32)`,
    /// 0o777777_u64    // it can annotate the type of the integer.
    /// ```
    Integer(NumBase)
}

/// The enumerate described the base of an integer in Fire.
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum NumBase {
    /// Binary numbers.
    ///
    /// for example: `0b101101`, `0b10010`.
    Bin,
    /// Octal numbers.
    ///
    /// for example: `0o1234567`, `0o123321`.
    Oct,
    /// Decimal numbers.
    ///
    /// for example: `1234567890`, `114514`.
    Dec,
    /// Hexadecimal numbers.
    ///
    /// for example: `0x7ffffff`, `0xffc66d`.
    Hex,
}