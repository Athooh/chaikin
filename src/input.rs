use macroquad::prelude::*;
use crate::animation::AnimationState;

pub enum InputAction {
    AddPoint(f32, f32),
    StartAnimation,
    ClearPoints,
    ReturnToDrawing,
    Quit,
}

pub struct InputHandler {
    // Add any input state here if needed
}

impl InputHandler {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn process_input(&mut self, animation: &AnimationState) -> Option<InputAction> {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Escape) {
            if animation.is_active() {
                return Some(InputAction::ReturnToDrawing);
            } else {
                return Some(InputAction::Quit);
            }
        }
        
        if is_key_pressed(KeyCode::C) {
            return Some(InputAction::ClearPoints);
        }
        
        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) && !animation.is_active() {
            let (x, y) = mouse_position();
            return Some(InputAction::AddPoint(x, y));
        }
        
        // Start animation on Enter
        if is_key_pressed(KeyCode::Enter) && !animation.is_active() {
            return Some(InputAction::StartAnimation);
        }
        
        None
    }
}