use super::registers::{Register, delay};
use super::gpio::GPIO;
use core::ops::DerefMut;
use core::ops::Deref;


#[repr(C)]
pub struct AuxRegisters {
    irq: Register<u32>,
    enables: Register<u32>
}

struct Auxiliary(u64);

impl Deref for Auxiliary {
    type Target = AuxRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *mut AuxRegisters) }
    }
}

impl DerefMut for Auxiliary {
    fn deref_mut(&mut self) -> &mut Self::Target  {
        unsafe { &mut *(self.0 as *mut AuxRegisters) }
    }
}

struct LineStatusRegister(u32);

impl LineStatusRegister {

    fn transmitter_idle(&self) -> bool {
        self.get()&0x40!=0
    }

    fn get(&self) -> u32 {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    fn transmitter_empty(&self) -> bool {
        self.get()&0x20!=0
    }

    fn receiver_overrun(&mut self) -> bool {
        self.get()&0x01!=0
    }

}

struct IODataRegister(u32);

impl IODataRegister {

    fn get(&self) -> u32 {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    fn put(&mut self,value:u32) {
        unsafe {core::ptr::write_volatile(&mut self.0,value)}
    }

    fn write_byte(&mut self, value:u8) {
        self.put(value as u32)
    }

    fn read_byte(&self) -> u8 {
        (self.get() & 0xFF) as u8
    }
}



#[repr(C)]
pub struct MiniUartRegisters {
    io_data: IODataRegister,                                    // 0x40 - Mini Uart I/O Data
    interrupt_enable: Register<u32>,                           // 0x44 - Mini Uart Interrupt Enable
    interrupt_identify: Register<u32>,   // 0x48
    line_control : Register<u32>,        // 0x4C
    modem_control : Register<u32>,                             // 0x50
    line_status: LineStatusRegister,           // 0x54
    modem_status: Register<u32>,                               // 0x58
    scratch: Register<u32>,                                    // 0x5C
    extra_control: Register<u32>,       // 0x60
    extra_status : Register<u32>,                               // 0x64
    baudrate: Register<u32>             // 0x68
}


struct MiniUart(u64);

impl core::ops::Deref for MiniUart {
    type Target = MiniUartRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *mut MiniUartRegisters) }
    }
}


impl DerefMut for MiniUart {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0 as *mut MiniUartRegisters) }
    }
}

pub struct Uart {
    aux:Auxiliary,
    mu: MiniUart,
    gpio: GPIO
}

impl Uart {
    pub fn new (pbase:u64) -> Uart {
        let aux = Auxiliary(pbase+0x00215000);
        let mu = MiniUart(pbase+0x00215040);
        let gpio = GPIO::new(pbase+0x00200000);

        Uart{aux,mu,gpio}
    }

    fn calc_baudrate(baudrate:u32) -> u32 {
        (250000000/(8*baudrate))-1
    }

    pub fn init(&mut self, baudrate: u32) {

        let mut selector:u32;



        use super::gpio::{GPIOPin, PinFunction, PinPullUpDown};

        self.gpio.set_function(GPIOPin::PIN14,PinFunction::ALT5);
        self.gpio.set_function(GPIOPin::PIN15,PinFunction::ALT5);


        self.gpio.pull_up_down(PinPullUpDown::DISABLED,&[14,15]);

        self.aux.enables.put(1);                   //Enable mini uart (this also enables access to it registers)
        self.mu.interrupt_enable.put(0);                //Disable receive and transmit interrupts
        self.mu.line_control.put(3);                //Enable 8 bit mode
        self.mu.modem_control.put(0);                //Set RTS line to be always high
        self.mu.baudrate.put(Uart::calc_baudrate(baudrate));             //Set baud rate to 115200

        self.mu.extra_control.put(3);               //Finally, enable transmitter and receiver
    }

    pub fn send (&mut self,  b:u8 ) {
        loop {
            if self.mu.line_status.transmitter_empty() {
                break;
            }

        }
        self.mu.io_data.write_byte(b);
    }

    pub fn recv (&mut self) -> u8 {
        loop {
            if self.mu.line_status.receiver_overrun() {
                break;
            }
        }
        self.mu.io_data.read_byte()
    }

    pub fn send_string(&mut self,str:&str) {
        for b in str.bytes() {
            self.send(b);
        }

    }

}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for b in s.bytes() {
            self.send(b)
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn calc_baudrate() {
        assert_eq!(super::Uart::calc_baudrate(115200), 270)
    }
}
