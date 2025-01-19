use core::arch::asm;

pub fn sbi_call(
    mut arg0: usize,
    mut arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    fid: usize,
    eid: usize,
) -> Result<usize, isize> {
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
        Err(arg0 as isize)
    }
}
