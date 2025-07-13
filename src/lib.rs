
use tokio::sync::mpsc;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Payment {
    pub sender: String,
    pub receiver: String,
    pub amount: Decimal,
}

pub async fn process_payments(mut rx: mpsc::Receiver<Payment>) -> Decimal {
    let mut total = Decimal::ZERO;
    while let Some(payment) = rx.recv().await {
        total += payment.amount;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_process_payments() {
        let (tx, rx) = mpsc::channel(100);
        let payments = vec![
            Payment {
                sender: "user_1".to_string(),
                receiver: "merchant".to_string(),
                amount: dec!(10.5),
            },
            Payment {
                sender: "user_2".to_string(),
                receiver: "merchant".to_string(),
                amount: dec!(20.0),
            },
        ];

        for payment in payments {
            tx.send(payment).await.unwrap();
        }
        drop(tx);

        let total = process_payments(rx).await;
        assert_eq!(total, dec!(30.5));
    }
}
