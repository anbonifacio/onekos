use core::arch::asm;

fn sbi_call(
    mut arg0: u32,
    mut arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    fid: u32,
    eid: u32,
) -> Result<u32, u32> {
    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0 => arg0,
            inout("a1") arg1 => arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
        );
    }

    if arg0 == 0 {
        Ok(arg1)
    } else {
        Err(arg0)
    }
}

pub fn putchar(ch: char) -> Result<u32, u32> {
    sbi_call(ch as u32, 0, 0, 0, 0, 0, 0, 1)
}
