use std::{io::Write};
use memmap2::{MmapMut};
    
fn main() {
    let buf: [u8;8] = [
        // LINUX
        // 0x48, 0x89, 0xf8,// mov rax, rdi
        // WINDOWS
        0x48, 0x89, 0xc8, // mov rax,rcx
        0x48, 0x83, 0xc0, 0x02, // add rax,0x2
        0xc3 // ret
    ];
    let mut mmap = MmapMut::map_anon(buf.len()).unwrap();
    (&mut mmap[..]).write(&buf).unwrap();
    let wr_mmap = mmap.make_exec().unwrap();
    let ptr = wr_mmap.as_ptr();
    let func: fn(i32) -> i32 = unsafe { std::mem::transmute::<*const u8, fn(i32) -> i32>(ptr) };
    println!("result = {}", func(2));
}
