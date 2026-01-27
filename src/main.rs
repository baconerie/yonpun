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
mod topbar;

use freya::prelude::*;

fn main() {
    // *Start* your app with a window and its root component
    launch(LaunchConfig::new()
        .with_window(
        WindowConfig::new(app)
            .with_title("Yonpun")
            .with_decorations(true)  // Hide window decorations (title bar)
            .with_transparency(false)
    ))
}

fn app() -> impl IntoElement {
    // Declare the *UI*
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(style::BACKGROUND)
        .color(Color::WHITE)
        .font_family(style::FONT_FAMILY)
        .child(topbar::Topbar)
}