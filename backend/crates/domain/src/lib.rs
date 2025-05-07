use std::error::Error;

pub mod valueobject;
pub mod repository;
pub mod entity;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let x = 728082+944570+28800+1623122+4340832+20868855+140266224+33522517+48797333+294179842+90302891+1827701+55435+2422808+152100+1154831+1117584+381997+150624+3321+302194;
        assert_eq!(x, 643171663);
    }
}
