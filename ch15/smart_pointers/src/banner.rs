use terminal_size::{terminal_size, Width};

pub fn banner(message: &str) {
  let lines = boxlines(message.len());
  println!("{}\n{}\n{}", lines, message, lines);
}

fn boxlines(default: usize) -> String {
  let size = terminal_size();
  if let Some((Width(termwidth), _)) = size {
    "-".repeat(termwidth as usize)
  } else {
    "-".repeat(default)
  }
}
