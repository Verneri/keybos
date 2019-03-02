use super::registers::Register;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct GPIORegisters {
    pub function_select_0: Register<u32>,
    pub function_select_1: Register<u32>,
    pub function_select_2: Register<u32>,
    pub function_select_3: Register<u32>,
    pub function_select_4: Register<u32>,
    pub function_select_5: Register<u32>,
    __reserved_0:u32,
    pub pin_output_set_0: Register<u32>,
    pub pin_output_set_1: Register<u32>,
    __reserved_1:u32,
    pub pin_output_clear_0: Register<u32>,
    pub pin_output_clear_1: Register<u32>,
    __reserved_3:u32,
    pub pin_level_0: Register<u32>,
    pub pin_level_1: Register<u32>,
    __reserved_4:u32,
    pub pin_event_detect_status_0: Register<u32>,
    pub pin_event_detect_status_1: Register<u32>,
    __reserved_5:u32,
    pub pin_rising_edge_detect_enable_0 : Register<u32>,
    pub pin_rising_edge_detect_enable_1 : Register<u32>,
    __reserved_6:u32,
    pub pin_falling_edge_detect_enable_0 : Register<u32>,
    pub pin_falling_edge_detect_enable_1 : Register<u32>,
    __reserved_7:u32,
    pub pin_high_detect_enable_0 : Register<u32>,
    pub pin_high_detect_enable_1 : Register<u32>,
    __reserved_8:u32,
    pub pin_low_detect_enable_0 : Register<u32>,
    pub pin_low_detect_enable_1 : Register<u32>,
    __reserved_9:u32,
    pub pin_async_rising_edge_detect_enable_0 : Register<u32>,
    pub pin_async_rising_edge_detect_enable_1 : Register<u32>,
    __reserved_10:u32,
    pub pin_async_falling_edge_detect_enable_0 : Register<u32>,
    pub pin_async_falling_edge_detect_enable_1 : Register<u32>,
    __reserved_11:u32,
    pub pin_pull_up_down_enable : Register<u32>,
    pub pin_pull_up_down_enable_clock_0 : Register<u32>,
    pub pin_pull_up_down_enable_clock_1 : Register<u32>,
    __reserved_12:[u32;4],
    pub test: Register<u32>
}
#[repr(u8)]
pub enum PinFunction {
    ALT0=0b100,
    ALT1=0b101,
    ALT2=0b110,
    ALT3=0b111,
    ALT4=0b011,
    ALT5=0b010,
    INPUT=0b000,
    OUTPUT=0b001
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum GPIOPin {
    PIN0,
    PIN1,
    PIN2,
    PIN3,
    PIN4,
    PIN5,
    PIN6,
    PIN7,
    PIN8,
    PIN9,
    PIN10,
    PIN11,
    PIN12,
    PIN13,
    PIN14,
    PIN15,
    PIN16,
    PIN17,
    PIN18,
    PIN19,
    PIN20,
    PIN21,
    PIN22,
    PIN23,
    PIN24,
    PIN25,
    PIN26,
    PIN27,
    PIN28,
    PIN29,
    PIN30,
    PIN31,
    PIN32,
    PIN33,
    PIN34,
    PIN35,
    PIN36,
    PIN37,
    PIN38,
    PIN39,
    PIN40,
    PIN41,
    PIN42,
    PIN43,
    PIN44,
    PIN45,
    PIN46,
    PIN47,
    PIN48,
    PIN49,
    PIN50,
    PIN51,
    PIN52,
    PIN53
}

pub enum PinPullUpDown {
    DISABLED = 0,
    PullUp = 1,
    PullDown = 2
}


impl GPIORegisters {



    pub fn set_function(&mut self,p:GPIOPin,alt:PinFunction) {

        let register = match p {
            ref y if (GPIOPin::PIN0..GPIOPin::PIN9).contains(y) => &mut (self.function_select_0),
            ref y if (GPIOPin::PIN10..GPIOPin::PIN19).contains(y) => &mut self.function_select_1,
            ref y if (GPIOPin::PIN20..GPIOPin::PIN29).contains(y) => &mut self.function_select_2,
            ref y if (GPIOPin::PIN30..GPIOPin::PIN39).contains(y) => &mut self.function_select_3,
            ref y if (GPIOPin::PIN40..GPIOPin::PIN49).contains(y) => &mut self.function_select_4,
            ref y if (GPIOPin::PIN50..GPIOPin::PIN53).contains(y) => &mut self.function_select_5,
            _ => panic!("unknown pin")
        };
        GPIORegisters::_fnsetter(alt as u8, (p as u8 % 10), register)

    }

    fn _fnsetter(setval: u8, fnum: u8, register: &mut Register<u32>) -> () {
        let mut cur = register.get();
        let bit_start = fnum * 3;
        cur &= !(0b111 << bit_start);
        cur |= (setval as u32) << (bit_start as u32);
        register.put(cur)
    }




    pub fn pull_up_down(&mut self, pu: PinPullUpDown, pins:&[u8]) {
        use super::registers::delay;
        let pinbyte= pins.iter().fold(0,|acc,x| acc | (1<<x) as u32);
        self.pin_pull_up_down_enable.put(pu as u32);
        delay(150);
        self.pin_pull_up_down_enable_clock_0.put(pinbyte);
        delay(150);
        self.pin_pull_up_down_enable_clock_0.put(0);
    }
}

pub struct GPIO(u64);

impl Deref for GPIO {
    type Target = GPIORegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *mut GPIORegisters) }
    }
}

impl DerefMut for GPIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0 as *mut GPIORegisters) }
    }
}
impl GPIO {

    pub fn new(address:u64) -> GPIO {
        GPIO(address)
    }
}

#[cfg(test)]
mod tests {
    use super::{GPIORegisters, GPIO};

    fn create_GPIO() -> GPIO {
        let size = core::mem::size_of::<GPIORegisters>();
        let mut uninit = core::mem::MaybeUninit::<GPIORegisters>::uninitialized();
        let ptr = uninit.as_mut_ptr();
        for i in 0..size {
            let iptr = ptr as *mut u8;
            unsafe {
                iptr.offset(i as isize).write(0);
            }
        }
        GPIO(ptr as u64)

    }

    #[test]
    fn set_function() {
        use super::{PinFunction, GPIOPin};
        let mut gpio = create_GPIO();
        gpio.set_function(GPIOPin::PIN15,PinFunction::ALT5);
        assert_eq!(gpio.function_select_1.get()&(0b111<<15),0b010<<15)
    }
}