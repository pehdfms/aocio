use reqwest::{
    blocking::{multipart::Form, Client},
    header::COOKIE,
};
use thiserror::Error;

use crate::common::{day::AocDay, part::AocPart, session::Session, year::AocYear};

pub struct AnswerSubmitter {
    session: Session,
}

// TODO add cooldown info
// TODO add too low / too high responses
#[derive(Debug, Error)]
pub enum SubmissionError {
    #[error("Tried to submit solution request, but met unexpected error")]
    RequestError,
    #[error("Unexpected response after a valid submission request, contact library author")]
    Unknown(String),
    #[error("Session token is not valid")]
    InvalidSession,
    #[error("Answering too fast, please wait before trying again")]
    CooldownPeriod,
    #[error("The part you wanted to answer is already complete, this puzzle is completed until Part {0}")]
    AlreadyComplete(AocPart),
    #[error("The answer you gave is incorrect")]
    IncorrectAnswer,
}

impl SubmissionError {
    pub fn from_response_text(text: &str) -> Option<SubmissionError> {
        if text.contains("That's the right answer!") {
            return None;
        }

        let possible_errors = vec![
            ("To play, please identify yourself", Self::InvalidSession),
            ("You gave an answer too recently", Self::CooldownPeriod),
            ("That's not the right answer", Self::IncorrectAnswer),
            (
                "The first half of this puzzle is complete!",
                Self::AlreadyComplete(AocPart::Part1),
            ),
            (
                "Both parts of this puzzle are complete!",
                Self::AlreadyComplete(AocPart::Part2),
            ),
        ];

        for (expected_text, resulting_error) in possible_errors {
            if text.contains(expected_text) {
                return Some(resulting_error);
            }
        }

        return Some(Self::Unknown(text.to_string()));
    }
}

impl From<reqwest::Error> for SubmissionError {
    fn from(_: reqwest::Error) -> Self {
        SubmissionError::RequestError
    }
}

impl AnswerSubmitter {
    pub fn new(session: Session) -> Self {
        Self { session }
    }

    pub fn submit(
        &self,
        year: AocYear,
        day: AocDay,
        part: AocPart,
        solution: &str,
    ) -> Result<(), SubmissionError> {
        let level = part as usize;

        let form = Form::new()
            .text("level", level.to_string())
            .text("answer", solution.to_string());

        let result = Client::builder()
            .cookie_store(true)
            .build()?
            .post(format!("https://adventofcode.com/{year}/day/{day}/answer"))
            .header(COOKIE, format!("session={}", self.session))
            .multipart(form)
            .send()?
            .text()?;

        SubmissionError::from_response_text(&result).map_or(Ok(()), |err| Err(err))
    }
}
