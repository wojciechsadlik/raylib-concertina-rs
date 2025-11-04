use std::collections::HashMap;
use std::fs;

use raylib::prelude::*;

use crate::SCREEN_WIDTH;

const BG_PATH: &str = "assets/concertina_bg/c_g_30_jeffries.png";
const HLS_PATH: &str = "assets/concertina_btn_hls/";

pub struct ConcertinaLayout {
    pub bg: Texture2D,
    pub btn_hls: HashMap<String, Texture2D>
}

fn load_btn_hls(rl: &mut RaylibHandle, thread: &RaylibThread,
    width: i32, height: i32) -> HashMap<String, Texture2D> {
    
    let mut btn_hls = HashMap::new();

    for entry in fs::read_dir(HLS_PATH).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        let mut btn_hl_img = Image::load_image(&path_str).unwrap();

        btn_hl_img.resize(width, height);

        let btn_hl =  rl.load_texture_from_image(thread, &btn_hl_img).unwrap();
        
        let file_stem = path.file_stem().unwrap();
        let name = file_stem.to_str().unwrap();
        btn_hls.insert(name.to_string(), btn_hl);
    }
    
    btn_hls
}

pub fn load_concertina_layout(
    rl: &mut RaylibHandle, thread: &RaylibThread) -> ConcertinaLayout {
    
    let mut bg_img = Image::load_image(&BG_PATH).unwrap();

    let scale = SCREEN_WIDTH as f32 / bg_img.width() as f32;
    let new_width = (bg_img.width() as f32 * scale) as i32;
    let new_height = (bg_img.height() as f32 * scale) as i32;
    bg_img.resize(new_width, new_height);

    let bg = rl.load_texture_from_image(thread, &bg_img).unwrap();
    let btn_hls = load_btn_hls(rl, thread, new_width, new_height);
    
    ConcertinaLayout { bg: bg, btn_hls: btn_hls }
}
