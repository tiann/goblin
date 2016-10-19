pub use elf::header::*;

elf_header!(u32);

pub const SIZEOF_EHDR: usize = 48;
pub const ELFCLASS: u8 = ELFCLASS32;

elf_header_impure_impl!(
    impl Header {
        elf_header_from_bytes!();
        elf_header_from_fd!();
        #[cfg(feature = "endian_fd")]
        pub fn parse<R: Read + Seek>(fd: &mut R) -> io::Result<Header> {
            use byteorder::{LittleEndian,BigEndian,ReadBytesExt};
            let mut elf_header = Header::default();

            elf_header.e_ident = [0; SIZEOF_IDENT];
            try!(fd.seek(Start(0)));
            try!(fd.read(&mut elf_header.e_ident));

            match elf_header.e_ident[EI_DATA] {
                ELFDATA2LSB => {
                    elf_header.e_type = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_machine = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_version = try!(fd.read_u32::<LittleEndian>());
                    elf_header.e_entry = try!(fd.read_u32::<LittleEndian>());
                    elf_header.e_phoff = try!(fd.read_u32::<LittleEndian>());
                    elf_header.e_shoff = try!(fd.read_u32::<LittleEndian>());
                    elf_header.e_flags = try!(fd.read_u32::<LittleEndian>());
                    elf_header.e_ehsize = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_phentsize = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_phnum = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_shentsize = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_shnum = try!(fd.read_u16::<LittleEndian>());
                    elf_header.e_shstrndx = try!(fd.read_u16::<LittleEndian>());
                    Ok(elf_header)
                },
                ELFDATA2MSB => {
                    elf_header.e_type = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_machine = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_version = try!(fd.read_u32::<BigEndian>());
                    elf_header.e_entry = try!(fd.read_u32::<BigEndian>());
                    elf_header.e_phoff = try!(fd.read_u32::<BigEndian>());
                    elf_header.e_shoff = try!(fd.read_u32::<BigEndian>());
                    elf_header.e_flags = try!(fd.read_u32::<BigEndian>());
                    elf_header.e_ehsize = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_phentsize = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_phnum = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_shentsize = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_shnum = try!(fd.read_u16::<BigEndian>());
                    elf_header.e_shstrndx = try!(fd.read_u16::<BigEndian>());
                    Ok(elf_header)
                },
                d => io_error!("Invalid ELF DATA type {:x}", d),
            }
        }
    });

elf_header_test_peek!(ELFCLASS);