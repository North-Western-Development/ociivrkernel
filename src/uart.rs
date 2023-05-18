use spinning_top::Spinlock;
use uart_16550::MmioSerialPort;

pub static CONSOLE: Spinlock<Option<Device>> = Spinlock::new(None);

pub struct Device {
    base: usize,
    serial_port: MmioSerialPort,
}

impl Device {
    /// Create a new UART device.
    /// # Safety
    /// `base` must be the base address of a UART device.
    pub unsafe fn new(base: usize) -> Self {
        use core::ptr::write_volatile;
        let addr = base as *mut u8;
        // Set data size to 8 bits.
        unsafe { write_volatile(addr.offset(3), 0b11) };
        // Enable FIFO.
        unsafe { write_volatile(addr.offset(2), 0b11) };
        // Enable receiver buffer interrupts.
        unsafe { write_volatile(addr.offset(1), 0b1) };
        // Return a new, initialised UART device.
        Device {
            base,
            serial_port: MmioSerialPort::new(base),
        }
    }

    pub fn init(&mut self) {
        self.serial_port.init();
    }
    pub fn put(&mut self, character: u8) {
        self.serial_port.send(character);
    }
}

impl core::fmt::Write for Device {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.put(c);
        }
        Ok(()) // there are never errors writing to UART :)
    }
}

pub unsafe fn init_console(base: usize) {
    let mut console = CONSOLE.lock();
    *console = Some(unsafe { Device::new(base) });
    console.as_mut().unwrap().init();
}

/// Prints a formatted string to the [CONSOLE].
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::uart::print_fmt(format_args!("{}", format_args!($($arg)*)));
    });
}

/// println prints a formatted string to the [CONSOLE] with a trailing newline character.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::uart::print_fmt(format_args!("{}\n", format_args!($($arg)*))));
}

pub fn print_fmt(args: core::fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE
        .lock()
        .as_mut()
        .map(|writer| writer.write_fmt(args).unwrap());
}
