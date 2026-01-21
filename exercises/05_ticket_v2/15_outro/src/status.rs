// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("Invalid status")]
    StatusIsInvalid,
}

impl Status {
    fn new(status: String) -> Result<Self, StatusError> {
        let status = Status::try_from(status)?;
        return Ok(status);
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let status = value.trim().to_lowercase();

        return if status == "todo" {
            Ok(Status::ToDo)
        } else if status == "done" {
            Ok(Status::Done)
        } else if status == "inprogress" {
            Ok(Status::InProgress)
        } else {
            Err(StatusError::StatusIsInvalid)
        };
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let status = value.trim().to_lowercase();

        return if status == "todo" {
            Ok(Status::ToDo)
        } else if status == "done" {
            Ok(Status::Done)
        } else if status == "inprogress" {
            Ok(Status::InProgress)
        } else {
            Err(StatusError::StatusIsInvalid)
        };
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
