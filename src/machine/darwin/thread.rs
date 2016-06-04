use machine::darwin::mach;
use machine;

#[derive(Debug)]
pub struct Thread
{
    pub id: mach::types::thread_act_t,
}

impl machine::MachineThread for Thread
{

}

