// & indicates that the function returns a reference
// 'static is a lifetime specifier that tells Rust this reference will live for the entire duration of the program
// str is the type of data being referred to (a string slice)
fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to build a calculator in Rust!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }
}
