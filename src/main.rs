#![feature(let_chains)]

use anyhow::Result;
use goblin::elf::Elf;
use nix::{unistd::Pid, sys::wait::waitpid};

mod bindings;
mod elf;
mod memmap;
mod ptrace;

fn main() -> Result<()> {
    let pid = 1634;
    
    let interp_path = std::fs::read_link(format!("/proc/{pid}/exe"))?;
    println!("interp_path: {:?}", interp_path);
    
    let file = std::fs::read(&interp_path)?;
    let elf = Elf::parse(&file)?;

    let req_sym = "_PyRuntime";
    let req_sym = elf::get_dynamic_symbol(&elf, req_sym).unwrap();
    println!("{:#x?}", req_sym);

    let (start, end) = memmap::get_line_from_memmap(pid, &interp_path)?.unwrap();
    println!("{} {} ", start, end);

    let start = 0x55a2915b6000;

    let runtime_addr = start + req_sym.st_value as usize;
    println!("runtime addr: {:x}", runtime_addr);
    let mut i = 0; 
    loop {
        i += 1;
        std::thread::sleep(std::time::Duration::new(0, 500_000));
        let p = ptrace::Ptrace::new(Pid::from_raw(pid as i32))?;
        let status = waitpid(Pid::from_raw(pid as i32), None)?;
        println!("waitpid status {i}: {:#?}", status);
        unsafe {
            // let preinit = std::ptr::addr_of!(runtime.preinitialized);

            let runtime = p.read(runtime_addr as *const bindings::_PyRuntimeState)?;
            
            let current_thread_addr = runtime.gilstate.tstate_current._value as *const bindings::PyThreadState;
            println!("current_thread: {:#x?}", current_thread_addr);
            if current_thread_addr.is_null() { continue; }
            let current_thread = p.read(current_thread_addr)?;
            
            let cframe_addr = current_thread.cframe;
            println!("cframe: {:#x?}", cframe_addr);
            if cframe_addr.is_null() {continue;}
            let cframe = p.read(cframe_addr)?;
            
            let iframe_addr = cframe.current_frame;
            println!("iframe: {:#x?}", iframe_addr);
            if iframe_addr.is_null() {continue;}
            let iframe = p.read(iframe_addr)?;

            let func_addr = iframe.f_func;
            println!("func: {:#x?}", func_addr);
            if func_addr.is_null() {continue;}
            let func = p.read(func_addr)?;

            let qualname_addr = func.func_qualname;
            let qualname_addr = qualname_addr as *const bindings::PyASCIIObject;
            println!("qualname: {:#x?}", qualname_addr);
            if qualname_addr.is_null() {continue;}
            let qualname = p.read(qualname_addr)?;

            let qualname_len = qualname.length as usize;
            let qualname_data_addr = qualname_addr.add(1);
            let qualname_data = p.read_bytes(qualname_data_addr as *const u8, qualname_len)?;
            println!("qualname_data: {}", std::str::from_utf8(&qualname_data)?);
        }
    }
}
