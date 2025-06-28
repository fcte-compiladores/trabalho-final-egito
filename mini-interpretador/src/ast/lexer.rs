
pub enum TokenKind {
    Number(i64),
    Mais,
    Menos,
    Asterisco,
    Barra,
    EsqPar,
    DirPar,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end:usize, literal: String) -> Self {
        Self { start, end, literal  }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind:TokenKind,
    span: TextSpan
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }

}


