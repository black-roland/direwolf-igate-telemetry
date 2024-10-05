use sd_journal::Journal;
use chrono;

pub fn get_sound_level() -> i32 {
  let journal = Journal::open_files(["system@6f88fa98141b4f26a9ffc8439e1f2388-000000000005ad1c-000623a4a1eeb386.journal"]).unwrap();

  let (_start, end) = journal.get_realtime_cutoff().unwrap();
  let since = end
      .checked_sub_signed(chrono::Duration::seconds(65))
      .unwrap();
  journal.seek_realtime(since).unwrap();

  journal.add_match("SYSLOG_IDENTIFIER=direwolf").unwrap();

  // journal.seek_tail().unwrap();
  journal.next().unwrap();

  println!("{}", journal.get_data("MESSAGE").unwrap());

  0
}
