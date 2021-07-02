//! use of custom parser

#![deny(missing_docs)]
#![deny(unsafe_code)]

mod parser;
use parser::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
