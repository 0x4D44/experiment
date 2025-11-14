use std::collections::HashSet;
use std::io::{self, IsTerminal};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, execute,
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
pub struct SubTestResult {
    pub name: String,
    pub value: String,
    pub unit: String,
}

#[derive(Clone, Debug)]
pub struct MemoryBandwidthPoint {
    pub buffer_size_kb: u64,
    pub copy_gbps: f64,
    pub scale_gbps: f64,
    pub triad_gbps: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreType {
    Performance,
    Efficient,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct CpuPerformancePoint {
    pub core_id: usize,
    pub core_type: CoreType,
    pub gops: f64,
    pub operation: String,
}

#[derive(Clone, Debug)]
pub enum ChartDataPoint {
    Memory(MemoryBandwidthPoint),
    Cpu(CpuPerformancePoint),
}

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
        sub_tests: Vec<SubTestResult>,
    },
    UpdateChart {
        data: ChartDataPoint,
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
                ..
            } => {
                let detail_suffix = detail.map(|d| format!(" - {d}")).unwrap_or_default();
                println!("[{}] {}{}", status.as_str(), name, detail_suffix);
            }
            UiMessage::UpdateChart { .. } => {
                // Ignored in plain UI mode
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
        // Process incoming messages from test orchestrator
        while let Ok(msg) = rx.try_recv() {
            app.handle(msg);
        }

        // Handle keyboard input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key);
            }
        }

        // Render UI
        terminal.draw(|f| app.draw(f))?;

        // Check exit condition
        if app.should_exit() {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;
    terminal.show_cursor()?;
    Ok(())
}

struct UiApp {
    rows: Vec<ModuleRow>,
    selected_index: Option<usize>,
    expanded_modules: HashSet<String>,
    exit_requested: bool,
    tests_complete: bool,
    banner: String,
    theme: Theme,
    memory_bandwidth_data: Vec<MemoryBandwidthPoint>,
    cpu_per_core_data: Vec<CpuPerformancePoint>,
}

impl Default for UiApp {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            selected_index: None,
            expanded_modules: HashSet::new(),
            exit_requested: false,
            tests_complete: false,
            banner: String::new(),
            theme: Theme::default(),
            memory_bandwidth_data: Vec::new(),
            cpu_per_core_data: Vec::new(),
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
                        sub_tests: Vec::new(),
                    });
                }
            }
            UiMessage::Update {
                name,
                status,
                detail,
                sub_tests,
            } => {
                if let Some(row) = self.rows.iter_mut().find(|r| r.name == name) {
                    row.status = status;
                    row.detail = detail;
                    row.sub_tests = sub_tests;
                } else {
                    self.rows.push(ModuleRow {
                        name,
                        status,
                        detail,
                        sub_tests,
                    });
                }

                // Check if all tests just completed
                if self.are_tests_complete() && !self.tests_complete {
                    self.tests_complete = true;
                    self.banner = "Tests complete • Press 'q' to quit • ↑↓ select • +/- expand".to_string();
                }
            }
            UiMessage::SetBanner { text } => {
                self.banner = text;
            }
            UiMessage::UpdateChart { data } => {
                match data {
                    ChartDataPoint::Memory(point) => {
                        self.memory_bandwidth_data.push(point);
                    }
                    ChartDataPoint::Cpu(point) => {
                        self.cpu_per_core_data.push(point);
                    }
                }
            }
            UiMessage::Shutdown => {
                self.exit_requested = true;
            }
        }
    }

    fn are_tests_complete(&self) -> bool {
        !self.rows.is_empty() && self.rows.iter().all(|row| row.status.is_terminal())
    }

    fn should_exit(&self) -> bool {
        self.exit_requested
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.exit_requested = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit_requested = true;
            }
            KeyCode::Down => {
                self.select_next();
            }
            KeyCode::Up => {
                self.select_previous();
            }
            KeyCode::Char('+') | KeyCode::Enter => {
                self.toggle_expand();
            }
            KeyCode::Char('-') => {
                self.collapse_selected();
            }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let next = self.selected_index
            .map(|i| (i + 1) % self.rows.len())
            .unwrap_or(0);
        self.selected_index = Some(next);
    }

    fn select_previous(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let prev = self.selected_index
            .map(|i| if i == 0 { self.rows.len() - 1 } else { i - 1 })
            .unwrap_or(0);
        self.selected_index = Some(prev);
    }

    fn toggle_expand(&mut self) {
        if let Some(idx) = self.selected_index {
            if let Some(row) = self.rows.get(idx) {
                let name = row.name.clone();
                if self.expanded_modules.contains(&name) {
                    self.expanded_modules.remove(&name);
                } else {
                    self.expanded_modules.insert(name);
                }
            }
        }
    }

    fn collapse_selected(&mut self) {
        if let Some(idx) = self.selected_index {
            if let Some(row) = self.rows.get(idx) {
                self.expanded_modules.remove(&row.name);
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.size();
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Module table + Charts
                Constraint::Length(1),  // Footer
            ])
            .split(area);

        // Split middle section into module list and charts
        let content_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),  // Module list
                Constraint::Percentage(60),  // Charts
            ])
            .split(layout[1]);

        // Render header
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

        // Build rows with expansion support
        let mut table_rows: Vec<Row> = Vec::new();
        for (idx, module_row) in self.rows.iter().enumerate() {
            let is_selected = self.selected_index == Some(idx);
            let is_expanded = self.expanded_modules.contains(&module_row.name);

            let mut rows = self.render_module_row(module_row, idx, is_selected, is_expanded);
            table_rows.append(&mut rows);
        }

        let widths = [
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Percentage(58),
        ];
        let table = Table::new(table_rows, widths)
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

        f.render_widget(table, content_layout[0]);

        // Render charts
        self.render_charts(content_layout[1], f);

        // Render footer
        self.render_footer(layout[2], f);
    }

    fn render_module_row(
        &self,
        row: &ModuleRow,
        idx: usize,
        is_selected: bool,
        is_expanded: bool,
    ) -> Vec<Row<'static>> {
        let mut rows = Vec::new();
        let palette = palette_for_module(&row.name);

        // Main module row
        let (badge_text, badge_style) = status_badge(row.status, &self.theme);
        let (detail_text, detail_style) = detail_cell(&row.detail, &palette, &self.theme);
        let prefix = if is_selected { "> " } else { "  " };
        let expand_indicator = if !row.sub_tests.is_empty() {
            if is_expanded { "▼ " } else { "▶ " }
        } else {
            ""
        };

        let main_row = Row::new(vec![
            Cell::from(format!("{}{}{}", prefix, expand_indicator, row.name))
                .style(Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD)),
            Cell::from(badge_text).style(badge_style),
            Cell::from(detail_text).style(detail_style),
        ])
        .style(Style::default().bg(
            if is_selected {
                self.theme.selected_bg
            } else {
                row_background(idx, row.status, &self.theme, &palette)
            }
        ));

        rows.push(main_row);

        // Sub-test rows (if expanded)
        if is_expanded {
            for sub_test in &row.sub_tests {
                let sub_row = Row::new(vec![
                    Cell::from(format!("    ├─ {}", sub_test.name))
                        .style(Style::default().fg(palette.detail)),
                    Cell::from(""),
                    Cell::from(format!("{} {}", sub_test.value, sub_test.unit))
                        .style(Style::default().fg(palette.detail)),
                ])
                .style(Style::default().bg(
                    if is_selected {
                        self.theme.selected_bg_dim
                    } else {
                        row_background(idx, row.status, &self.theme, &palette)
                    }
                ));
                rows.push(sub_row);
            }
        }

        rows
    }

    fn render_footer(&self, area: ratatui::layout::Rect, f: &mut Frame) {
        let status_text = if self.tests_complete {
            "Tests complete • Press 'q' to quit • ↑↓ select module • +/- expand/collapse"
        } else {
            "Running tests... • Ctrl+C to abort • ↑↓ select module • +/- expand/collapse"
        };

        let footer = Paragraph::new(status_text)
            .style(Style::default()
                .fg(if self.tests_complete {
                    Color::Green
                } else {
                    Color::Yellow
                })
                .add_modifier(Modifier::BOLD));

        f.render_widget(footer, area);
    }

    fn render_charts(&self, area: ratatui::layout::Rect, f: &mut Frame) {
        // Split chart area into two sections for memory and CPU charts
        let chart_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),  // Memory bandwidth chart
                Constraint::Percentage(50),  // CPU per-core chart
            ])
            .split(area);

        self.render_memory_bandwidth_chart(chart_layout[0], f);
        self.render_cpu_per_core_chart(chart_layout[1], f);
    }

    fn render_memory_bandwidth_chart(&self, area: ratatui::layout::Rect, f: &mut Frame) {
        if self.memory_bandwidth_data.is_empty() {
            let placeholder = Paragraph::new("Memory cache hierarchy test not yet run")
                .style(Style::default().fg(self.theme.placeholder_fg))
                .block(Block::default()
                    .title("Memory Bandwidth by Buffer Size (Cache Hierarchy)")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.table_border)));
            f.render_widget(placeholder, area);
            return;
        }

        // Simple text-based chart for now (will enhance with graphics later)
        let mut lines = vec![];
        lines.push(Line::from("Buffer Size → Bandwidth (GB/s)"));
        lines.push(Line::from("─".repeat(area.width as usize - 4)));

        for point in &self.memory_bandwidth_data {
            let size_str = if point.buffer_size_kb < 1024 {
                format!("{:>6} KB", point.buffer_size_kb)
            } else {
                format!("{:>6} MB", point.buffer_size_kb / 1024)
            };

            lines.push(Line::from(vec![
                Span::raw(format!("{:>8}: ", size_str)),
                Span::styled(
                    format!("copy {:.1}", point.copy_gbps),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("scale {:.1}", point.scale_gbps),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("triad {:.1}", point.triad_gbps),
                    Style::default().fg(Color::Magenta),
                ),
            ]));
        }

        let chart = Paragraph::new(lines)
            .block(Block::default()
                .title("Memory Bandwidth by Buffer Size (Cache Hierarchy)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.table_border)));

        f.render_widget(chart, area);
    }

    fn render_cpu_per_core_chart(&self, area: ratatui::layout::Rect, f: &mut Frame) {
        if self.cpu_per_core_data.is_empty() {
            let placeholder = Paragraph::new("CPU per-core test not yet run")
                .style(Style::default().fg(self.theme.placeholder_fg))
                .block(Block::default()
                    .title("CPU Performance by Core (E-core vs P-core)")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.table_border)));
            f.render_widget(placeholder, area);
            return;
        }

        // Simple text-based chart for now
        let mut lines = vec![];
        lines.push(Line::from("Core ID → Performance (GOPS)"));
        lines.push(Line::from("─".repeat(area.width as usize - 4)));

        for point in &self.cpu_per_core_data {
            let core_type_str = match point.core_type {
                CoreType::Performance => "P",
                CoreType::Efficient => "E",
                CoreType::Unknown => " ",
            };

            let color = match point.core_type {
                CoreType::Performance => Color::Cyan,
                CoreType::Efficient => Color::Green,
                CoreType::Unknown => Color::Gray,
            };

            lines.push(Line::from(vec![
                Span::raw(format!("Core {:>2} ", point.core_id)),
                Span::styled(format!("[{}]", core_type_str), Style::default().fg(color)),
                Span::raw(": "),
                Span::styled(
                    format!("{:.2} GOPS", point.gops),
                    Style::default().fg(color),
                ),
            ]));
        }

        let chart = Paragraph::new(lines)
            .block(Block::default()
                .title("CPU Performance by Core (E-core vs P-core)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.table_border)));

        f.render_widget(chart, area);
    }
}

struct ModuleRow {
    name: String,
    status: UiStatus,
    detail: Option<String>,
    sub_tests: Vec<SubTestResult>,
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
    selected_bg: Color,
    selected_bg_dim: Color,
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
            selected_bg: Color::Rgb(30, 35, 60),
            selected_bg_dim: Color::Rgb(20, 25, 45),
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
