use Result;
use Process;
use std;

#[cfg(target_os = "linux")]
type TargetProcess = linux::LinuxProcess;
#[cfg(target_os = "macos")]
type TargetProcess = darwin::DarwinProcess;

pub mod darwin;
pub mod linux;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Pointer(u64);

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Error {
    AddressOutOfRange(Pointer),
    InvalidPid(String),
}

impl std::error::Error for Error
{
    fn description(&self) -> &str {
        match *self {
            Error::AddressOutOfRange(..) => {
                "address out of range"
            },
            Error::InvalidPid(ref pid) => {
                "invalid process identifier"
            }
        }
    }
}

impl std::fmt::Display for Error
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(fmt, "{}", self.description())
    }
}

pub trait MachineProcess : Process
{
    fn instruction_pointer(&self) -> Pointer;
    fn set_instruction_pointer(&mut self, ip: Pointer) -> Result<()>;

    fn threads(&self) -> Result<::std::vec::IntoIter<Box<MachineThread>>>;
}

pub trait MachineThread : ::std::fmt::Debug
{
}

pub fn attach_to(pid: &str) -> Result<Box<MachineProcess>> {
    let pid = match pid.parse() {
        Ok(a) => a,
        Err(..) => {
            return Err(::Error::Machine(Error::InvalidPid(pid.to_owned())))
        },
    };

    linux::LinuxProcess::attach(pid).map(|p| Box::new(p) as Box<MachineProcess>)
}

