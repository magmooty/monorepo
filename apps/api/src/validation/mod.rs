use regex::Regex;
use validator::ValidationError;

pub fn validate_phone_number(phone_number: &String) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^\+201[0125][0-9]{8}$").unwrap();

    match regex.is_match(phone_number) {
        true => Ok(()),
        false => Err(ValidationError::new("Invalid phone number")),
    }
}
