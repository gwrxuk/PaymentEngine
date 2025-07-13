
use payment_engine::{Payment, process_payments};
use tokio::sync::mpsc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    let processor_handle = tokio::spawn(async move {
        process_payments(rx).await
    });

    for i in 0..10 {
        let payment = Payment {
            sender: format!("user_{}", i),
            receiver: "merchant".to_string(),
            amount: dec!(10.5) + Decimal::from(i),
        };
        if let Err(e) = tx.send(payment).await {
            eprintln!("Failed to send payment: {}", e);
            break;
        }
    }

    drop(tx);

    let total = processor_handle.await.unwrap();
    println!("Total payments processed: {}", total);
}
