use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

#[derive(Default, Debug)]
pub struct PlayerInput {
    pub up:    ButtonState,
    pub down:  ButtonState,
    pub left:  ButtonState,
    pub right: ButtonState,
}

impl PlayerInput {
    pub fn clear(&mut self) {
        self.up.release();
        self.down.release();
        self.left.release();
        self.right.release();
    }

    pub fn update(&mut self, input: &Input<KeyCode>) {
        self.clear();
        self.up.set_from(&input, KeyCode::W);
        self.up.set_from(&input, KeyCode::Up);
        self.left.set_from(&input, KeyCode::A);
        self.left.set_from(&input, KeyCode::Left);
        self.down.set_from(&input, KeyCode::S);
        self.down.set_from(&input, KeyCode::Down);
        self.right.set_from(&input, KeyCode::D);
        self.right.set_from(&input, KeyCode::Right);
    }

}

#[derive(Default, Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub held:    bool,
}

impl ButtonState {
    fn release(&mut self) {
        self.pressed = false;
        self.held = false;
    }

    fn set_from(&mut self, input: &Input<KeyCode>, key: KeyCode) {
        if input.just_pressed(key) {
            self.pressed = true;
            self.held = true;
        }

        if input.pressed(key) {
            self.held = true;
        }
    }
}

