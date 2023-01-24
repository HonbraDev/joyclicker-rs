use joycon_sys::input::ButtonsStatus;

#[derive(Debug, PartialEq, Eq)]
pub struct ButtonEvent {
    pub button: Button,
    pub action: ButtonAction,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    A,
    B,
    X,
    Y,
    Up,
    Down,
    Left,
    Right,
    L,
    ZL,
    R,
    ZR,
    SL,
    SR,
    L3,
    R3,
    Minus,
    Plus,
    Capture,
    Home,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonAction {
    Press,
    Release,
}

pub fn get_button_events(s1: &ButtonsStatus, s2: &ButtonsStatus) -> Vec<ButtonEvent> {
    let events = vec![
        perhaps_make_event(Button::A, s1.right.a(), s2.right.a()),
        perhaps_make_event(Button::B, s1.right.b(), s2.right.b()),
        perhaps_make_event(Button::X, s1.right.x(), s2.right.x()),
        perhaps_make_event(Button::Y, s1.right.y(), s2.right.y()),
        perhaps_make_event(Button::Up, s1.left.up(), s2.left.up()),
        perhaps_make_event(Button::Down, s1.left.down(), s2.left.down()),
        perhaps_make_event(Button::Left, s1.left.left(), s2.left.left()),
        perhaps_make_event(Button::Right, s1.left.right(), s2.left.right()),
        perhaps_make_event(Button::L, s1.left.l(), s2.left.l()),
        perhaps_make_event(Button::ZL, s1.left.zl(), s2.left.zl()),
        perhaps_make_event(Button::R, s1.right.r(), s2.right.r()),
        perhaps_make_event(Button::ZR, s1.right.zr(), s2.right.zr()),
        perhaps_make_event(
            Button::SL,
            s1.left.sl() || s1.right.sl(),
            s2.left.sl() || s2.right.sl(),
        ),
        perhaps_make_event(
            Button::SR,
            s1.left.sr() || s1.right.sr(),
            s2.left.sr() || s2.right.sr(),
        ),
        perhaps_make_event(Button::L3, s1.middle.lstick(), s2.middle.lstick()),
        perhaps_make_event(Button::R3, s1.middle.rstick(), s2.middle.rstick()),
        perhaps_make_event(Button::Minus, s1.middle.minus(), s2.middle.minus()),
        perhaps_make_event(Button::Plus, s1.middle.plus(), s2.middle.plus()),
        perhaps_make_event(Button::Capture, s1.middle.capture(), s2.middle.capture()),
        perhaps_make_event(Button::Home, s1.middle.home(), s2.middle.home()),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<_>>();

    events
}

fn perhaps_make_event(b: Button, b1: bool, b2: bool) -> Option<ButtonEvent> {
    if b1 != b2 {
        Some(ButtonEvent {
            button: b,
            action: if b2 {
                ButtonAction::Press
            } else {
                ButtonAction::Release
            },
        })
    } else {
        None
    }
}
