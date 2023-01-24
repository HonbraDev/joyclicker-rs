use anyhow::Result;
use uinput::event::{controller::Mouse, Keyboard};

pub fn make_rodent(name: &str) -> Result<uinput::Device> {
    let mouse = uinput::default()?
        .name(name)?
        .event(Mouse::Left)?
        .event(Mouse::Right)?
        // .event(Mouse::Middle)?
        // .event(Mouse::Side)?
        // .event(Mouse::Extra)?
        // .event(relative::Position::X)?
        // .event(relative::Position::Y)?
        // .event(relative::Wheel::Vertical)?
        // .event(relative::Wheel::Horizontal)?
        .event(Keyboard::All)?
        .create()?;

    Ok(mouse)
}
