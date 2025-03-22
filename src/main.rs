mod sound;

fn main() {
    tracing_subscriber::fmt::init();

    let mut sound = sound::Sound::new().expect("Failed to create Sound");
    sound
        .load(vec!["Master", "Master.strings"])
        .expect("Failed to load banks");

    for _ in 0..10 {
        sound.start("Jump").expect("Failed to play event");
        sound.update().expect("Failed to update Sound");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
