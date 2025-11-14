use std::io::{self, IsTerminal};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

#[derive(Clone, Debug)]
pub enum UiMessage {
    Register {
        name: String,
    },
    SetBanner {
        text: String,
    },
    Update {
        name: String,
        status: UiStatus,
        detail: Option<String>,
    },
    Shutdown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiStatus {
    Pending,
    Running,
    Success,
    Failure,
    Skipped,
}

impl UiStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            UiStatus::Success | UiStatus::Failure | UiStatus::Skipped
        )
    }

    fn as_str(self) -> &'static str {
        match self {
            UiStatus::Pending => "pending",
            UiStatus::Running => "running",
            UiStatus::Success => "success",
            UiStatus::Failure => "failure",
            UiStatus::Skipped => "skipped",
        }
    }
}

pub type UiSender = Sender<UiMessage>;

pub struct UiController {
    sender: Option<UiSender>,
    handle: Option<thread::JoinHandle<()>>,
}

impl UiController {
    pub fn start(enable: bool) -> Result<Self> {
        if !enable {
            return Ok(Self {
                sender: None,
                handle: None,
            });
        }

        let (tx, rx) = mpsc::channel();
        let use_terminal = io::stdout().is_terminal();
        let handle = thread::Builder::new()
            .name("benchctl-ui".into())
            .spawn(move || {
                if use_terminal {
                    if let Err(err) = run_terminal_ui(rx) {
                        eprintln!("TUI error: {err:?}");
                    }
                } else {
                    run_plain_ui(rx);
                }
            })?;

        Ok(Self {
            sender: Some(tx),
            handle: Some(handle),
        })
    }

    pub fn sender(&self) -> Option<UiSender> {
        self.sender.as_ref().map(|tx| tx.clone())
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.sender.take() {
            let _ = tx.send(UiMessage::Shutdown);
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for UiController {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn run_plain_ui(rx: Receiver<UiMessage>) {
    while let Ok(msg) = rx.recv() {
        match msg {
            UiMessage::Register { name } => {
                println!("[pending] {name}");
            }
            UiMessage::SetBanner { text } => {
                println!("{text}");
            }
            UiMessage::Update {
                name,
                status,
                detail,
            } => {
                let detail_suffix = detail.map(|d| format!(" - {d}")).unwrap_or_default();
                println!("[{}] {}{}", status.as_str(), name, detail_suffix);
            }
            UiMessage::Shutdown => break,
        }
    }
}

fn run_terminal_ui(rx: Receiver<UiMessage>) -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = UiApp::default();

    loop {
        while let Ok(msg) = rx.try_recv() {
            app.handle(msg);
        }

        terminal.draw(|f| app.draw(f))?;

        if app.should_exit() {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;
    terminal.show_cursor()?;
    Ok(())
}

struct UiApp {
    rows: Vec<ModuleRow>,
    exit_requested: bool,
    banner: String,
    theme: Theme,
}

impl Default for UiApp {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            exit_requested: false,
            banner: String::new(),
            theme: Theme::default(),
        }
    }
}

impl UiApp {
    fn handle(&mut self, msg: UiMessage) {
        match msg {
            UiMessage::Register { name } => {
                if !self.rows.iter().any(|r| r.name == name) {
                    self.rows.push(ModuleRow {
                        name,
                        status: UiStatus::Pending,
                        detail: None,
                    });
                }
            }
            UiMessage::Update {
                name,
                status,
                detail,
            } => {
                if let Some(row) = self.rows.iter_mut().find(|r| r.name == name) {
                    row.status = status;
                    row.detail = detail;
                } else {
                    self.rows.push(ModuleRow {
                        name,
                        status,
                        detail,
                    });
                }
            }
            UiMessage::SetBanner { text } => {
                self.banner = text;
            }
            UiMessage::Shutdown => {
                self.exit_requested = true;
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.exit_requested && self.rows.iter().all(|row| row.status.is_terminal())
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.size();
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        let banner_text = if self.banner.is_empty() {
            if self.rows.is_empty() {
                "waiting for modules..."
            } else {
                "module status"
            }
        } else {
            self.banner.as_str()
        };
        let header_line = Line::from(vec![
            Span::styled(
                "benchctl",
                Style::default()
                    .fg(self.theme.header_title_fg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                banner_text,
                Style::default().fg(if self.banner.is_empty() {
                    self.theme.banner_muted
                } else {
                    self.theme.banner_primary
                }),
            ),
        ]);
        let header = Paragraph::new(header_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.header_border))
                .style(Style::default().bg(self.theme.header_bg)),
        );
        f.render_widget(header, layout[0]);

        let rows: Vec<Row> = self
            .rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let palette = palette_for_module(&row.name);
                let (badge_text, badge_style) = status_badge(row.status, &self.theme);
                let (detail_text, detail_style) = detail_cell(&row.detail, &palette, &self.theme);
                Row::new(vec![
                    Cell::from(format!(" {}", row.name)).style(
                        Style::default()
                            .fg(palette.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from(badge_text).style(badge_style),
                    Cell::from(detail_text).style(detail_style),
                ])
                .style(Style::default().bg(row_background(
                    idx,
                    row.status,
                    &self.theme,
                    &palette,
                )))
            })
            .collect();

        let widths = [
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Percentage(58),
        ];
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec!["Module", "Status", "Detail"]).style(
                    Style::default()
                        .fg(self.theme.table_header_fg)
                        .bg(self.theme.table_header_bg)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .block(
                Block::default()
                    .title(Span::styled(
                        "Modules",
                        Style::default()
                            .fg(self.theme.header_title_fg)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.table_border))
                    .style(Style::default().bg(self.theme.table_bg)),
            )
            .column_spacing(2);

        f.render_widget(table, layout[1]);
    }
}

struct ModuleRow {
    name: String,
    status: UiStatus,
    detail: Option<String>,
}

struct ModulePalette {
    accent: Color,
    detail: Color,
    glow: Color,
}

fn palette_for_module(name: &str) -> ModulePalette {
    match name.to_lowercase().as_str() {
        "cpu" => ModulePalette {
            accent: Color::Rgb(255, 163, 72),
            detail: Color::Rgb(255, 214, 170),
            glow: Color::Rgb(45, 30, 20),
        },
        "memory" => ModulePalette {
            accent: Color::Rgb(91, 198, 255),
            detail: Color::Rgb(170, 225, 255),
            glow: Color::Rgb(15, 32, 50),
        },
        "disk" => ModulePalette {
            accent: Color::Rgb(224, 150, 255),
            detail: Color::Rgb(240, 200, 255),
            glow: Color::Rgb(44, 22, 46),
        },
        "network" => ModulePalette {
            accent: Color::Rgb(120, 255, 184),
            detail: Color::Rgb(184, 255, 220),
            glow: Color::Rgb(12, 36, 32),
        },
        _ => ModulePalette {
            accent: Color::Rgb(200, 200, 200),
            detail: Color::Rgb(170, 170, 170),
            glow: Color::Rgb(25, 25, 30),
        },
    }
}

fn status_badge(status: UiStatus, theme: &Theme) -> (String, Style) {
    let swatch = theme.status_palette.swatch(status);
    let label = format!(" {} ", status.as_str().to_uppercase());
    let style = Style::default()
        .fg(swatch.fg)
        .bg(swatch.bg)
        .add_modifier(Modifier::BOLD);
    (label, style)
}

fn detail_cell(detail: &Option<String>, palette: &ModulePalette, theme: &Theme) -> (String, Style) {
    if let Some(text) = detail {
        if !text.trim().is_empty() {
            return (text.clone(), Style::default().fg(palette.detail));
        }
    }
    (
        "waiting for metrics".to_string(),
        Style::default()
            .fg(theme.placeholder_fg)
            .add_modifier(Modifier::ITALIC),
    )
}

fn row_background(index: usize, status: UiStatus, theme: &Theme, palette: &ModulePalette) -> Color {
    match status {
        UiStatus::Running => palette.glow,
        UiStatus::Success => theme.success_bg,
        UiStatus::Failure => theme.failure_bg,
        UiStatus::Skipped => theme.skipped_bg,
        UiStatus::Pending => theme.zebra(index),
    }
}

#[derive(Clone, Copy)]
struct Theme {
    header_bg: Color,
    header_border: Color,
    header_title_fg: Color,
    banner_primary: Color,
    banner_muted: Color,
    table_border: Color,
    table_bg: Color,
    table_header_bg: Color,
    table_header_fg: Color,
    zebra_dark: Color,
    zebra_light: Color,
    success_bg: Color,
    failure_bg: Color,
    skipped_bg: Color,
    placeholder_fg: Color,
    status_palette: StatusPalette,
}

impl Theme {
    fn default() -> Self {
        Self {
            header_bg: Color::Rgb(10, 12, 24),
            header_border: Color::Rgb(70, 80, 134),
            header_title_fg: Color::Rgb(210, 222, 255),
            banner_primary: Color::Rgb(236, 240, 255),
            banner_muted: Color::Rgb(140, 148, 182),
            table_border: Color::Rgb(58, 64, 102),
            table_bg: Color::Rgb(8, 10, 20),
            table_header_bg: Color::Rgb(18, 22, 38),
            table_header_fg: Color::Rgb(213, 222, 255),
            zebra_dark: Color::Rgb(13, 15, 27),
            zebra_light: Color::Rgb(17, 20, 33),
            success_bg: Color::Rgb(16, 32, 27),
            failure_bg: Color::Rgb(45, 20, 24),
            skipped_bg: Color::Rgb(20, 24, 38),
            placeholder_fg: Color::Rgb(120, 128, 160),
            status_palette: StatusPalette::default(),
        }
    }

    fn zebra(&self, index: usize) -> Color {
        if index % 2 == 0 {
            self.zebra_dark
        } else {
            self.zebra_light
        }
    }
}

#[derive(Clone, Copy)]
struct StatusPalette {
    pending: StatusSwatch,
    running: StatusSwatch,
    success: StatusSwatch,
    failure: StatusSwatch,
    skipped: StatusSwatch,
}

impl StatusPalette {
    fn swatch(&self, status: UiStatus) -> StatusSwatch {
        match status {
            UiStatus::Pending => self.pending,
            UiStatus::Running => self.running,
            UiStatus::Success => self.success,
            UiStatus::Failure => self.failure,
            UiStatus::Skipped => self.skipped,
        }
    }
}

impl Default for StatusPalette {
    fn default() -> Self {
        Self {
            pending: StatusSwatch {
                fg: Color::Rgb(196, 203, 228),
                bg: Color::Rgb(36, 40, 62),
            },
            running: StatusSwatch {
                fg: Color::Rgb(255, 222, 168),
                bg: Color::Rgb(63, 42, 18),
            },
            success: StatusSwatch {
                fg: Color::Rgb(159, 255, 208),
                bg: Color::Rgb(20, 52, 42),
            },
            failure: StatusSwatch {
                fg: Color::Rgb(255, 155, 155),
                bg: Color::Rgb(64, 24, 27),
            },
            skipped: StatusSwatch {
                fg: Color::Rgb(190, 202, 255),
                bg: Color::Rgb(30, 40, 64),
            },
        }
    }
}

#[derive(Clone, Copy)]
struct StatusSwatch {
    fg: Color,
    bg: Color,
}
