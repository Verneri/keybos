pub struct Register<T>(T);

impl<T> Register<T> {

    pub fn put(&mut self, n:T) {
        unsafe{core::ptr::write_volatile(&mut self.0, n)}
    }

    pub fn get(&self) -> T {
        unsafe{core::ptr::read_volatile(&self.0)}
    }

}


pub fn delay(n:u64) {
    for _ in 0..n {
        unsafe {asm!("nop" :::: "volatile");}
    }
}

pub fn read_el() -> u64 {
    let reg;
    unsafe{asm!("mrs $0, CurrentEL" : "=r"(reg) ::: "volatile");}
    reg
}
