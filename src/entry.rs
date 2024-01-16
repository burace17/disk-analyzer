/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// #![windows_subsystem = "windows"]
#![feature(start)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
mod application;
mod directory;
mod events;
mod frontend { mod styling; }
mod analyzer;
// use application::gui;
use iced::Settings;

#[start]
// fn main(x: isize, y: *const *const u8) -> iced::Result {
fn init(_x: isize, _b: *const *const u8) -> isize {
// fn main() -> iced::Result {
   application::run(Settings::default());
   0
}
