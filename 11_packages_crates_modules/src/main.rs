mod sound;

fn main() {
  // Absolute path
  crate::sound::instrument::clarinet();

  // Relative path
  sound::instrument::clarinet();
}
