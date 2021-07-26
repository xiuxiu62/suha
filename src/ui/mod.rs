// terminal.clear()?;
// loop {
//     terminal.draw(|f| {
//         let chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .margin(0)
//             .constraints(
//                 [
//                     Constraint::Percentage(10),
//                     Constraint::Percentage(80),
//                     Constraint::Percentage(10),
//                 ]
//                 .as_ref(),
//             )
//             .split(f.size());
//         render_block(f, "Block 1", chunks[0]);
//         render_block(f, "Block 2", chunks[1]);
//         render_block(f, "Block 3", chunks[2]);
//     })?;
//     thread::sleep(TICK_RATE);
// }

// fn render_block(frame: &mut TermFrame, title: &str, chunk: Rect) {
//     let block = Block::default()
//         .title(title)
//         .borders(Borders::ALL)
//         .border_style(Style::default().fg(Color::Cyan));
//     frame.render_widget(block, chunk);
// }
