mod window;
mod input;
mod renderer;
mod chaikin;
mod animation;

use macroquad::prelude::*;
use window::Window;
use input::InputHandler;
use renderer::Renderer;
use chaikin::ChaikinAlgorithm;
use animation::AnimationState;

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {
    let _window = Window::new("Chaikin's Algorithm");
    let mut input_handler = InputHandler::new();
    let mut renderer = Renderer::new();
    let mut chaikin = ChaikinAlgorithm::new();
    let mut animation = AnimationState::new();
    
    loop {
        let dt = get_frame_time();
        
        // Handle input
        if let Some(action) = input_handler.process_input(&animation) {
            match action {
                input::InputAction::AddPoint(x, y) => {
                    chaikin.add_point(x, y);
                },
                input::InputAction::StartAnimation => {
                    if chaikin.points.len() >= 3 {
                        animation.start(&chaikin);
                    } else {
                        renderer.show_message("Need at least 3 points for animation", 2.0);
                    }
                },
                input::InputAction::ClearPoints => {
                    chaikin.clear_points();
                    animation.reset();
                },
                input::InputAction::Quit => break,
                input::InputAction::ReturnToDrawing => {
                    animation.stop();
                }
            }
        }
        
        // Update animation
        animation.update(dt, &chaikin);
        
        // Update message timer
        renderer.update(dt);
        
        // Render
        clear_background(BLACK);
        renderer.draw_points(&chaikin.points);
        
        if animation.is_active() {
            let current_points = &animation.get_current_step();
            renderer.draw_animation_step(current_points, &chaikin.points, animation.current_step, animation.total_steps());
        } else {
            renderer.draw_lines(&chaikin.points);
            renderer.draw_instructions(animation.is_active());
        }
        
        next_frame().await;
    }
}