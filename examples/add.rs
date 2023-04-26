use ljud::*;

fn main() {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(44100)
        .buffer_size(4)
        .set_graph(svec![
            ("~side", svec![add(32.)]),
            ("output", svec![add("~side")]),
        ]);
    println!("{:?}", ctx.next_block());
}
