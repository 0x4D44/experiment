/// Main game state and logic
use macroquad::prelude::*;
use crate::board::{Board, Score, BOARD_WIDTH, BOARD_HEIGHT};
use crate::pieces::{Piece, PieceType};
use crate::particles::ParticleSystem;
use crate::storage::HighScores;

const CELL_SIZE: f32 = 35.0;
const BOARD_OFFSET_X: f32 = 250.0;
const BOARD_OFFSET_Y: f32 = 50.0;

#[derive(Debug, PartialEq, Eq)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    state: GameState,
    board: Board,
    current_piece: Option<Piece>,
    next_piece: PieceType,
    held_piece: Option<PieceType>,
    can_hold: bool,
    score: Score,
    fall_timer: f32,
    lock_delay_timer: f32,
    move_timer: f32,
    particles: ParticleSystem,
    high_scores: HighScores,
    // Input handling
    das_timer: f32,  // Delayed Auto Shift
    arr_timer: f32,  // Auto Repeat Rate
    last_rotate_cw: bool,
    last_rotate_ccw: bool,
    last_hold: bool,
    last_hard_drop: bool,
}

impl Game {
    pub fn new() -> Self {
        let next_piece = PieceType::random();
        let high_scores = HighScores::load();

        Game {
            state: GameState::Menu,
            board: Board::new(),
            current_piece: None,
            next_piece,
            held_piece: None,
            can_hold: true,
            score: Score::new(),
            fall_timer: 0.0,
            lock_delay_timer: 0.0,
            move_timer: 0.0,
            particles: ParticleSystem::new(),
            high_scores,
            das_timer: 0.0,
            arr_timer: 0.0,
            last_rotate_cw: false,
            last_rotate_ccw: false,
            last_hold: false,
            last_hard_drop: false,
        }
    }

    fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.board.reset();
        self.score = Score::new();
        self.current_piece = None;
        self.next_piece = PieceType::random();
        self.held_piece = None;
        self.can_hold = true;
        self.fall_timer = 0.0;
        self.lock_delay_timer = 0.0;
        self.particles.clear();
        self.spawn_piece();
    }

    fn spawn_piece(&mut self) {
        let piece = Piece::new(self.next_piece);
        self.next_piece = PieceType::random();

        if self.board.is_valid_position(&piece) {
            self.current_piece = Some(piece);
            self.can_hold = true;
            self.lock_delay_timer = 0.0;
        } else {
            // Game over - can't spawn piece
            self.game_over();
        }
    }

    fn game_over(&mut self) {
        self.state = GameState::GameOver;
        self.high_scores.add_score(self.score.points);
        self.high_scores.save();

        // Create explosion effect
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board.grid[y][x].filled {
                    let px = BOARD_OFFSET_X + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                    let py = BOARD_OFFSET_Y + y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                    self.particles.emit(px, py, self.board.grid[y][x].color, 5);
                }
            }
        }
    }

    fn hold_piece(&mut self) {
        if !self.can_hold {
            return;
        }

        // Safety check: ensure we have a current piece
        if self.current_piece.is_none() {
            return;
        }

        let current = self.current_piece.as_ref().unwrap().piece_type;

        if let Some(held) = self.held_piece {
            self.current_piece = Some(Piece::new(held));
            self.held_piece = Some(current);
        } else {
            self.held_piece = Some(current);
            self.spawn_piece();
        }

        self.can_hold = false;
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            let old_x = piece.x;
            let old_y = piece.y;

            piece.x += dx;
            piece.y += dy;

            if !self.board.is_valid_position(piece) {
                piece.x = old_x;
                piece.y = old_y;
                return false;
            }

            // Reset lock delay if moved sideways
            if dy == 0 && dx != 0 {
                self.lock_delay_timer = 0.0;
            }

            return true;
        }
        false
    }

    fn try_rotate(&mut self, clockwise: bool) {
        if let Some(ref mut piece) = self.current_piece {
            let old_rotation = piece.rotation;
            let old_x = piece.x;
            let old_y = piece.y;

            if clockwise {
                piece.rotate_cw();
            } else {
                piece.rotate_ccw();
            }

            // Try wall kicks
            let kicks = piece.piece_type.get_wall_kicks(old_rotation, piece.rotation);

            for (kick_x, kick_y) in kicks {
                piece.x = old_x + kick_x;
                piece.y = old_y + kick_y;

                if self.board.is_valid_position(piece) {
                    self.lock_delay_timer = 0.0;
                    return;
                }
            }

            // Rotation failed, restore original state
            piece.rotation = old_rotation;
            piece.x = old_x;
            piece.y = old_y;
        }
    }

    fn hard_drop(&mut self) {
        if let Some(ref piece) = self.current_piece {
            let start_y = piece.y;
            let ghost_y = self.board.ghost_y(piece);
            let distance = ghost_y - start_y;

            if distance > 0 {
                self.score.add_hard_drop(distance as u32);
            }

            if let Some(ref mut piece) = self.current_piece {
                piece.y = ghost_y;
            }

            self.lock_piece();

            // Check for line clears and spawn new piece if none
            let lines_cleared = self.board.check_lines();
            if lines_cleared == 0 {
                self.spawn_piece();
            }
        }
    }

    fn lock_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            // Create particle effects
            for (x, y) in piece.blocks() {
                let px = BOARD_OFFSET_X + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                let py = BOARD_OFFSET_Y + y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                self.particles.emit(px, py, piece.color(), 3);
            }

            self.board.lock_piece(&piece);

            let lines_cleared = self.board.check_lines();
            if lines_cleared > 0 {
                // Emit particles for line clear
                for &line in &self.board.clearing_lines {
                    for x in 0..BOARD_WIDTH {
                        let px = BOARD_OFFSET_X + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                        let py = BOARD_OFFSET_Y + line as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                        self.particles.emit(px, py, self.board.grid[line][x].color, 8);
                    }
                }
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        // Update line clearing animation
        if self.board.update_clear_animation(dt) {
            let lines_cleared = self.board.clearing_lines.len();
            if lines_cleared > 0 {
                self.score.add_line_clear(lines_cleared);
            }
            self.spawn_piece();
            return;
        }

        if self.board.is_clearing() {
            return;
        }

        // Handle input
        self.handle_input(dt);

        // Auto-fall
        if self.current_piece.is_some() {
            self.fall_timer += dt;
            let fall_speed = self.score.fall_speed();

            if self.fall_timer >= fall_speed {
                self.fall_timer = 0.0;

                if !self.try_move(0, 1) {
                    // Can't move down, start lock delay
                    self.lock_delay_timer += fall_speed;

                    const LOCK_DELAY: f32 = 0.5;
                    if self.lock_delay_timer >= LOCK_DELAY {
                        self.lock_piece();
                        let lines_cleared = self.board.check_lines();
                        if lines_cleared == 0 {
                            self.spawn_piece();
                        }
                    }
                } else {
                    self.lock_delay_timer = 0.0;
                }
            }
        }
    }

    fn handle_input(&mut self, dt: f32) {
        const DAS_DELAY: f32 = 0.15;  // Delayed Auto Shift delay
        const ARR_RATE: f32 = 0.03;   // Auto Repeat Rate

        // Rotation (single press)
        let rotate_cw = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X);
        let rotate_ccw = is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::LeftControl);

        if rotate_cw && !self.last_rotate_cw {
            self.try_rotate(true);
        }
        if rotate_ccw && !self.last_rotate_ccw {
            self.try_rotate(false);
        }

        self.last_rotate_cw = rotate_cw;
        self.last_rotate_ccw = rotate_ccw;

        // Hold piece (single press)
        let hold = is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::LeftShift);
        if hold && !self.last_hold {
            self.hold_piece();
        }
        self.last_hold = hold;

        // Hard drop (single press)
        let hard_drop = is_key_pressed(KeyCode::Space);
        if hard_drop && !self.last_hard_drop {
            self.hard_drop();
        }
        self.last_hard_drop = hard_drop;

        // Horizontal movement with DAS/ARR
        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);

        if left && !right {
            self.das_timer += dt;
            self.arr_timer += dt;

            if is_key_pressed(KeyCode::Left) {
                self.try_move(-1, 0);
                self.das_timer = 0.0;
                self.arr_timer = 0.0;
            } else if self.das_timer >= DAS_DELAY && self.arr_timer >= ARR_RATE {
                self.try_move(-1, 0);
                self.arr_timer = 0.0;
            }
        } else if right && !left {
            self.das_timer += dt;
            self.arr_timer += dt;

            if is_key_pressed(KeyCode::Right) {
                self.try_move(1, 0);
                self.das_timer = 0.0;
                self.arr_timer = 0.0;
            } else if self.das_timer >= DAS_DELAY && self.arr_timer >= ARR_RATE {
                self.try_move(1, 0);
                self.arr_timer = 0.0;
            }
        } else {
            self.das_timer = 0.0;
            self.arr_timer = 0.0;
        }

        // Soft drop (continuous)
        if is_key_down(KeyCode::Down) {
            self.move_timer += dt;
            const SOFT_DROP_SPEED: f32 = 0.05;

            if self.move_timer >= SOFT_DROP_SPEED {
                if self.try_move(0, 1) {
                    self.score.add_soft_drop(1);
                }
                self.move_timer = 0.0;
            }
        } else {
            self.move_timer = 0.0;
        }

        // Pause
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
            self.state = GameState::Paused;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.particles.update(dt);

        match self.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    self.start_game();
                }
            }
            GameState::Playing => {
                self.update_playing(dt);
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
                    self.state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Q) {
                    self.state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    self.start_game();
                } else if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Playing => self.draw_playing(),
            GameState::Paused => {
                self.draw_playing();
                self.draw_pause_overlay();
            }
            GameState::GameOver => {
                self.draw_playing();
                self.draw_game_over();
            }
        }

        self.particles.draw();
    }

    fn draw_menu(&self) {
        let title = "TETRIS CHAMPION";
        let title_size = 60.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            (screen_width() - title_dims.width) / 2.0,
            200.0,
            title_size,
            YELLOW,
        );

        let subtitle = "Coding Challenge Edition";
        let subtitle_size = 25.0;
        let subtitle_dims = measure_text(subtitle, None, subtitle_size as u16, 1.0);
        draw_text(
            subtitle,
            (screen_width() - subtitle_dims.width) / 2.0,
            240.0,
            subtitle_size,
            LIGHTGRAY,
        );

        // High score
        let high_score = format!("High Score: {}", self.high_scores.get_high_score());
        let hs_dims = measure_text(&high_score, None, 30, 1.0);
        draw_text(
            &high_score,
            (screen_width() - hs_dims.width) / 2.0,
            320.0,
            30.0,
            GOLD,
        );

        // Controls
        let controls = vec![
            "Controls:",
            "",
            "Arrow Keys - Move",
            "Up/X - Rotate CW",
            "Z/Ctrl - Rotate CCW",
            "Space - Hard Drop",
            "Down - Soft Drop",
            "C/Shift - Hold",
            "P/Esc - Pause",
            "",
            "Press ENTER to Start",
        ];

        let start_y = 400.0;
        for (i, line) in controls.iter().enumerate() {
            let dims = measure_text(line, None, 20, 1.0);
            draw_text(
                line,
                (screen_width() - dims.width) / 2.0,
                start_y + i as f32 * 30.0,
                20.0,
                WHITE,
            );
        }
    }

    fn draw_playing(&self) {
        // Draw board background
        draw_rectangle(
            BOARD_OFFSET_X - 5.0,
            BOARD_OFFSET_Y - 5.0,
            BOARD_WIDTH as f32 * CELL_SIZE + 10.0,
            BOARD_HEIGHT as f32 * CELL_SIZE + 10.0,
            Color::from_rgba(30, 30, 40, 255),
        );

        // Draw grid
        for y in 0..=BOARD_HEIGHT {
            draw_line(
                BOARD_OFFSET_X,
                BOARD_OFFSET_Y + y as f32 * CELL_SIZE,
                BOARD_OFFSET_X + BOARD_WIDTH as f32 * CELL_SIZE,
                BOARD_OFFSET_Y + y as f32 * CELL_SIZE,
                1.0,
                Color::from_rgba(50, 50, 60, 255),
            );
        }
        for x in 0..=BOARD_WIDTH {
            draw_line(
                BOARD_OFFSET_X + x as f32 * CELL_SIZE,
                BOARD_OFFSET_Y,
                BOARD_OFFSET_X + x as f32 * CELL_SIZE,
                BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * CELL_SIZE,
                1.0,
                Color::from_rgba(50, 50, 60, 255),
            );
        }

        // Draw locked pieces
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board.grid[y][x].filled {
                    self.draw_cell(x as i32, y as i32, self.board.grid[y][x].color, 1.0);
                }
            }
        }

        // Draw ghost piece
        if let Some(ref piece) = self.current_piece {
            let ghost_y = self.board.ghost_y(piece);
            let mut ghost = piece.clone();
            ghost.y = ghost_y;

            for (x, y) in ghost.blocks() {
                if y >= 0 {
                    self.draw_cell(x, y, piece.color(), 0.2);
                }
            }
        }

        // Draw current piece
        if let Some(ref piece) = self.current_piece {
            for (x, y) in piece.blocks() {
                if y >= 0 {
                    self.draw_cell(x, y, piece.color(), 1.0);
                }
            }
        }

        // Draw clearing animation
        if !self.board.clearing_lines.is_empty() {
            let flash = ((self.board.clear_animation_timer * 10.0) as i32 % 2) as f32;
            for &line in &self.board.clearing_lines {
                for x in 0..BOARD_WIDTH {
                    self.draw_cell(x as i32, line as i32, WHITE, 0.5 + flash * 0.3);
                }
            }
        }

        // Draw UI
        self.draw_ui();
    }

    fn draw_cell(&self, x: i32, y: i32, color: Color, alpha: f32) {
        let px = BOARD_OFFSET_X + x as f32 * CELL_SIZE;
        let py = BOARD_OFFSET_Y + y as f32 * CELL_SIZE;

        let mut draw_color = color;
        draw_color.a = alpha;

        // Draw main block
        draw_rectangle(px + 2.0, py + 2.0, CELL_SIZE - 4.0, CELL_SIZE - 4.0, draw_color);

        // Draw highlight
        let mut highlight = draw_color;
        highlight.r = (highlight.r + 0.3).min(1.0);
        highlight.g = (highlight.g + 0.3).min(1.0);
        highlight.b = (highlight.b + 0.3).min(1.0);
        draw_rectangle(px + 2.0, py + 2.0, CELL_SIZE - 4.0, 3.0, highlight);
        draw_rectangle(px + 2.0, py + 2.0, 3.0, CELL_SIZE - 4.0, highlight);

        // Draw shadow
        let mut shadow = draw_color;
        shadow.r *= 0.5;
        shadow.g *= 0.5;
        shadow.b *= 0.5;
        draw_rectangle(px + 2.0, py + CELL_SIZE - 5.0, CELL_SIZE - 4.0, 3.0, shadow);
        draw_rectangle(px + CELL_SIZE - 5.0, py + 2.0, 3.0, CELL_SIZE - 4.0, shadow);
    }

    fn draw_ui(&self) {
        let ui_x = 50.0;
        let mut ui_y = 50.0;

        // Score
        draw_text("SCORE", ui_x, ui_y, 25.0, LIGHTGRAY);
        ui_y += 30.0;
        draw_text(&self.score.points.to_string(), ui_x, ui_y, 35.0, WHITE);
        ui_y += 60.0;

        // Level
        draw_text("LEVEL", ui_x, ui_y, 25.0, LIGHTGRAY);
        ui_y += 30.0;
        draw_text(&self.score.level.to_string(), ui_x, ui_y, 35.0, WHITE);
        ui_y += 60.0;

        // Lines
        draw_text("LINES", ui_x, ui_y, 25.0, LIGHTGRAY);
        ui_y += 30.0;
        draw_text(&self.score.lines_cleared.to_string(), ui_x, ui_y, 35.0, WHITE);
        ui_y += 60.0;

        // Combo
        if self.score.combo > 1 {
            draw_text("COMBO", ui_x, ui_y, 25.0, LIGHTGRAY);
            ui_y += 30.0;
            draw_text(&format!("{}x", self.score.combo), ui_x, ui_y, 35.0, YELLOW);
        }

        // Right side UI
        let right_x = BOARD_OFFSET_X + BOARD_WIDTH as f32 * CELL_SIZE + 30.0;
        let mut right_y = 50.0;

        // Next piece
        draw_text("NEXT", right_x, right_y, 25.0, LIGHTGRAY);
        right_y += 40.0;
        self.draw_preview_piece(self.next_piece, right_x, right_y);
        right_y += 120.0;

        // Hold piece
        draw_text("HOLD", right_x, right_y, 25.0, LIGHTGRAY);
        right_y += 40.0;
        if let Some(held) = self.held_piece {
            self.draw_preview_piece(held, right_x, right_y);
        }

        // High score
        let hs_y = BOARD_OFFSET_Y + BOARD_HEIGHT as f32 * CELL_SIZE - 50.0;
        draw_text("HIGH SCORE", right_x, hs_y, 20.0, LIGHTGRAY);
        draw_text(
            &self.high_scores.get_high_score().to_string(),
            right_x,
            hs_y + 25.0,
            25.0,
            GOLD,
        );
    }

    fn draw_preview_piece(&self, piece_type: PieceType, x: f32, y: f32) {
        let shape = piece_type.shape(0);
        let color = piece_type.color();

        let preview_size = 25.0;

        for (row, line) in shape.iter().enumerate() {
            for (col, &filled) in line.iter().enumerate() {
                if filled {
                    let px = x + col as f32 * preview_size;
                    let py = y + row as f32 * preview_size;

                    draw_rectangle(px, py, preview_size - 2.0, preview_size - 2.0, color);

                    // Highlight
                    let mut highlight = color;
                    highlight.r = (highlight.r + 0.3).min(1.0);
                    highlight.g = (highlight.g + 0.3).min(1.0);
                    highlight.b = (highlight.b + 0.3).min(1.0);
                    draw_rectangle(px, py, preview_size - 2.0, 2.0, highlight);
                    draw_rectangle(px, py, 2.0, preview_size - 2.0, highlight);
                }
            }
        }
    }

    fn draw_pause_overlay(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 180),
        );

        let text = "PAUSED";
        let size = 60.0;
        let dims = measure_text(text, None, size as u16, 1.0);
        draw_text(
            text,
            (screen_width() - dims.width) / 2.0,
            screen_height() / 2.0 - 50.0,
            size,
            WHITE,
        );

        let instruction = "Press P or ESC to continue";
        let inst_size = 25.0;
        let inst_dims = measure_text(instruction, None, inst_size as u16, 1.0);
        draw_text(
            instruction,
            (screen_width() - inst_dims.width) / 2.0,
            screen_height() / 2.0 + 20.0,
            inst_size,
            LIGHTGRAY,
        );

        let quit = "Press Q to quit";
        let quit_dims = measure_text(quit, None, 20, 1.0);
        draw_text(
            quit,
            (screen_width() - quit_dims.width) / 2.0,
            screen_height() / 2.0 + 60.0,
            20.0,
            LIGHTGRAY,
        );
    }

    fn draw_game_over(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 200),
        );

        let text = "GAME OVER";
        let size = 60.0;
        let dims = measure_text(text, None, size as u16, 1.0);
        draw_text(
            text,
            (screen_width() - dims.width) / 2.0,
            screen_height() / 2.0 - 100.0,
            size,
            RED,
        );

        let score_text = format!("Score: {}", self.score.points);
        let score_size = 35.0;
        let score_dims = measure_text(&score_text, None, score_size as u16, 1.0);
        draw_text(
            &score_text,
            (screen_width() - score_dims.width) / 2.0,
            screen_height() / 2.0 - 20.0,
            score_size,
            WHITE,
        );

        let lines_text = format!("Lines: {}", self.score.lines_cleared);
        let lines_dims = measure_text(&lines_text, None, 25, 1.0);
        draw_text(
            &lines_text,
            (screen_width() - lines_dims.width) / 2.0,
            screen_height() / 2.0 + 20.0,
            25.0,
            LIGHTGRAY,
        );

        if self.score.points == self.high_scores.get_high_score() && self.score.points > 0 {
            let new_high = "NEW HIGH SCORE!";
            let nh_dims = measure_text(new_high, None, 30, 1.0);
            draw_text(
                new_high,
                (screen_width() - nh_dims.width) / 2.0,
                screen_height() / 2.0 + 60.0,
                30.0,
                GOLD,
            );
        }

        let instruction = "Press ENTER to play again";
        let inst_dims = measure_text(instruction, None, 25, 1.0);
        draw_text(
            instruction,
            (screen_width() - inst_dims.width) / 2.0,
            screen_height() / 2.0 + 120.0,
            25.0,
            LIGHTGRAY,
        );

        let quit = "Press ESC for menu";
        let quit_dims = measure_text(quit, None, 20, 1.0);
        draw_text(
            quit,
            (screen_width() - quit_dims.width) / 2.0,
            screen_height() / 2.0 + 160.0,
            20.0,
            LIGHTGRAY,
        );
    }
}
