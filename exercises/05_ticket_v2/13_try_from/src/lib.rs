// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum InvalidStatusError {
    UnknownStatus(String),
}

impl std::fmt::Display for InvalidStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidStatusError::UnknownStatus(s) => write!(f, "Unknown status: '{}'", s),
        }
    }
}

impl std::error::Error for InvalidStatusError {}

impl TryFrom<String> for Status {
    
    type Error = InvalidStatusError;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ =>  Err(InvalidStatusError::UnknownStatus(value))
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = InvalidStatusError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ => Err(InvalidStatusError::UnknownStatus(value.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
