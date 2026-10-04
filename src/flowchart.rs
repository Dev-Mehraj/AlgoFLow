use crate::ast::{Program, Statement, StepId};
use eframe::egui::{self, Color32, Pos2, Rect, Shape, Stroke, Vec2};

pub fn render_gui(ctx: &egui::Context, program: &Program) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(".algo GUI Viewer — Decision Branches & Loopbacks");
        ui.separator();

        let painter = ui.painter();
        let mut y_offset = 80.0_f32;
        let x_center = 300.0_f32;
        let node_height = 40.0_f32;
        let node_width = 160.0_f32;

        let mut node_positions = Vec::new();

        // 1. Position nodes vertically
        for stmt in &program.statements {
            let pos = Pos2::new(x_center, y_offset);
            match stmt {
                Statement::Start => node_positions.push((pos, "Start".to_string(), false, None)),
                Statement::End => node_positions.push((pos, "End".to_string(), false, None)),
                Statement::StatementChecker(text) => node_positions.push((pos, text.clone(), false, None)),
                Statement::IfThen { id, condition, .. } => {
                    node_positions.push((pos, format!("If {}: {}", id, condition), true, Some(*id)));
                }
                Statement::ReturnTo(target) => {
                    let target_id = match target {
                        StepId::Number(n) => *n,
                        _ => 1,
                    };
                    node_positions.push((pos, format!("Return to Step {}", target_id), false, Some(target_id)));
                }
            }
            y_offset += 90.0_f32;
        }

        // 2. Draw connecting arrows & loopbacks
        for i in 0..node_positions.len().saturating_sub(1) {
            let (pos, _, is_decision, _) = &node_positions[i];
            let (next_pos, _, _, _) = &node_positions[i + 1];

            if *is_decision {
                // Downward branch: "Yes"
                let start_down = Pos2::new(pos.x, pos.y + 25.0_f32);
                let end_down = Pos2::new(next_pos.x, next_pos.y - 20.0_f32);
                painter.line_segment([start_down, end_down], Stroke::new(2.0_f32, Color32::GRAY));
                painter.text(
                    Pos2::new(pos.x + 10.0_f32, pos.y + 30.0_f32),
                    egui::Align2::LEFT_TOP,
                    "Yes",
                    egui::FontId::proportional(12.0_f32),
                    Color32::DARK_GREEN,
                );

                // Rightward branch: "No" (bypasses to line below)
                let right_out = Pos2::new(pos.x + 90.0_f32, pos.y);
                let right_bypass = Pos2::new(pos.x + 140.0_f32, pos.y);
                let right_reconnect = Pos2::new(pos.x + 140.0_f32, next_pos.y + 45.0_f32);
                let reconnect_target = Pos2::new(next_pos.x + 80.0_f32, next_pos.y + 45.0_f32);

                let no_path = vec![right_out, right_bypass, right_reconnect, reconnect_target];
                for w in no_path.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(2.0_f32, Color32::LIGHT_RED));
                }
                painter.text(
                    Pos2::new(pos.x + 95.0_f32, pos.y - 15.0_f32),
                    egui::Align2::LEFT_BOTTOM,
                    "No",
                    egui::FontId::proportional(12.0_f32),
                    Color32::RED,
                );
            } else {
                let start_pos = Pos2::new(pos.x, pos.y + 20.0_f32);
                let end_pos = Pos2::new(next_pos.x, next_pos.y - 20.0_f32);
                painter.line_segment([start_pos, end_pos], Stroke::new(2.0_f32, Color32::GRAY));
            }

            // Draw Return-To Backwards Loop
            if let Statement::ReturnTo(StepId::Number(target_id)) = &program.statements[i] {
                // Find Step ID position
                if let Some((target_pos, _, _, _)) = node_positions.iter().find(|(_, _, _, id)| *id == Some(*target_id)) {
                    let loop_start = Pos2::new(pos.x - 80.0_f32, pos.y);
                    let loop_left = Pos2::new(pos.x - 180.0_f32, pos.y);
                    let loop_up = Pos2::new(pos.x - 180.0_f32, target_pos.y);
                    let loop_target = Pos2::new(target_pos.x - 90.0_f32, target_pos.y);

                    let loop_points = vec![loop_start, loop_left, loop_up, loop_target];
                    for w in loop_points.windows(2) {
                        painter.line_segment([w[0], w[1]], Stroke::new(2.0_f32, Color32::BLUE));
                    }

                    // Arrowhead pointing back right into the target rhombus
                    let arrow_top = Pos2::new(loop_target.x - 8.0_f32, loop_target.y - 5.0_f32);
                    let arrow_bottom = Pos2::new(loop_target.x - 8.0_f32, loop_target.y + 5.0_f32);
                    painter.add(Shape::convex_polygon(
                        vec![loop_target, arrow_top, arrow_bottom],
                        Color32::BLUE,
                        Stroke::NONE,
                    ));
                }
            }
        }

        // 3. Draw Nodes (Statement Boxes & Rhombus Decisions)
        for (pos, label, is_decision, _) in node_positions {
            if is_decision {
                // Rhombus / Diamond decision node
                let top = Pos2::new(pos.x, pos.y - 25.0_f32);
                let right = Pos2::new(pos.x + 90.0_f32, pos.y);
                let bottom = Pos2::new(pos.x, pos.y + 25.0_f32);
                let left = Pos2::new(pos.x - 90.0_f32, pos.y);

                painter.add(Shape::convex_polygon(
                    vec![top, right, bottom, left],
                    Color32::from_rgb(255, 243, 191),
                    Stroke::new(2.0_f32, Color32::from_rgb(245, 159, 0)),
                ));
            } else {
                // Statement box
                let rect = Rect::from_center_size(pos, Vec2::new(node_width, node_height));
                painter.rect_filled(rect, 8.0_f32, Color32::from_rgb(209, 237, 255));
                painter.rect_stroke(rect, 8.0_f32, Stroke::new(2.0_f32, Color32::from_rgb(24, 100, 171)));
            }

            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(13.0_f32),
                Color32::BLACK,
            );
        }
    });
}
// Keep your existing render_gui function at the top...

pub fn generate_mermaid(program: &Program) -> String {
    let mut output = String::from("flowchart TD\n");

    for (idx, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Start => {
                output.push_str(&format!("    node{}[\"Start\"]\n", idx));
            }
            Statement::End => {
                output.push_str(&format!("    node{}[\"End\"]\n", idx));
            }
            Statement::StatementChecker(text) => {
                output.push_str(&format!("    node{}[\"{}\"]\n", idx, text));
            }
            Statement::IfThen { id, condition, .. } => {
                output.push_str(&format!("    node{}{{\"If {}: {}\"}}\n", idx, id, condition));
            }
            Statement::ReturnTo(target) => {
                let target_label = match target {
                    StepId::Number(n) => format!("Step {}", n),
                    StepId::Named(s) => s.clone(),
                };
                output.push_str(&format!("    node{}[\"Return to {}\"]\n", idx, target_label));
            }
        }
    }

    for (idx, stmt) in program.statements.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let prev_idx = idx - 1;
        let prev_stmt = &program.statements[prev_idx];

        match stmt {
            Statement::ReturnTo(target) => {
                output.push_str(&format!("    node{} --> node{}\n", prev_idx, idx));

                if let StepId::Number(target_id) = target {
                    for (t_idx, t_stmt) in program.statements.iter().enumerate() {
                        if let Statement::IfThen { id, .. } = t_stmt {
                            if id == target_id {
                                output.push_str(&format!("    node{} -.->|Loop| node{}\n", idx, t_idx));
                            }
                        }
                    }
                }
            }
            _ => {
                if let Statement::IfThen { .. } = prev_stmt {
                    output.push_str(&format!("    node{} -->|Yes| node{}\n", prev_idx, idx));
                } else if !matches!(prev_stmt, Statement::ReturnTo(_)) {
                    output.push_str(&format!("    node{} --> node{}\n", prev_idx, idx));
                }
            }
        }
    }

    output
}