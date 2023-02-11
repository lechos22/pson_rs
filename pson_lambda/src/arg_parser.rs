use crate::config::ArgConfig;

enum NextArg {
    File,
}

/**
 * Parse the command line arguments into a config struct.
 * Available arguments:
 * -h, --help: Display help message
 * -v, --version: Display version
 * -f, --file: Specify the file to run
 * -i, --interactive: Start in interactive (REPL) mode (default if no file is specified)
 * -t, --test: Run tests
 */
pub fn parse_args(args: &mut impl Iterator<Item = String>) -> Result<ArgConfig, String> {
    // parse the arguments purely in the functional style
    args
        .try_fold((ArgConfig::new(), Option::<NextArg>::None), |(mut config, next_arg), arg| -> Result<(ArgConfig, Option<NextArg>), String> {
            match next_arg {
                Some(NextArg::File) => {
                    config.file = Some(arg.to_string());
                    Ok((config, None))
                }
                _ => {
                    match arg.as_str() {
                        "-h" | "--help" => {
                            config.help = true;
                            Ok((config, None))
                        }
                        "-v" | "--version" => {
                            config.version = true;
                            Ok((config, None))
                        }
                        "-f" | "--file" => {
                            Ok((config, Some(NextArg::File)))
                        }
                        "-i" | "--interactive" => {
                            config.interactive = true;
                            Ok((config, None))
                        }
                        "-t" | "--test" => {
                            config.test = true;
                            Ok((config, None))
                        }
                        _ => {
                            if arg.starts_with("-") {
                                Err(format!("Unknown argument: {}", arg))
                            }
                            else if config.file.is_some() {
                                Err(format!("Only one file can be specified."))
                            }
                            else {
                                config.file = Some(arg.to_string());
                                Ok((config, None))
                            }
                        }
                    }
                }
            }
        })
        .map(|(config, _)| config)
}
