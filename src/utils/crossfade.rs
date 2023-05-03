use smallvec::SmallVec;

pub fn crossfade(fade_out: &[f32], fade_in: &[f32], state: usize) -> SmallVec<[f32; 1024]> {
    let len = fade_out.len();
    let fade_out_end = (state as f32 - 1.0) / 10.0;
    // println!("fade_out_end: {}", fade_out_end);

    let mut output = SmallVec::<[f32; 1024]>::with_capacity(len);

    for i in 0..len {
        let fade_ratio = i as f32 / (len - 1) as f32;
        let squared_fade_ratio = fade_ratio * fade_ratio; // from 0.0 to 1.0
        let fade_out_progress = fade_out_end + 0.1 * squared_fade_ratio;
        let fade_in_progress = (1.0 - fade_out_end) + 0.1 * squared_fade_ratio;
        output.push(fade_out[i] * fade_out_progress + fade_in[i] * fade_in_progress);
    }

    output
}
