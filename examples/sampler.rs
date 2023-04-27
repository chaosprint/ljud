use ljud::*;

fn main() {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(44100)
        .buffer_size(4)
        .set_graph(svec![("output", svec![audio_player("dun_dun_dun.wav")]),]);
    println!("{:?}", ctx.next_block());
}
