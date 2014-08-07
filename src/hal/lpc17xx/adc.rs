use hal::adc;

#[path="../../lib/ioreg.rs"] mod ioreg;

pub struct ADC {
    initialized: bool
}

impl ADC {
    pub fn new() -> ADC {
        ADC {
            initialized: false
        }
    }
}

impl adc::ADC for ADC {
    fn init(&self) {
        // Flip the 12th bit on the PCONP register to enable the ADC.
        // TODO: Move this into the lpc1768 adc implementation.
        // reg::PCONP.set_value(reg::PCONP.value() | (1 << 12));
        // Set the AD0CR to 1, set the clock divider to
        // /4/2 and enable the 21st bit.
        // reg::AD0CR.set_value(1 | (1 << 8) | (1 << 21));
        reg::PINSEL1.set_value(reg::PINSEL1.value() & !(0x3 << 14) | (0x1 << 14));
    }

    fn convert(&self, pin: (uint, uint)) -> u32 {
        // Start the ADC conversion **now**, so set the 24th bit to 1.
        reg::AD0CR.set_value(reg::AD0CR.value() | (1 << 24));

        // Loop until the DONE bit (31) is set to 1.
        while (reg::ADDR0.value() & (1 << 31)) == 0 {}

        // Disable the ADC conversion for the x axis.
        //reg::AD0CR.set_value(reg::AD0CR.value() | 0);
        (((reg::ADDR0.value() >> 4) & 0xfff) as f32 * 0.80566f32) as u32
    }
}


mod reg {
  use lib::volatile_cell::VolatileCell;

  ioreg!(PINSEL: u32, value)
  reg_rw!(PINSEL, u32, value, set_value, value)

  ioreg!(PCONP: u32, value)
  reg_rw!(PCONP, u32, value, set_value, value)

  ioreg!(AD0CR: u32, value)
  reg_rw!(AD0CR, u32, value, set_value, value)

  ioreg!(PCLKSEL0: u32, value)
  reg_rw!(PCLKSEL0, u32, value, set_value, value)

  ioreg!(ADDR0: u32, value)
  reg_rw!(ADDR0, u32, value, set_value, value)

  extern {
    #[link_name="lpc17xx_iomem_PINSEL0"]  pub static PINSEL0:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL1"]  pub static PINSEL1:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL2"]  pub static PINSEL2:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL3"]  pub static PINSEL3:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL4"]  pub static PINSEL4:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL7"]  pub static PINSEL7:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL9"]  pub static PINSEL9:  PINSEL;
    #[link_name="lpc17xx_iomem_PINSEL10"] pub static PINSEL10: PINSEL;
    #[link_name="lpc17xx_iomem_ADDR0"] pub static ADDR0: ADDR0;
    #[link_name="lpc17xx_iomem_AD0CR"] pub static AD0CR: AD0CR;
    #[link_name="lpc17xx_iomem_PCLKSEL0"] pub static PCLKSEL0: PCLKSEL0;
    #[link_name="lpc17xx_iomem_PCONP"] pub static PCONP: PCONP;
  }
}
