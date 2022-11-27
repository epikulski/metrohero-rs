//! Errors emitted by the MetroHero API.
use std::fmt;

/// Errors relating to communication with the MetroHero API.
#[derive(Debug, PartialEq, Eq)]
pub enum MetroHeroError {
    HttpError,
    ParseError,
    InvalidRequest,
    InvalidStation,
    InvalidTrainId,
    InvalidItinerary,
    AuthenticationError,
    RateLimited,
}

impl std::error::Error for MetroHeroError {}

impl fmt::Display for MetroHeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetroHeroError::HttpError => write!(f, "Error while communicating with MetroHero API"),
            MetroHeroError::ParseError => write!(f, "Error while parsing data from MetroHero API"),
            MetroHeroError::InvalidRequest => write!(f, "Request to MetroHero API was invalid"),
            MetroHeroError::InvalidStation => write!(f, "Provided station code or name is invalid"),
            MetroHeroError::InvalidItinerary => write!(f, "Provided itinerary is invalid"),
            MetroHeroError::AuthenticationError => {
                write!(f, "Provided MetroHero API key is invalid")
            }
            MetroHeroError::RateLimited => {
                write!(f, "Too many requests, limit is: 10/s and 50k/24hr")
            }
            MetroHeroError::InvalidTrainId => write!(f, "Provided Train ID is not valid"),
        }
    }
}

impl From<reqwest::Error> for MetroHeroError {
    fn from(_: reqwest::Error) -> Self {
        MetroHeroError::HttpError
    }
}

impl From<serde_json::Error> for MetroHeroError {
    fn from(_: serde_json::Error) -> Self {
        MetroHeroError::ParseError
    }
}

impl From<strum::ParseError> for MetroHeroError {
    fn from(_: strum::ParseError) -> Self {
        MetroHeroError::ParseError
    }
}
