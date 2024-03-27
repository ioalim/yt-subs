use std::process::Command;

pub fn get(ids: &Vec<String>) -> (Vec<u32>, u32) {
    let mut subs = Vec::with_capacity(ids.len());
    let mut total = 0;
    for id in ids {
        let url = format!("https://www.youtube.com/@{}", id);
        let ytdlp_output = Command::new("yt-dlp")
            .arg("--print")
            .arg("channel_follower_count")
            .arg(url)
            .arg("--max-downloads")
            .arg("1")
            .output()
            .expect("Failed to execute yt-dlp");
        let follower_count = String::from_utf8_lossy(&ytdlp_output.stdout)
            .trim()
            .parse::<u32>()
            .unwrap();
        subs.push(follower_count);
        total += follower_count;
    }
    (subs, total)
}
