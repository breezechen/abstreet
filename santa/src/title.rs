use geom::Percent;
use map_gui::tools::open_browser;
use widgetry::{
    ButtonBuilder, Color, ControlState, EdgeInsets, EventCtx, GeomBatch, GfxCtx, Key, Line, Panel,
    RewriteColor, SimpleState, State, Text, TextExt, Widget,
};

use crate::levels::Level;
use crate::{App, Transition};

pub struct TitleScreen;

impl TitleScreen {
    pub fn new_state(ctx: &mut EventCtx, app: &App) -> Box<dyn State<App>> {
        let mut level_buttons = Vec::new();
        for (idx, level) in app.session.levels.iter().enumerate() {
            if idx < app.session.levels_unlocked {
                level_buttons.push(unlocked_level(ctx, app, level, idx).margin_below(16));
            } else {
                level_buttons.push(locked_level(ctx, app, level, idx).margin_below(16));
            }
        }

        <dyn SimpleState<_>>::new_state(
            Panel::new_builder(Widget::col(vec![
                Line("15-minute Santa")
                    .display_title()
                    .into_widget(ctx)
                    .container()
                    .padding(16)
                    .bg(Color::BLACK.alpha(0.8))
                    .centered_horiz(),
                Text::from(
                    Line(
                        "Time for Santa to deliver presents in Seattle! But... COVID means no \
                         stopping in houses to munch on cookies (gluten-free and paleo, \
                         obviously). When your blood sugar gets low, you'll have to stop and \
                         refill your supply from a store. Those are close to where people live... \
                         right?",
                    )
                    .small_heading(),
                )
                .wrap_to_pct(ctx, 50)
                .into_widget(ctx)
                .container()
                .padding(16)
                .bg(Color::BLACK.alpha(0.8))
                .centered_horiz(),
                Widget::custom_row(level_buttons).flex_wrap(ctx, Percent::int(80)),
                Widget::row(vec![
                    map_gui::tools::home_btn(ctx),
                    ctx.style()
                        .btn_outline
                        .text("Credits")
                        .build_def(ctx)
                        .centered_vert(),
                    "Created by Dustin Carlino, Yuwen Li, & Michael Kirk"
                        .text_widget(ctx)
                        .centered_vert(),
                ])
                .centered_horiz()
                .align_bottom()
                .bg(Color::BLACK.alpha(0.8)),
            ]))
            .build_custom(ctx),
            Box::new(TitleScreen),
        )
    }
}

impl SimpleState<App> for TitleScreen {
    fn on_click(&mut self, ctx: &mut EventCtx, app: &mut App, x: &str, _: &Panel) -> Transition {
        match x {
            "Home" => Transition::Pop,
            "Credits" => Transition::Push(Credits::new_state(ctx)),
            x => {
                for level in &app.session.levels {
                    if x == level.title {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let map_name = level.map.clone();
                            if !abstio::file_exists(map_name.path()) {
                                return map_gui::tools::prompt_to_download_missing_data(
                                    ctx, map_name,
                                );
                            }
                        }

                        return Transition::Push(crate::before_level::Picker::new_state(
                            ctx,
                            app,
                            level.clone(),
                        ));
                    }
                }
                panic!("Unknown action {}", x);
            }
        }
    }

    fn other_event(&mut self, ctx: &mut EventCtx, app: &mut App) -> Transition {
        app.session.update_music(ctx);
        Transition::Keep
    }

    fn draw(&self, g: &mut GfxCtx, app: &App) {
        app.session.music.draw(g);
    }
}

fn level_btn(ctx: &mut EventCtx, app: &App, level: &Level, idx: usize) -> GeomBatch {
    let mut txt = Text::new();
    txt.add_line(Line(format!("LEVEL {}", idx + 1)).small_heading());
    txt.add_line(Line(&level.title).small_heading());
    txt.add_line(&level.description);
    let batch = txt.wrap_to_pct(ctx, 15).render_autocropped(ctx);

    // Add padding
    let (mut batch, hitbox) = batch
        .batch()
        .container()
        .padding(EdgeInsets {
            top: 20.0,
            bottom: 20.0,
            left: 10.0,
            right: 10.0,
        })
        .into_geom(ctx, None);
    batch.unshift(app.cs.unzoomed_bike, hitbox);
    batch
}

// TODO Preview the map, add padding, add the linear gradient...
fn locked_level(ctx: &mut EventCtx, app: &App, level: &Level, idx: usize) -> Widget {
    let mut batch = level_btn(ctx, app, level, idx);
    let hitbox = batch.get_bounds().get_rectangle();
    let center = hitbox.center();
    batch.push(app.cs.fade_map_dark, hitbox);
    batch.append(GeomBatch::load_svg(ctx, "system/assets/tools/locked.svg").centered_on(center));
    batch.into_widget(ctx)
}

fn unlocked_level(ctx: &mut EventCtx, app: &App, level: &Level, idx: usize) -> Widget {
    let normal = level_btn(ctx, app, level, idx);
    let hovered = normal
        .clone()
        .color(RewriteColor::Change(Color::WHITE, Color::WHITE.alpha(0.6)));

    ButtonBuilder::new()
        .custom_batch(normal, ControlState::Default)
        .custom_batch(hovered, ControlState::Hovered)
        .build_widget(ctx, &level.title)
}

struct Credits;

impl Credits {
    fn new_state(ctx: &mut EventCtx) -> Box<dyn State<App>> {
        <dyn SimpleState<_>>::new_state(
            Panel::new_builder(Widget::col(vec![
                Widget::row(vec![
                    Line("15-minute Santa").big_heading_plain().into_widget(ctx),
                    ctx.style().btn_close_widget(ctx),
                ]),
                link(
                    ctx,
                    "Created by the A/B Street team",
                    "https://abstreet.org"
                ),
                Text::from_multiline(vec![
                    Line("Lead: Dustin Carlino"),
                    Line("Programming & game design: Michael Kirk"),
                    Line("UI/UX: Yuwen Li"),
                ]).into_widget(ctx),
                link(
                    ctx,
                    "Santa made by @parallaxcreativedesign",
                    "https://www.instagram.com/parallaxcreativedesign/",
                ),
                link(
                    ctx,
                    "Map data thanks to OpenStreetMap contributors",
                    "https://www.openstreetmap.org/about"),
                link(ctx, "Land use data from Seattle GeoData", "https://data-seattlecitygis.opendata.arcgis.com/datasets/current-land-use-zoning-detail"),
                link(ctx, "Music from various sources", "https://github.com/a-b-street/abstreet/tree/master/data/system/assets/music/sources.md"),
                link(ctx, "Fonts and icons by various sources", "https://a-b-street.github.io/docs/howto/#data-source-licensing"),
                "Playtesting by Fridgehaus".text_widget(ctx),
                ctx.style().btn_outline.text("Back").hotkey(Key::Enter).build_def(ctx).centered_horiz(),
            ]))
            .build(ctx), Box::new(Credits))
    }
}

fn link(ctx: &mut EventCtx, label: &str, url: &str) -> Widget {
    ctx.style()
        .btn_plain
        .text(label)
        .build_widget(ctx, format!("open {}", url))
}

impl SimpleState<App> for Credits {
    fn on_click(&mut self, _: &mut EventCtx, _: &mut App, x: &str, _: &Panel) -> Transition {
        match x {
            "close" | "Back" => Transition::Pop,
            x => {
                if let Some(url) = x.strip_prefix("open ") {
                    open_browser(url);
                    return Transition::Keep;
                }

                unreachable!()
            }
        }
    }

    fn other_event(&mut self, ctx: &mut EventCtx, app: &mut App) -> Transition {
        app.session.update_music(ctx);
        Transition::Keep
    }

    fn draw(&self, g: &mut GfxCtx, app: &App) {
        app.session.music.draw(g);
    }
}
