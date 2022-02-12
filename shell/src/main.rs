fn main() {
    std::process::Command::new("vim")
        .arg("Cargo.lock")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
