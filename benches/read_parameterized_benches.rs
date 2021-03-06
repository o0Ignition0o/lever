use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use lever::table::prelude::*;

use std::sync::Arc;

const MAX_THREADS: usize = 8;
const OP_RANGES: &'static [usize] = &[100, 300, 500, 700, 1000, 3000, 5000];

fn pure_read(lotable: Arc<LOTable<String, u64>>, key: String, op_count: usize) {
    (0..op_count).into_iter().for_each(|_| {
        let _ = lotable.get(&key);
    })
}

fn bench_pure_reads(c: &mut Criterion) {
    let lotable: Arc<LOTable<String, u64>> = Arc::new(LOTable::new());
    let key: String = "CORE".into();
    let _ = lotable.insert(key.clone(), 123_456);

    let mut group = c.benchmark_group("parameterized_read");

    (1..MAX_THREADS).for_each(|tid| {
        for ops in OP_RANGES {
            group.throughput(Throughput::Elements((tid * *ops) as u64));
            group.bench_with_input(BenchmarkId::new(format!("{}", tid), ops), ops, |b, &i| {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(tid)
                    .build()
                    .unwrap();
                pool.install(|| b.iter(|| pure_read(lotable.clone(), key.clone(), i)));
            });
        }
    });
}

////////////////////////////////
//
// fn rw_pareto(lotable: Arc<LOTable<String, u64>>, key: String, dist: f64, thread_count: u64) {
//     let mut threads = vec![];
//
//     for thread_no in 0..thread_count {
//         let lotable = lotable.clone();
//         let key = key.clone();
//
//         let t = std::thread::Builder::new()
//             .name(format!("t_{}", thread_no))
//             .spawn(move || {
//                 if dist < 0.8_f64 {
//                     lotable.get(&key);
//                 } else {
//                     let data = lotable.get(&key).unwrap();
//                     let _ = lotable.insert(key, data + 1);
//                 }
//             })
//             .unwrap();
//
//         threads.push(t);
//     }
//
//     for t in threads.into_iter() {
//         t.join().unwrap();
//     }
// }
//
// fn bench_rw_pareto(c: &mut Criterion) {
//     let lotable = {
//         let table: LOTable<String, u64> = LOTable::new();
//         table.insert("data".into(), 1_u64);
//         Arc::new(table)
//     };
//     let key: String = "CORE".into();
//     let _ = lotable.insert(key.clone(), 123_456);
//
//     let threads = 8;
//
//     let mut group = c.benchmark_group("lotable_threaded_join_rw_pareto_throughput");
//     group.throughput(Throughput::Elements(threads as u64));
//     group.bench_function("rw_pareto", move |b| {
//         b.iter_batched(
//             || {
//                 let dist: f64 =
//                     1. / thread_rng().sample(Pareto::new(1., 5.0_f64.log(4.0_f64)).unwrap());
//                 (lotable.clone(), key.clone(), dist)
//             },
//             |vars| rw_pareto(vars.0, vars.1, vars.2, threads),
//             BatchSize::SmallInput,
//         )
//     });
// }
//
// ////////////////////////////////
//
// fn pure_writes(lotable: Arc<LOTable<String, u64>>, key: String, thread_count: u64) {
//     let mut threads = vec![];
//
//     for thread_no in 0..thread_count {
//         let lotable = lotable.clone();
//         let key = key.clone();
//
//         let t = std::thread::Builder::new()
//             .name(format!("t_{}", thread_no))
//             .spawn(move || {
//                 let _ = lotable.insert(key, thread_no);
//             })
//             .unwrap();
//
//         threads.push(t);
//     }
//
//     for t in threads.into_iter() {
//         t.join().unwrap();
//     }
// }
//
// fn bench_lotable_pure_writes(c: &mut Criterion) {
//     let lotable = {
//         let table: LOTable<String, u64> = LOTable::new();
//         table.insert("data".into(), 1_u64);
//         Arc::new(table)
//     };
//     let key: String = "CORE".into();
//     let _ = lotable.insert(key.clone(), 123_456);
//
//     let threads = 8;
//
//     let mut group = c.benchmark_group("lotable_threaded_join_write_throughput");
//     group.throughput(Throughput::Elements(threads as u64));
//     group.bench_function("pure writes", move |b| {
//         b.iter_batched(
//             || (lotable.clone(), key.clone()),
//             |vars| pure_writes(vars.0, vars.1, threads),
//             BatchSize::SmallInput,
//         )
//     });
// }

criterion_group! {
    name = parameterized_benches;
    config = Criterion::default();
    targets = bench_pure_reads
    // targets = bench_pure_reads, bench_rw_pareto, bench_lotable_pure_writes
}
criterion_main!(parameterized_benches);
