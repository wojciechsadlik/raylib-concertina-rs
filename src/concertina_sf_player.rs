use std::collections::HashSet;
use std::vec;
use std::fs::File;
use std::sync::Arc;

use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;

use crate::BTN_MAP;

const SF_PATH: &str = "assets/Real-Life_Recorded_Accordion.sf2";
pub const SAMPLE_RATE: u32 = 44100;

pub struct ConcertinaSFPlayer {
    synthesizer: Synthesizer,
    btns_on: HashSet<String>
}

impl ConcertinaSFPlayer {
    pub fn new() -> ConcertinaSFPlayer {
        let mut sf2 = File::open(SF_PATH).unwrap();
        let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

        let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
        let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

        ConcertinaSFPlayer {
            synthesizer: synthesizer,
            btns_on: HashSet::new()
        }
    }

    pub fn btn_on(&mut self, btn: &String) {
        if self.btns_on.contains(btn) {
            return;
        }
        let note_id = match BTN_MAP.get(btn) {
            Some(note_id) => note_id,
            None => return,
        }.to_owned();
        self.btns_on.insert(btn.clone());
        self.synthesizer.note_on(0, note_id, 100);
    }

    pub fn btn_off(&mut self, btn: &String) {
        let note_id = match BTN_MAP.get(btn) {
            Some(note_id) => note_id,
            None => return,
        }.to_owned();
        self.synthesizer.note_off(0, note_id);
        self.btns_on.remove(btn);
    }

    pub fn btns_reset(&mut self) {
        self.synthesizer.note_off_all(false);
        self.btns_on.clear();
    }

    pub fn render(&mut self, out_buf: &mut [u8]) {
        let mut left = vec![0_f32; out_buf.len()];
        let mut right = vec![0_f32; out_buf.len()];
        self.synthesizer.render(&mut left, &mut right);

        let maxed: Vec<f32> = left.iter().zip(right.iter())
            .map(|(&a, &b)| (a.max(b) + 1.0) / 2.0)
            .collect();
        
        for (i, &v) in maxed.iter().enumerate() {
            out_buf[i] = (v * 255.0).round() as u8;
        }
    }
}
