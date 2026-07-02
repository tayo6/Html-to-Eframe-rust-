use eframe::egui;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast;

// ==========================================================================
// TARGET ENTRYPOINTS (Native vs WebAssembly)
// ==========================================================================

// Desktop / Native main execution
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([570.0, 480.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Delay VST Plugin Replica",
        options,
        Box::new(|_cc| Box::new(DelayVstApp::default())),
    )
}

// WebAssembly Web Page main execution
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect log levels to browser console
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // Look up canvas item with id "the_canvas_id"
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("failed to find canvas element")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("canvas element is not HtmlCanvasElement");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Box::new(DelayVstApp::default())),
            )
            .await
            .expect("failed to start eframe runner");
    });
}

// ==========================================================================
// APP REPRESENTATION & IMPLEMENTATION
// ==========================================================================

struct DelayVstApp {
    tempo_index: usize,
    tempo_drag_accumulator: f32,
    regen_value: f32,
    mix_value: f32,
    output_value: f32,

    studio_mode: bool,
    auto_gain: bool,
    brightness_active: bool,
    color_active: bool,
    sparkle_active: bool,

    active_level_in: f32,
    active_level_out: f32,
}

impl Default for DelayVstApp {
    fn default() -> Self {
        Self {
            tempo_index: 2, 
            tempo_drag_accumulator: 0.0,
            regen_value: 0.722,
            mix_value: 0.537,
            output_value: 0.444,
            studio_mode: true,
            auto_gain: true,
            brightness_active: false,
            color_active: false,
            sparkle_active: false,
            active_level_in: 0.0,
            active_level_out: 0.0,
        }
    }
}

impl eframe::App for DelayVstApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        // Cast time to f32 to align float operations
        let time = ctx.input(|i| i.time) as f32;
        let target_in = (0.50f32 + (time * 1.8f32).sin().abs() * 0.22f32 + (time * 3.3f32).cos().abs() * 0.12f32).clamp(0.0f32, 1.0f32);
        let target_out = (0.45f32 + (time * 1.3f32).cos().abs() * 0.32f32 + (time * 4.2f32).sin().abs() * 0.15f32).clamp(0.0f32, 1.0f32);

        if target_in > self.active_level_in {
            self.active_level_in += (target_in - self.active_level_in) * 0.35f32;
        } else {
            self.active_level_in += (target_in - self.active_level_in) * 0.12f32;
        }

        if target_out > self.active_level_out {
            self.active_level_out += (target_out - self.active_level_out) * 0.35f32;
        } else {
            self.active_level_out += (target_out - self.active_level_out) * 0.12f32;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(18, 21, 27)))
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();
                let painter = ui.painter();

                painter.circle_filled(
                    full_rect.center(),
                    260.0,
                    egui::Color32::from_rgba_unmultiplied(27, 32, 42, 120),
                );

                let vst_rect = egui::Rect::from_center_size(full_rect.center(), egui::vec2(530.0, 440.0));

                for i in 1..=6 {
                    let shadow_rect = vst_rect.expand(i as f32 * 1.8);
                    painter.rect_stroke(
                        shadow_rect,
                        egui::Rounding::same(8.0 + i as f32),
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, (30 / i) as u8)),
                    );
                }

                painter.rect_filled(vst_rect, egui::Rounding::same(8.0), egui::Color32::from_rgb(245, 247, 250));
                painter.rect_stroke(vst_rect, egui::Rounding::same(8.0), egui::Stroke::new(1.2, egui::Color32::from_rgb(187, 196, 204)));

                ui.allocate_ui_at_rect(vst_rect, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    let top_rect = egui::Rect::from_min_size(vst_rect.min, egui::vec2(530.0, 160.0));
                    let mid_rect = egui::Rect::from_min_size(vst_rect.min + egui::vec2(0.0, 160.0), egui::vec2(530.0, 38.0));
                    let bottom_rect = egui::Rect::from_min_size(vst_rect.min + egui::vec2(0.0, 198.0), egui::vec2(530.0, 242.0));

                    draw_top_panel(ui, top_rect, self, time);
                    draw_mid_bar(ui, mid_rect, self);
                    draw_bottom_panel(ui, bottom_rect, self, ctx);
                });
            });
    }
}

fn draw_top_panel(ui: &mut egui::Ui, rect: egui::Rect, app: &DelayVstApp, time: f32) {
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        egui::Rounding { nw: 8.0, ne: 8.0, sw: 0.0, se: 0.0 },
        egui::Color32::from_rgb(9, 14, 18),
    );

    painter.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180)),
    );

    let logo_pos = rect.min + egui::vec2(20.0, 16.0);
    painter.text(
        logo_pos,
        egui::Align2::LEFT_TOP,
        "DELAY ▼",
        egui::FontId::proportional(13.0),
        egui::Color32::from_rgb(44, 229, 196),
    );

    let shift_x = (rect.width() - 380.0) / 2.0;
    let map_pt = |x: f32, y: f32, offset_y: f32| -> egui::Pos2 {
        rect.min + egui::vec2(x + shift_x, y + offset_y)
    };

    let left_offset_y = (time * 1.5f32).sin() * 4.0f32;
    let stroke_large = egui::Stroke::new(1.8, egui::Color32::from_rgb(44, 229, 196));

    let p_t1 = map_pt(150.0, 38.0, left_offset_y);
    let p_t2 = map_pt(185.0, 58.0, left_offset_y);
    let p_t3 = map_pt(150.0, 78.0, left_offset_y);
    let p_t4 = map_pt(115.0, 58.0, left_offset_y);

    painter.line_segment([p_t1, p_t2], stroke_large);
    painter.line_segment([p_t2, p_t3], stroke_large);
    painter.line_segment([p_t3, p_t4], stroke_large);
    painter.line_segment([p_t4, p_t1], stroke_large);

    let p_b1 = map_pt(150.0, 96.0, left_offset_y);
    let p_b2 = map_pt(185.0, 116.0, left_offset_y);
    let p_b3 = map_pt(150.0, 136.0, left_offset_y);
    let p_b4 = map_pt(115.0, 116.0, left_offset_y);

    painter.line_segment([p_b1, p_b2], stroke_large);
    painter.line_segment([p_b2, p_b3], stroke_large);
    painter.line_segment([p_b3, p_b4], stroke_large);
    painter.line_segment([p_b4, p_b1], stroke_large);

    painter.line_segment([p_t1, p_b1], stroke_large);
    painter.line_segment([p_t2, p_b2], stroke_large);
    painter.line_segment([p_t3, p_b3], stroke_large);
    painter.line_segment([p_t4, p_b4], stroke_large);

    let right_offset_y = ((time - 3.0f32) * 1.5f32).sin() * 4.0f32;
    let stroke_small = egui::Stroke::new(1.5, egui::Color32::from_rgb(44, 229, 196));

    let p_st1 = map_pt(255.0, 54.0, right_offset_y);
    let p_st2 = map_pt(276.0, 66.0, right_offset_y);
    let p_st3 = map_pt(255.0, 78.0, right_offset_y);
    let p_st4 = map_pt(234.0, 66.0, right_offset_y);

    painter.line_segment([p_st1, p_st2], stroke_small);
    painter.line_segment([p_st2, p_st3], stroke_small);
    painter.line_segment([p_st3, p_st4], stroke_small);
    painter.line_segment([p_st4, p_st1], stroke_small);

    let p_sb1 = map_pt(255.0, 86.0, right_offset_y);
    let p_sb2 = map_pt(276.0, 98.0, right_offset_y);
    let p_sb3 = map_pt(255.0, 110.0, right_offset_y);
    let p_sb4 = map_pt(234.0, 98.0, right_offset_y);

    painter.line_segment([p_sb1, p_sb2], stroke_small);
    painter.line_segment([p_sb2, p_sb3], stroke_small);
    painter.line_segment([p_sb3, p_sb4], stroke_small);
    painter.line_segment([p_sb4, p_sb1], stroke_small);

    painter.line_segment([p_st1, p_sb1], stroke_small);
    painter.line_segment([p_st2, p_sb2], stroke_small);
    painter.line_segment([p_st3, p_sb3], stroke_small);
    painter.line_segment([p_st4, p_sb4], stroke_small);
}

fn draw_mid_bar(ui: &mut egui::Ui, rect: egui::Rect, app: &mut DelayVstApp) {
    let painter = ui.painter();
    painter.rect_filled(rect, egui::Rounding::ZERO, egui::Color32::from_rgb(241, 243, 246));
    painter.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(225, 228, 232)),
    );

    let toggle_area = egui::Rect::from_min_max(
        rect.left_top() + egui::vec2(24.0, 5.0),
        rect.left_top() + egui::vec2(180.0, 33.0),
    );
    let toggle_response = ui.allocate_rect(toggle_area, egui::Sense::click());
    if toggle_response.clicked() {
        app.studio_mode = !app.studio_mode;
    }

    let studio_color = if app.studio_mode { egui::Color32::from_rgb(46, 53, 64) } else { egui::Color32::from_rgb(138, 148, 166) };
    let creative_color = if !app.studio_mode { egui::Color32::from_rgb(46, 53, 64) } else { egui::Color32::from_rgb(138, 148, 166) };

    painter.text(
        rect.left_top() + egui::vec2(24.0, 19.0),
        egui::Align2::LEFT_CENTER,
        "STUDIO",
        egui::FontId::proportional(9.0),
        studio_color,
    );

    let switch_rect = egui::Rect::from_min_size(rect.left_top() + egui::vec2(74.0, 10.5), egui::vec2(32.0, 17.0));
    let switch_bg = if app.studio_mode { egui::Color32::from_rgb(142, 153, 252) } else { egui::Color32::from_rgb(203, 213, 224) };
    painter.rect_filled(switch_rect, egui::Rounding::same(9.0), switch_bg);

    let handle_x = if app.studio_mode { switch_rect.left() + 2.0 } else { switch_rect.left() + 17.0 };
    let handle_rect = egui::Rect::from_min_size(egui::pos2(handle_x, switch_rect.top() + 2.0), egui::vec2(13.0, 13.0));
    painter.rect_filled(handle_rect, egui::Rounding::same(6.5), egui::Color32::WHITE);

    painter.text(
        rect.left_top() + egui::vec2(116.0, 19.0),
        egui::Align2::LEFT_CENTER,
        "CREATIVE",
        egui::FontId::proportional(9.0),
        creative_color,
    );

    let auto_gain_area = egui::Rect::from_min_max(
        rect.right_top() + egui::vec2(-110.0, 5.0),
        rect.right_top() + egui::vec2(-24.0, 33.0),
    );
    let ag_response = ui.allocate_rect(auto_gain_area, egui::Sense::click());
    if ag_response.clicked() {
        app.auto_gain = !app.auto_gain;
    }

    painter.text(
        rect.right_top() + egui::vec2(-48.0, 14.0),
        egui::Align2::RIGHT_CENTER,
        "AUTO",
        egui::FontId::proportional(8.0),
        egui::Color32::from_rgb(74, 85, 104),
    );
    painter.text(
        rect.right_top() + egui::vec2(-48.0, 24.0),
        egui::Align2::RIGHT_CENTER,
        "GAIN",
        egui::FontId::proportional(8.0),
        egui::Color32::from_rgb(74, 85, 104),
    );

    let led_rect = egui::Rect::from_min_size(rect.right_top() + egui::vec2(-42.0, 13.5), egui::vec2(18.0, 11.0));
    let led_color = if app.auto_gain {
        egui::Color32::from_rgb(89, 243, 140)
    } else {
        egui::Color32::from_rgb(160, 174, 192)
    };
    painter.rect_filled(led_rect, egui::Rounding::same(2.0), led_color);
}

fn draw_bottom_panel(ui: &mut egui::Ui, rect: egui::Rect, app: &mut DelayVstApp, ctx: &egui::Context) {
    let painter = ui.painter();
    painter.rect_filled(rect, egui::Rounding { nw: 0.0, ne: 0.0, sw: 8.0, se: 8.0 }, egui::Color32::from_rgb(247, 249, 250));

    let divider_x = rect.left() + 413.0;
    painter.line_segment(
        [egui::pos2(divider_x, rect.top()), egui::pos2(divider_x, rect.bottom())],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(225, 228, 232)),
    );

    let tempo_values = ["1/32", "1/16", "1/8", "1/4", "1/2", "1/1"];
    let tempo_center = egui::pos2(rect.left() + 80.0, rect.top() + 65.0);
    let tempo_rect = egui::Rect::from_center_size(tempo_center, egui::vec2(66.0, 66.0));

    let tempo_response = ui.allocate_rect(tempo_rect, egui::Sense::click_and_drag());
    if tempo_response.dragged() {
        let dy = tempo_response.drag_delta().y;
        if dy.abs() > 1.5f32 {
            app.tempo_drag_accumulator += dy;
            if app.tempo_drag_accumulator > 15.0f32 {
                if app.tempo_index > 0 { app.tempo_index -= 1; }
                app.tempo_drag_accumulator = 0.0f32;
            } else if app.tempo_drag_accumulator < -15.0f32 {
                if app.tempo_index < tempo_values.len() - 1 { app.tempo_index += 1; }
                app.tempo_drag_accumulator = 0.0f32;
            }
        }
    } else {
        app.tempo_drag_accumulator = 0.0f32;
    }

    if tempo_response.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::ResizeVertical);
    }

    painter.circle_stroke(tempo_center, 28.0, egui::Stroke::new(3.5, egui::Color32::from_rgb(226, 232, 240)));

    let progress = (app.tempo_index + 1) as f32 / tempo_values.len() as f32;
    let stroke_color = egui::Color32::from_rgb(44, 229, 196);

    let start_angle = -std::f32::consts::FRAC_PI_2;
    let sweep_angle = progress * 2.0f32 * std::f32::consts::PI;
    let num_segments = 32;
    let mut last_point = tempo_center + egui::vec2(start_angle.cos(), start_angle.sin()) * 28.0;
    for i in 1..=num_segments {
        let t = i as f32 / num_segments as f32;
        let angle = start_angle + sweep_angle * t;
        let next_point = tempo_center + egui::vec2(angle.cos(), angle.sin()) * 28.0;
        painter.line_segment([last_point, next_point], egui::Stroke::new(3.5, stroke_color));
        last_point = next_point;
    }

    painter.text(
        tempo_center,
        egui::Align2::CENTER_CENTER,
        tempo_values[app.tempo_index],
        egui::FontId::proportional(15.0),
        egui::Color32::from_rgb(45, 55, 72),
    );

    painter.text(
        tempo_center + egui::vec2(0.0, 48.0),
        egui::Align2::CENTER_CENTER,
        "TEMPO",
        egui::FontId::proportional(9.0),
        egui::Color32::from_rgb(113, 128, 150),
    );

    let regen_center = egui::pos2(rect.left() + 200.0, rect.top() + 65.0);
    let regen_rect = egui::Rect::from_center_size(regen_center, egui::vec2(52.0, 52.0));
    let regen_response = ui.allocate_rect(regen_rect, egui::Sense::click_and_drag());
    if regen_response.dragged() {
        let dy = regen_response.drag_delta().y;
        app.regen_value = (app.regen_value - dy * 0.005f32).clamp(0.0f32, 1.0f32);
    }
    if regen_response.hovered() {
        ctx.set_cursor_icon(if regen_response.dragged() { egui::CursorIcon::Grabbing } else { egui::CursorIcon::Grab });
    }
    draw_knob(painter, regen_center, 22.0, app.regen_value);
    painter.text(
        regen_center + egui::vec2(0.0, 48.0),
        egui::Align2::CENTER_CENTER,
        "REGEN",
        egui::FontId::proportional(9.0),
        egui::Color32::from_rgb(113, 128, 150),
    );

    let mix_center = egui::pos2(rect.left() + 325.0, rect.top() + 65.0);
    let mix_rect = egui::Rect::from_center_size(mix_center, egui::vec2(68.0, 68.0));
    let mix_response = ui.allocate_rect(mix_rect, egui::Sense::click_and_drag());
    if mix_response.dragged() {
        let dy = mix_response.drag_delta().y;
        app.mix_value = (app.mix_value - dy * 0.005f32).clamp(0.0f32, 1.0f32);
    }
    if mix_response.hovered() {
        ctx.set_cursor_icon(if mix_response.dragged() { egui::CursorIcon::Grabbing } else { egui::CursorIcon::Grab });
    }
    draw_knob(painter, mix_center, 29.0, app.mix_value);
    painter.text(
        mix_center + egui::vec2(0.0, 48.0),
        egui::Align2::CENTER_CENTER,
        "MIX",
        egui::FontId::proportional(9.0),
        egui::Color32::from_rgb(113, 128, 150),
    );

    let btn_width = 85.0;
    let btn_height = 64.0;
    let btn_y = rect.top() + 175.0;

    let bright_rect = egui::Rect::from_center_size(egui::pos2(rect.left() + 90.0, btn_y), egui::vec2(btn_width, btn_height));
    let bright_response = ui.allocate_rect(bright_rect, egui::Sense::click());
    if bright_response.clicked() {
        app.brightness_active = !app.brightness_active;
    }
    if bright_response.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    draw_filter_button(painter, bright_rect, "BRIGHTNESS", app.brightness_active, |p, center| {
        let radius = 12.0;
        let stroke_stripe = egui::Stroke::new(1.3, if app.brightness_active { egui::Color32::from_rgb(44, 229, 196) } else { egui::Color32::from_rgb(113, 128, 150) });
        p.circle_stroke(center, radius, stroke_stripe);

        let step = 2.5;
        let mut y_offset = -radius + 1.5;
        while y_offset < radius {
            let half_chord = (radius * radius - y_offset * y_offset).sqrt();
            let p1 = center + egui::vec2(-half_chord, y_offset);
            let p2 = center + egui::vec2(0.0, y_offset);
            p.line_segment([p1, p2], stroke_stripe);
            y_offset += step;
        }
        p.line_segment([center + egui::vec2(0.0, -radius), center + egui::vec2(0.0, radius)], stroke_stripe);
    });

    let color_rect = egui::Rect::from_center_size(egui::pos2(rect.left() + 205.0, btn_y), egui::vec2(btn_width, btn_height));
    let color_response = ui.allocate_rect(color_rect, egui::Sense::click());
    if color_response.clicked() {
        app.color_active = !app.color_active;
    }
    if color_response.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    draw_filter_button(painter, color_rect, "COLOR", app.color_active, |p, center| {
        let stroke = egui::Stroke::new(1.4, if app.color_active { egui::Color32::from_rgb(44, 229, 196) } else { egui::Color32::from_rgb(113, 128, 150) });
        let r_circle = 5.0;
        p.circle_stroke(center, r_circle, stroke);
        for i in 0..6 {
            let angle = (i as f32 * 60.0).to_radians();
            let offset = egui::vec2(angle.cos(), angle.sin()) * 5.5;
            p.circle_stroke(center + offset, r_circle, stroke);
        }
    });

    let sparkle_rect = egui::Rect::from_center_size(egui::pos2(rect.left() + 320.0, btn_y), egui::vec2(btn_width, btn_height));
    let sparkle_response = ui.allocate_rect(sparkle_rect, egui::Sense::click());
    if sparkle_response.clicked() {
        app.sparkle_active = !app.sparkle_active;
    }
    if sparkle_response.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    draw_filter_button(painter, sparkle_rect, "SPARKLE", app.sparkle_active, |p, center| {
        let color = if app.sparkle_active { egui::Color32::from_rgb(44, 229, 196) } else { egui::Color32::from_rgb(113, 128, 150) };

        let draw_star = |painter: &egui::Painter, c: egui::Pos2, r_out: f32, r_in: f32| {
            let mut points = Vec::with_capacity(8);
            for i in 0..8 {
                let angle = (i as f32 * 45.0).to_radians();
                let r = if i % 2 == 0 { r_out } else { r_in };
                points.push(c + egui::vec2(angle.cos(), angle.sin()) * r);
            }
            painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
        };

        draw_star(p, center + egui::vec2(0.0, -8.0), 4.5, 1.2);
        draw_star(p, center + egui::vec2(-7.0, 1.0), 3.5, 1.0);
        draw_star(p, center + egui::vec2(7.0, 2.0), 3.8, 1.1);
        draw_star(p, center + egui::vec2(0.0, 10.0), 4.8, 1.3);
        draw_star(p, center + egui::vec2(-8.0, -6.0), 2.5, 0.8);
        draw_star(p, center + egui::vec2(8.0, -7.0), 2.5, 0.8);
    });

    let cx = rect.left() + 413.0 + 58.5;

    let draw_vu_meter = |p: &egui::Painter, meter_cx: f32, level: f32, label: &str| {
        let meter_rect = egui::Rect::from_center_size(
            egui::pos2(meter_cx, rect.top() + 55.0),
            egui::vec2(11.0, 86.5),
        );
        p.rect_filled(meter_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(226, 232, 240));

        let lit_limit = (level * 24.0f32).round() as usize;
        let led_w = 7.0;
        let led_h = 2.0;
        let gap = 1.5;

        for i in 0..24 {
            let y = meter_rect.bottom() - 2.0 - i as f32 * (led_h + gap);
            let led_rect = egui::Rect::from_center_size(egui::pos2(meter_cx, y), egui::vec2(led_w, led_h));
            let is_lit = i < lit_limit;

            let led_color = if is_lit {
                if i < 16 {
                    egui::Color32::from_rgb(82, 236, 135)
                } else if i < 19 {
                    egui::Color32::from_rgb(76, 223, 242)
                } else if i < 22 {
                    egui::Color32::from_rgb(252, 163, 61)
                } else {
                    egui::Color32::from_rgb(255, 94, 94)
                }
            } else {
                egui::Color32::from_rgb(203, 213, 220)
            };
            p.rect_filled(led_rect, egui::Rounding::same(0.5), led_color);
        }

        p.text(
            egui::pos2(meter_cx, rect.top() + 112.0),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(8.0),
            egui::Color32::from_rgb(113, 128, 150),
        );
    };

    draw_vu_meter(painter, cx - 18.0, app.active_level_in, "IN");
    draw_vu_meter(painter, cx + 18.0, app.active_level_out, "OUT");

    let out_knob_center = egui::pos2(cx, rect.top() + 175.0);
    let out_knob_rect = egui::Rect::from_center_size(out_knob_center, egui::vec2(52.0, 52.0));
    let out_knob_response = ui.allocate_rect(out_knob_rect, egui::Sense::click_and_drag());
    if out_knob_response.dragged() {
        let dy = out_knob_response.drag_delta().y;
        app.output_value = (app.output_value - dy * 0.005f32).clamp(0.0f32, 1.0f32);
    }
    if out_knob_response.hovered() {
        ctx.set_cursor_icon(if out_knob_response.dragged() { egui::CursorIcon::Grabbing } else { egui::CursorIcon::Grab });
    }
    draw_knob(painter, out_knob_center, 22.0, app.output_value);
    painter.text(
        out_knob_center + egui::vec2(0.0, 48.0),
        egui::Align2::CENTER_CENTER,
        "OUTPUT",
        egui::FontId::proportional(9.0),
        egui::Color32::from_rgb(113, 128, 150),
    );
}

fn draw_knob(painter: &egui::Painter, center: egui::Pos2, radius: f32, val: f32) {
    painter.circle_stroke(
        center,
        radius + 4.0,
        egui::Stroke::new(1.0, egui::Color32::from_gray(180).linear_multiply(0.35)),
    );

    painter.circle_filled(center, radius, egui::Color32::from_rgb(203, 219, 231));
    painter.circle_filled(
        center - egui::vec2(radius * 0.15, radius * 0.15),
        radius * 0.8,
        egui::Color32::from_rgb(241, 246, 250),
    );

    painter.circle_stroke(center, radius, egui::Stroke::new(1.2, egui::Color32::from_rgb(184, 200, 213)));

    let min_angle = -135.0f32.to_radians();
    let max_angle = 135.0f32.to_radians();
    let rotation_angle = min_angle + val * (max_angle - min_angle);
    let angle_rad = -std::f32::consts::FRAC_PI_2 + rotation_angle;

    let pointer_dist = radius - 6.0;
    let pointer_pos = center + egui::vec2(angle_rad.cos(), angle_rad.sin()) * pointer_dist;
    painter.circle_filled(pointer_pos, 2.5, egui::Color32::from_rgb(74, 85, 104));
}

fn draw_filter_button<F>(painter: &egui::Painter, rect: egui::Rect, label: &str, active: bool, draw_icon: F)
where
    F: FnOnce(&egui::Painter, egui::Pos2),
{
    let bg_color = if active {
        egui::Color32::from_rgb(235, 253, 250)
    } else {
        egui::Color32::TRANSPARENT
    };
    let border_stroke = if active {
        egui::Stroke::new(1.2, egui::Color32::from_rgb(44, 229, 196))
    } else {
        egui::Stroke::NONE
    };

    painter.rect(rect, egui::Rounding::same(6.0), bg_color, border_stroke);

    let icon_center = egui::pos2(rect.center().x, rect.top() + 22.0);
    draw_icon(painter, icon_center);

    let label_color = if active {
        egui::Color32::from_rgb(44, 229, 196)
    } else {
        egui::Color32::from_rgb(113, 128, 150)
    };
    painter.text(
        egui::pos2(rect.center().x, rect.bottom() - 10.0),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(8.0),
        label_color,
    );
}