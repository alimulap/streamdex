use context::Context;

mod cli;
mod config;
mod context;
mod handler;
mod room;
mod runner;
mod utils;

fn main() {
    let cli = cli::parse();
    let config = config::get();
    let mut context = Context::new(config);

    match cli.subcommand() {
        Some(("live", sub_m)) => {
            context.handle("live", sub_m);
            handler::live(&context);
        }
        Some(("video", sub_m)) => {
            context.handle("video", sub_m);
            handler::video(&context);
        }
        Some(("playlist", _sub_m)) => {
            eprintln!("Not implemented yet");
        }
        Some(("allocate", sub_m)) => {
            eprintln!("Can only allocate 1 room, i thought i can use 1 port multiple times lmao");
            eprintln!("Basically this is useless for the time being");
            context.handle("allocate", sub_m);
            room::allocate(&context);
        }
        Some(("print-formats", sub_m)) => {
            context.handle("print-formats", sub_m);
            handler::print_formats(&context);
        }
        _ => println!("No command given"),
    }
}
