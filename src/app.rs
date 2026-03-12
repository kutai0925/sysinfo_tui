/******************************************************************************
 * Project: system_inspector_tui
 * File: app.rs
 * Date: 05.03.2026
 * Author: Korawit Utai
 *
 * 
 *
******************************************************************************/






use crate::metrics::Metrics;
use crate::ui::draw_ui;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Cpu,
    Memory,
    Temp,
    All,
}

pub struct App {
    pub mode: ViewMode,
    pub status: String,
    pub metrics: Metrics,
}

impl App {
    pub fn new() -> Self {
        let mut metrics = Metrics::new();
        metrics.refresh_all();

        Self {
            mode: ViewMode::All,
            status: "Ready. Keys: 1/2/3/a, r=refresh, q=quit".into(),
            metrics,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, self))?;

            if event::poll(Duration::from_millis(200))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('1') => {
                            self.mode = ViewMode::Cpu;
                            self.status = "View: CPU".into();
                        }
                        KeyCode::Char('2') => {
                            self.mode = ViewMode::Memory;
                            self.status = "View: Memory".into();
                        }
                        KeyCode::Char('3') => {
                            self.mode = ViewMode::Temp;
                            self.status = "View: Temperature".into();
                        }
                        KeyCode::Char('a') => {
                            self.mode = ViewMode::All;
                            self.status = "View: All".into();
                        }
                        KeyCode::Char('r') => {
                            self.metrics.refresh_all();
                            self.status = "Refreshed.".into();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}