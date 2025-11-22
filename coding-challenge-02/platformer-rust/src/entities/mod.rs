pub mod player;
pub mod platform;
pub mod enemy;
pub mod collectible;
pub mod checkpoint;

pub use player::Player;
pub use platform::{Platform, PlatformType};
pub use enemy::{Enemy, EnemyType};
pub use collectible::{Collectible, CollectibleType};
pub use checkpoint::Checkpoint;
