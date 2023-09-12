use crate::engine::Point;

const FLOOR: i16 = 475;

#[derive(Copy, Clone)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

#[derive(Copy, Clone)]
pub struct Idle;

#[derive(Copy, Clone)]
pub struct Running;

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context,
            _state: Running {},
        }
    }

    pub fn frame_name(&self) -> &str {
        "Idle"
    }
}

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        "Run"
    }
}
