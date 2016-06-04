use Process;
use machine::{Pointer, MachineProcess};
use Result;
use libc;
use std::mem;

extern crate mach;

pub struct DarwinProcess
{
    task: mach::port::mach_port_name_t,
}

impl DarwinProcess
{
    pub fn attach(pid: libc::c_int) -> Result<Self> {
        let task : mach::port::mach_port_name_t = 0;

        unsafe {
            let kret = mach::traps::task_for_pid(mach::traps::mach_task_self() as mach::port::mach_port_name_t,
                                          pid,
                                          mem::transmute(&task));
            if kret != mach::kern_return::KERN_SUCCESS {
                println!("Did not succeed in getting task for pid {}", pid);
                println!("kern_return_t error {}", kret);
                println!("");
                println!("Did you forget to run with 'sudo'? This script will");
                println!("probably fail without it.");
                panic!("couldn't do it");
            }
        }

        Ok(DarwinProcess {
            task: task,
        })
    }
}

impl MachineProcess for DarwinProcess
{
    fn instruction_pointer(&self) -> Pointer {
        unimplemented!();
    }

    fn set_instruction_pointer(&mut self, _pointer: Pointer) -> Result<()> {
        unimplemented!();
    }
}

impl Process for DarwinProcess
{
    fn suspend(&mut self) -> Result<()> {
        unsafe {
            let kret = mach::task::task_suspend(self.task as mach::types::task_t);

            if kret != mach::kern_return::KERN_SUCCESS {
                println!("Did not succeed in suspending task.");
                println!("kern_return_t error {}", kret);
                panic!("couldn't suspend task");
            }
        }

        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        unsafe {
            let kret = mach::task::task_resume(self.task);

            if kret != mach::kern_return::KERN_SUCCESS {
                println!("Did not succeed in resuming task.");
                println!("kern_return_t error {}", kret);
                panic!();
            }
        }

        Ok(())
    }
}

