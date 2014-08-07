/// A generic ADC trait to define the common capabilities across most boards. ADC,
/// analog-to-digital-converter is a central piece to read in analog signals in
/// terms of volts (or millivolts) from external components.
///
/// The ADC isn't always on by default and thus must be explicitly initialized. The
/// `init` method is always called before any conversions must be done.
pub trait ADC {

    /// Initialize the ADC component according to the specific board. For example, the lpc1768
    /// is defaulted with the ADC being completely shutdown. It requires a few steps until conversions
    /// can take place.
    fn init(&self);

    /// Take a reading from the analog component connected to a single pin and
    /// start the ADC conversions on it. This will return the raw readings from
    /// the ADC conversion process. On the lpc1768, you get values from 0 to 4096 (2^12)
    /// and thus 0 represents 0v (volts) and 4096 represents 3.3v. This may not be the case
    /// for all boards, however.
    fn convert(&self, pin: (uint, uint)) -> u32;

    /// Take a normal reading from the ADC but convert the result to volts.
    fn volts(&mut self, pin: (uint, uint)) -> f32 {
        self.millivolts(pin) as f32 / 1000f32
    }

    /// Take a normal reading from the ADC but convert the result to **millivolts**.
    fn millivolts(&mut self, pin: (uint, uint)) -> u32 {
        ((self.convert(pin) as f32) * 0.806) as u32
    }
}
