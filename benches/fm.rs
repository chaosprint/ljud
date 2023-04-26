use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ljud::*;

fn fm(c: &mut Criterion) {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(44100)
        .buffer_size(128)
        .set_graph(svec![
            (
                "~fm",
                svec![sin_osc().freq(black_box(10.)).amp(100.).boxed(), add(200.)],
            ),
            ("output", svec![sin_osc().freq("~fm").boxed()]),
        ]);

    c.bench_function("fm", |b| {
        b.iter(|| {
            ctx.next_block();
        })
    });
}

criterion_group!(benches, fm);
criterion_main!(benches);
