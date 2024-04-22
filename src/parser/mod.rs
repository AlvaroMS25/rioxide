mod cursor;
mod error;

pub use error::ParserError;

use cursor::Cursor;

pub struct Parser<'a> {
    cursor: Cursor<'a>
}

impl<'a> Parser<'a> {
    
}
