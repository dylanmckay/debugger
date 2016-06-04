use Process;
use machine::{Pointer, MachineProcess, MachineThread};
use Result;
use libc;
use std::mem;
use std::ptr;

pub use self::thread::Thread;

pub mod thread;

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

    fn threads(&self) -> Result<::std::vec::IntoIter<Box<MachineThread>>> {
        let thread_list: mach::types::thread_act_array_t = ptr::null_mut();
        let thread_count: mach::message::mach_msg_type_number_t = 0;

        unsafe {
            let kret = mach::task::task_threads(
                self.task as mach::types::task_t,
                mem::transmute(&thread_list),
                mem::transmute(&thread_count),
            );

            if kret == mach::kern_return::KERN_SUCCESS {
                let thread_ids = Vec::from_raw_parts(thread_list, thread_count as usize, thread_count as usize);

                let threads: Vec<_> = thread_ids.into_iter().
                    map(|thread_id| Thread { id: thread_id }).
                    map(|thread| Box::new(thread) as Box<MachineThread>).
                    collect();
                Ok(threads.into_iter())
            } else {
                panic!("error, kret: {}", kret);
            }
        }
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

