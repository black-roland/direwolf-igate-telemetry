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

    println!("{} {} {} 0 0", temp.round(), format_la(la), snd_lvl);
}
