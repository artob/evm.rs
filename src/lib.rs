// This is free and unencumbered software released into the public domain.

pub mod decode;
pub mod encode;
pub mod error;
pub mod opcode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
