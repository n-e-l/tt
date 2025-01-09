mod commands;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("tt")
        .about("Time track")
        .subcommand_required(true)
        .subcommand(
            Command::new("log")
                .about("Register working on something")
                .arg(arg!(project: [PROJECT])
                    .required(true)
                )
                .arg(arg!(time: [TIME]).long("time")
                    .require_equals(true)
                    .default_missing_value(None)
                )
        )
        .subcommand(
            Command::new("write")
                .about("Write down what's happening")
        )
        .subcommand(
            Command::new("show")
                .about("Print status")
                .arg(arg!(month: [MONTH]).long("month")
                    .required(false)
                    .default_missing_value(None))
        )
        .subcommand(
            Command::new("total")
                .about("Show the total counts for this year")
                .arg(arg!(month: [MONTH]).long("month")
                    .required(false)
                    .default_missing_value(None))
                .arg(arg!(year: [YEAR]).long("year")
                    .required(false)
                    .default_missing_value(None))
        )
}

fn main() -> std::io::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("log", sub_matches)) => {

            if let Some(project) = sub_matches.get_one::<String>("project").map(|s| s.to_string()) {

                let time = sub_matches.get_one::<String>("time");
                commands::log(project, time);
                Ok(())

            } else {
                println!("Please provide a project");
                return Ok(());
            }
        },
        Some(("write", _)) => {
            commands::write();
            Ok(())
        },
        Some(("show", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let year = sub_matches.get_one::<String>("year");
            commands::show(month, year);
            Ok(())
        },
        Some(("total", sub_matches)) => {
            let month = sub_matches.get_one::<String>("month");
            let year = sub_matches.get_one::<String>("year");
            commands::total(month, year);
            Ok(())
        },
        _ => unreachable!()
    }
}
