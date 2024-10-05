/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use systemstat::{System, Platform, LoadAverage};
mod direwolf;

fn format_la(la: LoadAverage) -> i32 {
    (la.five * 10.0).round() as i32
}

fn main() {
    let sys = System::new();

    let temp = sys.cpu_temp().unwrap();
    let la = sys.load_average().unwrap();

    let snd_lvl = direwolf::get_sound_level();

    println!("{} {} {}", temp.round(), format_la(la), snd_lvl);
}
