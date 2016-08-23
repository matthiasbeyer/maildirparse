use std::result::Result as RResult;

use error::MaildirError;

pub type Result<T> = RResult<T, MaildirError>;

