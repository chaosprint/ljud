use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ljud::*;

fn conv(c: &mut Criterion) {
    let elevation = "20"; // see the `full` folder
    let azimuth = "270"; // see the `full` folder

    let mut ctx = Context::new()
        .channels(2 as u8)
        .sample_rate(44100)
        .buffer_size(128 as u32)
        .set_graph(svec![(
            "output",
            svec![
                audio_player("assets/sounds/dun_dun_dun.wav")
                    .looping(black_box(false))
                    .boxed(),
                convolution([
                    &format!("assets/mit-hrtf/full/elev{elevation}/L{elevation}e{azimuth}a.wav"),
                    &format!("assets/mit-hrtf/full/elev{elevation}/L{elevation}e{azimuth}a.wav")
                ])
                .boxed()
            ]
        )]);

    c.bench_function("conv", |b| {
        b.iter(|| {
            ctx.next_block();
        })
    });
}

criterion_group!(benches, conv);
criterion_main!(benches);
