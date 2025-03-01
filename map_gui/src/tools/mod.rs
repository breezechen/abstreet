//! Assorted tools and UI states that're useful for applications built to display maps.

use abstio::MapName;
use geom::Polygon;
use widgetry::{lctrl, EventCtx, GfxCtx, Key, Line, Text, Widget};

pub use self::camera::{CameraState, DefaultMap};
pub use self::city_picker::CityPicker;
pub use self::colors::{ColorDiscrete, ColorLegend, ColorNetwork, ColorScale, DivergingScale};
pub use self::heatmap::{draw_isochrone, make_heatmap, Grid, HeatmapOptions};
pub use self::icons::{goal_marker, start_marker};
pub use self::labels::DrawRoadLabels;
pub use self::minimap::{Minimap, MinimapControls};
pub use self::navigate::Navigator;
pub use self::title_screen::{Executable, TitleScreen};
pub use self::turn_explorer::TurnExplorer;
pub use self::ui::{ChooseSomething, FilePicker, PopupMsg, PromptInput};
pub use self::url::URLManager;
use crate::AppLike;

#[cfg(not(target_arch = "wasm32"))]
pub use self::command::RunCommand;
#[cfg(not(target_arch = "wasm32"))]
pub use self::updater::prompt_to_download_missing_data;

mod camera;
mod city_picker;
mod colors;
#[cfg(not(target_arch = "wasm32"))]
mod command;
mod heatmap;
mod icons;
#[cfg(not(target_arch = "wasm32"))]
mod importer;
mod labels;
mod minimap;
mod navigate;
mod title_screen;
mod turn_explorer;
mod ui;
#[cfg(not(target_arch = "wasm32"))]
mod updater;
mod url;

// Update this ___before___ pushing the commit with "[rebuild] [release]".
const NEXT_RELEASE: &str = "0.2.66";

/// Returns the version of A/B Street to link to. When building for a release, this points to that
/// new release. Otherwise it points to the current dev version.
pub fn version() -> &'static str {
    if cfg!(feature = "release_s3") {
        NEXT_RELEASE
    } else {
        "dev"
    }
}

// TODO This is A/B Street specific
pub fn loading_tips() -> Text {
    Text::from_multiline(vec![
        Line("Have you tried..."),
        Line(""),
        Line("- simulating cities in Britain, Taiwan, Poland, and more?"),
        Line("- the 15-minute neighborhood tool?"),
        Line("- exploring all of the map layers?"),
        Line("- playing 15-minute Santa, our arcade game spin-off?"),
    ])
}

/// Make it clear the map can't be interacted with right now.
pub fn grey_out_map(g: &mut GfxCtx, app: &dyn AppLike) {
    g.fork_screenspace();
    // TODO - OSD height
    g.draw_polygon(
        app.cs().fade_map_dark,
        Polygon::rectangle(g.canvas.window_width, g.canvas.window_height),
    );
    g.unfork();
}

// TODO Associate this with maps, but somehow avoid reading the entire file when listing them.
pub fn nice_map_name(name: &MapName) -> &str {
    match name.city.country.as_ref() {
        "at" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("salzburg", "north") => "Salzburg (north)",
            ("salzburg", "south") => "Salzburg (south)",
            ("salzburg", "east") => "Salzburg (east)",
            ("salzburg", "west") => "Salzburg (west)",
            _ => &name.map,
        },
        "br" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("sao_paulo", "aricanduva") => "São Paulo (Avenue Aricanduva)",
            ("sao_paulo", "center") => "São Paulo (city center)",
            _ => &name.map,
        },
        "ca" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("montreal", "plateau") => "Montréal (Plateau)",
            _ => &name.map,
        },
        "ch" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("geneva", "center") => "Geneva",
            ("zurich", "center") => "Zürich (city center)",
            ("zurich", "north") => "Zürich (north)",
            ("zurich", "south") => "Zürich (south)",
            ("zurich", "east") => "Zürich (east)",
            ("zurich", "west") => "Zürich (west)",
            _ => &name.map,
        },
        "cz" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("frytek_mistek", "huge") => "Frýdek-Místek (entire area)",
            _ => &name.map,
        },
        "de" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("berlin", "center") => "Berlin (city center)",
            ("berlin", "neukolln") => "Berlin-Neukölln",
            ("bonn", "center") => "Bonn (city center)",
            ("bonn", "nordstadt") => "Bonn (Nordstadt)",
            ("bonn", "venusberg") => "Bonn (Venusberg)",
            ("rostock", "center") => "Rostock",
            _ => &name.map,
        },
        "fr" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("charleville_mezieres", "secteur1") => "Charleville-Mézières (secteur 1)",
            ("charleville_mezieres", "secteur2") => "Charleville-Mézières (secteur 2)",
            ("charleville_mezieres", "secteur3") => "Charleville-Mézières (secteur 3)",
            ("charleville_mezieres", "secteur4") => "Charleville-Mézières (secteur 4)",
            ("charleville_mezieres", "secteur5") => "Charleville-Mézières (secteur 5)",
            ("lyon", "center") => "Lyon",
            ("paris", "center") => "Paris (city center)",
            ("paris", "north") => "Paris (north)",
            ("paris", "south") => "Paris (south)",
            ("paris", "east") => "Paris (east)",
            ("paris", "west") => "Paris (west)",
            _ => &name.map,
        },
        "gb" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("allerton_bywater", "center") => "Allerton Bywater",
            ("ashton_park", "center") => "Ashton Park",
            ("aylesbury", "center") => "Aylesbury",
            ("aylesham", "center") => "Aylesham",
            ("bailrigg", "center") => "Bailrigg (Lancaster)",
            ("bath_riverside", "center") => "Bath Riverside",
            ("bicester", "center") => "Bicester",
            ("cambridge", "north") => "North Cambridge",
            ("castlemead", "center") => "Castlemead",
            ("chapelford", "center") => "Chapelford (Cheshire)",
            ("chapeltown_cohousing", "center") => "Chapeltown Cohousing",
            ("chorlton", "center") => "Chorlton",
            ("clackers_brook", "center") => "Clackers Brook",
            ("cricklewood", "center") => "Cricklewood",
            ("culm", "center") => "Culm",
            ("dickens_heath", "center") => "Dickens Heath",
            ("didcot", "center") => "Didcot (Harwell)",
            ("dunton_hills", "center") => "Dunton Hills",
            ("ebbsfleet", "center") => "Ebbsfleet (Dartford)",
            ("exeter_red_cow_village", "center") => "Exeter Red Cow Village",
            ("great_kneighton", "center") => "Great Kneighton (Cambridge)",
            ("halsnhead", "center") => "Halsnead",
            ("hampton", "center") => "Hampton",
            ("kergilliack", "center") => "Kergilliack",
            ("kidbrooke_village", "center") => "Kidbrooke Village",
            ("lcid", "center") => "Leeds Climate Innovation District",
            ("leeds", "central") => "Leeds (city center)",
            ("leeds", "huge") => "Leeds (entire area inside motorways)",
            ("leeds", "north") => "North Leeds",
            ("leeds", "west") => "West Leeds",
            ("lockleaze", "center") => "Lockleaze",
            ("london", "a5") => "London A5 (Hyde Park to Edgware)",
            ("london", "Camden") => "Camden",
            ("london", "southbank") => "London (Southbank)",
            ("long_marston", "center") => "Long Marston (Stratford)",
            ("marsh_barton", "center") => "Marsh Barton",
            ("micklefield", "center") => "Micklefield",
            ("newborough_road", "center") => "Newborough Road",
            ("newcastle_great_park", "center") => "Newcastle Great Park",
            ("northwick_park", "center") => "Northwick Park",
            ("poundbury", "center") => "Poundbury",
            ("priors_hall", "center") => "Priors Hall",
            ("st_albans", "center") => "St Albans",
            ("taunton_firepool", "center") => "Taunton Firepool",
            ("taunton_garden", "center") => "Taunton Garden",
            ("tresham", "center") => "Tresham",
            ("trumpington_meadows", "center") => "Trumpington Meadows",
            ("tyersal_lane", "center") => "Tyersal Lane",
            ("upton", "center") => "Upton",
            ("water_lane", "center") => "Water Lane",
            ("wichelstowe", "center") => "Wichelstowe",
            ("wixams", "center") => "Wixams",
            ("wynyard", "center") => "Wynyard",
            _ => &name.map,
        },
        "il" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("tel_aviv", "center") => "Tel Aviv (city center)",
            _ => &name.map,
        },
        "ir" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("tehran", "parliament") => "Tehran (near Parliament)",
            // TODO I'm not naming the other 9 maps in Tehran, because I'm not sure yet the
            // boundaries are the ones that a researcher needs.
            _ => &name.map,
        },
        "jp" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("hiroshima", "uni") => "Hiroshima University",
            _ => &name.map,
        },
        "ly" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("tripoli", "center") => "Tripoli",
            _ => &name.map,
        },
        "nz" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("auckland", "mangere") => "Māngere (Auckland)",
            _ => &name.map,
        },
        "pl" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("krakow", "center") => "Kraków (city center)",
            ("warsaw", "center") => "Warsaw (city center)",
            _ => &name.map,
        },
        "sg" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("jurong", "center") => "Jurong",
            _ => &name.map,
        },
        "tw" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("taipei", "center") => "Taipei (city center)",
            _ => &name.map,
        },
        "us" => match (name.city.city.as_ref(), name.map.as_ref()) {
            ("anchorage", "downtown") => "Anchorage",
            ("bellevue", "huge") => "Bellevue",
            ("beltsville", "i495") => "I-495 in Beltsville, MD",
            ("detroit", "downtown") => "Detroit",
            ("milwaukee", "downtown") => "Downtown Milwaukee",
            ("milwaukee", "oak_creek") => "Oak Creek",
            ("mt_vernon", "burlington") => "Burlington",
            ("mt_vernon", "downtown") => "Mt. Vernon",
            ("nyc", "lower_manhattan") => "Lower Manhattan",
            ("nyc", "midtown_manhattan") => "Midtown Manhattan",
            ("nyc", "downtown_brooklyn") => "Downtown Brooklyn",
            ("phoenix", "gilbert") => "Gilbert",
            ("phoenix", "loop101") => "Loop 101 (no local roads)",
            ("phoenix", "tempe") => "Tempe",
            ("providence", "downtown") => "Providence",
            ("san_francisco", "downtown") => "San Francisco",
            ("seattle", "arboretum") => "Arboretum",
            ("seattle", "central_seattle") => "Central Seattle",
            ("seattle", "downtown") => "Downtown Seattle",
            ("seattle", "huge_seattle") => "Seattle (entire area)",
            ("seattle", "lakeslice") => "Lake Washington corridor",
            ("seattle", "montlake") => "Montlake and Eastlake",
            ("seattle", "north_seattle") => "North Seattle",
            ("seattle", "phinney") => "Phinney Ridge",
            ("seattle", "qa") => "Queen Anne",
            ("seattle", "slu") => "South Lake Union",
            ("seattle", "south_seattle") => "South Seattle",
            ("seattle", "udistrict_ravenna") => "University District",
            ("seattle", "wallingford") => "Wallingford",
            ("seattle", "west_seattle") => "West Seattle",
            ("tucson", "center") => "Tucson",
            _ => &name.map,
        },
        _ => &name.map,
    }
}

pub fn nice_country_name(code: &str) -> &str {
    // If you add something here, please also add the flag to data/system/assets/flags.
    // https://github.com/hampusborgos/country-flags/tree/master/svg
    match code {
        "at" => "Austria",
        "br" => "Brazil",
        "ca" => "Canada",
        "ch" => "Switzerland",
        "cz" => "Czech Republic",
        "de" => "Germany",
        "fr" => "France",
        "gb" => "Great Britain",
        "il" => "Israel",
        "ir" => "Iran",
        "jp" => "Japan",
        "ly" => "Libya",
        "nz" => "New Zealand",
        "pl" => "Poland",
        "sg" => "Singapore",
        "tw" => "Taiwan",
        "us" => "United States of America",
        _ => code,
    }
}

pub fn open_browser<I: AsRef<str>>(url: I) {
    let _ = webbrowser::open(url.as_ref());
}

/// Returns the path to an executable. Native-only.
pub fn find_exe(cmd: &str) -> String {
    for dir in [
        // When running from source, prefer release builds, but fallback to debug. This might be
        // confusing when developing and not recompiling in release mode.
        "./target/release",
        "../target/release",
        "../../target/release",
        "./target/debug",
        "../target/debug",
        "../../target/debug",
        // When running from the .zip release
        ".",
        "..",
    ] {
        // Apparently std::path on Windows doesn't do any of this correction. We could build up a
        // PathBuf properly, I guess
        let path = if cfg!(windows) {
            format!("{}/{}.exe", dir, cmd).replace("/", "\\")
        } else {
            format!("{}/{}", dir, cmd)
        };
        if std::path::Path::new(&path).exists() {
            return path;
        }
    }
    panic!("Couldn't find the {} executable", cmd);
}

/// A button to change maps, with default keybindings
pub fn change_map_btn(ctx: &EventCtx, app: &dyn AppLike) -> Widget {
    ctx.style()
        .btn_popup_icon_text(
            "system/assets/tools/map.svg",
            nice_map_name(app.map().get_name()),
        )
        .hotkey(lctrl(Key::L))
        .build_widget(ctx, "change map")
}

/// A button to return to the title screen
pub fn home_btn(ctx: &EventCtx) -> Widget {
    ctx.style()
        .btn_plain
        .btn()
        .image_path("system/assets/pregame/logo.svg")
        .image_dims(50.0)
        .build_widget(ctx, "Home")
}

/// A standard way to group a home button back to the title screen, the title of the current app,
/// and a button to change maps. Callers must handle the `change map` and `home` click events.
pub fn app_header(ctx: &EventCtx, app: &dyn AppLike, title: &str) -> Widget {
    Widget::col(vec![
        Widget::row(vec![
            home_btn(ctx),
            Line(title).small_heading().into_widget(ctx).centered_vert(),
        ]),
        change_map_btn(ctx, app),
    ])
}
