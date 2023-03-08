struct Points {
    x: Vec<f32>,
    y: Vec<f32>
}
pub struct Sim {
    bounds: Boundaries,
    distance: Points,
    velocity: Points,
    acceleration: Points,
}
pub struct Boundaries {
    width: f32,
    height: f32,  
}

pub fn new_boundary(height: f32, width: f32)-> Boundaries {
    Boundaries { width, height }
}

pub fn setup(bounds: Boundaries, capacity: usize) -> Sim {
    Sim {
        bounds,
        distance: Points {
            x: vec![0.0; capacity],
            y: vec![0.0; capacity],
        },
        velocity: Points { x: vec![1.0; capacity], y: vec![0.0; capacity] },
        acceleration: Points { x: vec![0.05; capacity], y: vec![0.2; capacity]},
    }
}

impl Sim {
    pub fn run(&mut self) {
        let run_sim = true;
        while run_sim {
            for i in 0..self.distance.x.len() {
                self.check_bounds(i);
                self.increment_distance(i);
                self.increment_velocity(i);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn increment_distance(&mut self, i: usize) {
        self.distance.x[i] += self.velocity.x[i];
        self.distance.y[i] += self.velocity.y[i];
    }

    fn increment_velocity(&mut self, i: usize) {
        self.velocity.x[i] += self.acceleration.x[i];
        self.velocity.y[i] += self.acceleration.y[i];
    }

    fn check_bounds(&mut self, i: usize) {
        if self.distance.x[i] >= self.bounds.width && self.distance.x[i] <= 0.0 {
            self.acceleration.x[i] = -self.acceleration.x[i]
        }
        if self.distance.y[i] >= self.bounds.height && self.distance.y[i] <= 0.0 {
            self.acceleration.y[i] = -self.acceleration.y[i]
        }
    }
}