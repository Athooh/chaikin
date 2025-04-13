use crate::chaikin::{ChaikinAlgorithm, Point};

const ANIMATION_SPEED: f32 = 0.5; // seconds per step
const MAX_STEPS: usize = 7;

pub struct AnimationState {
    active: bool,
    animation_steps: Vec<Vec<Point>>,
    pub current_step: usize,
    timer: f32,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            active: false,
            animation_steps: Vec::new(),
            current_step: 0,
            timer: 0.0,
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn start(&mut self, chaikin: &ChaikinAlgorithm) {
        // Don't start animation if not enough points
        if chaikin.points.len() < 3 {
            return;
        }
        
        // Generate all steps of the animation
        self.animation_steps = chaikin.generate_steps(MAX_STEPS);
        self.active = true;
        self.current_step = 0;
        self.timer = 0.0;
    }
    
    pub fn stop(&mut self) {
        self.active = false;
    }
    
    pub fn reset(&mut self) {
        self.active = false;
        self.animation_steps.clear();
        self.current_step = 0;
        self.timer = 0.0;
    }
    
    pub fn update(&mut self, dt: f32, chaikin: &ChaikinAlgorithm) {
        if !self.active {
            return;
        }
        
        // Handle special cases
        if chaikin.points.len() <= 2 {
            self.active = false;
            return;
        }
        
        // Update animation timer
        self.timer += dt;
        if self.timer >= ANIMATION_SPEED {
            self.timer = 0.0;
            self.current_step = (self.current_step + 1) % self.animation_steps.len();
        }
    }
    
    pub fn get_current_step(&self) -> Vec<Point> {
        if self.active && !self.animation_steps.is_empty() {
            self.animation_steps[self.current_step].clone()
        } else {
            Vec::new()
        }
    }
    
    pub fn total_steps(&self) -> usize {
        if self.animation_steps.is_empty() {
            0
        } else {
            self.animation_steps.len() - 1
        }
    }
}