use std::path::PathBuf;

use anyhow::{anyhow, Context, Error, Result};

mod config;
mod murdock;

use clap::{crate_version, Arg, ArgAction, ArgGroup, ArgMatches, Command};

use config::{Config, Server};

fn clap() -> clap::Command {
    Command::new("murdock-cli")
        .version(crate_version!())
        .author("Kaspar Schleiser <kaspar@schleiser.de>")
        .about("Murdock API CLI client")
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .env("MURDOCK_CLI_CONFIG")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .subcommand(
            Command::new("servers")
                .about("manage Murdock servers")
                .subcommand_required(true)
                .infer_subcommands(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .arg(
                            Arg::new("name")
                                .action(ArgAction::Set)
                                .value_name("NAME")
                                .required(true)
                                .help("some name to reference this server"),
                        )
                        .arg(
                            Arg::new("url")
                                .action(ArgAction::Set)
                                .value_name("URL")
                                .required(true)
                                .help("URL where this server is reachable"),
                        )
                        .arg(
                            Arg::new("token")
                                .short('t')
                                .long("token")
                                .action(ArgAction::Set)
                                .value_name("TOKEN")
                                .help("optional github token"),
                        )
                        .arg(
                            Arg::new("default")
                                .help("set as default server")
                                .action(ArgAction::SetTrue)
                                .short('d')
                                .long("default"),
                        ),
                )
                .subcommand(
                    Command::new("edit")
                        .arg(
                            Arg::new("name")
                                .action(ArgAction::Set)
                                .value_name("NAME")
                                .required(true)
                                .help("server name"),
                        )
                        .arg(
                            Arg::new("default")
                                .help("set as default server")
                                .action(ArgAction::SetTrue)
                                .short('d')
                                .long("default"),
                        )
                        .arg(
                            Arg::new("url")
                                .short('u')
                                .long("url")
                                .action(ArgAction::Set)
                                .value_name("URL")
                                .help("URL where this server is reachable"),
                        )
                        .arg(
                            Arg::new("token")
                                .short('t')
                                .long("token")
                                .action(ArgAction::Set)
                                .value_name("TOKEN")
                                .help("optional github token"),
                        ),
                )
                .subcommand(
                    Command::new("delete").arg_required_else_help(true).arg(
                        Arg::new("name")
                            .action(ArgAction::Set)
                            .value_name("NAME")
                            .required(true)
                            .help("delete server config"),
                    ),
                )
                .subcommand(
                    Command::new("list").arg(
                        Arg::new("show_tokens")
                            .action(ArgAction::SetTrue)
                            .short('s')
                            .long("show-tokens")
                            .help("don't hide tokens"),
                    ),
                ),
        )
        .subcommand(
            Command::new("jobs")
                .about("manage multiple jobs")
                .arg_required_else_help(true)
                .infer_subcommands(true)
                .subcommand(
                    Command::new("delete")
                        .arg(
                            Arg::new("server")
                                .short('s')
                                .long("server")
                                .action(ArgAction::Set)
                                .value_name("SERVER")
                                .env("MURDOCK_CLI_SERVER")
                                .help("server name (default: use configured one)"),
                        )
                        .arg(
                            Arg::new("before")
                                .short('b')
                                .long("before")
                                .action(ArgAction::Set)
                                .value_name("DATE")
                                .help("delete jobs that where created before DATE"),
                        )
                        .arg(
                            Arg::new("age")
                                .short('a')
                                .long("age")
                                .action(ArgAction::Set)
                                .value_name("DAYS")
                                .help("delete jobs that where created more than AGE days ago"),
                        )
                        .group(
                            ArgGroup::new("deletion date")
                                .args(&["age", "before"])
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("job")
                .about("manage a job")
                .arg_required_else_help(true)
                .infer_subcommands(true)
                .subcommand(
                    Command::new("abort")
                        .arg(
                            Arg::new("server")
                                .short('s')
                                .long("server")
                                .action(ArgAction::Set)
                                .value_name("SERVER")
                                .env("MURDOCK_CLI_SERVER")
                                .help("server name (default: use configured one)"),
                        )
                        .arg(
                            Arg::new("uuid")
                                .action(ArgAction::Set)
                                .value_name("UUID")
                                .help("job uuid")
                                .required(true),
                        ),
                ),
        )
}

fn get_server<'a>(config: &'a Config, matches: &ArgMatches) -> Result<(String, &'a Server), Error> {
    let server: Option<&String> = matches.get_one("server");
    if let Some(server_name) = server {
        if let Some(server) = config.servers.get(server_name) {
            return Ok((server_name.clone(), server));
        }
    }
    config.get_default_server().map_or_else(
        || Err(anyhow!("no server specified, no default server configured")),
        |x| Ok(x),
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = clap().get_matches();

    // handle config file
    let app_dirs = platform_dirs::AppDirs::new(Some("murdock-cli"), true).unwrap();
    let default_config_file = app_dirs.config_dir.join("murdock-cli.yaml");

    let config_file: Option<&String> = matches.get_one("config");
    let config_file = config_file.map_or_else(|| default_config_file, PathBuf::from);

    let mut config = config::Config::from_file(&config_file)?;

    async fn doit(
        matches: ArgMatches,
        mut config: Config,
        config_file: PathBuf,
    ) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("servers", matches)) => match matches.subcommand() {
                Some(("add", matches)) => {
                    let name: &String = matches.get_one("name").unwrap();
                    let url: &String = matches.get_one("url").unwrap();
                    let token: Option<&String> = matches.get_one("token");
                    let set_default = *matches.get_one::<bool>("default").unwrap();

                    if config.servers.get(name).is_some() {
                        return Err(anyhow!("server config already exists, maybe use \"edit\"?"));
                    }

                    let server = config::ServerBuilder::default()
                        .url(url.clone())
                        .token(token.map(|x| x.clone()))
                        .build()?;

                    println!("murdock-cli: added server \"{name}\"");

                    if set_default {
                        println!("murdock-cli: set server \"{name}\" as default");
                        config.default_server = Some(name.clone());
                    }

                    config.servers.insert(name.clone(), server);

                    config.to_file(&config_file)?;
                }
                Some(("edit", matches)) => {
                    let name: &String = matches.get_one("name").unwrap();
                    let url: Option<&String> = matches.get_one("url");
                    let token: Option<&String> = matches.get_one("token");
                    let set_default = *matches.get_one::<bool>("default").unwrap();

                    let mut server = match config.servers.get(name) {
                        Some(server) => server.clone(),
                        None => return Err(anyhow!("server config not found, maybe add first?")),
                    };

                    let was_default = config
                        .default_server
                        .as_ref()
                        .map_or(false, |x| &x == &name);

                    if let Some(url) = url {
                        server.url = url.clone();
                    }
                    if let Some(token) = token {
                        server.token = Some(token.clone());
                    }

                    let mut have_changes = false;
                    if let Some(previous) = config.servers.insert(name.clone(), server.clone()) {
                        if previous != server {
                            println!("murdock-cli: updated server \"{name}\"");
                            have_changes = true;
                        }
                    }

                    if set_default && !was_default {
                        println!("murdock-cli: set server \"{name}\" as default");
                        config.default_server = Some(name.clone());
                        have_changes = true;
                    }

                    if have_changes {
                        config.to_file(&config_file)?;
                    }
                }
                Some(("delete", matches)) => {
                    let name: &String = matches.get_one("name").unwrap();
                    if let Some(_) = config.servers.remove(name) {
                        println!("murdock-cli: removed server \"{name}\"");
                        if config
                            .default_server
                            .as_ref()
                            .map_or(false, |x| &x == &name)
                        {
                            config.default_server = None;
                        }

                        config.to_file(&config_file)?;
                    }
                }
                Some(("list", matches)) => {
                    let show_tokens = *matches.get_one::<bool>("show_tokens").unwrap();

                    for (name, server) in config.servers {
                        let is_default =
                            config.default_server.as_ref().map_or(false, |x| x == &name);

                        let default_marker = if is_default { "*" } else { " " };
                        let token = server.token;
                        let token_str = if show_tokens {
                            if let Some(token) = &token {
                                &token
                            } else {
                                ""
                            }
                        } else {
                            "<has token>"
                        };

                        println!("{}{:16} {} {}", default_marker, name, server.url, token_str);
                    }
                }
                _ => {}
            },
            Some(("jobs", matches)) => match matches.subcommand() {
                Some(("delete", matches)) => {
                    use chrono::{Duration, Local, NaiveDate};
                    let before: Option<&String> = matches.get_one("before");
                    let age: Option<&String> = matches.get_one("age");

                    let before_date = if let Some(date) = before {
                        NaiveDate::parse_from_str(date, "%Y-%m-%d")
                            .context("parsing date (expecting %Y-%m-%d)")?
                    } else {
                        let age = i64::from_str_radix(age.unwrap(), 10)
                            .context("converting age (expecting number of days)")?;
                        Local::now()
                            .date_naive()
                            .checked_sub_signed(Duration::days(age))
                            .unwrap()
                    };

                    let (_server_name, server) = get_server(&config, &matches)?;
                    let token = server.token.clone();
                    let murdock = if let Some(token) = token {
                        murdock::Murdock::new(&server.url, Some(token.as_str()))
                    } else {
                        murdock::Murdock::new(&server.url, None)
                    };

                    let res = murdock.jobs_delete(before_date).await?;
                    println!("murdock-cli: deleted {} jobs", res.len());
                }
                _ => {}
            },
            Some(("job", matches)) => match matches.subcommand() {
                Some(("abort", matches)) => {
                    let uuid: Option<&String> = matches.get_one("uuid");
                    let uuid = uuid.expect("uuid required by clap");

                    let (_server_name, server) = get_server(&config, &matches)?;
                    let token = server.token.clone();
                    let murdock = if let Some(token) = token {
                        murdock::Murdock::new(&server.url, Some(token.as_str()))
                    } else {
                        murdock::Murdock::new(&server.url, None)
                    };

                    murdock.job_abort(uuid).await?;
                    println!("murdock-cli: cancelled job");
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    if let Err(e) = doit(matches, config, config_file).await {
        eprintln!("{e}");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_clap() {
        crate::clap().debug_assert();
    }
}
