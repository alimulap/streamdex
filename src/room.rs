use std::process::Command;

use crate::context::Context;

pub fn allocate(ctx: &Context) {
    let id = ctx.get("id").unwrap().as_string().unwrap();
    let delay = ctx.get("delay").unwrap().as_u32().unwrap();
    let mut process = Command::new("vlc")
        .arg(format!("http://localhost:8080/streamdex-{}.mp4", id))
        .arg("--loop")
        .arg(format!("vlc://pause:{}", delay))
        .spawn()
        .unwrap();
    process.wait().unwrap();
}
