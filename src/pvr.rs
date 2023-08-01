use std::io::IoSliceMut;

use anyhow::Result;
use nix::{
    sys::uio::{process_vm_readv, RemoteIoVec},
    unistd::Pid,
};

use crate::ProcMemRead;

pub struct Pvr(Pid);

impl Pvr {
    pub fn new(pid: impl Into<Pid>) -> Result<Self> {
        let pid = pid.into();
        Ok(Pvr(pid))
    }
}

impl ProcMemRead for Pvr {
    unsafe fn read_bytes(&self, addr: *const u8, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; len];
        let local_iov = IoSliceMut::new(&mut buf);

        let remote_iov = RemoteIoVec {
            base: addr as usize,
            len,
        };

        let _read_len = process_vm_readv(self.0, &mut [local_iov], &[remote_iov])?;
        // println!("read bytes: {read_len} == {len}");
        Ok(buf)
    }
}
