use colorful::{Color, Colorful};
use egui::{Color32, RichText};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax, Token, TokenType};
use egui_json_tree::{DefaultExpand, JsonTree};
use eframe::egui;
use log::{error, info};
mod json_syntax;
use reqwest;
use serde_json::Value;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, PartialEq, Clone)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

#[derive(PartialEq)]
enum EditorTab {
    Headers,
    Body,
    Query,
}

struct SnoozeApp {
    url: String,
    selected_method: HttpMethod,
    response_text: String,
    parsed_json: Option<Value>,
    last_duration: Option<Duration>,
    in_flight: bool,
    tx: Option<Sender<(String, Duration)>>,
    rx: Option<Receiver<(String, Duration)>>,
    headers: Vec<(String, String)>,
    request_body: String,
    queries: Vec<(String, String)>,
    divider: f32,
    selected_tab: EditorTab,
    search_input: String, // added for json search
}

impl Default for SnoozeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl SnoozeApp {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            url: "".to_owned(),
            selected_method: HttpMethod::default(),
            response_text: "".to_owned(),
            parsed_json: None,
            last_duration: None,
            in_flight: false,
            tx: Some(tx),
            rx: Some(rx),
            headers: vec![("".to_owned(), "".to_owned())],
            request_body: "".to_owned(),
            queries: vec![("".to_owned(), "".to_owned())],
            divider: 250.0,
            selected_tab: EditorTab::Headers,
            search_input: "".to_owned(),
        }
    }
}

impl eframe::App for SnoozeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

      let syntax = json_syntax::json_syntax();
        // drain channel
        if let Some(rx) = &self.rx {
            while let Ok((resp, dur)) = rx.try_recv() {
                info!("received response in {} ms", dur.as_millis());
                self.response_text = resp;
                self.last_duration = Some(dur);
                self.in_flight = false;
                self.parsed_json = serde_json::from_str(&self.response_text).ok();
            }
        }
        if self.in_flight {
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("snooze v0.1.1");
                if let Some(dur) = self.last_duration {
                    ui.label(format!(" | request took: {} ms", dur.as_millis()));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let avail_rect = ui.available_rect_before_wrap();
            let avail_width = avail_rect.width();
            let avail_height = avail_rect.height();
            let min_panel_width = 100.0;
            let divider_width = 4.0;
            let pad = 8.0;

            self.divider = self
                .divider
                .max(min_panel_width)
                .min(avail_width - min_panel_width - divider_width);

            let left_rect =
                egui::Rect::from_min_size(avail_rect.min, egui::vec2(self.divider, avail_height));
            let divider_rect = egui::Rect::from_min_size(
                egui::pos2(left_rect.max.x, avail_rect.min.y),
                egui::vec2(divider_width, avail_height),
            );
            let right_rect = egui::Rect::from_min_size(
                egui::pos2(divider_rect.max.x, avail_rect.min.y),
                egui::vec2(avail_width - self.divider - divider_width, avail_height),
            );

            let left_inner = egui::Rect::from_min_max(
                left_rect.min,
                egui::pos2(left_rect.max.x - pad, left_rect.max.y),
            );
            let right_inner = egui::Rect::from_min_max(
                egui::pos2(right_rect.min.x + pad, right_rect.min.y),
                right_rect.max,
            );

            // left pane: request options
            ui.allocate_ui_at_rect(left_inner, |ui| {
                ui.set_max_width(left_inner.width());

                ui.heading("request");
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    let methods = [
                        (HttpMethod::GET, Color32::from_rgb(97, 175, 255)),
                        (HttpMethod::POST, Color32::from_rgb(152, 230, 121)),
                        (HttpMethod::PUT, Color32::from_rgb(198, 120, 255)),
                        (HttpMethod::DELETE, Color32::from_rgb(255, 108, 117)),
                        (HttpMethod::PATCH, Color32::from_rgb(255, 192, 123)),
                        (HttpMethod::HEAD, Color32::from_rgb(86, 182, 230)),
                        (HttpMethod::OPTIONS, Color32::from_rgb(152, 195, 121)),
                    ];
                    for (method, color) in methods.iter() {
                        let txt = format!("{:?}", method);
                        let is_selected = self.selected_method == *method;
                        let btn_txt = RichText::new(&txt)
                            .color(if is_selected { Color32::WHITE } else { *color });
                        let mut button = egui::Button::new(btn_txt);
                        if is_selected {
                            button = button.fill(*color);
                        }
                        if ui.add(button).clicked() {
                            info!("changed method to {:?}", method);
                            self.selected_method = method.clone();
                        }
                    }
                });

                ui.add_space(10.0);
                let url_width = ui.available_width();
                ui.add(
                    egui::TextEdit::multiline(&mut self.url)
                        .hint_text("url")
                        .desired_rows(1)
                        .desired_width(url_width)
                        .font(egui::TextStyle::Monospace)
                        .clip_text(true),
                );

                if ui.button("send").clicked() {
                    if Url::parse(&self.url).is_err() {
                        error!("invalid url: {}", self.url);
                        self.response_text = format!("invalid url: {}", self.url);
                        self.parsed_json = None;
                        self.last_duration = None;
                    } else {
                        info!("{:?} {}", self.selected_method, self.url);
                        let request_body = self.request_body.clone();
                        let queries = self.queries.clone();
                        let headers = self.headers.clone();
                        let url = self.url.clone();
                        let method = self.selected_method.clone();
                        self.in_flight = true;
                        if let Some(tx) = self.tx.clone() {
                            std::thread::spawn(move || {
                                info!("executing request: {:?} {}", method, url);
                                let client = reqwest::blocking::Client::new();
                                let mut req_builder = match method {
                                    HttpMethod::GET => client.get(&url),
                                    HttpMethod::POST => client.post(&url),
                                    HttpMethod::PUT => client.put(&url),
                                    HttpMethod::DELETE => client.delete(&url),
                                    HttpMethod::PATCH => client.patch(&url),
                                    HttpMethod::HEAD => client.head(&url),
                                    HttpMethod::OPTIONS => {
                                        client.request(reqwest::Method::OPTIONS, &url)
                                    }
                                };
                                // add headers
                                for (key, value) in headers.iter() {
                                    if !key.is_empty() {
                                        req_builder = req_builder.header(key, value);
                                    }
                                }
                                let query_params: Vec<(&str, &str)> = queries
                                    .iter()
                                    .filter_map(|(k, v)| {
                                        if !k.is_empty() {
                                            Some((k.as_str(), v.as_str()))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                if !query_params.is_empty() {
                                    req_builder = req_builder.query(&query_params);
                                }
                                if matches!(
                                    method,
                                    HttpMethod::POST | HttpMethod::PUT | HttpMethod::PATCH
                                ) {
                                    req_builder = req_builder.body(request_body);
                                }
                                let start = Instant::now();
                                let response = req_builder.send();
                                let duration = start.elapsed();
                                let result = match response {
                                    Ok(resp) => match resp.text() {
                                        Ok(txt) => {
                                            info!("request succeeded in {} ms", duration.as_millis());
                                            txt
                                        }
                                        Err(e) => {
                                            error!("error reading response: {}", e);
                                            format!("error reading response: {}", e)
                                        }
                                    },
                                    Err(e) => {
                                        error!("request error: {}", e);
                                        format!("request error: {}", e)
                                    }
                                };
                                let _ = tx.send((result, duration));
                            });
                        }
                        self.response_text =
                            format!("{:?} {}", self.selected_method, self.url);
                        self.parsed_json = None;
                        self.last_duration = None;
                    }
                }

                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(self.selected_tab == EditorTab::Headers, "headers")
                        .clicked()
                    {
                        info!("switched editor tab to: headers");
                        self.selected_tab = EditorTab::Headers;
                    }
                    if ui
                        .selectable_label(self.selected_tab == EditorTab::Body, "body")
                        .clicked()
                    {
                        info!("switched editor tab to: body");
                        self.selected_tab = EditorTab::Body;
                    }
                    if ui
                        .selectable_label(self.selected_tab == EditorTab::Query, "query")
                        .clicked()
                    {
                        info!("switched editor tab to: query");
                        self.selected_tab = EditorTab::Query;
                    }
                });
                ui.separator();

                match self.selected_tab {
                    EditorTab::Headers => {
                        let mut to_remove = None;
                        egui::ScrollArea::horizontal()
                            .id_salt("headers_scroll")
                            .show(ui, |ui| {
                                for (i, header) in self.headers.iter_mut().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.set_max_width(ui.available_width());
                                        let delete_btn_width = 24.0;
                                        let spacing = 8.0;
                                        let field_width = (ui.available_width()
                                            - delete_btn_width
                                            - spacing * 2.0)
                                            / 2.0;
                                        if ui
                                            .add(
                                                egui::Button::new("×")
                                                    .fill(Color32::from_rgb(255, 88, 88))
                                                    .min_size(egui::vec2(delete_btn_width, 18.0)),
                                            )
                                            .clicked()
                                        {
                                            info!("removing header at index {}", i);
                                            to_remove = Some(i);
                                        }
                                        ui.add(
                                            egui::TextEdit::singleline(&mut header.0)
                                                .desired_width(field_width)
                                                .clip_text(true)
                                                .hint_text("key"),
                                        );
                                        ui.label(":");
                                        ui.add(
                                            egui::TextEdit::singleline(&mut header.1)
                                                .desired_width(field_width)
                                                .clip_text(true)
                                                .hint_text("value"),
                                        );
                                    });
                                }
                            });
                        if let Some(i) = to_remove {
                            self.headers.remove(i);
                        }
                        ui.add_space(4.0);
                        if ui.button("+ add header").clicked() {
                            info!("adding a new header");
                            self.headers.push(("".to_owned(), "".to_owned()));
                        }
                    }

                    EditorTab::Body => {
                        ui.label("body:");
                        egui::ScrollArea::horizontal()
                            .id_salt("body_scroll")
                            .show(ui, |ui| {
                                // use the code editor instead of the plain text editor
                                CodeEditor::default()
                                    .id_source("code_editor")
                                    .with_rows(12)
                                    .with_fontsize(14.0)
                                    .with_theme(ColorTheme::AYU_DARK)
                                    .with_syntax(syntax.clone())
                                    .with_numlines(true)
                                    .vscroll(true)
                                    .show(ui, &mut self.request_body);
                            });
                    }
                    EditorTab::Query => {
                        ui.label("query parameters:");
                        let mut to_remove = None;
                        egui::ScrollArea::horizontal()
                            .id_salt("queries_scroll")
                            .show(ui, |ui| {
                                for (i, query) in self.queries.iter_mut().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.set_max_width(ui.available_width());
                                        let delete_btn_width = 24.0;
                                        let spacing = 8.0;
                                        let field_width = (ui.available_width()
                                            - delete_btn_width
                                            - spacing * 2.0)
                                            / 2.0;
                                        if ui
                                            .add(
                                                egui::Button::new("×")
                                                    .fill(Color32::from_rgb(255, 88, 88))
                                                    .min_size(egui::vec2(delete_btn_width, 18.0)),
                                            )
                                            .clicked()
                                        {
                                            info!("removing query at index {}", i);
                                            to_remove = Some(i);
                                        }
                                        ui.add(
                                            egui::TextEdit::singleline(&mut query.0)
                                                .desired_width(field_width)
                                                .clip_text(true)
                                                .hint_text("key"),
                                        );
                                        ui.label("=");
                                        ui.add(
                                            egui::TextEdit::singleline(&mut query.1)
                                                .desired_width(field_width)
                                                .clip_text(true)
                                                .hint_text("value"),
                                        );
                                    });
                                }
                            });
                        if let Some(i) = to_remove {
                            self.queries.remove(i);
                        }
                        ui.add_space(4.0);
                        if ui.button("+ add query parameter").clicked() {
                            info!("adding a new query parameter");
                            self.queries.push(("".to_owned(), "".to_owned()));
                        }
                    }
                }
            });

            let divider_response =
                ui.interact(divider_rect, ui.id().with("divider"), egui::Sense::drag());
            if divider_response.dragged() {
                if let Some(pointer_pos) = divider_response.hover_pos() {
                    self.divider = (pointer_pos.x - avail_rect.min.x)
                        .max(min_panel_width)
                        .min(avail_width - min_panel_width - divider_width);
                    info!("divider moved to {}", self.divider);
                }
            }

            if divider_response.hovered() || divider_response.dragged() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ResizeHorizontal);
            }

            let divider_color = if divider_response.dragged() {
                egui::Color32::LIGHT_GRAY
            } else if divider_response.hovered() {
                egui::Color32::GRAY
            } else {
                egui::Color32::DARK_GRAY
            };

            ui.painter()
                .rect_filled(divider_rect, egui::Rounding::same(4.0), divider_color);

            // right pane: response & json search if applicable
            ui.allocate_ui_at_rect(right_inner, |ui| {
                ui.heading("response");
                ui.separator();

                if let Some(ref json) = self.parsed_json {
                    // search controls outside scroll area
                    ui.label("search:");
                    let (text_edit_response, clear_button_response) = ui
                        .horizontal(|ui| {
                            let text_edit_response =
                                ui.text_edit_singleline(&mut self.search_input);
                            let clear_button_response = ui.button("clear");
                            (text_edit_response, clear_button_response)
                        })
                        .inner;

                    // json tree
                    egui::ScrollArea::vertical()
                        .max_height(ui.available_height())
                        .show(ui, |ui| {
                            let tree_response = JsonTree::new("json_tree", json)
                                .default_expand(if self.search_input.is_empty() {
                                    DefaultExpand::All
                                } else {
                                    DefaultExpand::SearchResults(&self.search_input)
                                })
                                .show(ui);
                            if text_edit_response.changed() {
                                tree_response.reset_expanded(ui);
                            }
                            if clear_button_response.clicked() {
                                self.search_input.clear();
                                tree_response.reset_expanded(ui);
                            }
                            if ui.button("reset expanded").clicked() {
                                tree_response.reset_expanded(ui);
                            }
                        });
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(&self.response_text).monospace(),
                        ));
                    });
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let nat_options = eframe::NativeOptions::default();
    eframe::run_native(
        "snooze",
        nat_options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals.window_fill =
                egui::Color32::from_rgba_unmultiplied(20, 20, 20, 125);
            style.text_styles = [
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(22.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(14.0, egui::FontFamily::Monospace),
                ),
            ]
            .into();
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            style.spacing.window_margin = egui::Margin::same(10.0);
            style.spacing.button_padding = egui::vec2(6.0, 3.0);
            style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
            style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
            style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
            style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
            style.visuals.window_rounding = egui::Rounding::same(8.0);
            style.visuals.dark_mode = true;
            style.visuals.panel_fill = egui::Color32::from_rgb(0x14, 0x14, 0x14);
            // set primary button colors
            style.visuals.widgets.inactive.bg_fill =
                egui::Color32::from_rgb(0xf9, 0x26, 0x72);
            style.visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgb(0xff, 0x43, 0x8f);
            style.visuals.widgets.active.bg_fill =
                egui::Color32::from_rgb(0xd4, 0x1b, 0x5f);
            cc.egui_ctx.set_style(style);
            Ok(Box::new(SnoozeApp::new()))
        }),
    )
}
