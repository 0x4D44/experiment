//! Car physics implementation
//!
//! Provides specialized physics for F1 cars including tire forces, aerodynamics, and engine.

use super::engine::{BodyId, PhysicsBody};
use crate::data::car::CarSpec;
use glam::Vec3;

/// Tire grip levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TireGrip {
    /// Front left tire grip (0.0-1.0)
    pub front_left: f32,
    /// Front right tire grip (0.0-1.0)
    pub front_right: f32,
    /// Rear left tire grip (0.0-1.0)
    pub rear_left: f32,
    /// Rear right tire grip (0.0-1.0)
    pub rear_right: f32,
}

impl TireGrip {
    /// Perfect grip on all tires
    pub fn perfect() -> Self {
        Self {
            front_left: 1.0,
            front_right: 1.0,
            rear_left: 1.0,
            rear_right: 1.0,
        }
    }

    /// No grip (on ice)
    pub fn none() -> Self {
        Self {
            front_left: 0.0,
            front_right: 0.0,
            rear_left: 0.0,
            rear_right: 0.0,
        }
    }

    /// Average grip across all tires
    pub fn average(&self) -> f32 {
        (self.front_left + self.front_right + self.rear_left + self.rear_right) / 4.0
    }
}

/// Car physics state extending the basic physics body
#[derive(Debug, Clone)]
pub struct CarPhysics {
    /// Underlying physics body
    pub body: PhysicsBody,

    /// Car specification (performance characteristics)
    pub spec: CarSpec,

    /// Current engine RPM
    pub engine_rpm: f32,

    /// Current gear (0 = reverse, 1-6 = forward gears)
    pub gear: i8,

    /// Throttle input (0.0-1.0)
    pub throttle: f32,

    /// Brake input (0.0-1.0)
    pub brake: f32,

    /// Steering input (-1.0 to 1.0, left to right)
    pub steering: f32,

    /// Wheel speeds (rad/s) [FL, FR, RL, RR]
    pub wheel_speeds: [f32; 4],

    /// Tire temperatures (°C) [FL, FR, RL, RR]
    pub tire_temps: [f32; 4],

    /// Tire grip levels
    pub tire_grip: TireGrip,

    /// Is car on track surface?
    pub on_track: bool,

    /// Current speed (m/s)
    pub speed: f32,
}

impl CarPhysics {
    /// Create new car physics from specification
    pub fn new(id: BodyId, spec: CarSpec, position: Vec3) -> Self {
        let body = PhysicsBody::new(id, position, spec.mass);

        Self {
            body,
            spec,
            engine_rpm: 1000.0, // Idle RPM
            gear: 1,            // Start in first gear
            throttle: 0.0,
            brake: 0.0,
            steering: 0.0,
            wheel_speeds: [0.0; 4],
            tire_temps: [80.0; 4], // Optimal temp around 80°C
            tire_grip: TireGrip::perfect(),
            on_track: true,
            speed: 0.0,
        }
    }

    /// Update car physics for one timestep
    pub fn update(&mut self, dt: f32) {
        // Calculate current speed
        self.speed = self.body.velocity.length();

        // Update engine RPM based on wheel speed and gear
        self.update_engine_rpm();

        // Apply engine force
        if self.throttle > 0.0 {
            self.apply_engine_force();
        }

        // Apply braking force
        if self.brake > 0.0 {
            self.apply_braking_force();
        }

        // Apply steering
        if self.steering.abs() > 0.01 {
            self.apply_steering_force();
        }

        // Apply aerodynamic forces
        self.apply_aerodynamic_forces();

        // Apply tire friction
        self.apply_tire_friction();

        // Update tire temperatures
        self.update_tire_temps(dt);
    }

    /// Update engine RPM based on wheel speed
    fn update_engine_rpm(&mut self) {
        if self.gear <= 0 {
            self.engine_rpm = 1000.0;
            return;
        }

        // Simplified: RPM proportional to rear wheel speed
        let avg_rear_wheel = (self.wheel_speeds[2] + self.wheel_speeds[3]) / 2.0;
        let gear_ratio = match self.gear {
            1 => 3.5,
            2 => 2.5,
            3 => 1.8,
            4 => 1.4,
            5 => 1.1,
            6 => 0.9,
            _ => 1.0,
        };

        self.engine_rpm = (avg_rear_wheel * gear_ratio * 60.0 / (2.0 * std::f32::consts::PI))
            .max(1000.0)
            .min(self.spec.engine.max_rpm);
    }

    /// Apply engine force based on throttle
    fn apply_engine_force(&mut self) {
        // Get engine power at current RPM
        let power = self.interpolate_power_curve(self.engine_rpm);
        let torque = if self.engine_rpm > 0.0 {
            power * 1000.0 / self.engine_rpm // Convert kW to Nm
        } else {
            0.0
        };

        // Apply throttle
        let engine_torque = torque * self.throttle;

        // Convert torque to force (simplified)
        let wheel_radius = 0.3; // meters
        let force = engine_torque / wheel_radius;

        // Apply force in forward direction
        let forward = self.body.orientation * Vec3::X;
        self.body.add_force(forward * force);
    }

    /// Apply braking force
    fn apply_braking_force(&mut self) {
        // Maximum braking force (simplified)
        let max_brake_force = 15000.0; // Newtons
        let brake_force = max_brake_force * self.brake;

        // Apply opposite to velocity direction
        if self.speed > 0.1 {
            let brake_direction = -self.body.velocity.normalize();
            self.body.add_force(brake_direction * brake_force);
        }
    }

    /// Apply steering force
    fn apply_steering_force(&mut self) {
        // Steering only works when moving
        if self.speed < 0.5 {
            return;
        }

        // Calculate steering angle
        let max_steering_angle = 30.0_f32.to_radians();
        let steering_angle = self.steering * max_steering_angle;

        // Apply torque for rotation
        let steering_torque = steering_angle * self.speed * 100.0;
        self.body.add_torque(Vec3::new(0.0, steering_torque, 0.0));
    }

    /// Apply aerodynamic forces (drag and downforce)
    fn apply_aerodynamic_forces(&mut self) {
        let speed_squared = self.speed * self.speed;

        // Drag force: F = 0.5 * ρ * v² * Cd * A
        let air_density = 1.225; // kg/m³ at sea level
        let drag_force = 0.5 * air_density * speed_squared * self.spec.aerodynamics.drag * 2.0;

        // Apply drag opposite to velocity
        if self.speed > 0.1 {
            let drag_direction = -self.body.velocity.normalize();
            self.body.add_force(drag_direction * drag_force);
        }

        // Downforce (simplified, acts downward)
        let downforce = 0.5
            * air_density
            * speed_squared
            * self.spec.aerodynamics.downforce
            * 2.0;
        self.body.add_force(Vec3::new(0.0, -downforce, 0.0));
    }

    /// Apply tire friction
    fn apply_tire_friction(&mut self) {
        // Simplified tire friction
        let friction_coefficient = 0.8 * self.tire_grip.average();

        // Lateral friction (perpendicular to forward direction)
        let forward = self.body.orientation * Vec3::X;
        let lateral_velocity = self.body.velocity - forward * self.body.velocity.dot(forward);

        if lateral_velocity.length() > 0.1 {
            let lateral_friction = -lateral_velocity.normalize()
                * friction_coefficient
                * self.spec.mass
                * 9.81
                * 2.0;
            self.body.add_force(lateral_friction);
        }
    }

    /// Update tire temperatures based on usage
    fn update_tire_temps(&mut self, dt: f32) {
        // Simplified: tire temp increases with speed and friction
        let heat_rate = self.speed * 0.1 + self.steering.abs() * 5.0;
        let cooling_rate = 2.0;

        for temp in &mut self.tire_temps {
            *temp += (heat_rate - cooling_rate) * dt;
            *temp = temp.clamp(40.0, 120.0); // Realistic range
        }

        // Update grip based on temperature
        // Optimal around 80-90°C
        for (i, temp) in self.tire_temps.iter().enumerate() {
            let optimal_temp = 85.0;
            let temp_diff = (temp - optimal_temp).abs();
            let grip = (1.0 - temp_diff / 50.0).max(0.5);

            match i {
                0 => self.tire_grip.front_left = grip,
                1 => self.tire_grip.front_right = grip,
                2 => self.tire_grip.rear_left = grip,
                3 => self.tire_grip.rear_right = grip,
                _ => {}
            }
        }
    }

    /// Interpolate power from power curve
    fn interpolate_power_curve(&self, rpm: f32) -> f32 {
        if self.spec.engine.power_curve.is_empty() {
            return 0.0;
        }

        // Find surrounding points
        let mut lower_idx = 0;
        for (i, &(curve_rpm, _)) in self.spec.engine.power_curve.iter().enumerate() {
            if rpm >= curve_rpm {
                lower_idx = i;
            } else {
                break;
            }
        }

        // Clamp to curve bounds
        if lower_idx >= self.spec.engine.power_curve.len() - 1 {
            return self.spec.engine.power_curve.last().unwrap().1;
        }

        let (rpm1, power1) = self.spec.engine.power_curve[lower_idx];
        let (rpm2, power2) = self.spec.engine.power_curve[lower_idx + 1];

        // Linear interpolation
        let t = (rpm - rpm1) / (rpm2 - rpm1);
        power1 + (power2 - power1) * t
    }

    /// Set throttle input (0.0-1.0)
    pub fn set_throttle(&mut self, throttle: f32) {
        self.throttle = throttle.clamp(0.0, 1.0);
    }

    /// Set brake input (0.0-1.0)
    pub fn set_brake(&mut self, brake: f32) {
        self.brake = brake.clamp(0.0, 1.0);
    }

    /// Set steering input (-1.0 to 1.0)
    pub fn set_steering(&mut self, steering: f32) {
        self.steering = steering.clamp(-1.0, 1.0);
    }

    /// Shift to higher gear
    pub fn shift_up(&mut self) {
        if self.gear < 6 {
            self.gear += 1;
        }
    }

    /// Shift to lower gear
    pub fn shift_down(&mut self) {
        if self.gear > 1 {
            self.gear -= 1;
        }
    }

    /// Apply surface-based grip multiplier to all tires
    /// This modifies the tire grip based on the surface type (track, grass, gravel, etc.)
    pub fn apply_surface_grip(&mut self, surface_multiplier: f32) {
        // Multiply all tire grips by the surface multiplier
        self.tire_grip.front_left *= surface_multiplier;
        self.tire_grip.front_right *= surface_multiplier;
        self.tire_grip.rear_left *= surface_multiplier;
        self.tire_grip.rear_right *= surface_multiplier;

        // Clamp to valid range
        self.tire_grip.front_left = self.tire_grip.front_left.clamp(0.0, 1.0);
        self.tire_grip.front_right = self.tire_grip.front_right.clamp(0.0, 1.0);
        self.tire_grip.rear_left = self.tire_grip.rear_left.clamp(0.0, 1.0);
        self.tire_grip.rear_right = self.tire_grip.rear_right.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::car::{AeroSpec, CarDimensions, CarSpec, EngineSpec};

    fn create_test_car_spec() -> CarSpec {
        CarSpec {
            name: "Test Car".to_string(),
            team: "Test Team".to_string(),
            engine: EngineSpec {
                power_curve: vec![(5000.0, 400.0), (10000.0, 600.0), (15000.0, 550.0)],
                max_rpm: 15000.0,
                torque_curve: vec![],
                response: 0.8,
            },
            aerodynamics: AeroSpec {
                downforce: 2.5,
                drag: 0.9,
                front_wing: 15.0,
                rear_wing: 20.0,
            },
            mass: 505.0,
            dimensions: CarDimensions {
                length: 4.5,
                width: 2.0,
                height: 0.95,
                wheelbase: 2.8,
            },
            livery_colors: vec![(255, 0, 0)],
        }
    }

    #[test]
    fn test_car_physics_creation() {
        let spec = create_test_car_spec();
        let car = CarPhysics::new(BodyId(0), spec, Vec3::ZERO);
        assert_eq!(car.gear, 1);
        assert_eq!(car.throttle, 0.0);
    }

    #[test]
    fn test_tire_grip() {
        let grip = TireGrip::perfect();
        assert_eq!(grip.average(), 1.0);

        let no_grip = TireGrip::none();
        assert_eq!(no_grip.average(), 0.0);
    }

    #[test]
    fn test_throttle_clamp() {
        let spec = create_test_car_spec();
        let mut car = CarPhysics::new(BodyId(0), spec, Vec3::ZERO);
        car.set_throttle(1.5);
        assert_eq!(car.throttle, 1.0);

        car.set_throttle(-0.5);
        assert_eq!(car.throttle, 0.0);
    }

    #[test]
    fn test_gear_shifting() {
        let spec = create_test_car_spec();
        let mut car = CarPhysics::new(BodyId(0), spec, Vec3::ZERO);
        assert_eq!(car.gear, 1);

        car.shift_up();
        assert_eq!(car.gear, 2);

        car.shift_down();
        assert_eq!(car.gear, 1);

        car.shift_down();
        assert_eq!(car.gear, 1); // Can't go below 1
    }

    #[test]
    fn test_power_interpolation() {
        let spec = create_test_car_spec();
        let car = CarPhysics::new(BodyId(0), spec, Vec3::ZERO);

        let power = car.interpolate_power_curve(7500.0);
        assert!(power > 400.0 && power < 600.0); // Should be between curve points
    }
}
