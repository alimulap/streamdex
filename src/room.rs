use std::process::Command;

use crate::context::Context;

pub fn allocate(ctx: &Context) {
    let id = ctx.id.clone().unwrap();
    let delay = ctx.delay.unwrap();
    let mut process = Command::new("vlc")
        .arg(format!("http://localhost:8080/streamdex-{}.mp4", id))
        .arg("--loop")
        .arg(format!("vlc://pause:{}", delay))
        .spawn()
        .unwrap();
    process.wait().unwrap();
}
