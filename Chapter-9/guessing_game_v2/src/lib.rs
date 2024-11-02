pub mod validator {
    pub fn check_guess(guess : u32) -> Result<u32, String> {
        if guess > 0 && guess < 101 {
            return Ok(guess);
        }

        return Err(format!("The number {} is invalid", guess));
    }
}