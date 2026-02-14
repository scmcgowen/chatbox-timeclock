#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    PlayerNotFound,
    TimecardNotFound,
    AlreadyClockedIn,
    NotClockedIn,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DatabaseError(err) => write!(f, "Database error: {}", err),
            Error::PlayerNotFound => write!(f, "Player not found"),
            Error::TimecardNotFound => write!(f, "Timecard not found"),
            Error::AlreadyClockedIn => write!(f, "Already clocked in"),
            Error::NotClockedIn => write!(f, "Not clocked in"),
        }
    }
}

impl std::error::Error for Error {}
