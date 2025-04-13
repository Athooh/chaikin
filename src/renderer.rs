use macroquad::prelude::*;
use crate::chaikin::Point;

const POINT_RADIUS: f32 = 5.0;

pub struct Renderer {
    message: String,
    message_timer: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            message_timer: 0.0,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer -= dt;
        }
    }
    
    pub fn show_message(&mut self, message: &str, duration: f32) {
        self.message = message.to_string();
        self.message_timer = duration;
    }
    
    pub fn draw_points(&self, points: &[Point]) {
        for point in points {
            draw_circle(point.x, point.y, POINT_RADIUS, RED);
        }
    }
    
    pub fn draw_lines(&self, points: &[Point]) {
        if points.len() >= 2 {
            for i in 0..points.len() - 1 {
                let p1 = points[i];
                let p2 = points[i + 1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
            }
        }
    }
    
    pub fn draw_animation_step(&self, points: &[Point], original_points: &[Point], step: usize, total_steps: usize) {
        // Draw the original points
        for point in original_points {
            draw_circle(point.x, point.y, POINT_RADIUS, RED);
        }
        
        // Draw the animation points
        if points.len() >= 2 {
            for i in 0..points.len() - 1 {
                let p1 = points[i];
                let p2 = points[i + 1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, GREEN);
            }
        }
        
        // Draw step counter
        draw_text(
            &format!("Step: {}/{}", step, total_steps),
            10.0, 30.0, 20.0, LIGHTGRAY
        );
        
        draw_text("ESC: Return to drawing mode", 10.0, screen_height() - 10.0, 20.0, LIGHTGRAY);
    }
    
    pub fn draw_instructions(&self, is_animating: bool) {
        // Show instructions based on current state
        if !is_animating {
            draw_text(
                "Left-click: Add points | Enter: Start animation | C: Clear | ESC: Quit", 
                10.0, screen_height() - 10.0, 20.0, LIGHTGRAY
            );
        }
        
        // Show any active messages
        if self.message_timer > 0.0 {
            let text_size = measure_text(&self.message, None, 24, 1.0);
            draw_text(
                &self.message,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0,
                24.0,
                YELLOW,
            );
        }
    }
}