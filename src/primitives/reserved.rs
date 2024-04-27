use crate::macros::enum_from_str;

enum_from_str!{
    pub enum ReservedWords {
        If = "if",
        Cond = "cond",
        Else = "else",
        For = "for",
        ForAsterisk = "for*",
        ForList = "for/list",
        Let = "let",
        LetAsterisk = "let*",
        Define = "define",
        Lambda = "lambda",
        When = "when",
        Begin = "begin",
        Unless = "unless",
        Quote = "quote",
        Quasiquote = "quasiquote",
        Unquote = "unquote",
        ModulePlus = "module+",
        Provide = "provide",
        Struct = "struct"
    }
}


