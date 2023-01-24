use anyhow::{bail as yeet, Context, Result};
use joycon::{JoyCon, Report};
use joycon_sys::input::WhichController;
use std::thread::{self, JoinHandle};
use uinput::event::keyboard::Key;

use crate::{
    buttons::{get_button_events, Button, ButtonAction},
    security_nightmare,
};

pub struct DeviceController {
    name: String,
    joycon: JoyCon,
    previous_report: Option<Report>,
    rodent: uinput::Device,
}

impl DeviceController {
    fn on_report(&mut self, report: Report) -> anyhow::Result<()> {
        if let Some(previous_report) = &self.previous_report {
            let events = get_button_events(&previous_report.buttons, &report.buttons);

            for event in events {
                if let Some(uinput_event) = get_uinput_event_from_button(&event.button) {
                    self.log(&format!(
                        "{:?} {}",
                        uinput_event,
                        if event.action == ButtonAction::Press {
                            "pressed"
                        } else {
                            "released"
                        }
                    ));

                    self.press_or_release_event(uinput_event, &event.action)
                        .context("failed to send keypress")?;
                }
            }
        }

        self.previous_report = Some(report);

        Ok(())
    }

    fn press_or_release_event<T>(&mut self, btn: T, action: &ButtonAction) -> Result<()>
    where
        T: Into<uinput::Event>,
    {
        match action {
            ButtonAction::Press => self.press_btn(btn),
            ButtonAction::Release => self.release_btn(btn),
        }
    }

    fn press_btn<T>(&mut self, btn: T) -> Result<()>
    where
        T: Into<uinput::Event>,
    {
        self.rodent.send(btn, 1)?;
        self.rodent.synchronize()?;
        Ok(())
    }

    fn release_btn<T>(&mut self, btn: T) -> Result<()>
    where
        T: Into<uinput::Event>,
    {
        self.rodent.send(btn, 0)?;
        self.rodent.synchronize()?;
        Ok(())
    }

    pub fn new(mut joycon: JoyCon) -> Result<Self> {
        let name = get_joycon_name(&mut joycon)?;

        let rodent =
            security_nightmare::make_rodent(&name).context("failed to create uinput device")?;

        Ok(Self {
            name: name.to_string(),
            joycon,
            rodent,
            previous_report: None,
        })
    }

    pub fn run(mut self) -> JoinHandle<Result<()>> {
        println!("Running device: {:?}", self.joycon.get_dev_info().unwrap());
        thread::spawn(move || loop {
            let report = self.joycon.tick().context("failed to get report")?;
            self.on_report(report).context("failed to process report")?;
        })
    }

    fn log(&self, msg: &str) {
        println!("{}: {}", self.name, msg);
    }
}

fn get_joycon_name(joycon: &mut JoyCon) -> Result<String> {
    let info = joycon.get_dev_info().context("failed to get JoyCon info")?;

    let name = if info.which_controller == WhichController::LeftJoyCon {
        format!("{} (Left)", info.mac_address)
    } else if info.which_controller == WhichController::RightJoyCon {
        format!("{} (Right)", info.mac_address)
    } else if info.which_controller == WhichController::ProController {
        yeet!("Pro Controller is not supported.");
    } else {
        unreachable!("stinky");
    };

    Ok(name)
}

fn get_uinput_event_from_button(button: &Button) -> Option<uinput::Event> {
    match button {
        Button::A => Some(Key::Right),
        Button::Right => Some(Key::Right),

        Button::B => Some(Key::Down),
        Button::Down => Some(Key::Down),

        Button::X => Some(Key::Up),
        Button::Up => Some(Key::Up),

        Button::Y => Some(Key::Left),
        Button::Left => Some(Key::Left),

        Button::ZR => Some(Key::Right),
        Button::ZL => Some(Key::Right),

        _ => None,
    }
    .map(|key| key.into())
}
