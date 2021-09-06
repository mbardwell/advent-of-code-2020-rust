use std::process::Command;

fn main() {
    let _total_time = (1..=25).map(|day_num| {
            let day = format!("{:0>2}", day_num);
            let _cmd = Command::new("cargo")
                .args(&["run", "--release", "--bin", &day])
                .output()
                .unwrap();
    });
}
