use crate::{svec, SmallVec};

pub fn convolve(a: &[f32], b: &[f32]) -> SmallVec<[f32; 1024]> {
    let padding = 0;
    let a_len = a.len();
    let b_len = b.len();
    let padded_a_len = a_len + 2 * padding;
    let out_len = padded_a_len + b_len - 1;
    let mut out = svec![0.0; out_len];

    let mut padded_a = vec![0.0; padded_a_len];
    padded_a[padding..padding + a_len].copy_from_slice(a);

    for i in 0..out_len {
        let mut acc = 0.0;
        for j in 0..b_len {
            if i >= j && i - j < padded_a_len {
                acc += padded_a[i - j] * b[j];
            }
        }
        out[i] = acc;
    }
    out
}

// pub fn convolve(a: &[f32], b: &[f32]) -> SmallVec<[f32; 1024]> {
//     let a_len = a.len();
//     let b_len = b.len();
//     let out_len = a_len + b_len - 1;
//     let mut out = svec![0.0; out_len];

//     for i in 0..out_len {
//         let mut acc = 0.0;
//         for j in 0..b_len {
//             if i >= j && i - j < a_len {
//                 acc += a[i - j] * b[j];
//             }
//         }
//         out[i] = acc;
//     }
//     out
// }
