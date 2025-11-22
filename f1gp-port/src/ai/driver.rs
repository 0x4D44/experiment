//! AI driver implementation
//!
//! Provides AI control for opponent cars.

use crate::ai::racing_line::RacingLineFollower;
use crate::game::input::CarInput;
use crate::physics::CarPhysics;
use glam::{Vec2, Vec3};

/// Information about a nearby car for AI decision making
#[derive(Debug, Clone)]
pub struct NearbyCarInfo {
    /// Position of the car
    pub position: Vec3,

    /// Velocity of the car
    pub velocity: Vec3,

    /// Distance to this car
    pub distance: f32,

    /// Whether this car is ahead (true) or behind (false)
    pub is_ahead: bool,
}

/// AI driver personality/skill parameters
#[derive(Debug, Clone, Copy)]
pub struct DriverPersonality {
    /// Aggression level (0.0-1.0) - affects overtaking behavior
    pub aggression: f32,

    /// Consistency (0.0-1.0) - lower value = more mistakes
    pub consistency: f32,

    /// Overall skill (0.0-1.0) - affects lap time
    pub skill: f32,

    /// Wet weather skill (0.0-1.0)
    pub wet_skill: f32,

    /// Reaction time (seconds) - delay in inputs
    pub reaction_time: f32,
}

impl DriverPersonality {
    /// Create a personality for Ayrton Senna (legendary skill)
    pub fn senna() -> Self {
        Self {
            aggression: 0.9,
            consistency: 0.95,
            skill: 1.0,
            wet_skill: 1.0,
            reaction_time: 0.05,
        }
    }

    /// Create a personality for Nigel Mansell (aggressive, skilled)
    pub fn mansell() -> Self {
        Self {
            aggression: 0.95,
            consistency: 0.85,
            skill: 0.95,
            wet_skill: 0.80,
            reaction_time: 0.06,
        }
    }

    /// Create a personality for Alain Prost (smooth, consistent)
    pub fn prost() -> Self {
        Self {
            aggression: 0.6,
            consistency: 0.98,
            skill: 0.95,
            wet_skill: 0.90,
            reaction_time: 0.07,
        }
    }

    /// Create an average AI driver
    pub fn average() -> Self {
        Self {
            aggression: 0.5,
            consistency: 0.7,
            skill: 0.7,
            wet_skill: 0.6,
            reaction_time: 0.12,
        }
    }

    /// Create a rookie AI driver
    pub fn rookie() -> Self {
        Self {
            aggression: 0.4,
            consistency: 0.5,
            skill: 0.5,
            wet_skill: 0.4,
            reaction_time: 0.15,
        }
    }
}

/// AI driver state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    /// Normal racing
    Racing,

    /// Attempting to overtake
    Overtaking,

    /// Defending position
    Defending,

    /// In pit lane
    Pitting,

    /// Recovering from spin/crash
    Recovering,
}

/// AI driver controller
pub struct AIDriver {
    /// Driver name
    pub name: String,

    /// Driver personality
    pub personality: DriverPersonality,

    /// Current AI state
    pub state: AIState,

    /// Racing line follower
    racing_line: Option<RacingLineFollower>,

    /// PID controller state for speed control
    speed_error_integral: f32,
    speed_error_prev: f32,

    /// Throttle/brake smoothing
    current_throttle: f32,
    current_brake: f32,
    current_steering: f32,

    /// Lateral offset from racing line for overtaking/defending (meters)
    lateral_offset: f32,

    /// Target lateral offset (smoothly interpolated)
    target_lateral_offset: f32,

    /// Time spent in current state (for state transitions)
    state_timer: f32,
}

impl AIDriver {
    /// Create a new AI driver
    pub fn new(name: String, personality: DriverPersonality) -> Self {
        Self {
            name,
            personality,
            state: AIState::Racing,
            racing_line: None,
            speed_error_integral: 0.0,
            speed_error_prev: 0.0,
            current_throttle: 0.0,
            current_brake: 0.0,
            current_steering: 0.0,
            lateral_offset: 0.0,
            target_lateral_offset: 0.0,
            state_timer: 0.0,
        }
    }

    /// Set the racing line follower
    pub fn set_racing_line(&mut self, racing_line: RacingLineFollower) {
        self.racing_line = Some(racing_line);
    }

    /// Update AI and compute inputs for the car
    pub fn update(
        &mut self,
        car: &CarPhysics,
        nearby_cars: &[NearbyCarInfo],
        delta_time: f32,
    ) -> CarInput {
        // If no racing line, return neutral inputs
        if self.racing_line.is_none() {
            return CarInput::default();
        }

        // Update state timer
        self.state_timer += delta_time;

        // Update AI state based on nearby cars
        self.update_ai_state(car, nearby_cars);

        // Get target point and speed from racing line
        let target_speed = self
            .racing_line
            .as_ref()
            .unwrap()
            .get_target_speed(car.body.position);

        // Apply skill modifier to target speed
        let adjusted_target_speed = target_speed * (0.7 + self.personality.skill * 0.3);

        // Adjust target speed based on state and nearby cars
        let final_target_speed = self.adjust_target_speed(adjusted_target_speed, car, nearby_cars);

        // Calculate steering using racing line with lateral offset
        let forward_3d = car.body.orientation * glam::Vec3::X;
        let car_forward = Vec2::new(forward_3d.x, forward_3d.z);

        // Smooth lateral offset transitions
        let offset_smoothness = 0.05;
        self.lateral_offset +=
            (self.target_lateral_offset - self.lateral_offset) * offset_smoothness;

        let target_steering = self.calculate_steering_with_offset(car.body.position, car_forward);

        // Smooth steering (add human-like imperfection)
        let steering_smoothness = 0.1 + self.personality.skill * 0.1;
        self.current_steering += (target_steering - self.current_steering) * steering_smoothness;

        // Add small random variation based on consistency
        let steering_noise = (1.0 - self.personality.consistency) * 0.05;
        let random_offset = (fastrand::f32() - 0.5) * steering_noise;
        self.current_steering = (self.current_steering + random_offset).clamp(-1.0, 1.0);

        // Calculate throttle and brake using PID controller
        let (throttle, brake) =
            self.calculate_speed_control(car.speed, final_target_speed, delta_time);

        // Smooth throttle/brake inputs
        let input_smoothness = 0.15;
        self.current_throttle += (throttle - self.current_throttle) * input_smoothness;
        self.current_brake += (brake - self.current_brake) * input_smoothness;

        CarInput {
            throttle: self.current_throttle,
            brake: self.current_brake,
            steering: self.current_steering,
            shift_up: false, // TODO: Implement automatic shifting
            shift_down: false,
        }
    }

    /// Update AI state based on nearby cars
    fn update_ai_state(&mut self, car: &CarPhysics, nearby_cars: &[NearbyCarInfo]) {
        // Find closest car ahead and behind
        let car_ahead = nearby_cars
            .iter()
            .filter(|c| c.is_ahead && c.distance < 50.0)
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        let car_behind = nearby_cars
            .iter()
            .filter(|c| !c.is_ahead && c.distance < 30.0)
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        // State machine logic
        match self.state {
            AIState::Racing => {
                // Check if we should start overtaking
                if let Some(ahead) = car_ahead {
                    if ahead.distance < 20.0 {
                        // Car is close ahead, check if we're faster
                        let ahead_speed = ahead.velocity.length();
                        if car.speed > ahead_speed * 1.05 {
                            // We're significantly faster, attempt overtake
                            if fastrand::f32() < self.personality.aggression {
                                self.state = AIState::Overtaking;
                                self.state_timer = 0.0;
                                // Choose overtaking side (prefer inside of next corner)
                                self.target_lateral_offset =
                                    if fastrand::bool() { 4.0 } else { -4.0 };
                            }
                        }
                    }
                }

                // Check if we should start defending
                if let Some(behind) = car_behind {
                    if behind.distance < 15.0 {
                        let behind_speed = behind.velocity.length();
                        if behind_speed > car.speed * 1.05 {
                            // Car behind is faster, defend if aggressive
                            if fastrand::f32() < self.personality.aggression * 0.7 {
                                self.state = AIState::Defending;
                                self.state_timer = 0.0;
                                // Block racing line
                                self.target_lateral_offset = 0.0;
                            }
                        }
                    }
                }
            }

            AIState::Overtaking => {
                // Check if overtake is complete
                if car_ahead.is_none() || car_ahead.map(|c| c.distance).unwrap_or(100.0) > 25.0 {
                    // Overtake complete, return to racing line
                    self.state = AIState::Racing;
                    self.target_lateral_offset = 0.0;
                } else if self.state_timer > 5.0 {
                    // Overtake taking too long, give up
                    self.state = AIState::Racing;
                    self.target_lateral_offset = 0.0;
                }
            }

            AIState::Defending => {
                // Check if we're still being chased
                if car_behind.is_none() || car_behind.map(|c| c.distance).unwrap_or(100.0) > 25.0 {
                    // No longer being pressured
                    self.state = AIState::Racing;
                    self.target_lateral_offset = 0.0;
                } else if self.state_timer > 3.0 {
                    // Defend for limited time, then race normally
                    self.state = AIState::Racing;
                    self.target_lateral_offset = 0.0;
                }
            }

            AIState::Recovering => {
                // Return to racing after some time
                if self.state_timer > 2.0 && car.on_track {
                    self.state = AIState::Racing;
                    self.target_lateral_offset = 0.0;
                }
            }

            AIState::Pitting => {
                // TODO: Implement pit lane logic
            }
        }
    }

    /// Adjust target speed based on nearby cars (collision avoidance)
    fn adjust_target_speed(
        &self,
        base_speed: f32,
        car: &CarPhysics,
        nearby_cars: &[NearbyCarInfo],
    ) -> f32 {
        let mut speed = base_speed;

        // Find closest car directly ahead (within 30 degrees)
        let forward_3d = car.body.orientation * glam::Vec3::X;
        let car_forward = Vec2::new(forward_3d.x, forward_3d.z).normalize();

        for nearby in nearby_cars {
            if !nearby.is_ahead {
                continue;
            }

            // Check if car is in front of us
            let to_car = nearby.position - car.body.position;
            let to_car_2d = Vec2::new(to_car.x, to_car.z);
            let dot = to_car_2d.normalize().dot(car_forward);

            // Only consider cars ahead of us (dot > 0.866 = ~30 degrees)
            if dot > 0.866 && nearby.distance < 25.0 {
                // Reduce speed based on distance
                let speed_reduction = (1.0 - (nearby.distance / 25.0)) * 0.5;
                let nearby_speed = nearby.velocity.length();
                speed = speed.min(nearby_speed - speed_reduction * 10.0);
            }
        }

        // In overtaking mode, push harder
        if self.state == AIState::Overtaking {
            speed *= 1.05;
        }

        speed.max(5.0) // Never go below 5 m/s
    }

    /// Calculate steering with lateral offset from racing line
    fn calculate_steering_with_offset(&self, position: Vec3, car_forward: Vec2) -> f32 {
        // Get base steering from racing line
        let base_steering = if let Some(ref racing_line) = self.racing_line {
            racing_line.calculate_steering(position, car_forward)
        } else {
            0.0
        };

        // Apply lateral offset (simplified - just add to steering)
        // Positive offset = right, negative = left
        let offset_steering = self.lateral_offset * 0.1;

        (base_steering + offset_steering).clamp(-1.0, 1.0)
    }

    /// Calculate throttle and brake using PID controller
    fn calculate_speed_control(
        &mut self,
        current_speed: f32,
        target_speed: f32,
        delta_time: f32,
    ) -> (f32, f32) {
        let speed_error = target_speed - current_speed;

        // PID constants (tuned for smooth control)
        let kp = 0.05; // Proportional gain
        let ki = 0.01; // Integral gain
        let kd = 0.02; // Derivative gain

        // Update integral
        self.speed_error_integral += speed_error * delta_time;
        self.speed_error_integral = self.speed_error_integral.clamp(-10.0, 10.0); // Anti-windup

        // Calculate derivative
        let speed_error_derivative = (speed_error - self.speed_error_prev) / delta_time.max(0.001);
        self.speed_error_prev = speed_error;

        // PID output
        let control_output =
            kp * speed_error + ki * self.speed_error_integral + kd * speed_error_derivative;

        // Convert to throttle/brake
        if control_output > 0.0 {
            // Need to accelerate
            let throttle = control_output.clamp(0.0, 1.0);
            (throttle, 0.0)
        } else {
            // Need to brake
            let brake = (-control_output).clamp(0.0, 1.0);
            (0.0, brake)
        }
    }

    /// Reset PID controller state
    pub fn reset_controller(&mut self) {
        self.speed_error_integral = 0.0;
        self.speed_error_prev = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_creation() {
        let senna = DriverPersonality::senna();
        assert_eq!(senna.skill, 1.0);
        assert!(senna.aggression > 0.8);

        let rookie = DriverPersonality::rookie();
        assert!(rookie.skill < 0.6);
        assert!(rookie.consistency < 0.6);
    }

    #[test]
    fn test_ai_driver_creation() {
        let personality = DriverPersonality::average();
        let driver = AIDriver::new("Test Driver".to_string(), personality);

        assert_eq!(driver.name, "Test Driver");
        assert_eq!(driver.state, AIState::Racing);
        assert!(driver.racing_line.is_none());
    }

    #[test]
    fn test_speed_control() {
        let personality = DriverPersonality::average();
        let mut driver = AIDriver::new("Test".to_string(), personality);

        // Test acceleration (current speed < target)
        let (throttle, brake) = driver.calculate_speed_control(30.0, 50.0, 0.016);
        assert!(throttle > 0.0);
        assert_eq!(brake, 0.0);

        // Test braking (current speed > target)
        let (throttle, brake) = driver.calculate_speed_control(70.0, 50.0, 0.016);
        assert_eq!(throttle, 0.0);
        assert!(brake > 0.0);
    }

    #[test]
    fn test_ai_state_transitions() {
        let personality = DriverPersonality::average();
        let mut driver = AIDriver::new("Test".to_string(), personality);

        assert_eq!(driver.state, AIState::Racing);

        driver.state = AIState::Overtaking;
        assert_eq!(driver.state, AIState::Overtaking);

        driver.state = AIState::Defending;
        assert_eq!(driver.state, AIState::Defending);
    }

    #[test]
    fn test_reset_controller() {
        let personality = DriverPersonality::average();
        let mut driver = AIDriver::new("Test".to_string(), personality);

        // Accumulate some state
        driver.calculate_speed_control(30.0, 50.0, 0.016);
        driver.calculate_speed_control(35.0, 50.0, 0.016);

        driver.reset_controller();

        assert_eq!(driver.speed_error_integral, 0.0);
        assert_eq!(driver.speed_error_prev, 0.0);
    }
}
