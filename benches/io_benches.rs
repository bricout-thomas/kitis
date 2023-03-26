use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bracket_terminal::prelude::*;

// function arguments should be passed inside a blackbox to avoid compiler optimizations

fn get_bterm() -> BTerm {
    BTermBuilder::simple(40, 40).unwrap().build().unwrap()
}

fn bench_terminal_buffer(c: char) {
    let mut ctx = get_bterm();
    let mut buffer = String::with_capacity(40);
    for _ in 0..40 {
        buffer.push(c);
    }
    ctx.print(0, 0, buffer);
}

fn bench_terminal_calls(c: char) {
    let mut ctx = get_bterm();
    for i in 0..40 {
        ctx.print(i, 0, c);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("buffer", |b| {
        b.iter(|| {bench_terminal_buffer(black_box('c'))})
    });
    c.bench_function("calls", |b| {
        b.iter(|| {bench_terminal_calls(black_box('c'))})
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);