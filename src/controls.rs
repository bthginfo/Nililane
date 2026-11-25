use bevy::prelude::*;

pub const A: u16 = 1;
pub const B: u16 = 1 << 1;
pub const X: u16 = 1 << 2;
pub const Y: u16 = 1 << 3;
pub const START: u16 = 1 << 4;
pub const LEFT: u16 = 1 << 5;
pub const RIGHT: u16 = 1 << 6;
pub const UP: u16 = 1 << 7;
pub const DOWN: u16 = 1 << 8;
pub const LB: u16 = 1 << 9;
pub const RB: u16 = 1 << 10;
pub const R3: u16 = 1 << 11;
pub const VIEW: u16 = 1 << 12;
pub const L3: u16 = 1 << 13;

#[derive(Default)]
pub struct Pad {
    pub held: u16,
    pub pressed: u16,
    pub left: Vec2,
    pub zoom: f32,
    pub pan: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn vector(self) -> Vec2 {
        match self {
            Self::Left => Vec2::NEG_X,
            Self::Right => Vec2::X,
            Self::Up => Vec2::NEG_Y,
            Self::Down => Vec2::Y,
        }
    }
}

/// The input device the player touched last, so hints can match it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Device {
    #[default]
    Keys,
    Pad,
    Pointer,
}

#[derive(Resource, Default)]
pub struct Controls {
    pub pressed: u16,
    pub device: Device,
    sensed: bool,
    pub direction: Option<Direction>,
    pub zoom: f32,
    pub pan: Vec2,
    pub walk: Vec2,
    repeating: Option<Direction>,
    repeat_in: f32,
}

impl Controls {
    pub fn got(&self, button: u16) -> bool {
        self.pressed & button != 0
    }
}

#[allow(clippy::too_many_arguments)]
pub fn poll(
    mut controls: ResMut<Controls>,
    keyboard: Res<ButtonInput<KeyCode>>,
    pads: Query<&Gamepad>,
    time: Res<Time<Real>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut cursor: EventReader<bevy::window::CursorMoved>,
) {
    let mut input = Pad::default();
    let mut pad_used = false;
    for (key, bit) in [
        (KeyCode::Enter, A),
        (KeyCode::Space, A),
        (KeyCode::Escape, B),
        (KeyCode::KeyX, X),
        (KeyCode::KeyY, Y),
        (KeyCode::KeyP, START),
        (KeyCode::ArrowUp, UP),
        (KeyCode::ArrowDown, DOWN),
        (KeyCode::ArrowLeft, LEFT),
        (KeyCode::ArrowRight, RIGHT),
        (KeyCode::KeyQ, LB),
        (KeyCode::KeyE, RB),
        (KeyCode::KeyN, R3),
        (KeyCode::KeyF, VIEW),
        (KeyCode::KeyL, L3),
        (KeyCode::KeyW, UP),
        (KeyCode::KeyS, DOWN),
        (KeyCode::KeyA, LEFT),
        (KeyCode::KeyD, RIGHT),
    ] {
        if keyboard.pressed(key) {
            input.held |= bit;
        }
        if keyboard.just_pressed(key) {
            input.pressed |= bit;
        }
    }
    input.zoom =
        f32::from(keyboard.pressed(KeyCode::Equal)) - f32::from(keyboard.pressed(KeyCode::Minus));

    for pad in &pads {
        for (button, bit) in [
            (GamepadButton::South, A),
            (GamepadButton::East, B),
            (GamepadButton::West, X),
            (GamepadButton::North, Y),
            (GamepadButton::Start, START),
            (GamepadButton::LeftTrigger, LB),
            (GamepadButton::RightTrigger, RB),
            (GamepadButton::RightThumb, R3),
            (GamepadButton::LeftThumb, L3),
            (GamepadButton::Select, VIEW),
            (GamepadButton::DPadUp, UP),
            (GamepadButton::DPadDown, DOWN),
            (GamepadButton::DPadLeft, LEFT),
            (GamepadButton::DPadRight, RIGHT),
        ] {
            if pad.pressed(button) {
                input.held |= bit;
            }
            if pad.just_pressed(button) {
                input.pressed |= bit;
            }
        }
        pad_used |= pad.get_just_pressed().next().is_some()
            || pad.left_stick().length_squared() > 0.25
            || pad.right_stick().length_squared() > 0.25;
        input.left += pad.left_stick();
        input.pan += pad.right_stick();
        input.zoom += pad.get(GamepadAxis::RightZ).unwrap_or(0.0)
            - pad.get(GamepadAxis::LeftZ).unwrap_or(0.0);
    }

    #[cfg(target_os = "android")]
    {
        let pad = android::poll();
        pad_used |= android::pad_used();
        input.held |= pad.held;
        input.pressed |= pad.pressed;
        input.left += pad.left;
        input.pan += pad.pan;
        input.zoom += pad.zoom;
    }

    let moved: f32 = cursor
        .read()
        .filter_map(|event| event.delta)
        .map(Vec2::length)
        .sum();
    let pointer =
        mouse.get_just_pressed().next().is_some() || touches.any_just_pressed() || moved > 12.0;
    let keys = keyboard.get_just_pressed().next().is_some();
    if !controls.sensed {
        controls.sensed = true;
        controls.device = if pads.iter().next().is_some() {
            Device::Pad
        } else if cfg!(target_os = "android") {
            Device::Pointer
        } else {
            Device::Keys
        };
    }
    let device = if pad_used {
        Some(Device::Pad)
    } else if keys {
        Some(Device::Keys)
    } else if pointer {
        Some(Device::Pointer)
    } else {
        None
    };
    if let Some(device) = device
        && controls.device != device
    {
        controls.device = device;
    }
    controls.pressed = input.pressed;
    controls.zoom = input.zoom.clamp(-1.0, 1.0);
    controls.pan = input.pan.clamp_length_max(1.0);
    let axis = |negative, positive| {
        f32::from(input.held & positive != 0) - f32::from(input.held & negative != 0)
    };
    let digital = Vec2::new(axis(LEFT, RIGHT), axis(UP, DOWN));
    let stick = Vec2::new(input.left.x, -input.left.y);
    let magnitude = ((stick.length() - 0.16) / 0.84).clamp(0.0, 1.0);
    controls.walk = (digital + stick.normalize_or_zero() * magnitude).clamp_length_max(1.0);
    let direction = [
        (UP, Direction::Up),
        (DOWN, Direction::Down),
        (LEFT, Direction::Left),
        (RIGHT, Direction::Right),
    ]
    .into_iter()
    .find(|(bit, _)| (input.held | input.pressed) & bit != 0)
    .map(|(_, d)| d)
    .or_else(|| stick_direction(input.left));
    controls.direction = None;
    if direction != controls.repeating {
        controls.direction = direction;
        controls.repeating = direction;
        controls.repeat_in = 0.34;
    } else if direction.is_some() {
        controls.repeat_in -= time.delta_secs();
        if controls.repeat_in <= 0.0 {
            controls.direction = direction;
            controls.repeat_in = 0.14;
        }
    }
}

fn stick_direction(stick: Vec2) -> Option<Direction> {
    if stick.length_squared() < 0.48f32.powi(2) {
        return None;
    }
    if stick.x.abs() > stick.y.abs() {
        Some(if stick.x < 0.0 {
            Direction::Left
        } else {
            Direction::Right
        })
    } else {
        Some(if stick.y < 0.0 {
            Direction::Down
        } else {
            Direction::Up
        })
    }
}

pub fn phone_edition() -> bool {
    #[cfg(target_os = "android")]
    return android::phone_edition();
    #[cfg(not(target_os = "android"))]
    false
}

pub fn insets() -> [u16; 4] {
    #[cfg(target_os = "android")]
    return android::insets();
    #[cfg(not(target_os = "android"))]
    [0; 4]
}

#[cfg(target_os = "android")]
mod android {
    use std::{
        ffi::c_void,
        sync::{
            Mutex,
            atomic::{AtomicBool, Ordering},
        },
    };

    use ndk_sys::*;

    use super::*;

    struct NativePad {
        id: Option<i32>,
        keys: u16,
        hat: u16,
        pending: u16,
        left: Vec2,
        pan: Vec2,
        zoom: f32,
    }

    impl NativePad {
        const EMPTY: Self = Self {
            id: None,
            keys: 0,
            hat: 0,
            pending: 0,
            left: Vec2::ZERO,
            pan: Vec2::ZERO,
            zoom: 0.0,
        };
    }

    static PAD: Mutex<NativePad> = Mutex::new(NativePad::EMPTY);
    static INSETS: Mutex<[u16; 4]> = Mutex::new([0; 4]);
    static PHONE_EDITION: AtomicBool = AtomicBool::new(false);
    static PAD_USED: AtomicBool = AtomicBool::new(false);

    /// Whether a controller sent input since the last frame (the system back
    /// gesture alone does not count).
    pub fn pad_used() -> bool {
        PAD_USED.swap(false, Ordering::Relaxed)
    }

    pub fn phone_edition() -> bool {
        PHONE_EDITION.load(Ordering::Relaxed)
    }

    #[unsafe(no_mangle)]
    extern "system" fn Java_game_nililane_GameActivity_publishPhoneEdition(
        _env: *mut c_void,
        _class: *mut c_void,
        phone: u8,
    ) {
        PHONE_EDITION.store(phone != 0, Ordering::Relaxed);
    }

    pub fn poll() -> Pad {
        let mut pad = PAD.lock().unwrap_or_else(|error| error.into_inner());
        Pad {
            held: pad.keys | pad.hat,
            pressed: std::mem::take(&mut pad.pending),
            left: pad.left,
            pan: pad.pan,
            zoom: pad.zoom,
        }
    }

    pub fn insets() -> [u16; 4] {
        *INSETS.lock().unwrap_or_else(|error| error.into_inner())
    }

    fn key_bit(code: u32) -> Option<u16> {
        Some(match code {
            AKEYCODE_BUTTON_A => A,
            AKEYCODE_BUTTON_B | AKEYCODE_BACK => B,
            AKEYCODE_BUTTON_X => X,
            AKEYCODE_BUTTON_Y => Y,
            AKEYCODE_BUTTON_START => START,
            AKEYCODE_DPAD_LEFT => LEFT,
            AKEYCODE_DPAD_RIGHT => RIGHT,
            AKEYCODE_DPAD_UP => UP,
            AKEYCODE_DPAD_DOWN => DOWN,
            AKEYCODE_BUTTON_L1 => LB,
            AKEYCODE_BUTTON_R1 => RB,
            AKEYCODE_BUTTON_THUMBR => R3,
            AKEYCODE_BUTTON_THUMBL => L3,
            AKEYCODE_BUTTON_SELECT => VIEW,
            _ => return None,
        })
    }

    fn controller_event(event: *const AInputEvent) -> bool {
        // NativeActivity owns the event until AInputQueue_finishEvent returns it.
        let (source, kind, id) = unsafe {
            (
                AInputEvent_getSource(event) as u32,
                AInputEvent_getType(event) as u32,
                AInputEvent_getDeviceId(event),
            )
        };
        let is_source = |candidate| source & candidate == candidate;
        let mut pad = PAD.lock().unwrap_or_else(|error| error.into_inner());
        if kind == AINPUT_EVENT_TYPE_KEY {
            let code = unsafe { AKeyEvent_getKeyCode(event) } as u32;
            let gamepad = is_source(AINPUT_SOURCE_GAMEPAD) || is_source(AINPUT_SOURCE_DPAD);
            if !gamepad && code != AKEYCODE_BACK {
                return false;
            }
            let Some(bit) = key_bit(code) else {
                return false;
            };
            let down = unsafe { AKeyEvent_getAction(event) } as u32 == AKEY_EVENT_ACTION_DOWN;
            if gamepad && down {
                PAD_USED.store(true, Ordering::Relaxed);
            }
            if gamepad && pad.id != Some(id) {
                *pad = NativePad {
                    id: Some(id),
                    ..NativePad::EMPTY
                };
            }
            if down && pad.keys & bit == 0 {
                pad.pending |= bit;
            }
            if down {
                pad.keys |= bit;
            } else {
                pad.keys &= !bit;
            }
            return true;
        }
        if kind != AINPUT_EVENT_TYPE_MOTION || !is_source(AINPUT_SOURCE_JOYSTICK) {
            return false;
        }
        if pad.id != Some(id) {
            *pad = NativePad {
                id: Some(id),
                ..NativePad::EMPTY
            };
        }
        let axis =
            |axis| unsafe { AMotionEvent_getAxisValue(event, axis as i32, 0) }.clamp(-1.0, 1.0);
        pad.left = Vec2::new(axis(AMOTION_EVENT_AXIS_X), -axis(AMOTION_EVENT_AXIS_Y));
        let right = Vec2::new(axis(AMOTION_EVENT_AXIS_Z), -axis(AMOTION_EVENT_AXIS_RZ));
        pad.pan = if right.length_squared() > 0.025 {
            right
        } else {
            Vec2::new(axis(AMOTION_EVENT_AXIS_RX), -axis(AMOTION_EVENT_AXIS_RY))
        };
        pad.zoom = axis(AMOTION_EVENT_AXIS_RTRIGGER).max(axis(AMOTION_EVENT_AXIS_GAS))
            - axis(AMOTION_EVENT_AXIS_LTRIGGER).max(axis(AMOTION_EVENT_AXIS_BRAKE));
        if pad.left.length_squared() > 0.25 || pad.pan.length_squared() > 0.25 {
            PAD_USED.store(true, Ordering::Relaxed);
        }
        let x = axis(AMOTION_EVENT_AXIS_HAT_X);
        let y = axis(AMOTION_EVENT_AXIS_HAT_Y);
        let hat = u16::from(x < -0.5) * LEFT
            | u16::from(x > 0.5) * RIGHT
            | u16::from(y < -0.5) * UP
            | u16::from(y > 0.5) * DOWN;
        if hat & !pad.hat != 0 {
            PAD_USED.store(true, Ordering::Relaxed);
        }
        pad.pending |= hat & !pad.hat;
        pad.hat = hat;
        true
    }

    unsafe extern "C" {
        fn __real_AInputQueue_getEvent(
            queue: *mut AInputQueue,
            event: *mut *mut AInputEvent,
        ) -> i32;
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn __wrap_AInputQueue_getEvent(
        queue: *mut AInputQueue,
        event: *mut *mut AInputEvent,
    ) -> i32 {
        loop {
            let result = unsafe { __real_AInputQueue_getEvent(queue, event) };
            if result < 0 || !controller_event(unsafe { *event }) {
                return result;
            }
            unsafe { AInputQueue_finishEvent(queue, *event, 1) };
        }
    }

    #[unsafe(no_mangle)]
    extern "system" fn Java_game_nililane_GameActivity_resetController(
        _env: *mut c_void,
        _class: *mut c_void,
        id: i32,
    ) {
        let mut pad = PAD.lock().unwrap_or_else(|error| error.into_inner());
        if id < 0 || pad.id == Some(id) {
            *pad = NativePad::EMPTY;
        }
    }

    #[unsafe(no_mangle)]
    extern "system" fn Java_game_nililane_GameActivity_publishInsets(
        _env: *mut c_void,
        _class: *mut c_void,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) {
        *INSETS.lock().unwrap_or_else(|error| error.into_inner()) =
            [left, top, right, bottom].map(|n| n.clamp(0, u16::MAX as i32) as u16);
    }
}
