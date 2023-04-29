use crate::{svec, SmallVec};

pub fn convolve(a: &[f32], b: &[f32]) -> SmallVec<[f32; 1024]> {
    let a_len = a.len();
    let b_len = b.len();
    let out_len = a_len + b_len - 1;
    let mut out = svec![0.0; out_len];

    for i in 0..out_len {
        let mut acc = 0.0;
        for j in 0..b_len {
            if i >= j && i - j < a_len {
                acc += a[i - j] * b[j];
            }
        }
        out[i] = acc;
    }
    out
}
