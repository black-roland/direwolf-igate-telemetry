/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use systemstat::{LoadAverage, Memory, Platform, System};
mod direwolf;

fn format_la(la: LoadAverage) -> i32 {
    (la.five * 10.0).round() as i32
}

fn format_mem(mem: Memory) -> u64 {
    let free_pct = (mem.free.as_u64() * 100) / mem.total.as_u64();

    100 - free_pct
}

fn main() {
    let sys = System::new();

    let la = sys.load_average().unwrap();
    let mem = sys.memory().unwrap();
    let temp = sys.cpu_temp().unwrap();

    let snd_lvl = direwolf::get_sound_level();

    println!("{} {} {} {}", format_la(la), format_mem(mem), temp.round(), snd_lvl);
}
