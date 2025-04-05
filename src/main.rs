use std::process::{Command, Stdio};

fn main() {
    let mut bookticker_process = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg("bookticker")
        .stdout(Stdio::inherit()) // Use the same stdout as the parent
        .spawn()
        .expect("Failed to start bookticker listener");

    // let mut order_update_process = Command::new("cargo")
    //     .arg("run")
    //     .arg("--release")
    //     .arg("--bin")
    //     .arg("order_update")
    //     .stdout(Stdio::inherit()) // Use the same stdout as the parent
    //     .spawn()
    //     .expect("Failed to start order update listener");

    let _ = bookticker_process
        .wait()
        .expect("Bookticker listener failed");
    // let _ = order_update_process
    //     .wait()
    //     .expect("Order update listener failed");

    println!("Both listeners have finished.");
}
