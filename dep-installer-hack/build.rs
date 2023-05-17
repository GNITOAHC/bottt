fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("cmake")
            .arg("youtube-dl")
            .arg("ffmpeg")
            // can add more here
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }
    }
}
