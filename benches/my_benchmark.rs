use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};

use wordList::{byte_list, string_set, word_list};

const FILE: &str = "src/byte_list.rs";

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Load");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("byte_list", |b| {
        b.iter(|| byte_list::ByteMatch::load(black_box(FILE)))
    });
    group.bench_function("word_list", |b| {
        b.iter(|| word_list::WordMatch::load(black_box(FILE)))
    });
    group.bench_function("string_set", |b| {
        b.iter(|| string_set::SetMatch::load(black_box(FILE)))
    });

    group.finish();
}

fn seek(c: &mut Criterion) {
    let bl = byte_list::ByteMatch::load(black_box(FILE)).unwrap();
    let wl = word_list::WordMatch::load(black_box(FILE)).unwrap();
    let ss = string_set::SetMatch::load(black_box(FILE)).unwrap();

    let lines = &[
        " * and <lxboot> is the name of a bootblock to merge in.  This bootblock",
        " * MCPCIA is the internal name for a core logic chipset which provides",
        "static inline unsigned long",
        " * (b) require any knowledge of processes at this stage",
    ];

    let mut group = c.benchmark_group("Seek");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("byte_list", |b| {
        b.iter(|| {
            for s in lines {
                bl.contains(s);
            }
        })
    });
    group.bench_function("word_list", |b| {
        b.iter(|| {
            for s in lines {
                wl.contains(s);
            }
        })
    });
    group.bench_function("string_set", |b| {
        b.iter(|| {
            for s in lines {
                ss.contains(s);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark, seek);
criterion_main!(benches);
