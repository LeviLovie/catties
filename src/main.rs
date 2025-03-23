mod config;
mod sound;

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect};
use std::time::{Duration, Instant};
use std_utils::{errors::*, paths::rel_path, Result};
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::Config::from_file("assets/config.yaml".into())
        .logmsg("Error while parsing \"config.yaml\"")?;

    let tiles_config_path = rel_path(&config.tiles.config.clone())?;
    let tiles = map::Tiles::from_file(tiles_config_path.clone())
        .logmsg(&format!("Error while parsing {:?}", tiles_config_path))?;

    let frame_time = Duration::from_secs_f64(1.0 / config.defaults.fps as f64);

    let mut sound = sound::Sound::new().expect("Failed to create Sound");
    sound
        .load(vec!["Master", "Master.strings"])
        .expect("Failed to load banks");

    sdl2::image::init(sdl2::image::InitFlag::PNG).anyhow()?;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Catties", config.defaults.width, config.defaults.height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let tiles_texture = texture_creator
        .load_texture(rel_path(&config.tiles.file)?)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    sound.start("Jump").expect("Failed to play event");
                    sound.update().expect("Failed to update Sound");
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for (i, tile) in tiles.iter().enumerate() {
            let source_rect = Rect::new(
                tile.rect.x as i32 - tile.rect.o as i32,
                tile.rect.y as i32,
                tile.rect.w,
                tile.rect.h,
            );

            let dest_rect = Rect::new(
                2 + 128 * i as i32 + 48,
                64,
                tile.rect.w * 2,
                tile.rect.h * 2,
            );

            canvas
                .copy(&tiles_texture, source_rect, dest_rect)
                .anyhow()?;
        }

        canvas.present();
        let elapsed = last_update.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        } else {
            info!("Frame took too long: {:?}", elapsed);
        }
        last_update = Instant::now();
    }

    return Ok(());
}
