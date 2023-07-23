use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use esp32_nimble::{uuid128, BLEDevice, NimbleProperties};


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let ble_device = BLEDevice::take();

    let server = ble_device.get_server();
    server.on_connect(|_| {
        ::log::info!("Client connected");
        ::log::info!("Multi-connect support: start advertising");
        ble_device.get_advertising().start().unwrap();
    });

    let service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

    // A static characteristic
    let static_characteristic = service.lock().create_characteristic(
        uuid128!("00000000-dead-beef-0000-000000000000"),
        NimbleProperties::READ,
        );
    static_characteristic.lock().set_value("Hello, world!".as_bytes());


    let ble_advertising = ble_device.get_advertising();
    ble_advertising
        .name("ESP32-Luciole-demo")
        .add_service_uuid(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

    ble_advertising.start().unwrap();

    let mut counter = 0;
    info!("Hello, world!");
    loop {
        esp_idf_hal::delay::FreeRtos::delay_ms(1000);
        counter += 1;
    }
}

