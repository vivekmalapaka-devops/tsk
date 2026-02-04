mod cli;
mod commands;
mod display;
mod store;
mod time;
mod todo;

use clap::Parser;
use cli::{Cli, Command};
use commands::list::Filter;
use display::DisplayConfig;
use store::Store;

fn main() {
    let cli = Cli::parse();

    let config = DisplayConfig::new(cli.get_color_mode());

    let mut store = match Store::load() {
        Ok(s) => s,
        Err(e) => {
            display::print_error(&format!("Could not load store: {}", e));
            std::process::exit(1);
        }
    };

    match cli.command {
        Some(Command::Add { text, p, t }) => {
            commands::add::run(text, p, t, &mut store, &config);
        }

        Some(Command::Ls) | None => {
            commands::list::run(&store, &config, cli.sort_by, Filter::Open, &cli.tags);
        }

        Some(Command::All) => {
            commands::list::run(&store, &config, cli.sort_by, Filter::All, &cli.tags);
        }

        Some(Command::Today) => {
            commands::today::run(&store, &config);
        }

        Some(Command::Week) => {
            commands::list::run(&store, &config, cli.sort_by, Filter::Week, &cli.tags);
        }

        Some(Command::Overdue) => {
            commands::list::run(&store, &config, cli.sort_by, Filter::Overdue, &cli.tags);
        }

        Some(Command::Done { ids }) => {
            commands::done::run(ids, &mut store, &config);
        }

        Some(Command::Delete { ids }) => {
            commands::delete::run(ids, &mut store, &config);
        }

        Some(Command::Edit {
            id,
            text,
            p,
            t,
            clear_time,
            clear_priority,
        }) => {
            commands::edit::run(id, text, p, t, clear_time, clear_priority, &mut store, &config);
        }

        Some(Command::Clear) => {
            commands::clear::run(&mut store, &config);
        }

        Some(Command::Stats) => {
            commands::stats::run(&store, &config);
        }
    }
}
