use std::fmt;
use std::error::Error;

#[derive(Debug,Clone,PartialEq)]
pub struct TicketTitle(String);

#[derive(Debug, PartialEq)]
pub enum TicketTitleError {
    Empty,
    TooLong {
        max_length: usize,
    },
}

impl fmt::Display for TicketTitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketTitleError::Empty => write!(f, "The title cannot be empty"),
            TicketTitleError::TooLong { max_length } => {
                write!(
                    f,
                    "The title cannot be longer than {} bytes",
                    max_length
                )
            }
        }
    }
}

impl Error for TicketTitleError {}


impl TicketTitle {
    
    const MAX_LENGTH: usize = 50;

    fn validate(value: &str) -> Result<TicketTitle, TicketTitleError> {
        if value.is_empty() {
            return Err(TicketTitleError::Empty);
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(TicketTitleError::TooLong {
                max_length: Self::MAX_LENGTH,
            });
        }
        Ok(TicketTitle(value.to_string()))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TicketTitle::validate(&value)
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketTitle::validate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
