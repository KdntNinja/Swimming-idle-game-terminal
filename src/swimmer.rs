pub struct Swimmer {
    pub name: String,            // Name of the swimmer
    pub progress: usize,         // Total progress made by the swimmer
    pub speed: f64, // Speed of the swimmer (changed from usize to f64 for fractional speeds)
    pub lengths: i32, // Number of lengths completed by the swimmer
    pub upgrade_cost: usize, // Cost (in lengths) to upgrade the swimmer's speed
    pub position: usize, // Position in the current lane (0-100)
    pub direction: bool, // true = right, false = left
    pub upgrade_multiplier: f64, // Multiplier for upgrade costs
}

impl Swimmer {
    /// Creates a new swimmer with the given name and speed
    ///
    /// # Arguments
    /// * `name` - The swimmer's name
    /// * `speed` - The swimmer's initial speed
    ///
    /// # Returns
    /// A new Swimmer instance
    pub fn new(name: &str, speed: f64) -> Self {
        Self {
            name: name.to_string(),
            progress: 0_usize,
            speed,
            lengths: 0_i32,
            upgrade_cost: 10_usize,      // Starting upgrade cost
            position: 0_usize,           // Always start at far left
            direction: true,             // Always start moving right
            upgrade_multiplier: 1.2_f64, // Reduced from 1.5 to slow progression
        }
    }

    /// Updates the swimmer's position and counts lengths
    pub fn swim(&mut self) {
        // Add to overall progress
        self.progress += self.speed.round() as usize;

        // Move the swimmer position
        if self.direction {
            // Moving right
            self.position += self.speed.round() as usize;
            if self.position >= 100_usize {
                // Reached the right end
                self.lengths += 1_i32; // Count a length

                // Set position to exactly at the right edge
                self.position = 100_usize;
                // Change direction to start swimming back left
                self.direction = false;
            }
        } else {
            // Moving left
            if self.position <= self.speed.round() as usize {
                // Reached the left end
                self.lengths += 1_i32; // Count a length

                // Set position to exactly at the left edge
                self.position = 0_usize;
                // Change direction to start swimming back right
                self.direction = true;
            } else {
                self.position -= self.speed.round() as usize;
            }
        }
    }

    /// Attempts to upgrade the swimmer's speed using lengths as currency
    ///
    /// # Returns
    /// `true` if successful, `false` if not enough lengths
    pub fn upgrade(&mut self) -> bool {
        // Check if player has enough lengths to upgrade
        if self.lengths >= self.upgrade_cost as i32 {
            self.lengths -= self.upgrade_cost as i32;
            self.speed += 0.5_f64; // Slower increase - now only 0.5 per upgrade instead of 1

            // Calculate new upgrade cost with the multiplier
            self.upgrade_cost = (self.upgrade_cost as f64 * self.upgrade_multiplier) as usize;

            // Slightly increase the multiplier each time
            self.upgrade_multiplier += 0.02_f64;

            return true;
        }
        false
    }

    /// Gets the display speed (rounded to one decimal place)
    pub fn display_speed(&self) -> String {
        format!("{:.1}", self.speed)
    }
}
