use anyhow::Result;
use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;

use crate::ProcMemRead;

pub struct Ptrace(Pid);

impl Ptrace {
    pub fn new(pid: impl Into<Pid>) -> Result<Self> {
        let pid = pid.into();
        ptrace::attach(pid)?;
        let _ = waitpid(pid, None)?;
        Ok(Ptrace(pid))
    }
}

impl Drop for Ptrace {
    fn drop(&mut self) {
        let pid = self.0;
        ptrace::detach(pid, None).expect("error occurred when detaching the process");
    }
}

impl ProcMemRead for Ptrace {
    unsafe fn read_bytes(&self, addr: *const u8, len: usize) -> Result<Vec<u8>> {
        let addr = addr as usize;
        let mut bv = Vec::new();

        let mut read_size = 0;
        while read_size < len {
            let val = ptrace::read(self.0, (addr + read_size) as ptrace::AddressType)? as u64;
            bv.extend(val.to_le_bytes());
            read_size += 8
        }

        bv.truncate(len);
        Ok(bv)
    }
}
