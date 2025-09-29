#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::{info, println};
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// ESP-IDF bootloader descriptor
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Config ile max CPU clock
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Heap allocator (blink için gerekli değil, ama korundu)
    esp_alloc::heap_allocator!(size: 64 * 1024);

    // LED pin'i: GPIO2 output, başlangıçta low, varsayılan OutputConfig
    // Not: Kartınıza göre GPIO2 yerine GPIO48 (örneğin, RGB LED için) kullanabilirsiniz
    let mut led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    info!("Blink basliyor!");

    loop {
        // LED'i toggle et
        // 1000 ms (1 saniye) delay
        delay(20);
        led.set_low();
        info!("LED toggled!");

        // 1000 ms (1 saniye) delay
        delay(20);
        led.set_high();
    }
}

fn delay(ms: u64) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(ms) {}
    println!("{}", delay_start.elapsed());

}
