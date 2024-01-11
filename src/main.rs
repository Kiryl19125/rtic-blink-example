#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [SPI1])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f1xx_hal::prelude::*;
    use stm32f1xx_hal::timer::SysDelay;
    use stm32f1xx_hal::{gpio::Output, gpio::PushPull, gpio::PC13};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        val_1: usize,
        val_2: usize,
        led: PC13<Output<PushPull>>,
        delay: SysDelay,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let rcc = ctx.device.RCC.constrain();
        let mut flash = ctx.device.FLASH.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(16.MHz())
            .freeze(&mut flash.acr);

        let mut gpioc = ctx.device.GPIOC.split();
        let delay = ctx.core.SYST.delay(&clocks);

        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        print_hello::spawn().unwrap();
        // print_hello_2::spawn().unwrap();

        return (
            Shared {},
            Local {
                val_1: 1,
                val_2: 2,
                led,
                delay,
            },
            init::Monotonics(),
        );
    }

    #[idle()]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(local = [val_1, led, delay])]
    fn print_hello(ctx: print_hello::Context) {
        loop {
            rprintln!("Hello, {}", ctx.local.val_1);
            ctx.local.led.toggle();
            ctx.local.delay.delay_ms(500_u32);
        }
    }

    #[task(local = [val_2])]
    fn print_hello_2(ctx: print_hello_2::Context) {
        let val_2 = ctx.local.val_2;
        *val_2 += 1;
        for _ in 0..10 {
            rprintln!("Hello_2, {}", val_2);
            for _ in 0..1_00_000 {}
        }
    }
}

// Example without RTIC
//
// use cortex_m_rt::entry;
// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//
//     let cp = pac::Peripherals::take().unwrap();
//     let dp = cortex_m::Peripherals::take().unwrap();
//
//     let mut flash = cp.FLASH.constrain();
//     let rcc = cp.RCC.constrain();
//
// let clocks = rcc
//     .cfgr
//     .use_hse(8.MHz())
//     .sysclk(16.MHz())
//     .freeze(&mut flash.acr);
//
//     let mut delay = dp.SYST.delay(&clocks);
//     let mut gpioc = cp.GPIOC.split();
//     let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
//
//     loop {
//         led.toggle();
//         rprintln!("Blink!");
//         delay.delay_ms(100_u16);
//     }
// }
