pub struct Device {
    base: usize,
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
        unsafe { write_volatile(addr.offset(2), 0b1) };
        // Enable receiver buffer interrupts.
        unsafe { write_volatile(addr.offset(1), 0b1) };
        // Return a new, initialised UART device.
        Device { base }
    }

    pub fn put(&mut self, character: u8) {
        let ptr = self.base as *mut u8;
        // UNSAFE: fine as long as self.base is valid
        unsafe {
            core::ptr::write_volatile(ptr, character);
        }
    }
}
