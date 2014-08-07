use hal::adc::ADC;

/// The driver for the MMA7361 accelerometer component.
/// This is a 3-axis analog accelerometer, so we need
/// to hold 3 different pins for each axis.
pub struct MMA7361<A> {
    adc: A,
    x: (uint, uint),
    y: (uint, uint),
    z: (uint, uint)
}

#[deriving(PartialEq)]
pub enum Axis {
    XAxis,
    YAxis,
    ZAxis
}

impl<A: ADC> MMA7361<A> {

    pub fn new(adc: A, x: (uint, uint), y: (uint, uint), z: (uint, uint)) -> MMA7361<A> {
        adc.init();

        MMA7361 {
            adc: adc,
            x: x,
            y: y,
            z: z
        }
    }

    pub fn read(&self, axis: (uint, uint)) -> u32 {
        self.adc.convert(axis)
    }
}
