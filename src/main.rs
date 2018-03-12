extern crate cryptoaudit_config as config;
extern crate cryptoaudit_core as core;

extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use config::GlobalConfig;

fn main() {
    // First create a global config object
    let global_config = GlobalConfig::new();

    let args = App::new("Cryptoaudit")
		.version("0.1")
		.author("The Catallaxy Team")
		.about("Catallaxy cryptocurrency auditing suite.")

		// specification of all the server commands and options
	    .subcommand(SubCommand::with_name("sign")
	                .about("Sign a message with multiple keys with multiple blockchains")
	                .arg(Arg::with_name("file")
	                     .short("f")
	                     .long("file")
	                     .help("The JSON file containing the private key")
	                     .takes_value(true))
	                .arg(Arg::with_name("message")
	                     .short("m")
	                     .long("message")
	                     .help("The message to sign")
	                	.takes_value(true)))

	    // specification of all the client commands and options
	    .subcommand(SubCommand::with_name("verify")
	                .about("Verify a signature JSON file generated by the sign command")
					.arg(Arg::with_name("file")
	                     .short("f")
	                     .long("file")
	                     .help("The JSON file containing the private key")
	                     .takes_value(true))
	                .arg(Arg::with_name("message")
	                     .short("m")
	                     .long("message")
	                     .help("The message to sign")
	                	.takes_value(true)))


		.get_matches();

    match args.subcommand() {
        // server commands and options
        ("sign", Some(sign_args)) => {
            sign_command(sign_args, global_config);
        }

        // client commands and options
        ("verify", Some(verify_args)) => {
            verify_command(verify_args, global_config);
        }

        _ => {
            // won't attempt to just start with defaults, and will reject
            println!("Unknown command.");
            println!("Use 'grin help' for a list of all commands.");
        }
    }
}

/// Handles the server part of the command line, mostly running, starting and
/// stopping the Grin blockchain server. Processes all the command line
/// arguments
/// to build a proper configuration and runs Grin with that configuration.
fn sign_command(sign_args: &ArgMatches, global_config: GlobalConfig) {
	let mut sign_config = global_config.members.unwrap().sign_config;

	if let Some(file) = sign_args.value_of("file") {
		sign_config.file = file.parse().unwrap();
	}

	if let Some(message) = sign_args.value_of("message") {
		sign_config.message = message.parse().unwrap();
	}

    core::example::example();
}

/// Handles the server part of the command line, mostly running, starting and
/// stopping the Grin blockchain server. Processes all the command line
/// arguments
/// to build a proper configuration and runs Grin with that configuration.
fn verify_command(verify_args: &ArgMatches, global_config: GlobalConfig) {
	let mut verify_config = global_config.members.unwrap().verify_config;

	if let Some(file) = verify_args.value_of("file") {
		verify_config.file = file.parse().unwrap();
	}

	if let Some(message) = verify_args.value_of("message") {
		verify_config.message = message.parse().unwrap();
	}

}
