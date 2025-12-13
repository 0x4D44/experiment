//! Pit stop and tire strategy system
//!
//! Handles pit stops, tire changes, fuel loads, and race strategy.
//! Based on 1991 F1 regulations with refueling banned (fuel load fixed at start).

/// Tire compound types available in F1GP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TireCompound {
    /// Type A - Hardest compound, longest life, least grip
    A,
    /// Type B - Medium-hard compound
    B,
    /// Type C - Medium compound (default)
    C,
    /// Type D - Medium-soft compound
    D,
    /// Qualifying tires - Maximum grip, very short life
    Qualifying,
}

impl TireCompound {
    /// Get grip multiplier for this compound (higher = more grip)
    pub fn grip_multiplier(&self) -> f32 {
        match self {
            TireCompound::A => 0.92,
            TireCompound::B => 0.96,
            TireCompound::C => 1.00,
            TireCompound::D => 1.04,
            TireCompound::Qualifying => 1.10,
        }
    }

    /// Get degradation rate (higher = faster wear)
    pub fn degradation_rate(&self) -> f32 {
        match self {
            TireCompound::A => 0.0008,
            TireCompound::B => 0.0012,
            TireCompound::C => 0.0016,
            TireCompound::D => 0.0022,
            TireCompound::Qualifying => 0.0050,
        }
    }

    /// Get expected tire life in laps (approximate)
    pub fn expected_life_laps(&self) -> u32 {
        match self {
            TireCompound::A => 50,
            TireCompound::B => 40,
            TireCompound::C => 30,
            TireCompound::D => 22,
            TireCompound::Qualifying => 8,
        }
    }

    /// Get compound name for display
    pub fn name(&self) -> &'static str {
        match self {
            TireCompound::A => "Hard (A)",
            TireCompound::B => "Medium-Hard (B)",
            TireCompound::C => "Medium (C)",
            TireCompound::D => "Soft (D)",
            TireCompound::Qualifying => "Qualifying",
        }
    }

    /// Get short name (single letter)
    pub fn short_name(&self) -> &'static str {
        match self {
            TireCompound::A => "A",
            TireCompound::B => "B",
            TireCompound::C => "C",
            TireCompound::D => "D",
            TireCompound::Qualifying => "Q",
        }
    }
}

/// Tire state for a single tire
#[derive(Debug, Clone, Copy)]
pub struct TireState {
    /// Current wear level (1.0 = new, 0.0 = completely worn)
    pub wear: f32,

    /// Current temperature (°C)
    pub temperature: f32,

    /// Compound type
    pub compound: TireCompound,

    /// Is tire flat/punctured?
    pub is_flat: bool,
}

impl TireState {
    /// Create new tire with given compound
    pub fn new(compound: TireCompound) -> Self {
        Self {
            wear: 1.0,
            temperature: 80.0, // Starting temp
            compound,
            is_flat: false,
        }
    }

    /// Get current grip level (combination of compound, wear, and temperature)
    pub fn grip_level(&self) -> f32 {
        if self.is_flat {
            return 0.1; // Minimal grip on flat tire
        }

        let compound_grip = self.compound.grip_multiplier();
        let wear_grip = self.wear_grip_factor();
        let temp_grip = self.temperature_grip_factor();

        compound_grip * wear_grip * temp_grip
    }

    /// Grip factor based on wear
    fn wear_grip_factor(&self) -> f32 {
        // Grip drops off as tires wear
        // Above 50% wear: full grip
        // Below 50%: linear drop to 70% grip at 0% wear
        if self.wear > 0.5 {
            1.0
        } else {
            0.7 + (self.wear * 0.6)
        }
    }

    /// Grip factor based on temperature
    fn temperature_grip_factor(&self) -> f32 {
        // Optimal temp range: 80-100°C
        // Below 60°C or above 120°C: reduced grip
        if self.temperature < 60.0 {
            0.85 + (self.temperature / 60.0) * 0.15
        } else if self.temperature > 120.0 {
            1.0 - ((self.temperature - 120.0) / 80.0).min(0.3)
        } else if self.temperature >= 80.0 && self.temperature <= 100.0 {
            1.0
        } else if self.temperature < 80.0 {
            0.95 + ((self.temperature - 60.0) / 20.0) * 0.05
        } else {
            0.95 + ((120.0 - self.temperature) / 20.0) * 0.05
        }
    }

    /// Update tire state for one physics tick
    pub fn update(&mut self, speed: f32, lateral_g: f32, dt: f32) {
        if self.is_flat {
            return;
        }

        // Wear increases with speed and lateral load
        let speed_factor = (speed / 80.0).max(0.5); // Normalize around 80 m/s (~288 km/h)
        let lateral_factor = 1.0 + lateral_g.abs() * 0.5;
        let wear_rate = self.compound.degradation_rate() * speed_factor * lateral_factor;

        self.wear = (self.wear - wear_rate * dt).max(0.0);

        // Temperature changes based on load
        let heat_gain = speed * lateral_g.abs() * 0.001;
        let heat_loss = (self.temperature - 40.0) * 0.01; // Ambient cooling
        self.temperature += (heat_gain - heat_loss) * dt;
        self.temperature = self.temperature.clamp(20.0, 150.0);
    }
}

/// Complete tire set (4 tires)
#[derive(Debug, Clone)]
pub struct TireSet {
    pub front_left: TireState,
    pub front_right: TireState,
    pub rear_left: TireState,
    pub rear_right: TireState,
}

impl TireSet {
    /// Create new tire set with given compound
    pub fn new(compound: TireCompound) -> Self {
        Self {
            front_left: TireState::new(compound),
            front_right: TireState::new(compound),
            rear_left: TireState::new(compound),
            rear_right: TireState::new(compound),
        }
    }

    /// Get average grip level across all tires
    pub fn average_grip(&self) -> f32 {
        (self.front_left.grip_level()
            + self.front_right.grip_level()
            + self.rear_left.grip_level()
            + self.rear_right.grip_level())
            / 4.0
    }

    /// Get average wear across all tires
    pub fn average_wear(&self) -> f32 {
        (self.front_left.wear + self.front_right.wear + self.rear_left.wear + self.rear_right.wear)
            / 4.0
    }

    /// Get minimum wear (worst tire)
    pub fn min_wear(&self) -> f32 {
        self.front_left
            .wear
            .min(self.front_right.wear)
            .min(self.rear_left.wear)
            .min(self.rear_right.wear)
    }

    /// Check if any tire is flat
    pub fn has_flat(&self) -> bool {
        self.front_left.is_flat
            || self.front_right.is_flat
            || self.rear_left.is_flat
            || self.rear_right.is_flat
    }

    /// Update all tires
    pub fn update(&mut self, speed: f32, lateral_g: f32, dt: f32) {
        // Front tires take more lateral load in corners
        let front_lateral = lateral_g * 1.1;
        let rear_lateral = lateral_g * 0.9;

        self.front_left.update(speed, front_lateral, dt);
        self.front_right.update(speed, front_lateral, dt);
        self.rear_left.update(speed, rear_lateral, dt);
        self.rear_right.update(speed, rear_lateral, dt);
    }

    /// Get the compound (assumes all same)
    pub fn compound(&self) -> TireCompound {
        self.front_left.compound
    }
}

/// Pit stop state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitStopPhase {
    /// Not in pit lane
    NotInPits,
    /// Entering pit lane (speed limit)
    PitEntry,
    /// Stopped in pit box
    InBox,
    /// Work in progress (tire change, etc.)
    Working,
    /// Pit stop complete, leaving
    PitExit,
}

/// Pit stop request (what work to do)
#[derive(Debug, Clone)]
pub struct PitStopRequest {
    /// New tire compound to fit
    pub new_tires: Option<TireCompound>,
    /// Front wing adjustment (-10 to +10)
    pub front_wing_adjust: i8,
    /// Rear wing adjustment (-10 to +10)
    pub rear_wing_adjust: i8,
}

impl Default for PitStopRequest {
    fn default() -> Self {
        Self {
            new_tires: Some(TireCompound::C),
            front_wing_adjust: 0,
            rear_wing_adjust: 0,
        }
    }
}

impl PitStopRequest {
    /// Calculate pit stop duration based on work requested
    pub fn duration(&self) -> f32 {
        let mut time = 0.0;

        // Base stop time (jack up, release)
        time += 2.0;

        // Tire change
        if self.new_tires.is_some() {
            time += 6.0; // ~6 seconds for tire change (1991 era)
        }

        // Wing adjustments
        if self.front_wing_adjust != 0 {
            time += 1.5;
        }
        if self.rear_wing_adjust != 0 {
            time += 1.5;
        }

        time
    }
}

/// Pit stop manager for a single car
#[derive(Debug, Clone)]
pub struct PitStopManager {
    /// Current pit stop phase
    pub phase: PitStopPhase,

    /// Current tire set
    pub tires: TireSet,

    /// Time remaining in current pit stop
    pub stop_time_remaining: f32,

    /// Queued pit stop request
    pub pit_request: Option<PitStopRequest>,

    /// Number of pit stops completed
    pub stops_completed: u32,

    /// Total time spent in pits
    pub total_pit_time: f32,

    /// Pit lane speed limit (m/s) - typically 60 km/h = 16.67 m/s
    pub pit_speed_limit: f32,
}

impl PitStopManager {
    /// Create new pit stop manager with initial tires
    pub fn new(initial_compound: TireCompound) -> Self {
        Self {
            phase: PitStopPhase::NotInPits,
            tires: TireSet::new(initial_compound),
            stop_time_remaining: 0.0,
            pit_request: None,
            stops_completed: 0,
            total_pit_time: 0.0,
            pit_speed_limit: 16.67, // 60 km/h
        }
    }

    /// Request a pit stop with specified work
    pub fn request_stop(&mut self, request: PitStopRequest) {
        self.pit_request = Some(request);
    }

    /// Enter pit lane
    pub fn enter_pit_lane(&mut self) {
        if self.phase == PitStopPhase::NotInPits {
            self.phase = PitStopPhase::PitEntry;
        }
    }

    /// Arrive at pit box
    pub fn arrive_at_box(&mut self) {
        if self.phase == PitStopPhase::PitEntry {
            self.phase = PitStopPhase::InBox;

            // Start work if there's a request
            if let Some(request) = &self.pit_request {
                self.stop_time_remaining = request.duration();
                self.phase = PitStopPhase::Working;
            }
        }
    }

    /// Update pit stop progress
    pub fn update(&mut self, dt: f32) {
        if self.phase == PitStopPhase::Working {
            self.stop_time_remaining -= dt;
            self.total_pit_time += dt;

            if self.stop_time_remaining <= 0.0 {
                // Complete the pit stop
                self.complete_stop();
            }
        }
    }

    /// Complete pit stop work
    fn complete_stop(&mut self) {
        if let Some(request) = self.pit_request.take() {
            // Fit new tires if requested
            if let Some(compound) = request.new_tires {
                self.tires = TireSet::new(compound);
            }

            // Wing adjustments would affect car setup (handled elsewhere)
        }

        self.stops_completed += 1;
        self.phase = PitStopPhase::PitExit;
    }

    /// Exit pit lane
    pub fn exit_pit_lane(&mut self) {
        if self.phase == PitStopPhase::PitExit {
            self.phase = PitStopPhase::NotInPits;
        }
    }

    /// Check if car is in pits
    pub fn is_in_pits(&self) -> bool {
        self.phase != PitStopPhase::NotInPits
    }

    /// Check if car is stationary in box
    pub fn is_stationary(&self) -> bool {
        matches!(self.phase, PitStopPhase::InBox | PitStopPhase::Working)
    }

    /// Get recommended pit window based on tire wear
    pub fn recommended_pit_window(&self, total_laps: u32, current_lap: u32) -> Option<(u32, u32)> {
        let remaining_laps = total_laps.saturating_sub(current_lap);
        let tire_life = self.tires.compound().expected_life_laps();
        let current_wear = self.tires.average_wear();

        // Estimate laps remaining on current tires
        let laps_on_tires = ((1.0 - current_wear) * tire_life as f32) as u32;

        if laps_on_tires >= remaining_laps {
            // Can finish on current tires
            None
        } else {
            // Need to pit
            let optimal_lap = current_lap + laps_on_tires.saturating_sub(3);
            let window_start = optimal_lap.saturating_sub(2);
            let window_end = optimal_lap + 2;
            Some((
                window_start.max(current_lap + 1),
                window_end.min(total_laps - 1),
            ))
        }
    }
}

/// Race strategy for AI decision making
#[derive(Debug, Clone)]
pub struct RaceStrategy {
    /// Planned pit stops (lap numbers)
    pub planned_stops: Vec<u32>,

    /// Tire compound for each stint
    pub stint_compounds: Vec<TireCompound>,

    /// Aggression level for tire management (0.0 = conservative, 1.0 = push hard)
    pub aggression: f32,
}

impl RaceStrategy {
    /// Create one-stop strategy
    pub fn one_stop(race_laps: u32) -> Self {
        let stop_lap = race_laps / 2;
        Self {
            planned_stops: vec![stop_lap],
            stint_compounds: vec![TireCompound::C, TireCompound::C],
            aggression: 0.5,
        }
    }

    /// Create two-stop strategy
    pub fn two_stop(race_laps: u32) -> Self {
        let stop1 = race_laps / 3;
        let stop2 = (race_laps * 2) / 3;
        Self {
            planned_stops: vec![stop1, stop2],
            stint_compounds: vec![TireCompound::D, TireCompound::C, TireCompound::C],
            aggression: 0.7,
        }
    }

    /// Create no-stop strategy (for short races)
    pub fn no_stop() -> Self {
        Self {
            planned_stops: vec![],
            stint_compounds: vec![TireCompound::B],
            aggression: 0.4,
        }
    }

    /// Check if should pit this lap
    pub fn should_pit(&self, current_lap: u32, stops_made: u32) -> bool {
        if let Some(&planned_lap) = self.planned_stops.get(stops_made as usize) {
            current_lap >= planned_lap
        } else {
            false
        }
    }

    /// Get compound for next stint
    pub fn next_compound(&self, stops_made: u32) -> TireCompound {
        self.stint_compounds
            .get((stops_made + 1) as usize)
            .copied()
            .unwrap_or(TireCompound::C)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tire_compound_properties() {
        assert!(TireCompound::D.grip_multiplier() > TireCompound::A.grip_multiplier());
        assert!(TireCompound::D.degradation_rate() > TireCompound::A.degradation_rate());
    }

    #[test]
    fn test_tire_state_creation() {
        let tire = TireState::new(TireCompound::C);
        assert_eq!(tire.wear, 1.0);
        assert!(!tire.is_flat);
    }

    #[test]
    fn test_tire_grip_degradation() {
        let mut tire = TireState::new(TireCompound::C);
        let initial_grip = tire.grip_level();

        // Simulate wear - below 50% threshold where grip drops
        tire.wear = 0.4;
        let mid_grip = tire.grip_level();

        tire.wear = 0.2;
        let low_grip = tire.grip_level();

        assert!(
            initial_grip > mid_grip,
            "Initial grip {} should be > mid grip {}",
            initial_grip,
            mid_grip
        );
        assert!(
            mid_grip > low_grip,
            "Mid grip {} should be > low grip {}",
            mid_grip,
            low_grip
        );
    }

    #[test]
    fn test_tire_set() {
        let tires = TireSet::new(TireCompound::C);
        assert_eq!(tires.average_wear(), 1.0);
        assert!(!tires.has_flat());
    }

    #[test]
    fn test_tire_set_flat() {
        let mut tires = TireSet::new(TireCompound::C);
        tires.front_left.is_flat = true;
        assert!(tires.has_flat());
    }

    #[test]
    fn test_pit_stop_request_duration() {
        let request = PitStopRequest {
            new_tires: Some(TireCompound::C),
            front_wing_adjust: 0,
            rear_wing_adjust: 0,
        };
        assert!((request.duration() - 8.0).abs() < 0.1); // 2 + 6 seconds

        let request_with_wings = PitStopRequest {
            new_tires: Some(TireCompound::C),
            front_wing_adjust: 5,
            rear_wing_adjust: -3,
        };
        assert!((request_with_wings.duration() - 11.0).abs() < 0.1); // 2 + 6 + 1.5 + 1.5
    }

    #[test]
    fn test_pit_stop_phases() {
        let mut manager = PitStopManager::new(TireCompound::C);
        assert_eq!(manager.phase, PitStopPhase::NotInPits);

        manager.enter_pit_lane();
        assert_eq!(manager.phase, PitStopPhase::PitEntry);

        manager.request_stop(PitStopRequest::default());
        manager.arrive_at_box();
        assert_eq!(manager.phase, PitStopPhase::Working);

        // Simulate time passing
        manager.update(10.0);
        assert_eq!(manager.phase, PitStopPhase::PitExit);
        assert_eq!(manager.stops_completed, 1);

        manager.exit_pit_lane();
        assert_eq!(manager.phase, PitStopPhase::NotInPits);
    }

    #[test]
    fn test_tire_change_in_pit() {
        let mut manager = PitStopManager::new(TireCompound::C);
        manager.tires.front_left.wear = 0.3; // Worn tires

        manager.enter_pit_lane();
        manager.request_stop(PitStopRequest {
            new_tires: Some(TireCompound::D),
            front_wing_adjust: 0,
            rear_wing_adjust: 0,
        });
        manager.arrive_at_box();
        manager.update(10.0);

        // New tires should be fitted
        assert_eq!(manager.tires.compound(), TireCompound::D);
        assert_eq!(manager.tires.average_wear(), 1.0);
    }

    #[test]
    fn test_race_strategy_one_stop() {
        let strategy = RaceStrategy::one_stop(60);
        assert_eq!(strategy.planned_stops.len(), 1);
        assert_eq!(strategy.planned_stops[0], 30);
    }

    #[test]
    fn test_race_strategy_should_pit() {
        let strategy = RaceStrategy::one_stop(60);

        assert!(!strategy.should_pit(25, 0)); // Before window
        assert!(strategy.should_pit(30, 0)); // At planned lap
        assert!(strategy.should_pit(35, 0)); // After planned lap
        assert!(!strategy.should_pit(35, 1)); // Already pitted
    }

    #[test]
    fn test_pit_window_calculation() {
        let manager = PitStopManager::new(TireCompound::C);

        // Fresh tires on a 30 lap race - might need to pit
        let window = manager.recommended_pit_window(50, 1);
        assert!(window.is_some());
    }

    #[test]
    fn test_tire_temperature_grip() {
        let mut tire = TireState::new(TireCompound::C);

        // Cold tire
        tire.temperature = 40.0;
        let cold_grip = tire.temperature_grip_factor();

        // Optimal temp
        tire.temperature = 90.0;
        let optimal_grip = tire.temperature_grip_factor();

        // Hot tire
        tire.temperature = 130.0;
        let hot_grip = tire.temperature_grip_factor();

        assert!(optimal_grip > cold_grip);
        assert!(optimal_grip > hot_grip);
    }
}
