pub mod resample;
pub use resample::*;

pub mod convolution;
pub use convolution::*;

pub mod ringbuf;
pub use ringbuf::*;

pub mod crossfade;
pub use crossfade::*;

// #[cfg(feature = "plot")]
use gnuplot::*;

// #[cfg(feature = "plot")]
pub fn plot_mono(signal: &[f32], title: &str) {
    let x = (0..signal.len()).collect::<Vec<_>>();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(&x, signal, &[Caption("mono audio")]);
    fg.show().unwrap();
}
