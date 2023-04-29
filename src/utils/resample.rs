use arrayvec::ArrayVec;

pub fn linear_resample<const N: usize>(input: &[f32], output: &mut ArrayVec<f32, N>) {
    let input_len = input.len();
    let output_len = output.len();
    let scale = (input_len - 1) as f32 / (output_len - 1) as f32;

    for i in 0..output_len {
        let pos = i as f32 * scale;
        let pos_floor = pos.floor() as usize;
        let pos_ceil = pos.ceil() as usize;

        if pos_ceil >= input_len {
            output[i] = input[input_len - 1];
        } else if pos_floor == pos_ceil {
            output[i] = input[pos_floor];
        } else {
            let weight = pos - pos_floor as f32;
            output[i] = input[pos_floor] * (1.0 - weight) + input[pos_ceil] * weight;
        }
    }
}

// avoid generate new buffer
// const BUFFER_SIZE: usize = 1024;

// fn main() {
//     let input_signal: &[f32] = &[1.0, 2.0, 3.0, 4.0, 5.0];
//     let mut output_signal = ArrayVec::<_, BUFFER_SIZE>::new();
//     output_signal.extend((0..10).map(|_| 0.0));

//     linear_resample::<BUFFER_SIZE>(input_signal, &mut output_signal);

//     println!("{:?}", output_signal);
// }

pub fn cubic_interpolation(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let a0 = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
    let a1 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let a2 = -0.5 * y0 + 0.5 * y2;
    let a3 = y1;

    a0 * t * t * t + a1 * t * t + a2 * t + a3
}

pub fn cubic_resample<const N: usize>(input: &[f32], output: &mut ArrayVec<f32, N>) {
    let input_len = input.len();
    let output_len = output.len();
    let scale = (input_len - 1) as f32 / (output_len - 1) as f32;

    for i in 0..output_len {
        let pos = i as f32 * scale;
        let pos_floor = pos.floor() as usize;

        let y0 = if pos_floor == 0 {
            input[pos_floor]
        } else {
            input[pos_floor - 1]
        };
        let y1 = input[pos_floor];
        let y2 = if pos_floor + 1 < input_len {
            input[pos_floor + 1]
        } else {
            input[input_len - 1]
        };
        let y3 = if pos_floor + 2 < input_len {
            input[pos_floor + 2]
        } else {
            input[input_len - 1]
        };

        let t = pos - pos_floor as f32;
        output[i] = cubic_interpolation(y0, y1, y2, y3, t);
    }
}

// fn main() {
//     let input_signal: &[f32] = &[1.0, 2.0, 3.0, 4.0, 5.0];
//     let mut output_signal = ArrayVec::<f32, 1024>::new();
//     output_signal.extend((0..10).map(|_| 0.0));

//     cubic_resample::<1024>(input_signal, &mut output_signal);

//     println!("{:?}", output_signal);
// }
