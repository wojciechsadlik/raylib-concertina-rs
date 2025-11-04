use std::collections::{HashSet, VecDeque};

use raylib::prelude::*;
use concertina_btn::*;
use concertina_layout::*;
use concertina_sf_player::*;
use keyboard_mapping::*;

mod concertina_btn;
mod concertina_layout;
mod concertina_sf_player;
mod keyboard_mapping;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 960;
const MAX_SAMPLES_PER_UPDATE: i32 = 4096;

fn main() {
    let (mut rl, rl_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raylib Concertina")
        .build();

    let rl_audio = RaylibAudio::init_audio_device().unwrap();
    rl_audio.set_audio_stream_buffer_size_default(MAX_SAMPLES_PER_UPDATE);
    let mut stream = rl_audio.new_audio_stream(SAMPLE_RATE, 8, 1);
    stream.play();
    let mut stream_buf = [128_u8; (MAX_SAMPLES_PER_UPDATE*2) as usize];

    let mut btns_player = ConcertinaSFPlayer::new();

    let concertina = load_concertina_layout(&mut rl, &rl_thread);
    let concertina_anchor = (0, SCREEN_HEIGHT / 2 - concertina.bg.height() / 2);

    rl.set_target_fps(30);

    let mut pressed_keys: HashSet<consts::KeyboardKey> = HashSet::new();
    let mut was_pushing = false;
    while !rl.window_should_close() {
        let is_pushing = rl.is_key_down(consts::KeyboardKey::KEY_SPACE);
        if !(is_pushing && was_pushing) {
            btns_player.btns_reset();
            was_pushing = is_pushing;
        }

        let mut pushed_btn_hls: VecDeque<&Texture2D> = VecDeque::new();
        loop {
            let key_pressed = match rl.get_key_pressed() {
                Some(key_code) => key_code,
                None => break,
            };
            pressed_keys.insert(key_pressed);
        }

        let were_pressed_keys = pressed_keys.clone();
        pressed_keys.retain(|&key| rl.is_key_down(key));

        let released_keys: HashSet<consts::KeyboardKey>
            = were_pressed_keys.difference(&pressed_keys).cloned().collect();
        for key in &released_keys {
            if let Some(btn_nr) = KEYBOARD_MAP.get(&key) {
                let c_btn = ConcertinaBtn::new(btn_nr.to_string(), is_pushing);
                let btn_id = c_btn.to_string();
                btns_player.btn_off(&btn_id);
            }
        }

        for key in &pressed_keys {
            if let Some(btn_nr) = KEYBOARD_MAP.get(&key) {
                let c_btn = ConcertinaBtn::new(btn_nr.to_string(), is_pushing);
                let btn_id = c_btn.to_string();
                btns_player.btn_on(&btn_id);
                let btn_hl = concertina.btn_hls.get(&btn_id).unwrap();
                pushed_btn_hls.push_back(btn_hl);
            }
        }

        if stream.is_processed() {
            btns_player.render(&mut stream_buf);
            let shift = (rl.get_time().fract() * MAX_SAMPLES_PER_UPDATE as f64)
                .round() as usize;
            let buf_frame = (shift,
                             shift + MAX_SAMPLES_PER_UPDATE as usize);
            stream.update(&stream_buf[buf_frame.0..buf_frame.1]);
        }

        let mut d = rl.begin_drawing(&rl_thread);

        d.clear_background(Color::WHITE);
        d.draw_texture(&concertina.bg,
            concertina_anchor.0, concertina_anchor.1, Color::WHITE);
        if is_pushing {
            d.draw_texture(&concertina.btn_hls.get("push").unwrap(),
                concertina_anchor.0, concertina_anchor.1, Color::WHITE);
        } else {
            d.draw_texture(&concertina.btn_hls.get("pull").unwrap(),
                concertina_anchor.0, concertina_anchor.1, Color::WHITE);
        }

        while !pushed_btn_hls.is_empty() {
            d.draw_texture(
                &pushed_btn_hls.pop_back().unwrap(),
                concertina_anchor.0, concertina_anchor.1, Color::WHITE);
        }
    }
}
