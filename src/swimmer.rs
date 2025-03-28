pub struct Swimmer {
    pub name: String,        // Name of the swimmer
    pub progress: usize,     // Total progress made by the swimmer
    pub speed: usize,        // Speed of the swimmer (affects progress and position updates)
    pub lengths: i32,        // Number of lengths completed by the swimmer
    pub upgrade_cost: usize, // Cost (in lengths) to upgrade the swimmer's speed
    pub position: usize,     // Position in the current lane (0-100)
    pub direction: bool,     // true = right, false = left
}

impl Swimmer {
    pub fn new(name: &str, speed: usize) -> Self {
        Self {
            name: name.to_string(),
            progress: 0,
            speed,
            lengths: 0,
            upgrade_cost: 10,
            position: 0,     // Always start at far left
            direction: true, // Always start moving right
        }
    }

    pub fn swim(&mut self) {
        // Add to overall progress
        self.progress += self.speed;

        // Move the swimmer position
        if self.direction {
            // Moving right
            self.position += self.speed;
            if self.position >= 100 {
                // Reached the right end
                self.lengths += 1; // Count a length

                // Reset to start from left again
                self.position = 0;
                self.direction = false; // Change direction to left
            }
        } else {
            // Moving left
            if self.position <= self.speed {
                // Reached the left end
                self.lengths += 1; // Count a length

                // Reset to start from right again
                self.position = 100;
                self.direction = true; // Change direction to right
            } else {
                self.position -= self.speed;
            }
        }
    }

    pub fn upgrade(&mut self) -> bool {
        // Check if player has enough lengths to upgrade
        if self.lengths >= self.upgrade_cost as i32 {
            self.lengths -= self.upgrade_cost as i32;
            self.speed += 1; // Increase speed
            self.upgrade_cost = (self.upgrade_cost as f64 * 1.5) as usize; // Increase cost by 50%
            return true;
        }
        false
    }
}
