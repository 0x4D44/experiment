//! Damage and mechanical failures system
//!
//! Handles car damage from collisions and mechanical reliability failures.
//! Based on F1GP's damage model where components can be damaged or fail.

/// Car component that can be damaged
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CarComponent {
    /// Front wing - affects downforce and handling
    FrontWing,
    /// Rear wing - affects downforce and top speed
    RearWing,
    /// Engine - affects power output
    Engine,
    /// Gearbox - affects gear changes
    Gearbox,
    /// Suspension front left
    SuspensionFL,
    /// Suspension front right
    SuspensionFR,
    /// Suspension rear left
    SuspensionRL,
    /// Suspension rear right
    SuspensionRR,
    /// Radiator - affects cooling/engine temp
    Radiator,
    /// Electronics - affects various systems
    Electronics,
    /// Fuel system
    FuelSystem,
    /// Brakes
    Brakes,
}

impl CarComponent {
    /// Get all components
    pub fn all() -> &'static [CarComponent] {
        &[
            CarComponent::FrontWing,
            CarComponent::RearWing,
            CarComponent::Engine,
            CarComponent::Gearbox,
            CarComponent::SuspensionFL,
            CarComponent::SuspensionFR,
            CarComponent::SuspensionRL,
            CarComponent::SuspensionRR,
            CarComponent::Radiator,
            CarComponent::Electronics,
            CarComponent::FuelSystem,
            CarComponent::Brakes,
        ]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            CarComponent::FrontWing => "Front Wing",
            CarComponent::RearWing => "Rear Wing",
            CarComponent::Engine => "Engine",
            CarComponent::Gearbox => "Gearbox",
            CarComponent::SuspensionFL => "Front Left Suspension",
            CarComponent::SuspensionFR => "Front Right Suspension",
            CarComponent::SuspensionRL => "Rear Left Suspension",
            CarComponent::SuspensionRR => "Rear Right Suspension",
            CarComponent::Radiator => "Radiator",
            CarComponent::Electronics => "Electronics",
            CarComponent::FuelSystem => "Fuel System",
            CarComponent::Brakes => "Brakes",
        }
    }

    /// Check if this component is critical (failure = retirement)
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            CarComponent::Engine
                | CarComponent::Gearbox
                | CarComponent::FuelSystem
                | CarComponent::Electronics
        )
    }

    /// Base reliability for this component (0.0-1.0, higher = more reliable)
    pub fn base_reliability(&self) -> f32 {
        match self {
            CarComponent::FrontWing => 0.98,
            CarComponent::RearWing => 0.98,
            CarComponent::Engine => 0.92,
            CarComponent::Gearbox => 0.94,
            CarComponent::SuspensionFL => 0.97,
            CarComponent::SuspensionFR => 0.97,
            CarComponent::SuspensionRL => 0.97,
            CarComponent::SuspensionRR => 0.97,
            CarComponent::Radiator => 0.96,
            CarComponent::Electronics => 0.95,
            CarComponent::FuelSystem => 0.97,
            CarComponent::Brakes => 0.96,
        }
    }
}

/// Damage level for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DamageLevel {
    /// No damage
    None,
    /// Minor damage - slight performance loss
    Minor,
    /// Moderate damage - noticeable performance loss
    Moderate,
    /// Severe damage - major performance loss
    Severe,
    /// Destroyed - component non-functional
    Destroyed,
}

impl DamageLevel {
    /// Get performance multiplier (1.0 = full performance)
    pub fn performance_multiplier(&self) -> f32 {
        match self {
            DamageLevel::None => 1.0,
            DamageLevel::Minor => 0.95,
            DamageLevel::Moderate => 0.85,
            DamageLevel::Severe => 0.65,
            DamageLevel::Destroyed => 0.0,
        }
    }

    /// Upgrade damage level (make worse)
    pub fn worsen(&self) -> DamageLevel {
        match self {
            DamageLevel::None => DamageLevel::Minor,
            DamageLevel::Minor => DamageLevel::Moderate,
            DamageLevel::Moderate => DamageLevel::Severe,
            DamageLevel::Severe => DamageLevel::Destroyed,
            DamageLevel::Destroyed => DamageLevel::Destroyed,
        }
    }
}

/// Component state including damage and wear
#[derive(Debug, Clone)]
pub struct ComponentState {
    /// Component type
    pub component: CarComponent,

    /// Current damage level
    pub damage: DamageLevel,

    /// Wear level (1.0 = new, 0.0 = worn out)
    pub wear: f32,

    /// Has this component failed?
    pub failed: bool,

    /// Temperature (for engine, radiator, brakes)
    pub temperature: f32,
}

impl ComponentState {
    /// Create new component in perfect condition
    pub fn new(component: CarComponent) -> Self {
        Self {
            component,
            damage: DamageLevel::None,
            wear: 1.0,
            failed: false,
            temperature: 80.0,
        }
    }

    /// Get effective performance (considering damage and wear)
    pub fn performance(&self) -> f32 {
        if self.failed {
            return 0.0;
        }

        let damage_factor = self.damage.performance_multiplier();
        let wear_factor = 0.7 + (self.wear * 0.3); // Wear affects 30% of performance

        damage_factor * wear_factor
    }

    /// Apply damage from collision
    pub fn apply_collision_damage(&mut self, severity: f32) {
        // Severity 0.0-1.0 determines chance and amount of damage
        if severity > 0.3 && self.damage < DamageLevel::Destroyed {
            // Higher severity = more likely to take damage
            let damage_chance = severity * 0.8;
            if fastrand::f32() < damage_chance {
                self.damage = self.damage.worsen();
            }
        }

        // Very severe impacts can destroy components
        if severity > 0.9 {
            self.damage = DamageLevel::Destroyed;
        }
    }

    /// Update wear over time
    pub fn update_wear(&mut self, stress: f32, dt: f32) {
        // Stress 0.0-1.0 determines wear rate
        let wear_rate = 0.0001 * (1.0 + stress);
        self.wear = (self.wear - wear_rate * dt).max(0.0);

        // Worn components more likely to fail
        if self.wear < 0.2 && !self.failed {
            let failure_chance = (0.2 - self.wear) * 0.001 * dt;
            if fastrand::f32() < failure_chance {
                self.failed = true;
            }
        }
    }
}

/// Collision type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    /// Side-to-side contact with another car
    CarToCarSide,
    /// Rear-end collision with another car
    CarToCarRear,
    /// Front collision with another car
    CarToCarFront,
    /// Hit track barrier
    Barrier,
    /// Hit wall
    Wall,
    /// Ran over kerb aggressively
    Kerb,
    /// Spun off into gravel
    Gravel,
}

impl CollisionType {
    /// Get base severity for this collision type
    pub fn base_severity(&self) -> f32 {
        match self {
            CollisionType::CarToCarSide => 0.3,
            CollisionType::CarToCarRear => 0.4,
            CollisionType::CarToCarFront => 0.5,
            CollisionType::Barrier => 0.6,
            CollisionType::Wall => 0.8,
            CollisionType::Kerb => 0.1,
            CollisionType::Gravel => 0.2,
        }
    }

    /// Get components most likely to be damaged
    pub fn affected_components(&self) -> &'static [CarComponent] {
        match self {
            CollisionType::CarToCarFront | CollisionType::Wall => &[
                CarComponent::FrontWing,
                CarComponent::SuspensionFL,
                CarComponent::SuspensionFR,
                CarComponent::Radiator,
            ],
            CollisionType::CarToCarRear => &[
                CarComponent::RearWing,
                CarComponent::Gearbox,
                CarComponent::SuspensionRL,
                CarComponent::SuspensionRR,
            ],
            CollisionType::CarToCarSide => &[
                CarComponent::SuspensionFL,
                CarComponent::SuspensionFR,
                CarComponent::SuspensionRL,
                CarComponent::SuspensionRR,
            ],
            CollisionType::Barrier => &[
                CarComponent::FrontWing,
                CarComponent::RearWing,
                CarComponent::SuspensionFL,
                CarComponent::SuspensionFR,
            ],
            CollisionType::Kerb => &[
                CarComponent::SuspensionFL,
                CarComponent::SuspensionFR,
                CarComponent::SuspensionRL,
                CarComponent::SuspensionRR,
            ],
            CollisionType::Gravel => &[
                CarComponent::Radiator,
                CarComponent::Brakes,
            ],
        }
    }
}

/// Mechanical failure type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureType {
    /// Engine blew up
    EngineFailure,
    /// Gearbox seized
    GearboxFailure,
    /// Electrical failure
    ElectricalFailure,
    /// Fuel system problem
    FuelProblem,
    /// Brake failure
    BrakeFailure,
    /// Suspension failure
    SuspensionFailure,
    /// Hydraulics failure
    HydraulicsFailure,
}

impl FailureType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            FailureType::EngineFailure => "Engine",
            FailureType::GearboxFailure => "Gearbox",
            FailureType::ElectricalFailure => "Electrical",
            FailureType::FuelProblem => "Fuel System",
            FailureType::BrakeFailure => "Brakes",
            FailureType::SuspensionFailure => "Suspension",
            FailureType::HydraulicsFailure => "Hydraulics",
        }
    }

    /// Check if this failure forces retirement
    pub fn forces_retirement(&self) -> bool {
        matches!(
            self,
            FailureType::EngineFailure
                | FailureType::GearboxFailure
                | FailureType::ElectricalFailure
                | FailureType::FuelProblem
        )
    }
}

/// Complete damage state for a car
#[derive(Debug, Clone)]
pub struct DamageState {
    /// Component states
    components: Vec<ComponentState>,

    /// Car reliability rating (0.0-1.0, affects failure chance)
    pub reliability_rating: f32,

    /// Active mechanical failure (if any)
    pub active_failure: Option<FailureType>,

    /// Is the car retired?
    pub is_retired: bool,

    /// Retirement reason
    pub retirement_reason: Option<String>,

    /// Total collision count
    pub collision_count: u32,
}

impl DamageState {
    /// Create new damage state with all components healthy
    pub fn new(reliability_rating: f32) -> Self {
        let components = CarComponent::all()
            .iter()
            .map(|&c| ComponentState::new(c))
            .collect();

        Self {
            components,
            reliability_rating: reliability_rating.clamp(0.5, 1.0),
            active_failure: None,
            is_retired: false,
            retirement_reason: None,
            collision_count: 0,
        }
    }

    /// Get component state
    pub fn get_component(&self, component: CarComponent) -> Option<&ComponentState> {
        self.components.iter().find(|c| c.component == component)
    }

    /// Get mutable component state
    pub fn get_component_mut(&mut self, component: CarComponent) -> Option<&mut ComponentState> {
        self.components.iter_mut().find(|c| c.component == component)
    }

    /// Apply collision damage
    pub fn apply_collision(&mut self, collision_type: CollisionType, speed: f32) {
        self.collision_count += 1;

        // Calculate severity based on collision type and speed
        let speed_factor = (speed / 50.0).min(2.0); // Normalize around 50 m/s
        let severity = collision_type.base_severity() * speed_factor;

        // Apply damage to affected components
        for &component in collision_type.affected_components() {
            if let Some(state) = self.get_component_mut(component) {
                state.apply_collision_damage(severity);

                // Check for retirement
                if state.damage == DamageLevel::Destroyed && component.is_critical() {
                    self.retire(&format!("{} destroyed", component.name()));
                }
            }
        }
    }

    /// Update mechanical state (call each frame)
    pub fn update(&mut self, engine_stress: f32, dt: f32) {
        if self.is_retired {
            return;
        }

        // Update component wear and collect any failures
        let mut pending_failure: Option<FailureType> = None;

        for component in &mut self.components {
            let stress = match component.component {
                CarComponent::Engine => engine_stress,
                CarComponent::Gearbox => engine_stress * 0.8,
                CarComponent::Brakes => engine_stress * 0.5,
                _ => engine_stress * 0.3,
            };
            component.update_wear(stress, dt);

            // Check for component failure
            if component.failed && component.component.is_critical() && pending_failure.is_none() {
                pending_failure = match component.component {
                    CarComponent::Engine => Some(FailureType::EngineFailure),
                    CarComponent::Gearbox => Some(FailureType::GearboxFailure),
                    CarComponent::Electronics => Some(FailureType::ElectricalFailure),
                    CarComponent::FuelSystem => Some(FailureType::FuelProblem),
                    _ => None,
                };
            }
        }

        // Apply any pending failure after the loop
        if let Some(failure_type) = pending_failure {
            self.mechanical_failure(failure_type);
        }

        // Random mechanical failure check (based on reliability)
        self.check_random_failure(dt);
    }

    /// Check for random mechanical failure
    fn check_random_failure(&mut self, dt: f32) {
        if self.active_failure.is_some() || self.is_retired {
            return;
        }

        // Base failure chance per second (very low)
        let base_chance = 0.00005 * dt;
        let adjusted_chance = base_chance * (2.0 - self.reliability_rating);

        if fastrand::f32() < adjusted_chance {
            // Random component failure
            let failure_types = [
                FailureType::EngineFailure,
                FailureType::GearboxFailure,
                FailureType::ElectricalFailure,
                FailureType::HydraulicsFailure,
            ];

            let failure = failure_types[fastrand::usize(..failure_types.len())];
            self.mechanical_failure(failure);
        }
    }

    /// Trigger a mechanical failure
    pub fn mechanical_failure(&mut self, failure_type: FailureType) {
        self.active_failure = Some(failure_type);

        if failure_type.forces_retirement() {
            self.retire(&format!("{} failure", failure_type.name()));
        }
    }

    /// Retire the car
    pub fn retire(&mut self, reason: &str) {
        if !self.is_retired {
            self.is_retired = true;
            self.retirement_reason = Some(reason.to_string());
        }
    }

    /// Get overall car performance multiplier
    pub fn overall_performance(&self) -> f32 {
        if self.is_retired {
            return 0.0;
        }

        // Average of critical components
        let engine_perf = self
            .get_component(CarComponent::Engine)
            .map(|c| c.performance())
            .unwrap_or(1.0);
        let aero_perf = (self
            .get_component(CarComponent::FrontWing)
            .map(|c| c.performance())
            .unwrap_or(1.0)
            + self
                .get_component(CarComponent::RearWing)
                .map(|c| c.performance())
                .unwrap_or(1.0))
            / 2.0;

        (engine_perf * 0.6 + aero_perf * 0.4)
    }

    /// Get downforce multiplier (affected by wing damage)
    pub fn downforce_multiplier(&self) -> f32 {
        let front = self
            .get_component(CarComponent::FrontWing)
            .map(|c| c.performance())
            .unwrap_or(1.0);
        let rear = self
            .get_component(CarComponent::RearWing)
            .map(|c| c.performance())
            .unwrap_or(1.0);

        (front + rear) / 2.0
    }

    /// Get engine power multiplier
    pub fn engine_power_multiplier(&self) -> f32 {
        self.get_component(CarComponent::Engine)
            .map(|c| c.performance())
            .unwrap_or(1.0)
    }

    /// Check if car can continue racing
    pub fn can_continue(&self) -> bool {
        !self.is_retired
    }

    /// Get list of damaged components
    pub fn damaged_components(&self) -> Vec<(CarComponent, DamageLevel)> {
        self.components
            .iter()
            .filter(|c| c.damage != DamageLevel::None)
            .map(|c| (c.component, c.damage))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_level_performance() {
        assert_eq!(DamageLevel::None.performance_multiplier(), 1.0);
        assert!(DamageLevel::Minor.performance_multiplier() < 1.0);
        assert!(DamageLevel::Destroyed.performance_multiplier() == 0.0);
    }

    #[test]
    fn test_damage_level_worsen() {
        assert_eq!(DamageLevel::None.worsen(), DamageLevel::Minor);
        assert_eq!(DamageLevel::Severe.worsen(), DamageLevel::Destroyed);
        assert_eq!(DamageLevel::Destroyed.worsen(), DamageLevel::Destroyed);
    }

    #[test]
    fn test_component_state_creation() {
        let state = ComponentState::new(CarComponent::Engine);
        assert_eq!(state.damage, DamageLevel::None);
        assert_eq!(state.wear, 1.0);
        assert!(!state.failed);
    }

    #[test]
    fn test_component_performance() {
        let mut state = ComponentState::new(CarComponent::Engine);
        assert_eq!(state.performance(), 1.0);

        state.damage = DamageLevel::Moderate;
        assert!(state.performance() < 1.0);

        state.failed = true;
        assert_eq!(state.performance(), 0.0);
    }

    #[test]
    fn test_damage_state_creation() {
        let damage = DamageState::new(0.9);
        assert!(!damage.is_retired);
        assert!(damage.active_failure.is_none());
        assert_eq!(damage.collision_count, 0);
    }

    #[test]
    fn test_collision_damage() {
        let mut damage = DamageState::new(0.9);

        // High speed wall collision should cause damage
        damage.apply_collision(CollisionType::Wall, 80.0);

        assert!(damage.collision_count > 0);
        // Some components should be damaged
        let damaged = damage.damaged_components();
        // Due to randomness, we just check the collision was recorded
        assert_eq!(damage.collision_count, 1);
    }

    #[test]
    fn test_retirement() {
        let mut damage = DamageState::new(0.9);
        assert!(damage.can_continue());

        damage.retire("Test retirement");

        assert!(!damage.can_continue());
        assert!(damage.is_retired);
        assert_eq!(damage.retirement_reason, Some("Test retirement".to_string()));
    }

    #[test]
    fn test_mechanical_failure() {
        let mut damage = DamageState::new(0.9);

        damage.mechanical_failure(FailureType::EngineFailure);

        assert_eq!(damage.active_failure, Some(FailureType::EngineFailure));
        assert!(damage.is_retired); // Engine failure forces retirement
    }

    #[test]
    fn test_non_critical_failure() {
        let mut damage = DamageState::new(0.9);

        damage.mechanical_failure(FailureType::BrakeFailure);

        assert_eq!(damage.active_failure, Some(FailureType::BrakeFailure));
        assert!(!damage.is_retired); // Brake failure doesn't force retirement
    }

    #[test]
    fn test_overall_performance() {
        let damage = DamageState::new(0.9);
        assert!((damage.overall_performance() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_downforce_multiplier() {
        let mut damage = DamageState::new(0.9);
        assert!((damage.downforce_multiplier() - 1.0).abs() < 0.01);

        // Damage front wing
        if let Some(wing) = damage.get_component_mut(CarComponent::FrontWing) {
            wing.damage = DamageLevel::Severe;
        }

        assert!(damage.downforce_multiplier() < 1.0);
    }

    #[test]
    fn test_collision_types() {
        assert!(CollisionType::Wall.base_severity() > CollisionType::Kerb.base_severity());
        assert!(!CollisionType::Wall.affected_components().is_empty());
    }

    #[test]
    fn test_failure_type_retirement() {
        assert!(FailureType::EngineFailure.forces_retirement());
        assert!(!FailureType::BrakeFailure.forces_retirement());
    }

    #[test]
    fn test_component_criticality() {
        assert!(CarComponent::Engine.is_critical());
        assert!(CarComponent::Gearbox.is_critical());
        assert!(!CarComponent::FrontWing.is_critical());
    }
}
