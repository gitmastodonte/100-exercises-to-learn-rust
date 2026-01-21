// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

use std::fmt::Display;
use thiserror::Error;

use crate::Ticket;

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

impl Display for TicketDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("The description cannot be empty")]
    DescriptionIsEmpty,
}

impl TicketDescription {
    fn new(description: String) -> Result<Self, TicketDescriptionError> {
        if description.is_empty() {
            return Err(TicketDescriptionError::DescriptionIsEmpty);
        }

        if description.len() > 50 {
            return Err(TicketDescriptionError::DescriptionTooLong);
        }

        return Ok(TicketDescription(description));
    }
}

// impl From<String> for TicketDescription {
//     fn from(value: String) -> TicketDescription {
//         match TicketDescription::new(value.clone()) {
//             Ok(description) => description,
//             Err(err) => match err {
//                 TicketDescriptionError::DescriptionIsEmpty => TicketDescription("".into()),
//                 TicketDescriptionError::DescriptionTooLong => TicketDescription(value.clone().split_at(50).0.into()),
//             },
//         }
//     }
// }

// impl From<&str> for TicketDescription {
//     fn from(value: &str) -> TicketDescription {
//         match TicketDescription::new(value.into()) {
//             Ok(description) => description,
//             Err(err) => match err {
//                 TicketDescriptionError::DescriptionIsEmpty => TicketDescription("".into()),
//                 TicketDescriptionError::DescriptionTooLong => TicketDescription(value.clone().split_at(50).0.into()),
//             },
//         }
//     }
// }

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<TicketDescription, TicketDescriptionError> {
        match TicketDescription::new(value.clone()) {
            Ok(description) => Ok(description),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<TicketDescription, TicketDescriptionError> {
        match TicketDescription::new(value.into()) {
            Ok(description) => Ok(description),
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
