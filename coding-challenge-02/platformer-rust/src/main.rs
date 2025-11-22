mod audio;
mod camera;
mod entities;
mod level;
mod particles;
mod physics;
mod ui;

use audio::AudioSystem;
use camera::Camera;
use entities::*;
use level::Level;
use macroquad::prelude::*;
use particles::ParticleSystem;
use physics::{resolve_collision, AABB};
use ui::{Background, Menu, HUD};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

struct Game {
    state: GameState,
    player: Player,
    levels: Vec<Level>,
    current_level: usize,
    camera: Camera,
    particles: ParticleSystem,
    audio: AudioSystem,
    background: Background,
    hud: HUD,
    menu: Menu,
    was_on_ground: bool,
}

impl Game {
    fn new() -> Self {
        let levels = Self::load_all_levels();
        let first_level = &levels[0];
        let (spawn_x, spawn_y) = first_level.get_spawn_point();

        let mut camera = Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        camera.set_bounds(0.0, 0.0, first_level.data.width, first_level.data.height);

        Self {
            state: GameState::MainMenu,
            player: Player::new(spawn_x, spawn_y),
            levels,
            current_level: 0,
            camera,
            particles: ParticleSystem::new(),
            audio: AudioSystem::new(),
            background: Background::new(),
            hud: HUD::new(),
            menu: Menu::new(vec!["Start Game", "Controls", "Quit"]),
            was_on_ground: false,
        }
    }

    fn load_all_levels() -> Vec<Level> {
        // Try to load levels from files, fallback to hardcoded levels
        let mut levels = Vec::new();

        for i in 1..=5 {
            let path = format!("levels/level{}.json", i);
            if let Ok(level) = Level::load_from_file(&path) {
                levels.push(level);
            }
        }

        // If no levels loaded, create default levels
        if levels.is_empty() {
            levels = Self::create_default_levels();
        }

        levels
    }

    fn create_default_levels() -> Vec<Level> {
        vec![
            Self::create_level_1(),
            Self::create_level_2(),
            Self::create_level_3(),
            Self::create_level_4(),
            Self::create_level_5(),
        ]
    }

    fn create_level_1() -> Level {
        use level::*;

        let data = LevelData {
            name: "Tutorial Valley".to_string(),
            width: 2000.0,
            height: 600.0,
            spawn_x: 100.0,
            spawn_y: 400.0,
            platforms: vec![
                // Ground platforms
                PlatformData {
                    x: 0.0,
                    y: 500.0,
                    width: 400.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 500.0,
                    y: 500.0,
                    width: 200.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 800.0,
                    y: 500.0,
                    width: 400.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                // Platforms to jump on
                PlatformData {
                    x: 450.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 650.0,
                    y: 350.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1300.0,
                    y: 400.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1500.0,
                    y: 500.0,
                    width: 500.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
            ],
            enemies: vec![EnemyData {
                x: 900.0,
                y: 450.0,
                enemy_type: EnemyType::Walker,
                patrol_start: Some(850.0),
                patrol_end: Some(1100.0),
                radius: None,
            }],
            collectibles: vec![
                CollectibleData {
                    x: 450.0,
                    y: 350.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 500.0,
                    y: 350.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 650.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Gem,
                },
                CollectibleData {
                    x: 1000.0,
                    y: 450.0,
                    collectible_type: CollectibleType::DoubleJump,
                },
            ],
            checkpoints: vec![CheckpointData { x: 1300.0, y: 400.0 }],
            goal_x: 1850.0,
            goal_y: 450.0,
        };

        Level::from_data(data)
    }

    fn create_level_2() -> Level {
        use level::*;

        let data = LevelData {
            name: "Moving Platforms".to_string(),
            width: 2500.0,
            height: 600.0,
            spawn_x: 100.0,
            spawn_y: 400.0,
            platforms: vec![
                PlatformData {
                    x: 0.0,
                    y: 500.0,
                    width: 300.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                // Moving platforms
                PlatformData {
                    x: 400.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(600.0),
                    end_y: Some(450.0),
                    speed: Some(80.0),
                },
                PlatformData {
                    x: 700.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(700.0),
                    end_y: Some(300.0),
                    speed: Some(60.0),
                },
                PlatformData {
                    x: 900.0,
                    y: 350.0,
                    width: 120.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1100.0),
                    end_y: Some(350.0),
                    speed: Some(70.0),
                },
                PlatformData {
                    x: 1300.0,
                    y: 500.0,
                    width: 200.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1600.0,
                    y: 400.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1850.0,
                    y: 400.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 2100.0,
                    y: 500.0,
                    width: 400.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
            ],
            enemies: vec![
                EnemyData {
                    x: 1350.0,
                    y: 450.0,
                    enemy_type: EnemyType::Walker,
                    patrol_start: Some(1300.0),
                    patrol_end: Some(1450.0),
                    radius: None,
                },
                EnemyData {
                    x: 1000.0,
                    y: 250.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(80.0),
                },
            ],
            collectibles: vec![
                CollectibleData {
                    x: 500.0,
                    y: 400.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 700.0,
                    y: 250.0,
                    collectible_type: CollectibleType::Gem,
                },
                CollectibleData {
                    x: 1000.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 1700.0,
                    y: 350.0,
                    collectible_type: CollectibleType::HealthPack,
                },
            ],
            checkpoints: vec![CheckpointData { x: 1350.0, y: 500.0 }],
            goal_x: 2350.0,
            goal_y: 450.0,
        };

        Level::from_data(data)
    }

    fn create_level_3() -> Level {
        use level::*;

        let data = LevelData {
            name: "Sky High".to_string(),
            width: 2000.0,
            height: 800.0,
            spawn_x: 100.0,
            spawn_y: 650.0,
            platforms: vec![
                PlatformData {
                    x: 0.0,
                    y: 700.0,
                    width: 250.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 350.0,
                    y: 600.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 550.0,
                    y: 500.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 750.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 950.0,
                    y: 300.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1150.0,
                    y: 250.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1400.0,
                    y: 350.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1400.0),
                    end_y: Some(550.0),
                    speed: Some(50.0),
                },
                PlatformData {
                    x: 1600.0,
                    y: 700.0,
                    width: 400.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
            ],
            enemies: vec![
                EnemyData {
                    x: 550.0,
                    y: 350.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(60.0),
                },
                EnemyData {
                    x: 950.0,
                    y: 200.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(70.0),
                },
                EnemyData {
                    x: 1700.0,
                    y: 650.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(1650.0),
                    patrol_end: Some(1900.0),
                    radius: None,
                },
            ],
            collectibles: vec![
                CollectibleData {
                    x: 350.0,
                    y: 550.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 550.0,
                    y: 450.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 750.0,
                    y: 350.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 1150.0,
                    y: 200.0,
                    collectible_type: CollectibleType::ExtraLife,
                },
            ],
            checkpoints: vec![CheckpointData {
                x: 1200.0,
                y: 250.0,
            }],
            goal_x: 1850.0,
            goal_y: 650.0,
        };

        Level::from_data(data)
    }

    fn create_level_4() -> Level {
        use level::*;

        let data = LevelData {
            name: "Danger Zone".to_string(),
            width: 3000.0,
            height: 600.0,
            spawn_x: 100.0,
            spawn_y: 400.0,
            platforms: vec![
                PlatformData {
                    x: 0.0,
                    y: 500.0,
                    width: 300.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 400.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 550.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 700.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 900.0,
                    y: 400.0,
                    width: 120.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1100.0),
                    end_y: Some(300.0),
                    speed: Some(90.0),
                },
                PlatformData {
                    x: 1300.0,
                    y: 500.0,
                    width: 200.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1600.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1800.0),
                    end_y: Some(400.0),
                    speed: Some(100.0),
                },
                PlatformData {
                    x: 2000.0,
                    y: 350.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(2000.0),
                    end_y: Some(500.0),
                    speed: Some(80.0),
                },
                PlatformData {
                    x: 2200.0,
                    y: 450.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 2500.0,
                    y: 500.0,
                    width: 500.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
            ],
            enemies: vec![
                EnemyData {
                    x: 600.0,
                    y: 350.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(80.0),
                },
                EnemyData {
                    x: 1350.0,
                    y: 450.0,
                    enemy_type: EnemyType::Walker,
                    patrol_start: Some(1300.0),
                    patrol_end: Some(1450.0),
                    radius: None,
                },
                EnemyData {
                    x: 1700.0,
                    y: 250.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(100.0),
                },
                EnemyData {
                    x: 2250.0,
                    y: 400.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(2200.0),
                    patrol_end: Some(2320.0),
                    radius: None,
                },
                EnemyData {
                    x: 2700.0,
                    y: 450.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(2600.0),
                    patrol_end: Some(2850.0),
                    radius: None,
                },
            ],
            collectibles: vec![
                CollectibleData {
                    x: 450.0,
                    y: 400.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 1000.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Gem,
                },
                CollectibleData {
                    x: 1350.0,
                    y: 450.0,
                    collectible_type: CollectibleType::HealthPack,
                },
                CollectibleData {
                    x: 2000.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Coin,
                },
            ],
            checkpoints: vec![
                CheckpointData {
                    x: 1350.0,
                    y: 500.0,
                },
                CheckpointData {
                    x: 2250.0,
                    y: 450.0,
                },
            ],
            goal_x: 2850.0,
            goal_y: 450.0,
        };

        Level::from_data(data)
    }

    fn create_level_5() -> Level {
        use level::*;

        let data = LevelData {
            name: "Final Challenge".to_string(),
            width: 3500.0,
            height: 700.0,
            spawn_x: 100.0,
            spawn_y: 550.0,
            platforms: vec![
                PlatformData {
                    x: 0.0,
                    y: 600.0,
                    width: 250.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 350.0,
                    y: 550.0,
                    width: 80.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 480.0,
                    y: 500.0,
                    width: 80.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 650.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(850.0),
                    end_y: Some(350.0),
                    speed: Some(100.0),
                },
                PlatformData {
                    x: 1000.0,
                    y: 300.0,
                    width: 120.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1200.0),
                    end_y: Some(300.0),
                    speed: Some(110.0),
                },
                PlatformData {
                    x: 1400.0,
                    y: 450.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1550.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1700.0,
                    y: 500.0,
                    width: 150.0,
                    height: 20.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 1950.0,
                    y: 400.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(1950.0),
                    end_y: Some(250.0),
                    speed: Some(70.0),
                },
                PlatformData {
                    x: 2150.0,
                    y: 350.0,
                    width: 100.0,
                    height: 20.0,
                    platform_type: PlatformType::Moving,
                    end_x: Some(2350.0),
                    end_y: Some(350.0),
                    speed: Some(90.0),
                },
                PlatformData {
                    x: 2550.0,
                    y: 450.0,
                    width: 120.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 2750.0,
                    y: 400.0,
                    width: 120.0,
                    height: 20.0,
                    platform_type: PlatformType::Disappearing,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
                PlatformData {
                    x: 2950.0,
                    y: 600.0,
                    width: 550.0,
                    height: 100.0,
                    platform_type: PlatformType::Solid,
                    end_x: None,
                    end_y: None,
                    speed: None,
                },
            ],
            enemies: vec![
                EnemyData {
                    x: 500.0,
                    y: 400.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(70.0),
                },
                EnemyData {
                    x: 800.0,
                    y: 250.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(90.0),
                },
                EnemyData {
                    x: 1100.0,
                    y: 200.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(60.0),
                },
                EnemyData {
                    x: 1750.0,
                    y: 450.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(1700.0),
                    patrol_end: Some(1820.0),
                    radius: None,
                },
                EnemyData {
                    x: 2250.0,
                    y: 250.0,
                    enemy_type: EnemyType::Flyer,
                    patrol_start: None,
                    patrol_end: None,
                    radius: Some(80.0),
                },
                EnemyData {
                    x: 3100.0,
                    y: 550.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(3000.0),
                    patrol_end: Some(3300.0),
                    radius: None,
                },
                EnemyData {
                    x: 3200.0,
                    y: 550.0,
                    enemy_type: EnemyType::Patroller,
                    patrol_start: Some(3100.0),
                    patrol_end: Some(3400.0),
                    radius: None,
                },
            ],
            collectibles: vec![
                CollectibleData {
                    x: 350.0,
                    y: 500.0,
                    collectible_type: CollectibleType::Coin,
                },
                CollectibleData {
                    x: 800.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Gem,
                },
                CollectibleData {
                    x: 1100.0,
                    y: 250.0,
                    collectible_type: CollectibleType::HealthPack,
                },
                CollectibleData {
                    x: 1750.0,
                    y: 450.0,
                    collectible_type: CollectibleType::ExtraLife,
                },
                CollectibleData {
                    x: 2250.0,
                    y: 300.0,
                    collectible_type: CollectibleType::Gem,
                },
                CollectibleData {
                    x: 2650.0,
                    y: 400.0,
                    collectible_type: CollectibleType::HealthPack,
                },
            ],
            checkpoints: vec![
                CheckpointData {
                    x: 1750.0,
                    y: 500.0,
                },
                CheckpointData {
                    x: 2600.0,
                    y: 450.0,
                },
            ],
            goal_x: 3350.0,
            goal_y: 550.0,
        };

        Level::from_data(data)
    }

    fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.current_level = 0;
        self.load_level(0);
    }

    fn load_level(&mut self, level_index: usize) {
        if level_index >= self.levels.len() {
            self.state = GameState::Victory;
            self.audio.play_victory();
            return;
        }

        self.current_level = level_index;
        let level = &self.levels[level_index];
        let (spawn_x, spawn_y) = level.get_spawn_point();

        self.player = Player::new(spawn_x, spawn_y);
        self.camera
            .set_bounds(0.0, 0.0, level.data.width, level.data.height);
        self.particles.clear();
        self.was_on_ground = false;

        self.hud
            .show_message(&format!("Level {}: {}", level_index + 1, level.data.name), 3.0);
    }

    fn update(&mut self, delta_time: f32) {
        match self.state {
            GameState::MainMenu => {
                if let Some(choice) = self.menu.handle_input() {
                    match choice {
                        0 => self.start_game(),
                        1 => {
                            // Show controls (we'll just start for now)
                            self.start_game();
                        }
                        2 => {
                            // Quit
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
            GameState::Playing => {
                self.update_gameplay(delta_time);

                // Pause on ESC
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Paused;
                }
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Q) {
                    self.state = GameState::MainMenu;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.state = GameState::MainMenu;
                }
            }
            GameState::Victory => {
                if is_key_pressed(KeyCode::Enter) {
                    self.state = GameState::MainMenu;
                }
            }
        }
    }

    fn update_gameplay(&mut self, delta_time: f32) {
        let level = &mut self.levels[self.current_level];

        // Update level
        level.update(delta_time);

        // Update HUD
        self.hud.update(delta_time);

        // Handle player input
        self.player.handle_input();

        // Store previous ground state for landing detection
        let was_on_ground_before = self.player.body.on_ground;

        // Apply physics
        self.player.body.apply_gravity(delta_time);
        self.player.body.on_ground = false;

        // Update player position with collision detection
        let new_x = self.player.body.position.x + self.player.body.velocity.x * delta_time;
        let new_y = self.player.body.position.y + self.player.body.velocity.y * delta_time;

        let player_aabb = AABB::new(new_x, new_y, self.player.body.size.x, self.player.body.size.y);

        // Check collisions with platforms
        for platform in &mut level.platforms {
            if !platform.active {
                continue;
            }

            let platform_aabb = platform.aabb();
            if player_aabb.intersects(&platform_aabb) {
                let collision = resolve_collision(&player_aabb, &platform_aabb);

                if collision.collided {
                    // Resolve collision
                    if collision.normal.y < 0.0 {
                        // Collision from top (player landing)
                        self.player.body.position.y -= collision.penetration;
                        self.player.body.velocity.y = 0.0;
                        self.player.body.on_ground = true;

                        // Trigger disappearing platforms
                        platform.trigger();

                        // Landing particle effect
                        if !was_on_ground_before {
                            self.particles.emit_landing(
                                self.player.body.position.x + self.player.body.size.x / 2.0,
                                self.player.body.position.y + self.player.body.size.y,
                            );
                            self.audio.play_land();
                        }
                    } else if collision.normal.y > 0.0 {
                        // Collision from bottom
                        self.player.body.position.y += collision.penetration;
                        self.player.body.velocity.y = 0.0;
                    } else if collision.normal.x != 0.0 {
                        // Horizontal collision
                        self.player.body.position.x += collision.normal.x * collision.penetration;
                        self.player.body.velocity.x = 0.0;
                    }
                }
            }
        }

        // Apply velocity after collision resolution
        if !self.player.body.on_ground {
            self.player.body.position.y = new_y;
        }
        self.player.body.position.x = new_x;

        // Update player state
        self.player.update(delta_time);

        // Check for jump particle effect
        if was_on_ground_before && !self.player.body.on_ground && self.player.body.velocity.y < 0.0 {
            self.particles.emit_jump(
                self.player.body.position.x + self.player.body.size.x / 2.0,
                self.player.body.position.y + self.player.body.size.y,
            );
            self.audio.play_jump();
        }

        // Check collisions with enemies
        let player_aabb = self.player.body.aabb();
        for enemy in &mut level.enemies {
            if !enemy.alive {
                continue;
            }

            let enemy_aabb = enemy.aabb();
            if player_aabb.intersects(&enemy_aabb) {
                // Check if player is stomping enemy (from above)
                if self.player.body.velocity.y > 0.0
                    && player_aabb.center().y < enemy_aabb.center().y
                {
                    // Stomp enemy
                    enemy.kill();
                    self.player.body.velocity.y = -250.0; // Bounce
                    self.player.add_score(200);
                    self.particles.emit_enemy_death(
                        enemy.body.position.x + enemy.body.size.x / 2.0,
                        enemy.body.position.y + enemy.body.size.y / 2.0,
                    );
                    self.audio.play_enemy_death();
                } else {
                    // Take damage
                    self.player.take_damage(1);
                    self.particles.emit_damage(
                        self.player.body.position.x + self.player.body.size.x / 2.0,
                        self.player.body.position.y + self.player.body.size.y / 2.0,
                    );
                    self.audio.play_damage();
                }
            }
        }

        // Check collisions with collectibles
        for collectible in &mut level.collectibles {
            if collectible.collected {
                continue;
            }

            if player_aabb.intersects(&collectible.aabb()) {
                collectible.collect();
                self.player.add_score(collectible.score_value());

                match collectible.collectible_type {
                    CollectibleType::Coin => {
                        self.player.collect_coin();
                    }
                    CollectibleType::Gem => {
                        self.player.collect_coin(); // Gems also count as special coins
                    }
                    CollectibleType::HealthPack => {
                        self.player.heal(1);
                    }
                    CollectibleType::ExtraLife => {
                        self.player.add_life();
                    }
                    CollectibleType::DoubleJump => {
                        self.player.can_double_jump = true;
                        self.hud.show_message("Double Jump Unlocked!", 2.0);
                    }
                }

                let color = match collectible.collectible_type {
                    CollectibleType::Coin => GOLD,
                    CollectibleType::Gem => BLUE,
                    CollectibleType::HealthPack => RED,
                    CollectibleType::ExtraLife => PINK,
                    CollectibleType::DoubleJump => GREEN,
                };

                self.particles.emit_collect(
                    collectible.position.x,
                    collectible.position.y,
                    color,
                );
                self.audio.play_collect();
            }
        }

        // Check collisions with checkpoints
        for (i, checkpoint) in level.checkpoints.iter_mut().enumerate() {
            if checkpoint.activated {
                continue;
            }

            if player_aabb.intersects(&checkpoint.aabb()) {
                checkpoint.activate();
                level.last_checkpoint = i + 1;
                self.hud.show_message("Checkpoint Activated!", 2.0);
                self.audio.play_checkpoint();
            }
        }

        // Check goal collision
        let goal_aabb = AABB::new(
            level.data.goal_x - 30.0,
            level.data.goal_y - 30.0,
            60.0,
            60.0,
        );
        if player_aabb.intersects(&goal_aabb) {
            level.completed = true;
            self.audio.play_level_complete();
            self.load_level(self.current_level + 1);
            return;
        }

        // Check if player fell off the map or died
        if self.player.body.position.y > level.data.height || self.player.is_dead() {
            if self.player.lives <= 0 {
                self.state = GameState::GameOver;
                self.audio.play_game_over();
            } else {
                let (spawn_x, spawn_y) = level.get_spawn_point();
                self.player.respawn(spawn_x, spawn_y);
                level.reset();
            }
        }

        // Update camera
        self.camera.follow(
            self.player.body.position.x + self.player.body.size.x / 2.0,
            self.player.body.position.y + self.player.body.size.y / 2.0,
        );
        self.camera.update(delta_time);

        // Update particles
        self.particles.update(delta_time);
    }

    fn draw(&self) {
        match self.state {
            GameState::MainMenu => {
                self.menu.draw("RUST PLATFORMER");
            }
            GameState::Playing | GameState::Paused => {
                self.draw_gameplay();

                if self.state == GameState::Paused {
                    // Draw pause overlay
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::new(0.0, 0.0, 0.0, 0.5),
                    );

                    let text = "PAUSED";
                    let text_width = measure_text(text, None, 60, 1.0).width;
                    draw_text(
                        text,
                        screen_width() / 2.0 - text_width / 2.0,
                        screen_height() / 2.0,
                        60.0,
                        WHITE,
                    );

                    let hint = "ESC to resume, Q to quit";
                    let hint_width = measure_text(hint, None, 20, 1.0).width;
                    draw_text(
                        hint,
                        screen_width() / 2.0 - hint_width / 2.0,
                        screen_height() / 2.0 + 50.0,
                        20.0,
                        GRAY,
                    );
                }
            }
            GameState::GameOver => {
                ui::draw_game_over(self.player.score, self.current_level + 1);
            }
            GameState::Victory => {
                ui::draw_victory(self.player.score, self.levels.len());
            }
        }
    }

    fn draw_gameplay(&self) {
        // Draw background with parallax
        self.background.draw(self.camera.position.x);

        // Apply camera
        self.camera.apply();

        // Draw level
        let level = &self.levels[self.current_level];
        level.draw();

        // Draw player
        self.player.draw();

        // Draw particles
        self.particles.draw();

        // Reset camera for UI
        self.camera.reset();

        // Draw HUD
        self.hud.draw(
            self.player.health,
            self.player.lives,
            self.player.score,
            self.player.coins,
            self.current_level + 1,
        );

        // Draw controls hint at bottom
        draw_text(
            "Controls: Arrow Keys/WASD to move, SPACE to jump",
            20.0,
            screen_height() - 10.0,
            16.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Platformer - Coding Challenge".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let delta_time = get_frame_time();

        game.update(delta_time);
        game.draw();

        next_frame().await;
    }
}
