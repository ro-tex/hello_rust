use std::time::Duration;

use tokio::time::sleep;

async fn read_from_db() -> String {
    sleep(Duration::from_millis(2000)).await;
    "DB result".to_owned()
}

async fn future_fn(i: i32) {
    println!("this is the future!");
    let s1 = read_from_db().await;
    println!("[{i}] Got one: {s1}");
    let s2 = read_from_db().await;
    println!("[{i}] Got another: {s2}");
}

pub async fn run_async() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            future_fn(i).await;
        });
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}
