use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub locked_version: Option<String>,
    pub kind: String,
    pub enabled_features: Vec<String>,
    pub disabled_features: Vec<String>,
    // pub all_features: Vec<String>,
}

impl Dependency {
    pub fn kind_color(&self) -> Color {
        match self.kind.as_str() {
            "normal" => Color::Green,
            "dev" => Color::Blue,
            "build" => Color::Magenta,
            s if s.starts_with("target(") => Color::Cyan,
            _ => Color::DarkGray,
        }
    }

    pub fn compact_line(&self) -> Line<'static> {
        let spans = vec![
            Span::raw(format!("{: <20}", &self.name)),
            if let Some(locked) = &self.locked_version {
                if locked != &self.version {
                    Span::styled(
                        format!("{: <12} → {}", locked, self.version),
                        Style::default().fg(Color::Yellow),
                    )
                } else {
                    Span::raw(format!("{: <12}", &self.version))
                }
            } else {
                Span::raw(format!("{: <12}", &self.version))
            },
            Span::styled(
                format!("{: <10}", &self.kind),
                Style::default().fg(self.kind_color()),
            ),
        ];

        Line::from(spans)
    }

    pub fn features_lines(&self) -> Vec<Line<'static>> {
        let mut lines =
            Vec::with_capacity(self.enabled_features.len() + self.disabled_features.len() + 3);

        // Enabled section
        if !self.enabled_features.is_empty() {
            lines.push(Line::from(Span::styled(
                "Enabled features:",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )));

            for feat in &self.enabled_features {
                lines.push(Line::from(vec![
                    Span::styled("  [x] ", Style::default().fg(Color::Green)),
                    Span::styled(feat.to_string(), Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Disabled section
        if !self.disabled_features.is_empty() {
            for feat in &self.disabled_features {
                lines.push(Line::from(vec![
                    Span::styled("  [ ] ", Style::default().fg(Color::DarkGray)),
                    Span::raw(feat.to_string()),
                ]));
            }
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "- no features available",
                Style::default().fg(Color::DarkGray),
            )));
        }

        lines
    }
}
