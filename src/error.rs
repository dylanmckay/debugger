use machine;
use std;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error
{
    Machine(machine::Error),
    Io(std::io::Error),
}

impl std::error::Error for Error
{
    fn description(&self) -> &str {
        match *self {
            Error::Machine(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Machine(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Machine(ref i) => std::fmt::Display::fmt(i, fmt),
            Error::Io(ref i) => std::fmt::Display::fmt(i, fmt),
        }
    }
}
