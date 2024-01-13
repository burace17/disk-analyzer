/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// #![windows_subsystem = "windows"]
#![feature(start)]
#![allow(dead_code)]
#![allow(unused_imports)]
mod application;
mod directory;
// use application::gui;
use iced::Settings;

// #[start]
// fn main(x: isize, y: *const *const u8) -> iced::Result {
// fn init() -> iced::Result {
fn main() -> iced::Result {
   application::run(Settings::default())
}
