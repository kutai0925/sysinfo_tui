/******************************************************************************
 * Project: system_inspector_tui
 * File: ui.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/















use crate::app::{App, ViewMode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(0),    
            Constraint::Length(2),
        ])
        .split(area);

   
    let header = Paragraph::new("SYSINFO_TUI")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    
    let content_text = match app.mode {
        ViewMode::Cpu => app.metrics.cpu_text(),
        ViewMode::Memory => app.metrics.mem_text(),
        ViewMode::Temp => app.metrics.temp_text(),
        ViewMode::All => format!(
            "{}\n\n{}\n\n{}",
            app.metrics.cpu_text(),
            app.metrics.mem_text(),
            app.metrics.temp_text()
        ),
    };

    let content = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    f.render_widget(content, chunks[1]);

 
    let footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(chunks[2]);

    let help_text = "Keys: 1=CPU  2=MEM  3=TEMP  a=ALL  r=REFRESH  q=QUIT";
    let help = Paragraph::new(help_text).style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(help, footer[0]);

    let status = Paragraph::new(app.status.as_str());
    f.render_widget(status, footer[1]);
}