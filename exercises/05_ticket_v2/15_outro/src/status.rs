use std::fmt;
use std::error::Error;

#[derive(Debug,Clone,PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum StatusError {
    Invalid{ invalid : String }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusError::Invalid { invalid} => write!(f, "Invalid status {}. Valid ones are: ToDo, InProgress, Done",invalid)
        }
    }
}

impl Error for StatusError {}

impl Status {
    

    fn validate(value: &str) -> Result<Status, StatusError> {
        match value.to_uppercase().as_str(){
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ => Err(StatusError::Invalid { invalid: value.to_string() })
        }
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::validate(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::validate(value)
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
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
