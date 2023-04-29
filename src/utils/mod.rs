pub mod resample;
pub use resample::*;

pub mod convolution;
pub use convolution::*;

pub mod ringbuf;
pub use ringbuf::*;

#[cfg(feature = "plot")]
use gnuplot::*;

#[cfg(feature = "plot")]
pub fn plot_mono(signal: &[f32]) {
    let x = (0..signal.len()).collect::<Vec<_>>();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(&x, signal, &[Caption("left")]);
    fg.show().unwrap();
}
