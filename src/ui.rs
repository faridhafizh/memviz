use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Gauge, Paragraph, Sparkline,
    },
    Frame,
};
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // title
            Constraint::Length(4),  // RAM gauge
            Constraint::Length(6),  // RAM sparkline
            Constraint::Length(6),  // swap sparkline
            Constraint::Min(10),    // memory map canvas
        ])
        .split(frame.area());

    // Title bar
    let title = Paragraph::new("memviz - Linux Memory & ROM Map")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(title, chunks[0]);

    // RAM gauge
    let ram_gauge = Gauge::default()
        .block(Block::default().title("RAM usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.ram_percent() as u16);
    frame.render_widget(ram_gauge, chunks[1]);

    // RAM history sparkline
    let ram_spark = Sparkline::default()
        .block(
            Block::default()
                .title("RAM history (last 120 samples)")
                .borders(Borders::ALL),
        )
        .data(&app.ram_history)
        .max(app.mem.total);
    frame.render_widget(ram_spark, chunks[2]);

    // Swap history sparkline
    let swap_spark = Sparkline::default()
        .block(
            Block::default()
                .title("Swap history")
                .borders(Borders::ALL),
        )
        .data(&app.swap_history)
        .max(app.mem.swap_total.max(1));
    frame.render_widget(swap_spark, chunks[3]);

    // Physical memory / ROM map
    draw_memory_map(frame, chunks[4], app);
}

fn draw_memory_map(frame: &mut Frame, area: Rect, app: &App) {
    let max_addr = app.regions.iter().map(|r| r.end).max().unwrap_or(1) as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("Physical memory map (ROM highlighted)")
                .borders(Borders::ALL),
        )
        .x_bounds([0.0, max_addr])
        .y_bounds([0.0, 1.0])
        .paint(|ctx| {
            let height = 1.0;
            for region in &app.regions {
                let color = if region.label.to_lowercase().contains("rom") {
                    Color::Red
                } else if region.label.contains("System RAM") {
                    Color::Green
                } else if region.label.contains("reserved") {
                    Color::Yellow
                } else {
                    Color::Gray
                };

                ctx.draw(&Rectangle {
                    x: region.start as f64,
                    y: 0.0,
                    width: (region.end - region.start) as f64,
                    height,
                    color,
                });
            }
        });

    frame.render_widget(canvas, area);
}
