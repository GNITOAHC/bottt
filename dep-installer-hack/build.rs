fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        if !std::process::Command::new("curl")
            .arg("-L")
            .arg("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp")
            .arg("-o")
            .arg("/usr/local/bin/yt-dlp")
            .status()
            .expect("failed to run curl")
            .success()
        {
            panic!("failed to download yt-dlp")
        }
        if !std::process::Command::new("chmod")
            .arg("a+rx")
            .arg("/usr/local/bin/yt-dlp")
            .status()
            .expect("failed to run chmod")
            .success()
        {
            panic!("failed to chmod yt-dlp")
        }
        if !std::process::Command::new("yt-dlp")
            .arg("--update-to")
            .arg("nightly")
            .status()
            .expect("failed to run yt-dlp")
            .success()
        {
            panic!("failed to run yt-dlp")
        }
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("cmake")
            .arg("libopus-dev")
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
