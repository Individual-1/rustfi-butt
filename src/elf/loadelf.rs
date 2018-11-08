
type Elf64_Addr     = u64;
type Elf64_Off      = u64;
type Elf64_Half     = u16;
type Elf64_Word     = u32;
type Elf64_Sword    = i32;
type Elf64_Xword    = u64;
type Elf64_Sxword   = i64;

#[repr(C)]
struct Elf64_Ehdr {
    e_ident:        [u8; 16],
    e_type:         Elf64_Half,
    e_machine:      Elf64_Half,
    e_version:      Elf64_Word,
    e_entry:        Elf64_Addr,
    e_phoff:        Elf64_Off,
    e_shoff:        Elf64_Off,
    e_flags:        Elf64_Word,
    e_ehsize:       Elf64_Half,
    e_phentsize:    Elf64_Half,
    e_phnum:        Elf64_Half,
    e_shentsize:    Elf64_Half,
    e_shnum:        Elf64_Half,
    e_shstrndx:     Elf64_Half
}