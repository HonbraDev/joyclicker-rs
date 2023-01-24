use anyhow::Result;
use device_controller::DeviceController;
use hidapi::HidApi;
use joycon::JoyCon;
use joycon_sys::{HID_IDS, NINTENDO_VENDOR_ID};

mod device_controller;
mod security_nightmare;
mod buttons;

fn main() -> Result<()> {
    let handles = get_devices()?
        .into_iter()
        .map(|device| device.run())
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().expect("the thread should join")?;
    }

    /* mouse.send(uinput::event::controller::Mouse::Left, 1)?;
    mouse.synchronize()?; */

    Ok(())
}

pub fn get_devices() -> Result<Vec<DeviceController>> {
    let api = HidApi::new()?;

    api.device_list()
        .filter(|device_info| {
            device_info.vendor_id() == NINTENDO_VENDOR_ID
                && HID_IDS.contains(&device_info.product_id())
        })
        .map(|device_info| {
            let device = device_info.open_device(&api)?;
            let joycon = JoyCon::new(device, device_info.clone())?;
            let controller = DeviceController::new(joycon)?;
            Ok(controller)
        })
        .collect::<Result<Vec<_>>>()
}
