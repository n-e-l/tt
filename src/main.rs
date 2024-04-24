mod commands;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("tt")
        .about("Time track")
        .subcommand_required(true)
        .subcommand(
            Command::new("log")
                .about("Register working on something")
                .arg(arg!(project: [PROJECT]))
        )
        .subcommand(
            Command::new("show")
                .about("Print status")
        )
}

fn main() -> std::io::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("log", sub_matches)) => {
            let project = sub_matches.get_one::<String>("project").map(|s| s.to_string()).expect("Couldn't read project id");
            commands::log(project);
            Ok(())
        },
        Some(("show", _sub_matches)) => {
            commands::show();
            Ok(())
        },
        _ => unreachable!()
    }
}
