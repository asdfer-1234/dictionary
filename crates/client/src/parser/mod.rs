use dictionary::Dictionary;

pub mod korean_stdict;
//pub mod english
//pub mod german
//pub mod han

pub mod parse_error;
pub use parse_error::ParseError;

trait Parser<T: Dictionary> {
    fn parse(args: &[String]) -> T;
}
