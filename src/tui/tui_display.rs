use std::io::{stdout, Stdout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph,
    },
    Frame, Terminal,
};
use crate::hardware::display::{Chip8Display, Display};

pub struct TuiDisplay {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    chip8_display: Chip8Display,
    counter: u64,
    should_quit: bool,
}

impl TuiDisplay {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            chip8_display: Chip8Display::new(),
            counter: 0,
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.draw()?;

            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        KeyCode::Char(' ') => {
                            self.counter += 1;
                        }
                        KeyCode::Char('r') => {
                            self.counter = 0;
                            self.chip8_display.clear();
                        }
                        KeyCode::Char('t') => {
                            // Toggle some pixels for demo
                            self.toggle_demo_pixels();
                        }
                        _ => {}
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let display_clone = self.chip8_display.clone();
        let counter = self.counter;

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Min(20),
                    Constraint::Length(5),
                ])
                .split(f.size());

            // Title block
            let title = Paragraph::new("CHIP-8 Emulator - TUI Mode")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title("CHIP-8 TUI"));
            f.render_widget(title, chunks[0]);

            // Main display area
            let display_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[1]);

            // CHIP-8 Display
            Self::render_chip8_display_static(&display_clone, f, display_area[0]);

            // Info panel
            Self::render_info_panel_static(&display_clone, counter, f, display_area[1]);

            // Controls
            let controls = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::raw("Space: Increment counter  "),
                    Span::raw("T: Toggle demo pixels"),
                ]),
                Line::from(vec![
                    Span::raw("R: Reset  "),
                    Span::raw("Q/Esc: Quit"),
                ]),
            ])
            .block(Block::default().borders(Borders::ALL).title("Controls"));
            f.render_widget(controls, chunks[2]);
        })?;

        Ok(())
    }

    fn render_chip8_display_static(display: &Chip8Display, f: &mut Frame, area: Rect) {
        // Create a simple text-based representation of the display
        let mut display_text = String::new();

        for y in 0..display.height() {
            for x in 0..display.width() {
                if display.get_pixel(x, y) != 0 {
                    display_text.push('█');
                } else {
                    display_text.push(' ');
                }
            }
            if y < display.height() - 1 {
                display_text.push('\n');
            }
        }

        let display_widget = Paragraph::new(display_text)
            .block(Block::default().borders(Borders::ALL).title("CHIP-8 Display (64x32)"))
            .style(Style::default().fg(Color::White));

        f.render_widget(display_widget, area);
    }

    fn render_info_panel_static(display: &Chip8Display, counter: u64, f: &mut Frame, area: Rect) {
        let info_items = vec![
            ListItem::new(format!("Counter: {}", counter)),
            ListItem::new(format!("Display: {}x{}",
                display.width(),
                display.height()
            )),
            ListItem::new("Status: Running"),
            ListItem::new("Mode: TUI"),
        ];

        let info_list = List::new(info_items)
            .block(Block::default().borders(Borders::ALL).title("Info"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");

        f.render_widget(info_list, area);
    }

    fn toggle_demo_pixels(&mut self) {
        // Create a simple demo pattern
        let patterns = [
            // Pattern 1: Simple cross
            vec![(32, 16), (31, 16), (33, 16), (32, 15), (32, 17)],
            // Pattern 2: Small square
            vec![(10, 10), (11, 10), (10, 11), (11, 11)],
            // Pattern 3: Diagonal line
            vec![(5, 5), (6, 6), (7, 7), (8, 8), (9, 9)],
        ];

        let pattern_idx = (self.counter as usize) % patterns.len();
        let pattern = &patterns[pattern_idx];

        // Clear display first
        self.chip8_display.clear();

        // Draw the pattern
        for &(x, y) in pattern {
            self.chip8_display.set_pixel(x, y, 1);
        }
    }
}

impl Drop for TuiDisplay {
    fn drop(&mut self) {
        // Restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}
