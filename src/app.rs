use color_eyre::eyre::Result;
use ratatui::widgets::ListState;

use crate::{cargo_ops::metadata::load_dependencies, models::dependency::Dependency};

#[derive(Debug)]
pub struct App {
    pub dependencies: Vec<Dependency>,
    pub selected: ListState,
}

impl App {
    pub fn new() -> Result<Self> {
        let dependencies = load_dependencies()?;

        let mut selected = ListState::default();
        if !dependencies.is_empty() {
            selected.select(Some(0));
        }

        Ok(Self {
            dependencies,
            selected,
        })
    }

    pub fn next(&mut self) {
        let i = self.selected.selected().unwrap_or(0);
        let next = (i + 1) % self.dependencies.len();
        self.selected.select(Some(next));
    }

    pub fn previous(&mut self) {
        let i = self.selected.selected().unwrap_or(0);
        let prev = if i == 0 {
            self.dependencies.len().saturating_sub(1)
        } else {
            i - 1
        };
        self.selected.select(Some(prev));
    }

    pub fn selected_dep(&self) -> Option<&Dependency> {
        self.selected
            .selected()
            .and_then(|i| self.dependencies.get(i))
    }
}
