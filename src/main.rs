/*
Copyright 2026 baconerie

This file is part of Yonpun.

Yonpun is free software: you can redistribute it and/or modify it under the 
terms of the GNU General Public License as published by the Free Software 
Foundation, either version 3 of the License, or (at your option) any later
version.

Yonpun is distributed in the hope that it will be useful, but WITHOUT ANY 
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A 
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
Yonpun. If not, see <https://www.gnu.org/licenses/>. 
*/

mod style;
mod components;
mod core;

use freya::prelude::*;
use freya::radio;

#[derive(Default, Clone)]
struct AppState {
    current_tab: String
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
enum AppStateChannels {
    CurrentTab
}
impl radio::RadioChannel<AppState> for AppStateChannels {}

fn main() {
    launch(LaunchConfig::new()
        .with_window(
        WindowConfig::new(app)
            .with_title("Yonpun")
            .with_decorations(true)
            .with_transparency(false)
        )
        .with_font(
            "Merriweather",
            Bytes::from_static(include_bytes!("./assets/merriweather.ttf"))
        )
        .with_font(
            "Nunito Extra Bold",
            Bytes::from_static(include_bytes!("./assets/nunito_extra_bold.ttf"))
        )
        .with_font(
            "Open Sans",
            Bytes::from_static(include_bytes!("./assets/open_sans.ttf"))
        )

    )
}

fn app() -> impl IntoElement {
    radio::use_init_radio_station::<AppState, AppStateChannels>(|| AppState{current_tab: String::from("Dashboard")});
    let radio = radio::use_radio(AppStateChannels::CurrentTab);
    let mut res  = rect()
        .content(Content::Fit)
        .width(Size::fill())
        .height(Size::fill())
        .background(style::BACKGROUND)
        .color(Color::WHITE)
        .font_family(style::FONT_FAMILY)
        .child(components::Topbar);

    if radio.read().current_tab == "Dashboard" {
        res = res.child(components::Dashboard);
    }
    else if radio.read().current_tab == "To-do" {
        res = res.child(components::ToDo);
    }

    res
}