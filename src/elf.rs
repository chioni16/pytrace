use goblin::elf::{sym::Sym, Elf};

pub fn get_dynamic_symbol(elf: &Elf, req_sym: &str) -> Option<Sym> {
    elf
        .dynsyms
        .iter()
        .filter(|sym| if let Some(name) = elf.dynstrtab.get_at(sym.st_name) && name == req_sym { true } else  { false })
        .next()
}
