use Result;

pub trait Process
{
    fn suspend(&mut self) -> Result<()>;
    fn resume(&mut self) -> Result<()>;
}

