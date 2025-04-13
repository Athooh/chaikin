#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct ChaikinAlgorithm {
    pub points: Vec<Point>,
}

impl ChaikinAlgorithm {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
    
    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(Point { x, y });
    }
    
    pub fn clear_points(&mut self) {
        self.points.clear();
    }
    
    pub fn apply_step(&self, points: &[Point]) -> Vec<Point> {
        // Need at least 2 points to apply Chaikin's algorithm
        if points.len() < 2 {
            return points.to_vec();
        }

        let mut result = Vec::new();
        
        // For an open curve
        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];
            
            // Keep the first point
            if i == 0 {
                result.push(p0);
            }
            
            // Q (1/4 point)
            result.push(Point {
                x: 0.75 * p0.x + 0.25 * p1.x,
                y: 0.75 * p0.y + 0.25 * p1.y,
            });
            
            // R (3/4 point)
            result.push(Point {
                x: 0.25 * p0.x + 0.75 * p1.x,
                y: 0.25 * p0.y + 0.75 * p1.y,
            });
            
            // Keep the last point
            if i == points.len() - 2 {
                result.push(p1);
            }
        }
        
        result
    }
    
    pub fn generate_steps(&self, num_steps: usize) -> Vec<Vec<Point>> {
        let mut steps = Vec::new();
        steps.push(self.points.clone());
        
        let mut current_points = self.points.clone();
        for _ in 0..num_steps {
            current_points = self.apply_step(&current_points);
            steps.push(current_points.clone());
        }
        
        steps
    }
}