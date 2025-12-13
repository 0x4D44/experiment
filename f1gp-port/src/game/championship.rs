//! Championship mode implementation
//!
//! Tracks driver and constructor standings across a full F1 season.
//! Uses the 1991 F1 points system: 10-6-4-3-2-1 for positions 1-6.

use std::collections::HashMap;

/// Points awarded for race positions (1991 F1 system)
/// Position 1 = 10 points, 2 = 6, 3 = 4, 4 = 3, 5 = 2, 6 = 1
const POINTS_TABLE: [u32; 6] = [10, 6, 4, 3, 2, 1];

/// 1991 F1 Season calendar (16 races)
pub const SEASON_CALENDAR: [&str; 16] = [
    "Phoenix",     // USA
    "Interlagos",  // Brazil
    "Imola",       // San Marino
    "Monaco",      // Monaco
    "Montreal",    // Canada
    "Mexico City", // Mexico
    "Magny-Cours", // France
    "Silverstone", // Great Britain
    "Hockenheim",  // Germany
    "Hungaroring", // Hungary
    "Spa",         // Belgium
    "Monza",       // Italy
    "Estoril",     // Portugal
    "Barcelona",   // Spain
    "Suzuka",      // Japan
    "Adelaide",    // Australia
];

/// Maps track file names to circuit names
pub fn track_to_circuit_name(track_file: &str) -> &'static str {
    match track_file {
        "F1CT01.DAT" => "Phoenix",
        "F1CT02.DAT" => "Interlagos",
        "F1CT03.DAT" => "Imola",
        "F1CT04.DAT" => "Monaco",
        "F1CT05.DAT" => "Montreal",
        "F1CT06.DAT" => "Mexico City",
        "F1CT07.DAT" => "Magny-Cours",
        "F1CT08.DAT" => "Silverstone",
        "F1CT09.DAT" => "Hockenheim",
        "F1CT10.DAT" => "Hungaroring",
        "F1CT11.DAT" => "Spa",
        "F1CT12.DAT" => "Monza",
        "F1CT13.DAT" => "Estoril",
        "F1CT14.DAT" => "Barcelona",
        "F1CT15.DAT" => "Suzuka",
        "F1CT16.DAT" => "Adelaide",
        _ => "Unknown",
    }
}

/// Returns the track file for a given round (1-16)
pub fn round_to_track_file(round: usize) -> Option<&'static str> {
    match round {
        1 => Some("F1CT01.DAT"),
        2 => Some("F1CT02.DAT"),
        3 => Some("F1CT03.DAT"),
        4 => Some("F1CT04.DAT"),
        5 => Some("F1CT05.DAT"),
        6 => Some("F1CT06.DAT"),
        7 => Some("F1CT07.DAT"),
        8 => Some("F1CT08.DAT"),
        9 => Some("F1CT09.DAT"),
        10 => Some("F1CT10.DAT"),
        11 => Some("F1CT11.DAT"),
        12 => Some("F1CT12.DAT"),
        13 => Some("F1CT13.DAT"),
        14 => Some("F1CT14.DAT"),
        15 => Some("F1CT15.DAT"),
        16 => Some("F1CT16.DAT"),
        _ => None,
    }
}

/// Driver championship entry
#[derive(Debug, Clone)]
pub struct DriverStanding {
    /// Driver name
    pub name: String,
    /// Team name
    pub team: String,
    /// Total championship points
    pub points: u32,
    /// Number of wins
    pub wins: u32,
    /// Number of podiums (top 3)
    pub podiums: u32,
    /// Number of pole positions
    pub poles: u32,
    /// Number of fastest laps
    pub fastest_laps: u32,
    /// Race results (position in each race, None if DNS/DNF)
    pub race_results: Vec<Option<u32>>,
}

impl DriverStanding {
    /// Create a new driver standing entry
    pub fn new(name: &str, team: &str) -> Self {
        Self {
            name: name.to_string(),
            team: team.to_string(),
            points: 0,
            wins: 0,
            podiums: 0,
            poles: 0,
            fastest_laps: 0,
            race_results: Vec::new(),
        }
    }

    /// Record a race result
    pub fn record_result(&mut self, position: Option<u32>, had_pole: bool, had_fastest_lap: bool) {
        self.race_results.push(position);

        if had_pole {
            self.poles += 1;
        }

        if had_fastest_lap {
            self.fastest_laps += 1;
        }

        if let Some(pos) = position {
            // Award points for positions 1-6
            if (1..=6).contains(&pos) {
                self.points += POINTS_TABLE[(pos - 1) as usize];
            }

            if pos == 1 {
                self.wins += 1;
            }

            if pos <= 3 {
                self.podiums += 1;
            }
        }
    }
}

/// Constructor (team) championship entry
#[derive(Debug, Clone)]
pub struct ConstructorStanding {
    /// Team name
    pub name: String,
    /// Total championship points
    pub points: u32,
    /// Number of wins
    pub wins: u32,
    /// Driver names for this constructor
    pub drivers: Vec<String>,
}

impl ConstructorStanding {
    /// Create a new constructor standing entry
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: 0,
            wins: 0,
            drivers: Vec::new(),
        }
    }

    /// Add a driver to this constructor
    pub fn add_driver(&mut self, driver_name: &str) {
        if !self.drivers.contains(&driver_name.to_string()) {
            self.drivers.push(driver_name.to_string());
        }
    }

    /// Record points from a race result
    pub fn record_points(&mut self, points: u32, is_win: bool) {
        self.points += points;
        if is_win {
            self.wins += 1;
        }
    }
}

/// Race result for a single race
#[derive(Debug, Clone)]
pub struct RaceResult {
    /// Circuit name
    pub circuit: String,
    /// Round number (1-16)
    pub round: usize,
    /// Finishing order (driver names in order)
    pub finishing_order: Vec<String>,
    /// DNF drivers (driver name -> reason)
    pub dnfs: HashMap<String, String>,
    /// Pole position driver
    pub pole_sitter: String,
    /// Fastest lap driver
    pub fastest_lap_driver: String,
    /// Fastest lap time (seconds)
    pub fastest_lap_time: f32,
}

/// Championship state
pub struct Championship {
    /// Season year
    pub year: u32,
    /// Current round (1-16)
    pub current_round: usize,
    /// Driver standings
    pub driver_standings: Vec<DriverStanding>,
    /// Constructor standings
    pub constructor_standings: Vec<ConstructorStanding>,
    /// Race results for completed races
    pub race_results: Vec<RaceResult>,
    /// Is championship complete?
    pub is_complete: bool,
}

impl Championship {
    /// Create a new championship season
    pub fn new(year: u32) -> Self {
        Self {
            year,
            current_round: 1,
            driver_standings: Vec::new(),
            constructor_standings: Vec::new(),
            race_results: Vec::new(),
            is_complete: false,
        }
    }

    /// Initialize championship with drivers and teams
    pub fn initialize_drivers(&mut self, drivers: Vec<(String, String)>) {
        // Create driver standings
        for (driver_name, team_name) in &drivers {
            self.driver_standings
                .push(DriverStanding::new(driver_name, team_name));

            // Ensure constructor exists
            if !self
                .constructor_standings
                .iter()
                .any(|c| c.name == *team_name)
            {
                self.constructor_standings
                    .push(ConstructorStanding::new(team_name));
            }

            // Add driver to constructor
            if let Some(constructor) = self
                .constructor_standings
                .iter_mut()
                .find(|c| c.name == *team_name)
            {
                constructor.add_driver(driver_name);
            }
        }
    }

    /// Get the current circuit name
    pub fn current_circuit(&self) -> &'static str {
        if self.current_round >= 1 && self.current_round <= 16 {
            SEASON_CALENDAR[self.current_round - 1]
        } else {
            "Unknown"
        }
    }

    /// Get the current track file
    pub fn current_track_file(&self) -> Option<&'static str> {
        round_to_track_file(self.current_round)
    }

    /// Record a race result
    pub fn record_race_result(&mut self, result: RaceResult) {
        // Update driver standings
        for (position, driver_name) in result.finishing_order.iter().enumerate() {
            let pos = (position + 1) as u32;
            let had_pole = result.pole_sitter == *driver_name;
            let had_fastest = result.fastest_lap_driver == *driver_name;

            if let Some(standing) = self
                .driver_standings
                .iter_mut()
                .find(|d| d.name == *driver_name)
            {
                standing.record_result(Some(pos), had_pole, had_fastest);

                // Update constructor points
                if let Some(constructor) = self
                    .constructor_standings
                    .iter_mut()
                    .find(|c| c.name == standing.team)
                {
                    let points = if (1..=6).contains(&pos) {
                        POINTS_TABLE[(pos - 1) as usize]
                    } else {
                        0
                    };
                    constructor.record_points(points, pos == 1);
                }
            }
        }

        // Record DNFs
        for driver_name in result.dnfs.keys() {
            if let Some(standing) = self
                .driver_standings
                .iter_mut()
                .find(|d| d.name == *driver_name)
            {
                standing.record_result(None, false, false);
            }
        }

        // Store race result
        self.race_results.push(result);

        // Advance to next round
        self.current_round += 1;
        if self.current_round > 16 {
            self.is_complete = true;
        }

        // Sort standings
        self.sort_standings();
    }

    /// Sort driver and constructor standings by points
    pub fn sort_standings(&mut self) {
        // Sort drivers by points (descending), then by wins as tiebreaker
        self.driver_standings.sort_by(|a, b| {
            b.points
                .cmp(&a.points)
                .then_with(|| b.wins.cmp(&a.wins))
                .then_with(|| b.podiums.cmp(&a.podiums))
        });

        // Sort constructors by points
        self.constructor_standings
            .sort_by(|a, b| b.points.cmp(&a.points).then_with(|| b.wins.cmp(&a.wins)));
    }

    /// Get driver's current championship position (1-based)
    pub fn driver_position(&self, driver_name: &str) -> Option<usize> {
        self.driver_standings
            .iter()
            .position(|d| d.name == driver_name)
            .map(|p| p + 1)
    }

    /// Get constructor's current championship position (1-based)
    pub fn constructor_position(&self, team_name: &str) -> Option<usize> {
        self.constructor_standings
            .iter()
            .position(|c| c.name == team_name)
            .map(|p| p + 1)
    }

    /// Get the championship leader
    pub fn leader(&self) -> Option<&DriverStanding> {
        self.driver_standings.first()
    }

    /// Get points gap between two drivers
    pub fn points_gap(&self, driver1: &str, driver2: &str) -> Option<i32> {
        let p1 = self
            .driver_standings
            .iter()
            .find(|d| d.name == driver1)
            .map(|d| d.points)?;
        let p2 = self
            .driver_standings
            .iter()
            .find(|d| d.name == driver2)
            .map(|d| d.points)?;
        Some(p1 as i32 - p2 as i32)
    }

    /// Check if championship is mathematically decided
    pub fn is_decided(&self) -> bool {
        if self.driver_standings.len() < 2 {
            return true;
        }

        let remaining_races = 16 - (self.current_round - 1);
        let max_remaining_points = remaining_races as u32 * POINTS_TABLE[0]; // 10 per race

        let leader = &self.driver_standings[0];
        let second = &self.driver_standings[1];

        leader.points > second.points + max_remaining_points
    }

    /// Get the number of completed races
    pub fn races_completed(&self) -> usize {
        self.race_results.len()
    }

    /// Get the number of remaining races
    pub fn races_remaining(&self) -> usize {
        16 - self.races_completed()
    }
}

/// Create a default 1991 F1 season with all drivers
pub fn create_1991_season() -> Championship {
    let mut championship = Championship::new(1991);

    // 1991 F1 Season drivers (based on F1GP game data)
    let drivers = vec![
        ("Ayrton Senna".to_string(), "McLaren".to_string()),
        ("Gerhard Berger".to_string(), "McLaren".to_string()),
        ("Nigel Mansell".to_string(), "Williams".to_string()),
        ("Riccardo Patrese".to_string(), "Williams".to_string()),
        ("Alain Prost".to_string(), "Ferrari".to_string()),
        ("Jean Alesi".to_string(), "Ferrari".to_string()),
        ("Nelson Piquet".to_string(), "Benetton".to_string()),
        ("Roberto Moreno".to_string(), "Benetton".to_string()),
        ("Thierry Boutsen".to_string(), "Ligier".to_string()),
        ("Erik Comas".to_string(), "Ligier".to_string()),
        ("Ivan Capelli".to_string(), "Leyton House".to_string()),
        ("Mauricio Gugelmin".to_string(), "Leyton House".to_string()),
        ("Stefano Modena".to_string(), "Tyrrell".to_string()),
        ("Satoru Nakajima".to_string(), "Tyrrell".to_string()),
        ("Martin Brundle".to_string(), "Brabham".to_string()),
        ("Mark Blundell".to_string(), "Brabham".to_string()),
        ("Pierluigi Martini".to_string(), "Minardi".to_string()),
        ("Gianni Morbidelli".to_string(), "Minardi".to_string()),
        ("Andrea de Cesaris".to_string(), "Jordan".to_string()),
        ("Bertrand Gachot".to_string(), "Jordan".to_string()),
        ("Aguri Suzuki".to_string(), "Lola".to_string()),
        ("Eric Bernard".to_string(), "Lola".to_string()),
        ("JJ Lehto".to_string(), "Dallara".to_string()),
        ("Emanuele Pirro".to_string(), "Dallara".to_string()),
        ("Michele Alboreto".to_string(), "Footwork".to_string()),
        ("Alex Caffi".to_string(), "Footwork".to_string()),
    ];

    championship.initialize_drivers(drivers);
    championship
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_championship_creation() {
        let champ = Championship::new(1991);
        assert_eq!(champ.year, 1991);
        assert_eq!(champ.current_round, 1);
        assert!(!champ.is_complete);
    }

    #[test]
    fn test_1991_season_creation() {
        let champ = create_1991_season();
        assert_eq!(champ.driver_standings.len(), 26);
        assert!(!champ.constructor_standings.is_empty());
    }

    #[test]
    fn test_driver_standing_points() {
        let mut standing = DriverStanding::new("Test Driver", "Test Team");
        standing.record_result(Some(1), true, true); // Win
        assert_eq!(standing.points, 10);
        assert_eq!(standing.wins, 1);
        assert_eq!(standing.podiums, 1);
        assert_eq!(standing.poles, 1);
        assert_eq!(standing.fastest_laps, 1);

        standing.record_result(Some(2), false, false);
        assert_eq!(standing.points, 16); // 10 + 6

        standing.record_result(Some(7), false, false); // No points
        assert_eq!(standing.points, 16);
    }

    #[test]
    fn test_race_result_recording() {
        let mut champ = create_1991_season();

        let result = RaceResult {
            circuit: "Phoenix".to_string(),
            round: 1,
            finishing_order: vec![
                "Ayrton Senna".to_string(),
                "Alain Prost".to_string(),
                "Nelson Piquet".to_string(),
            ],
            dnfs: HashMap::new(),
            pole_sitter: "Ayrton Senna".to_string(),
            fastest_lap_driver: "Ayrton Senna".to_string(),
            fastest_lap_time: 92.5,
        };

        champ.record_race_result(result);

        // Check Senna's standing
        let senna = champ
            .driver_standings
            .iter()
            .find(|d| d.name == "Ayrton Senna")
            .unwrap();
        assert_eq!(senna.points, 10);
        assert_eq!(senna.wins, 1);
        assert_eq!(senna.poles, 1);
        assert_eq!(senna.fastest_laps, 1);

        // Check Prost's standing
        let prost = champ
            .driver_standings
            .iter()
            .find(|d| d.name == "Alain Prost")
            .unwrap();
        assert_eq!(prost.points, 6);

        // Check standings are sorted correctly
        assert_eq!(champ.driver_standings[0].name, "Ayrton Senna");

        // Check round advanced
        assert_eq!(champ.current_round, 2);
    }

    #[test]
    fn test_championship_sorting() {
        let mut champ = Championship::new(1991);
        champ.initialize_drivers(vec![
            ("Driver A".to_string(), "Team 1".to_string()),
            ("Driver B".to_string(), "Team 2".to_string()),
            ("Driver C".to_string(), "Team 1".to_string()),
        ]);

        // Give Driver B more points
        champ.driver_standings[1].points = 20;
        champ.driver_standings[0].points = 10;
        champ.driver_standings[2].points = 15;

        champ.sort_standings();

        assert_eq!(champ.driver_standings[0].name, "Driver B");
        assert_eq!(champ.driver_standings[1].name, "Driver C");
        assert_eq!(champ.driver_standings[2].name, "Driver A");
    }

    #[test]
    fn test_points_gap() {
        let mut champ = Championship::new(1991);
        champ.initialize_drivers(vec![
            ("Driver A".to_string(), "Team 1".to_string()),
            ("Driver B".to_string(), "Team 2".to_string()),
        ]);

        champ.driver_standings[0].points = 30;
        champ.driver_standings[1].points = 20;

        assert_eq!(champ.points_gap("Driver A", "Driver B"), Some(10));
        assert_eq!(champ.points_gap("Driver B", "Driver A"), Some(-10));
    }

    #[test]
    fn test_track_mapping() {
        assert_eq!(track_to_circuit_name("F1CT04.DAT"), "Monaco");
        assert_eq!(track_to_circuit_name("F1CT12.DAT"), "Monza");
        assert_eq!(round_to_track_file(4), Some("F1CT04.DAT"));
    }

    #[test]
    fn test_season_calendar() {
        assert_eq!(SEASON_CALENDAR.len(), 16);
        assert_eq!(SEASON_CALENDAR[0], "Phoenix");
        assert_eq!(SEASON_CALENDAR[15], "Adelaide");
    }

    #[test]
    fn test_championship_completion() {
        let mut champ = Championship::new(1991);
        champ.initialize_drivers(vec![("Test".to_string(), "Team".to_string())]);

        // Simulate 16 races
        for round in 1..=16 {
            let result = RaceResult {
                circuit: SEASON_CALENDAR[round - 1].to_string(),
                round,
                finishing_order: vec!["Test".to_string()],
                dnfs: HashMap::new(),
                pole_sitter: "Test".to_string(),
                fastest_lap_driver: "Test".to_string(),
                fastest_lap_time: 90.0,
            };
            champ.record_race_result(result);
        }

        assert!(champ.is_complete);
        assert_eq!(champ.races_completed(), 16);
        assert_eq!(champ.races_remaining(), 0);
    }
}
