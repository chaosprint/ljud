use ljud::*;

fn main() {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(44100)
        .buffer_size(4)
        .set_graph(svec![
            (
                "~fm",
                svec![sin_osc().freq(10.).amp(100.).boxed(), add(200.)],
            ),
            ("output", svec![sin_osc().freq("~fm").boxed()]),
        ]);
    println!("{:?}", ctx.next_block());
}
