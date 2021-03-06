use coreutils_core::os::group::Groups;

mod cli;

fn main() {
    let matches = cli::create_app().get_matches();

    let id = matches.is_present("id");

    let groups = if let Some(name) = matches.value_of("USER") {
        match Groups::from_username(name) {
            Ok(g) => g,
            Err(err) => {
                eprintln!("groups: {}", err);
                std::process::exit(1);
            },
        }
    } else {
        match Groups::caller() {
            Ok(g) => g,
            Err(err) => {
                eprintln!("groups: {}", err);
                std::process::exit(1);
            },
        }
    };

    if !groups.is_empty() {
        if id {
            groups.iter().for_each(|g| print!("{}:{} ", g.name(), g.id()));
        } else {
            groups.iter().for_each(|g| print!("{} ", g.name()));
        }
    }
    println!();
}
