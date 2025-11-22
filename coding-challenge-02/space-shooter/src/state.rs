/// Game state enum representing different screens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}
