use tokio::time::{self, Duration};

// tokio::time::sleepを使って、決まった時間を図るタイマーを作る
#[tokio::main]
async fn main() {
    let wait_sec = 5;
    let (_, mut wait) = tokio::sync::oneshot::channel::<()>();
    let (sleep_tx, mut sleep_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        time::sleep(Duration::from_secs(wait_sec)).await;
        sleep_tx.send("done").unwrap();
    });

    loop {
        tokio::select! {
            _ = &mut sleep_rx => {
                 println!("{} secondes elapsed", wait_sec);
                 break;
            }
            _ = &mut wait => {}
        }
    }
}
