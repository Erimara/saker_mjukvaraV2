use validator::{validate_email};

pub fn email_validation(email: String) -> Result<String, &'static str> {
    if validate_email(&email) {

        return Ok(email)
    } else {
        Err("Invalid email")
    }
}
