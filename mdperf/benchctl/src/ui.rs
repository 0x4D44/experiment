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

    fn color(self) -> Color {
        match self {
            UiStatus::Pending => Color::Gray,
            UiStatus::Running => Color::Yellow,
            UiStatus::Success => Color::Green,
            UiStatus::Failure => Color::Red,
            UiStatus::Skipped => Color::Blue,
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

#[derive(Default)]
struct UiApp {
    rows: Vec<ModuleRow>,
    exit_requested: bool,
    banner: String,
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

        let header_text = if self.rows.is_empty() {
            if self.banner.is_empty() {
                "benchctl – waiting for modules..."
            } else {
                self.banner.as_str()
            }
        } else {
            if self.banner.is_empty() {
                "benchctl – module status"
            } else {
                self.banner.as_str()
            }
        };
        let header = Paragraph::new(Line::from(header_text))
            .block(Block::default().title("benchctl").borders(Borders::ALL));
        f.render_widget(header, layout[0]);

        let rows: Vec<Row> = self
            .rows
            .iter()
            .map(|row| {
                Row::new(vec![
                    Cell::from(row.name.clone()),
                    Cell::from(Span::styled(
                        row.status.as_str(),
                        Style::default().fg(row.status.color()),
                    )),
                    Cell::from(row.detail.clone().unwrap_or_default()),
                ])
            })
            .collect();

        let widths = [
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Percentage(58),
        ];
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec!["Module", "Status", "Detail"])
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .block(Block::default().title("Modules").borders(Borders::ALL))
            .column_spacing(2);

        f.render_widget(table, layout[1]);
    }
}

struct ModuleRow {
    name: String,
    status: UiStatus,
    detail: Option<String>,
}
