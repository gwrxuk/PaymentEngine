use criterion::{Criterion, criterion_group, criterion_main};
use real_time_streaming_payment_engine::{Payment, process_payments};
use rust_decimal::Decimal;
use std::hint::black_box;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

fn benchmark_payment_processing(c: &mut Criterion) {
    c.bench_function("payment_processing", |b| {
        let runtime = Runtime::new().unwrap();
        b.to_async(runtime).iter(|| async {
            let (tx, rx) = mpsc::channel(1000);
            let processor_handle = tokio::spawn(process_payments(rx));

            for i in 0..1000 {
                let payment = Payment {
                    sender: format!("user_{}", i),
                    receiver: "merchant".to_string(),
                    amount: Decimal::new(i as i64, 2),
                };
                tx.send(black_box(payment)).await.unwrap();
            }

            drop(tx);
            black_box(processor_handle.await.unwrap());
        });
    });
}

criterion_group!(benches, benchmark_payment_processing);
criterion_main!(benches);
