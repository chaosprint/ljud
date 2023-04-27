use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ljud::*;

fn conv(c: &mut Criterion) {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(44100)
        .buffer_size(128)
        .set_graph(svec![(
            "output",
            svec![
                audio_player("dun_dun_dun.wav")
                    .looping(black_box(true))
                    .boxed(),
                convolution("full/elev-10/L-10e000a.wav").boxed()
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
