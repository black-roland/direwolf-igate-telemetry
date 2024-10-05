use std::process::Command;
use regex::Regex;

pub fn get_sound_level() -> i32 {
  let journalctl_out = Command::new("journalctl")
    .args(&["-o", "cat", "SYSLOG_IDENTIFIER=direwolf", "--since", "65 seconds ago", "--reverse"])
    .output()
    .expect("Failed to execute journalctl");

  let stdout = String::from_utf8_lossy(&journalctl_out.stdout);
  let level_re = Regex::new(r"(?m)receive audio level CH0 ([0-9]{1,3})$").unwrap();

  let Some(captures) = level_re.captures(&stdout) else {
    return 0;
  };

  captures
    .get(1)
    .map_or(0, |m| m.as_str().parse::<i32>().unwrap())
}
