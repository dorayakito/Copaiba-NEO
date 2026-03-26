use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use crate::audio::PitchData;

pub fn compute_pitch_data(samples: &[f32], sample_rate: u32) -> Option<PitchData> {
    if samples.is_empty() { return None; }

    // Window ~30ms for good pitch detection down to ~80Hz
    let window_size = 1024.min(samples.len());
    let hop_size = window_size / 2;
    
    let mut detector = McLeodDetector::new(window_size, hop_size);
    
    let mut times = Vec::new();
    let mut frequencies = Vec::new();
    
    let mut i = 0;
    while i + window_size <= samples.len() {
        let chunk = &samples[i..i + window_size];
        
        // McLeod results: Better for musical pitch than YIN in some cases
        if let Some(pitch) = detector.get_pitch(chunk, sample_rate as usize, 1.0, 0.15) {
            let time_ms = (i as f64 / sample_rate as f64) * 1000.0;
            times.push(time_ms);
            frequencies.push(pitch.frequency as f64);
        } else {
            // Push 0 or NaN to indicate unvoiced?
            // Usually for a curve we just skip or push 0.0
            let time_ms = (i as f64 / sample_rate as f64) * 1000.0;
            times.push(time_ms);
            frequencies.push(0.0);
        }
        
        i += hop_size;
    }
    
    if times.is_empty() { return None; }
    
    Some(PitchData { times, frequencies })
}
