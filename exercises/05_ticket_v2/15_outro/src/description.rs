use std::fmt;
use std::error::Error;

#[derive(Debug,Clone,PartialEq)]
pub struct TicketDescription(String);

#[derive(Debug, PartialEq)]
pub enum TicketDescriptionError {
    Empty,
    TooLong {
        max_length: usize,
    },
}

impl fmt::Display for TicketDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketDescriptionError::Empty => write!(f, "The description cannot be empty"),
            TicketDescriptionError::TooLong { max_length } => {
                write!(
                    f,
                    "The description cannot be longer than {} bytes",
                    max_length
                )
            }
        }
    }
}

impl Error for TicketDescriptionError {}


impl TicketDescription {
    
    const MAX_LENGTH: usize = 500;

    fn validate(value: &str) -> Result<TicketDescription, TicketDescriptionError> {
        if value.is_empty() {
            return Err(TicketDescriptionError::Empty);
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(TicketDescriptionError::TooLong {
                max_length: Self::MAX_LENGTH,
            });
        }
        Ok(TicketDescription(value.to_string()))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TicketDescription::validate(&value)
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketDescription::validate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
