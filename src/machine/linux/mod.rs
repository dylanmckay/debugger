use Process;
use machine::{Pointer, MachineProcess, MachineThread};
use Result;

use libc;

// TODO: get this from libc, but it can't be found
pub const PTRACE_ATTACH: libc::c_int = 16;

pub struct LinuxProcess
{
    trace_id: libc::c_int,
    pid: libc::pid_t,
}

impl LinuxProcess
{
    pub fn attach(pid: libc::pid_t) -> Result<Self> {
        let trace_id = unsafe {
            libc::ptrace(
                PTRACE_ATTACH,
                pid,
                0 as *mut _,
                0 as _,
            )
        };

        if trace_id == -1 {
            Err(::Error::Io(::std::io::Error::last_os_error()))
        } else {
            Ok(LinuxProcess {
                trace_id: trace_id,
                pid: pid,
            })
        }
    }
}

impl MachineProcess for LinuxProcess
{
    fn instruction_pointer(&self) -> Pointer {
        unimplemented!();
    }

    fn set_instruction_pointer(&mut self, _pointer: Pointer) -> Result<()> {
        unimplemented!();
    }

    fn threads(&self) -> Result<::std::vec::IntoIter<Box<MachineThread>>> {
        unimplemented!();
    }
}

impl Process for LinuxProcess
{
    fn suspend(&mut self) -> Result<()> {
        unimplemented!();
    }

    fn resume(&mut self) -> Result<()> {
        unimplemented!();
    }
}

