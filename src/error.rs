use std::error::Error;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MaildirErrorKind {
    IOError,
    NotAMaildirError,
}

impl MaildirErrorKind {

    fn into_error_with_cause(self, cause: Box<Error>) -> MaildirError {
        MaildirError::new(self, Some(cause))
    }

}

impl Into<MaildirError> for MaildirErrorKind {

    fn into(self) -> MaildirError {
        MaildirError::new(self, None)
    }

}

#[derive(Debug)]
pub struct MaildirError {
    kind: MaildirErrorKind,
    cause: Option<Box<Error>>,
}

impl MaildirError {

    pub fn new(kind: MaildirErrorKind, cause: Option<Box<Error>>) -> MaildirError {
        MaildirError {
            kind: kind,
            cause: cause,
        }
    }

    pub fn kind(&self) -> MaildirErrorKind {
        self.kind
    }

}

fn errkind_to_str(k: &MaildirErrorKind) -> &'static str {
    match *k {
        IOError => "IO Error",
        NotAMaildirError => "Not a maildir"
    }
}

impl Display for MaildirError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", errkind_to_str(&self.kind)));
        Ok(())
    }

}

impl Error for MaildirError {

    fn description(&self) -> &str {
        errkind_to_str(&self.kind)
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

