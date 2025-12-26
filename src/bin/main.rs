#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb565};
use esp_alloc as _;
use esp_backtrace as _;

#[cfg(target_arch = "riscv32")]
use esp_hal::interrupt::software::SoftwareInterruptControl;

use esp_hal::timer::timg::TimerGroup;
use esp_hal::{
    gpio::{Output, OutputConfig},
    spi::master::Spi,
    time::Rate,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    #[cfg(target_arch = "riscv32")]
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(
        timg0.timer0,
        #[cfg(target_arch = "riscv32")]
        sw_int.software_interrupt0,
    );

    println!("Initializing display...");

    let out_config = OutputConfig::default();

    let sck = peripherals.GPIO36;
    let mosi = peripherals.GPIO35;
    let cs = Output::new(peripherals.GPIO37, esp_hal::gpio::Level::Low, out_config);
    let dc = Output::new(peripherals.GPIO34, esp_hal::gpio::Level::Low, out_config);
    let mut rst = Output::new(peripherals.GPIO33, esp_hal::gpio::Level::High, out_config);
    let mut bl = Output::new(peripherals.GPIO38, esp_hal::gpio::Level::High, out_config);

    let spi_conf = esp_hal::spi::master::Config::default()
        .with_frequency(Rate::from_mhz(60))
        .with_mode(esp_hal::spi::Mode::_0);

    let spi = Spi::new(peripherals.SPI2, spi_conf)
        .unwrap()
        .with_sck(sck)
        .with_mosi(mosi);

    rst.set_low();
    Timer::after(Duration::from_millis(10)).await;
    rst.set_high();
    Timer::after(Duration::from_millis(120)).await;

    let di = display_interface_spi::SPIInterface::new(
        embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap(),
        dc,
    );

    let mut display = mipidsi::Builder::new(mipidsi::models::ST7789, di)
        .display_size(135, 240)
        .display_offset(52, 40)
        .orientation(mipidsi::options::Orientation::new().rotate(mipidsi::options::Rotation::Deg90))
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
        .init(&mut embassy_time::Delay)
        .unwrap();

    bl.set_high();

    println!("Display initialized!");

    display.clear(Rgb565::BLACK).unwrap();

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new(
        "Hello\nThis is Rust on\nCardputer!",
        Point { x: 40, y: 60 },
        text_style,
    )
    .draw(&mut display)
    .unwrap();

    println!("Miau displayed!");

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
