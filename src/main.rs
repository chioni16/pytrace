#![feature(let_chains)]

use anyhow::{anyhow, Result};
use goblin::elf::Elf;
use nix::unistd::Pid;

mod bindings;
mod elf;
mod memmap;
mod ptrace;
mod pvr;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let pid = args.next().ok_or(anyhow!("Pass me the PID"))?;
    let pid = pid.parse::<usize>()?;
    let pause = args
        .next()
        .ok_or(anyhow!("Do you want to pause the inferior process?"))?;
    let pause = pause == "yes";

    let interp_path = std::fs::read_link(format!("/proc/{pid}/exe"))?;

    let file = std::fs::read(&interp_path)?;
    let elf = Elf::parse(&file)?;

    let req_sym = "_PyRuntime";
    let req_sym = elf::get_dynamic_symbol(&elf, req_sym).unwrap();

    let (start, _end) = memmap::get_line_from_memmap(pid, &interp_path)?.unwrap();
    let start = 0x5588451b6000;

    let runtime_addr = start + req_sym.st_value as usize;
    loop {
        std::thread::sleep(std::time::Duration::new(0, 500_000));
        let p: Box<dyn ProcMemRead> = if pause {
            Box::new(ptrace::Ptrace::new(Pid::from_raw(pid as i32))?)
        } else {
            Box::new(pvr::Pvr::new(Pid::from_raw(pid as i32))?)
        };
        unsafe {
            let runtime = read_type(&*p, runtime_addr as *const bindings::_PyRuntimeState)?;

            let current_thread_addr =
                runtime.gilstate.tstate_current._value as *const bindings::PyThreadState;
            if current_thread_addr.is_null() {
                continue;
            }
            let current_thread = read_type(&*p, current_thread_addr)?;

            let cframe_addr = current_thread.cframe;
            if cframe_addr.is_null() {
                continue;
            }
            let cframe = read_type(&*p, cframe_addr)?;

            let iframe_addr = cframe.current_frame;
            if iframe_addr.is_null() {
                continue;
            }
            let iframe = read_type(&*p, iframe_addr)?;

            let func_addr = iframe.f_func;
            if func_addr.is_null() {
                continue;
            }
            let func = read_type(&*p, func_addr)?;

            let qualname_addr = func.func_qualname;
            let qualname_addr = qualname_addr as *const bindings::PyASCIIObject;
            if qualname_addr.is_null() {
                continue;
            }
            let qualname = read_type(&*p, qualname_addr)?;

            let qualname_len = qualname.length as usize;
            let qualname_data_addr = qualname_addr.add(1);
            let qualname_data = p.read_bytes(qualname_data_addr as *const u8, qualname_len)?;
            println!("{}", std::str::from_utf8(&qualname_data)?);
        }
    }
}

trait ProcMemRead {
    unsafe fn read_bytes(&self, addr: *const u8, len: usize) -> Result<Vec<u8>>;
}

unsafe fn read_type<T: Copy>(reader: &dyn ProcMemRead, addr: *const T) -> Result<T> {
    let addr = addr as usize;
    let type_size = std::mem::size_of::<T>();

    let tv = reader.read_bytes(addr as *const u8, type_size)?;

    let tv = unsafe {
        let tv = &*tv.as_ptr().cast::<T>();
        *tv
    };
    Ok(tv)
}
