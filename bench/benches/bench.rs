#![allow(unused)]
fn main() {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    pub fn criterion_benchmark(c: &mut Criterion) {
        // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);
}

fn normal_diff(a: Vec<Pixel>, b: Vec<Pixel>) -> Vec<Pixel> {
    let mut differences = vec![];

    for p in &a {
        if !b.contains(p) {
            differences.push(p.clone());
        }
    }

    for p in b {
        if !a.contains(&p) {
            differences.push(p);
        }
    }

    differences
}

fn fast_diff(a: Vec<Pixel>, b: Vec<Pixel>) -> Vec<Pixel> {
    let mut a = a;
    let mut b = b;

    for pa in a {
        for pb in b.clone() {
            if pa == pb {
                b.retain(|x| x != &pa);
            }
        }
    }

    // TODO

    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pixel {
    x: u32,
    y: u32,
    content: String,
}
