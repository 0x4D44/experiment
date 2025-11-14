//! Car specification and database
//!
//! Data structures for car performance characteristics, team data, and driver information.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete car specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarSpec {
    /// Car/Team name
    pub name: String,

    /// Team identifier
    pub team: String,

    /// Engine specifications
    pub engine: EngineSpec,

    /// Aerodynamic specifications
    pub aerodynamics: AeroSpec,

    /// Car mass in kilograms
    pub mass: f32,

    /// Car dimensions
    pub dimensions: CarDimensions,

    /// Team livery colors (RGB)
    pub livery_colors: Vec<(u8, u8, u8)>,
}

/// Engine specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSpec {
    /// Power curve: (RPM, Power in kW)
    pub power_curve: Vec<(f32, f32)>,

    /// Maximum RPM
    pub max_rpm: f32,

    /// Torque curve: (RPM, Torque in Nm)
    pub torque_curve: Vec<(f32, f32)>,

    /// Engine response (0.0 to 1.0, higher = more responsive)
    pub response: f32,
}

/// Aerodynamic specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeroSpec {
    /// Downforce coefficient
    pub downforce: f32,

    /// Drag coefficient
    pub drag: f32,

    /// Front wing angle (degrees)
    pub front_wing: f32,

    /// Rear wing angle (degrees)
    pub rear_wing: f32,
}

/// Car physical dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarDimensions {
    /// Length in meters
    pub length: f32,

    /// Width in meters
    pub width: f32,

    /// Height in meters
    pub height: f32,

    /// Wheelbase in meters
    pub wheelbase: f32,
}

/// Driver information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    /// Driver name
    pub name: String,

    /// Driver number
    pub number: u8,

    /// Team
    pub team: String,

    /// Skill ratings (0-100)
    pub skills: DriverSkills,
}

/// Driver skill ratings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverSkills {
    /// Overall pace (0-100)
    pub pace: u8,

    /// Consistency (0-100)
    pub consistency: u8,

    /// Wet weather skill (0-100)
    pub wet_weather: u8,

    /// Overtaking skill (0-100)
    pub overtaking: u8,

    /// Defensive driving (0-100)
    pub defending: u8,
}

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// Team name
    pub name: String,

    /// Team colors (RGB)
    pub colors: Vec<(u8, u8, u8)>,

    /// Drivers in this team
    pub drivers: Vec<String>,
}

/// Car database containing all cars, drivers, and teams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarDatabase {
    /// All car specifications
    cars: HashMap<String, CarSpec>,

    /// All teams
    teams: HashMap<String, Team>,

    /// All drivers
    drivers: HashMap<String, Driver>,
}

impl CarDatabase {
    /// Create a new empty car database
    pub fn new() -> Self {
        Self {
            cars: HashMap::new(),
            teams: HashMap::new(),
            drivers: HashMap::new(),
        }
    }

    /// Add a car to the database
    pub fn add_car(&mut self, car: CarSpec) {
        self.cars.insert(car.name.clone(), car);
    }

    /// Add a team to the database
    pub fn add_team(&mut self, team: Team) {
        self.teams.insert(team.name.clone(), team);
    }

    /// Add a driver to the database
    pub fn add_driver(&mut self, driver: Driver) {
        self.drivers.insert(driver.name.clone(), driver);
    }

    /// Get a car by name
    pub fn get_car(&self, name: &str) -> Option<&CarSpec> {
        self.cars.get(name)
    }

    /// Get a team by name
    pub fn get_team(&self, name: &str) -> Option<&Team> {
        self.teams.get(name)
    }

    /// Get a driver by name
    pub fn get_driver(&self, name: &str) -> Option<&Driver> {
        self.drivers.get(name)
    }

    /// Get all cars
    pub fn cars(&self) -> impl Iterator<Item = &CarSpec> {
        self.cars.values()
    }

    /// Get all teams
    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.values()
    }

    /// Get all drivers
    pub fn drivers(&self) -> impl Iterator<Item = &Driver> {
        self.drivers.values()
    }

    /// Get number of cars
    pub fn car_count(&self) -> usize {
        self.cars.len()
    }

    /// Get number of teams
    pub fn team_count(&self) -> usize {
        self.teams.len()
    }

    /// Get number of drivers
    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    /// Create a sample database for testing
    pub fn create_sample() -> Self {
        let mut db = Self::new();

        // Add sample teams (1991 F1 season)
        let teams = vec![
            ("McLaren", vec![(255, 255, 255), (255, 0, 0)]),
            ("Williams", vec![(0, 0, 255), (255, 255, 255)]),
            ("Ferrari", vec![(255, 0, 0), (255, 255, 255)]),
        ];

        for (name, colors) in teams {
            db.add_team(Team {
                name: name.to_string(),
                colors,
                drivers: vec![],
            });
        }

        // Add sample drivers
        let drivers = vec![
            ("Ayrton Senna", 1, "McLaren", 98, 95, 99, 98, 96),
            ("Gerhard Berger", 2, "McLaren", 88, 85, 80, 82, 83),
            ("Nigel Mansell", 5, "Williams", 95, 90, 88, 92, 89),
            ("Riccardo Patrese", 6, "Williams", 85, 88, 82, 80, 85),
            ("Alain Prost", 27, "Ferrari", 96, 98, 95, 90, 94),
            ("Jean Alesi", 28, "Ferrari", 87, 80, 85, 88, 82),
        ];

        for (name, number, team, pace, consistency, wet, overtaking, defending) in drivers {
            db.add_driver(Driver {
                name: name.to_string(),
                number,
                team: team.to_string(),
                skills: DriverSkills {
                    pace,
                    consistency,
                    wet_weather: wet,
                    overtaking,
                    defending,
                },
            });
        }

        // Add sample cars
        for team in &["McLaren", "Williams", "Ferrari"] {
            db.add_car(CarSpec {
                name: team.to_string(),
                team: team.to_string(),
                engine: EngineSpec {
                    power_curve: vec![(5000.0, 400.0), (10000.0, 600.0), (15000.0, 550.0)],
                    max_rpm: 15000.0,
                    torque_curve: vec![(5000.0, 500.0), (10000.0, 600.0), (15000.0, 450.0)],
                    response: 0.85,
                },
                aerodynamics: AeroSpec {
                    downforce: 2.5,
                    drag: 0.9,
                    front_wing: 15.0,
                    rear_wing: 20.0,
                },
                mass: 505.0, // 1991 minimum weight
                dimensions: CarDimensions {
                    length: 4.5,
                    width: 2.0,
                    height: 0.95,
                    wheelbase: 2.8,
                },
                livery_colors: if team == &"McLaren" {
                    vec![(255, 255, 255), (255, 0, 0)]
                } else if team == &"Williams" {
                    vec![(0, 0, 255), (255, 255, 255)]
                } else {
                    vec![(255, 0, 0), (255, 255, 255)]
                },
            });
        }

        db
    }
}

impl Default for CarDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_database_creation() {
        let db = CarDatabase::new();
        assert_eq!(db.car_count(), 0);
        assert_eq!(db.team_count(), 0);
        assert_eq!(db.driver_count(), 0);
    }

    #[test]
    fn test_add_car() {
        let mut db = CarDatabase::new();
        let car = CarSpec {
            name: "Test Car".to_string(),
            team: "Test Team".to_string(),
            engine: EngineSpec {
                power_curve: vec![],
                max_rpm: 15000.0,
                torque_curve: vec![],
                response: 0.8,
            },
            aerodynamics: AeroSpec {
                downforce: 2.0,
                drag: 0.8,
                front_wing: 10.0,
                rear_wing: 15.0,
            },
            mass: 500.0,
            dimensions: CarDimensions {
                length: 4.0,
                width: 1.8,
                height: 0.9,
                wheelbase: 2.5,
            },
            livery_colors: vec![(255, 0, 0)],
        };

        db.add_car(car);
        assert_eq!(db.car_count(), 1);
        assert!(db.get_car("Test Car").is_some());
    }

    #[test]
    fn test_sample_database() {
        let db = CarDatabase::create_sample();
        assert_eq!(db.car_count(), 3);
        assert_eq!(db.team_count(), 3);
        assert_eq!(db.driver_count(), 6);

        // Test specific drivers
        let senna = db.get_driver("Ayrton Senna");
        assert!(senna.is_some());
        assert_eq!(senna.unwrap().number, 1);
        assert_eq!(senna.unwrap().skills.pace, 98);
    }

    #[test]
    fn test_team_lookup() {
        let db = CarDatabase::create_sample();
        let mclaren = db.get_team("McLaren");
        assert!(mclaren.is_some());
        assert_eq!(mclaren.unwrap().colors.len(), 2);
    }
}
