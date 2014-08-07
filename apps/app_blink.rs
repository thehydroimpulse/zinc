#![feature(phase)]
#![crate_type="staticlib"]
#![no_std]

extern crate core;
extern crate zinc;
#[phase(plugin)] extern crate macro_platformtree;

use zinc::hal::lpc17xx::pin::Pin;
use zinc::drivers::chario::CharIO;
use core::option::Some;
use zinc::drivers::mma7361::MMA7361;

platformtree!(
  lpc17xx@mcu {
    clock {
      source = "main-oscillator";
      source_frequency = 12_000_000;
      pll {
        m = 50;
        n = 3;
        divisor = 4;
      }
    }

    timer {
      timer@1 {
        counter = 25;
        divisor = 4;
      }
    }

    uart {
      uart@0 {
        baud_rate = 115200;
        mode = "8N1";
        tx = &uart_tx;
        rx = &uart_rx;
      }
    }

    gpio {
      0 {
        uart_tx@2;
        uart_rx@3;
      }
      1 {
        led1@18 { direction = "out"; }
        led2@20 { direction = "out"; }
        led3@21 { direction = "out"; }
      }
    }
  }

  os {
    single_task {
      loop = "run";
      args {
        timer = &timer;
        led1 = &led1;
        led2 = &led2;
        led3 = &led3;
        uart = &uart;
      }
    }
  }
)

#[no_split_stack]
fn run(args: &pt::run_args) {
    use zinc::hal::pin::{GPIO, High, Low};
    use zinc::hal::lpc17xx::pin::{Pin, Port0, AltFunction1, GPIO};
    use zinc::hal::timer::Timer;
    use zinc::hal::pin::{In, Out};
    use zinc::hal::lpc17xx::adc::ADC;

    args.uart.puts("Starting...");
    args.uart.puts("\n\r");


    let adc = ADC::new();
    let x   = (0, 0);
    let y   = (0, 1);
    let z   = (0, 2);
    let acc = MMA7361::new(adc, x, y, z);

    loop {
        let x = acc.read(x);
        args.uart.puts("X (mvolts): ");
        // args.uart.puti(x);
        args.uart.puts("\n\r");
        args.uart.puts(" G Force: ");
        args.uart.puti(x / 800);
        args.uart.puts("\n\r");
    }
}
