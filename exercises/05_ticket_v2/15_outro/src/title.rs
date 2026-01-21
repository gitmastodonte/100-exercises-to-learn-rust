// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

use std::fmt::Display;
use thiserror::Error;

use crate::Ticket;

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

impl Display for TicketTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Error)]
pub enum TicketTitleError {
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("The title cannot be empty")]
    TitleIsEmpty,
}

impl TicketTitle {
    fn new(title: String) -> Result<Self, TicketTitleError> {
        if title.is_empty() {
            return Err(TicketTitleError::TitleIsEmpty);
        }

        if title.len() > 50 {
            return Err(TicketTitleError::TitleTooLong);
        }

        return Ok(TicketTitle(title));
    }
}

// impl From<String> for TicketTitle {
//     fn from(value: String) -> TicketTitle {
//         match TicketTitle::new(value.clone()) {
//             Ok(title) => title,
//             Err(err) => match err {
//                 TicketTitleError::TitleIsEmpty => TicketTitle("".into()),
//                 TicketTitleError::TitleTooLong => TicketTitle(value.clone().split_at(50).0.into()),
//             },
//         }
//     }
// }

// impl From<&str> for TicketTitle {
//     fn from(value: &str) -> TicketTitle {
//         match TicketTitle::new(value.into()) {
//             Ok(title) => title,
//             Err(err) => match err {
//                 TicketTitleError::TitleIsEmpty => TicketTitle("".into()),
//                 TicketTitleError::TitleTooLong => TicketTitle(value.clone().split_at(50).0.into()),
//             },
//         }
//     }
// }

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match TicketTitle::new(value.clone()) {
            Ok(title) => Ok(title),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match TicketTitle::new(value.into()) {
            Ok(title) => Ok(title),
            Err(err) => Err(err),
        }
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
