use std::{fs::OpenOptions, io::{Read, Seek, Write}};

fn main() -> std::io::Result<()> {
  let seq_file_path = {
    let mut path = std::env::temp_dir();
    path.push("seq");
    path
  };

  let mut seq_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(seq_file_path)?;

  let mut seq_str = String::new();
  seq_file.read_to_string(&mut seq_str)?;

  let new_seq = (i32::from_str_radix(seq_str.trim(), 10).unwrap_or(-1) + 1) % 100;

  seq_file.set_len(0)?;
  seq_file.rewind()?;
  seq_file.write_all(format!("{}", new_seq).as_bytes())?;

  println!("{}", new_seq);

  Ok(())
}
