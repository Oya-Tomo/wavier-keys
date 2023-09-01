#![no_main]
#![no_std]

use core::{convert::Infallible, u8};

use cortex_m::{asm, prelude::*};
use cortex_m_rt::entry;

use defmt;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::ExtU32;
use keycodes::{KeyCodes, ModifierMasks};
use layout::{
    FN_KEY_POS, KEY_LAYOUT, KEY_LAYOUT_WITH_FN, LEFT_ALT, LEFT_CTRL, LEFT_GUI, LEFT_SHIFT,
    RIGHT_ALT, RIGHT_CTRL, RIGHT_GUI, RIGHT_SHIFT,
};
use panic_probe as _;

use rp_pico::{self, hal::usb::UsbBus};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::{
        HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
    },
};

mod keycodes;
mod layout;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

pub type Column<'a> = &'a dyn InputPin<Error = Infallible>;
pub type Row<'a> = &'a mut dyn OutputPin<Error = Infallible>;
pub type MatrixState = [[bool; 15]; 5];

#[entry]
fn main() -> ! {
    let mut dp = rp_pico::hal::pac::Peripherals::take().unwrap();
    let mut watchdog = rp_pico::hal::Watchdog::new(dp.WATCHDOG);
    let clocks = rp_pico::hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        dp.XOSC,
        dp.CLOCKS,
        dp.PLL_SYS,
        dp.PLL_USB,
        &mut dp.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let timer = rp_pico::hal::Timer::new(dp.TIMER, &mut dp.RESETS);

    let bus = UsbBus::new(
        dp.USBCTRL_REGS,
        dp.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut dp.RESETS,
    );
    let bus_allocator = UsbBusAllocator::new(bus);

    let mut hid = HIDClass::new_with_settings(
        &bus_allocator,
        KeyboardReport::desc(),
        5,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::NotSupported,
        },
    );

    let mut usb_dev = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2718, 0x2818))
        .manufacturer("Oya-Tomo")
        .product("Wavier-Keys")
        .serial_number("2023.9.1.001")
        .build();

    let sio = rp_pico::hal::Sio::new(dp.SIO);
    let pins = rp_pico::Pins::new(dp.IO_BANK0, dp.PADS_BANK0, sio.gpio_bank0, &mut dp.RESETS);

    let col0 = pins.gpio0.into_pull_up_input();
    let col1 = pins.gpio1.into_pull_up_input();
    let col2 = pins.gpio2.into_pull_up_input();
    let col3 = pins.gpio3.into_pull_up_input();
    let col4 = pins.gpio4.into_pull_up_input();
    let col5 = pins.gpio5.into_pull_up_input();
    let col6 = pins.gpio6.into_pull_up_input();
    let col7 = pins.gpio7.into_pull_up_input();
    let col8 = pins.gpio8.into_pull_up_input();
    let col9 = pins.gpio9.into_pull_up_input();
    let col10 = pins.gpio10.into_pull_up_input();
    let col11 = pins.gpio11.into_pull_up_input();
    let col12 = pins.gpio12.into_pull_up_input();
    let col13 = pins.gpio13.into_pull_up_input();
    let col14 = pins.gpio14.into_pull_up_input();

    let mut row0 = pins.gpio20.into_push_pull_output();
    let mut row1 = pins.gpio19.into_push_pull_output();
    let mut row2 = pins.gpio18.into_push_pull_output();
    let mut row3 = pins.gpio17.into_push_pull_output();
    let mut row4 = pins.gpio16.into_push_pull_output();

    let cols: &[Column] = &[
        &col0, &col1, &col2, &col3, &col4, &col5, &col6, &col7, &col8, &col9, &col10, &col11,
        &col12, &col13, &col14,
    ];
    let rows: &mut [Row] = &mut [&mut row0, &mut row1, &mut row2, &mut row3, &mut row4];

    let mut countdown = timer.count_down();
    countdown.start(5.millis());

    loop {
        usb_dev.poll(&mut [&mut hid]);
        if countdown.wait().is_ok() {
            let mtx = scan_key_switch(cols, rows);
            let report = build_report(mtx);
            hid.push_input(&report).ok();
        }
        hid.pull_raw_output(&mut [0; 64]).ok();
    }
}

fn scan_key_switch(cols: &[Column], rows: &mut [Row]) -> MatrixState {
    let mut state: MatrixState = [[false; 15]; 5];
    for row in 0..rows.len() {
        rows[row].set_low().unwrap();
        asm::delay(10);
        for col in 0..cols.len() {
            state[row][col] = cols[col].is_low().unwrap();
        }
        rows[row].set_high().unwrap();
        asm::delay(10);
    }
    return state;
}

fn build_report(state: MatrixState) -> KeyboardReport {
    let mut modif = 0;
    let mut key_codes: [u8; 6] = [0; 6];
    let mut count = 0;

    if state[FN_KEY_POS[0]][FN_KEY_POS[1]] {
        for row in 0..state.len() {
            for col in 0..state[0].len() {
                if state[row][col] && KEY_LAYOUT_WITH_FN[row][col] != KeyCodes::Reserved {
                    if count < 6 {
                        key_codes[count] = KEY_LAYOUT_WITH_FN[row][col] as u8;
                        count += 1;
                    } else {
                        key_codes = [KeyCodes::ErrOvf as u8; 6];
                        break;
                    }
                }
            }
        }
    } else {
        for row in 0..state.len() {
            for col in 0..state[0].len() {
                if state[row][col] && KEY_LAYOUT[row][col] != KeyCodes::Reserved {
                    if count < 6 {
                        key_codes[count] = KEY_LAYOUT[row][col] as u8;
                        count += 1;
                    } else {
                        key_codes = [KeyCodes::ErrOvf as u8; 6];
                        break;
                    }
                }
            }
        }
    }

    if state[LEFT_CTRL[0]][LEFT_CTRL[1]] {
        modif |= ModifierMasks::LeftCtrl as u8;
    }
    if state[LEFT_SHIFT[0]][LEFT_SHIFT[1]] {
        modif |= ModifierMasks::LeftShift as u8;
    }
    if state[LEFT_ALT[0]][LEFT_ALT[1]] {
        modif |= ModifierMasks::LeftAlt as u8;
    }
    if state[LEFT_GUI[0]][LEFT_GUI[1]] {
        modif |= ModifierMasks::LeftGui as u8;
    }

    if state[RIGHT_CTRL[0]][RIGHT_CTRL[1]] {
        modif |= ModifierMasks::RightCtrl as u8;
    }
    if state[RIGHT_SHIFT[0]][RIGHT_SHIFT[1]] {
        modif |= ModifierMasks::RightShift as u8;
    }
    if state[RIGHT_ALT[0]][RIGHT_ALT[1]] {
        modif |= ModifierMasks::RightAlt as u8;
    }
    if state[RIGHT_GUI[0]][RIGHT_GUI[1]] {
        modif |= ModifierMasks::RightGui as u8;
    }

    KeyboardReport {
        modifier: modif,
        reserved: 0,
        leds: 0,
        keycodes: key_codes,
    }
}
