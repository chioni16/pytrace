use anyhow::Result;
use nix::sys::ptrace;
use nix::unistd::Pid;

pub struct Ptrace(Pid);

impl Ptrace {
    pub fn new(pid: impl Into<Pid>) -> Result<Self> {
        let pid = pid.into();
        ptrace::attach(pid)?;
        Ok(Ptrace(pid))
    }

    pub unsafe fn read<T: Copy>(&self, addr: *const T) -> Result<T> {
        let addr = addr as usize;
        let type_size = std::mem::size_of::<T>();

        let tv = self.read_bytes(addr as *const u8, type_size)?;

        let tv = unsafe {
            let tv = &*tv.as_ptr().cast::<T>();
            *tv
        };
        Ok(tv)
    }

    pub unsafe fn read_bytes(&self, addr: *const u8, len: usize) -> Result<Vec<u8>> {
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

impl Drop for Ptrace {
    fn drop(&mut self) {
        println!("called the drop");
        let pid = self.0;
        ptrace::detach(pid, None).expect("error occurred when detaching the process");
    }
}