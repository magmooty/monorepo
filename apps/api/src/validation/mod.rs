use regex::Regex;
use validator::ValidationError;

pub fn validate_phone_number(phone_number: &String) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^\+201[0125][0-9]{8}$").unwrap();

    match regex.is_match(phone_number) {
        true => Ok(()),
        false => Err(ValidationError::new("Invalid phone number")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone_number() {
        assert!(validate_phone_number(&String::from("+201234567890")).is_ok());
        assert!(validate_phone_number(&String::from("+20123456789")).is_err());
        assert!(validate_phone_number(&String::from("+2012345678901")).is_err());
        assert!(validate_phone_number(&String::from("201234567890")).is_err());
        assert!(validate_phone_number(&String::from("+20123456789a")).is_err());
    }
}
