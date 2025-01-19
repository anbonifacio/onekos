pub fn memset(buf: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        unsafe {
            *buf.add(i) = c;
        }
    }
}
