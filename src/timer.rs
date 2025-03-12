use tokio::time::{sleep, Duration};

pub async fn start_timer(name: &str, duration: u32) {
    println!("Start timer {} for {} Seconds", name, duration);

    for i in (1..=duration).rev() {
        println!("Remaining time:{}", i);
        sleep(Duration::from_secs(1)).await;
    }
    println!("{} is finished", name);
}
