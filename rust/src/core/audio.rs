use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::get_time;
use std::f32::consts::PI;

/// Bộ tổng hợp âm thanh thời gian thực bằng thuật toán (Procedural WAV Synthesizer)
pub struct SoundBank {
    pub shoot_light: Sound,
    pub shoot_heavy: Sound,
    pub tesla_zap: Sound,
    pub explosion: Sound,
    pub powerup: Sound,
    last_shoot_time: f64,
    last_explosion_time: f64,
}

impl SoundBank {
    pub async fn new() -> Self {
        let shoot_light = load_sound_from_bytes(&synth_wav(0.08, |t, p| {
            let freq = 680.0 * (1.0 - p * 0.72);
            ((t * freq * 2.0 * PI).sin() * 0.55 + (t * freq * 5.0 * PI).sin() * 0.25)
                * (1.0 - p).powi(2)
        }))
        .await
        .unwrap();

        let shoot_heavy = load_sound_from_bytes(&synth_wav(0.18, |t, p| {
            let freq = 260.0 * (1.0 - p * 0.65);
            let wave = (t * freq * 2.0 * PI).sin() * 0.65;
            let grit = ((t * 1950.0).sin() * (t * 830.0).cos()) * 0.35;
            (wave + grit) * (1.0 - p).powf(1.5)
        }))
        .await
        .unwrap();

        let tesla_zap = load_sound_from_bytes(&synth_wav(0.12, |t, p| {
            let mod_f = (t * 95.0 * 2.0 * PI).sin();
            let carrier = (t * (950.0 + mod_f * 380.0) * 2.0 * PI).sin();
            carrier * (1.0 - p) * 0.55
        }))
        .await
        .unwrap();

        let explosion = load_sound_from_bytes(&synth_wav(0.32, |t, p| {
            let sub = (t * 85.0 * (1.0 - p * 0.5) * 2.0 * PI).sin() * 0.75;
            // Nhiễu giả lập tiếng nổ trầm
            let noise = ((t * 12345.67).sin() * 43758.545).fract() * 2.0 - 1.0;
            (sub + noise * 0.35) * (1.0 - p).powi(2)
        }))
        .await
        .unwrap();

        let powerup = load_sound_from_bytes(&synth_wav(0.24, |t, p| {
            let freq = if p < 0.45 { 523.25 } else { 783.99 };
            (t * freq * 2.0 * PI).sin() * (1.0 - p * 0.7) * 0.55
        }))
        .await
        .unwrap();

        Self {
            shoot_light,
            shoot_heavy,
            tesla_zap,
            explosion,
            powerup,
            last_shoot_time: 0.0,
            last_explosion_time: 0.0,
        }
    }

    pub fn play_weapon_fire(&mut self, weapon_id: usize, master_vol: f32) {
        if master_vol <= 0.01 {
            return;
        }
        let now = get_time();
        if now - self.last_shoot_time < 0.045 {
            return;
        }
        self.last_shoot_time = now;

        let (snd, base_vol) = match weapon_id {
            0 | 2 => (&self.shoot_light, 0.22),
            3 => (&self.tesla_zap, 0.28),
            _ => (&self.shoot_heavy, 0.32),
        };
        play_sound(
            snd,
            PlaySoundParams {
                looped: false,
                volume: (base_vol * master_vol).clamp(0.0, 1.0),
            },
        );
    }

    pub fn play_explosion(&mut self, master_vol: f32) {
        if master_vol <= 0.01 {
            return;
        }
        let now = get_time();
        if now - self.last_explosion_time < 0.08 {
            return;
        }
        self.last_explosion_time = now;
        play_sound(
            &self.explosion,
            PlaySoundParams {
                looped: false,
                volume: (0.34 * master_vol).clamp(0.0, 1.0),
            },
        );
    }

    pub fn play_powerup(&self, master_vol: f32) {
        if master_vol <= 0.01 {
            return;
        }
        play_sound(
            &self.powerup,
            PlaySoundParams {
                looped: false,
                volume: (0.38 * master_vol).clamp(0.0, 1.0),
            },
        );
    }
}

/// Tạo bộ đệm WAV PCM 16-bit 22050Hz hoàn toàn trong bộ nhớ RAM
fn synth_wav<F>(duration_sec: f32, mut sample_fn: F) -> Vec<u8>
where
    F: FnMut(f32, f32) -> f32,
{
    let sample_rate: u32 = 22050;
    let num_samples = (duration_sec * sample_rate as f32) as u32;
    let data_size = num_samples * 2;
    let file_size = 36 + data_size;

    let mut buf = Vec::with_capacity((file_size + 8) as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes()); // PCM
    buf.extend_from_slice(&1u16.to_le_bytes()); // Mono
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let progress = i as f32 / num_samples.max(1) as f32;
        let val = sample_fn(t, progress).clamp(-1.0, 1.0);
        let pcm = (val * 32760.0) as i16;
        buf.extend_from_slice(&pcm.to_le_bytes());
    }
    buf
}
