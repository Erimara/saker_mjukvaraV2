use validator::{validate_email};

pub fn email_validation(email: String) -> Result<String, &'static str> {
    if validate_email(&email) {
        println!("valid email");
        return Ok(email)
    } else {
        println!("invalid email");
        Err("Invalid email")
    }
}
