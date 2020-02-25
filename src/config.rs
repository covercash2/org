use clap::{App, Arg};

const PATH: &'static str = "/mnt/space/notes/emacs.org";

const STATUS_LABELS: [&'static str; 3] = ["TODO", "STARTED", "DONE"];

pub struct Config {
    pub file_path: String,
    pub status_labels: Vec<String>,
}

impl Config {
    pub fn from_command_line_parameters() -> Result<Config, &'static str> {
        let matches = App::new("org")
            .version("0.1.0")
            .author("Chris Overcash <covercash2@gmail.com>")
            .about("command line parser for org formatted files")
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("path to org file to parse"),
            )
            .arg(
                Arg::with_name("status_labels")
                    .short("l")
                    .long("labels")
                    .takes_value(true)
                    .help("comma (',') separated values, e.g. TODO,STARTED,DONE"),
            )
            .get_matches();

        let file = matches.value_of("file").unwrap_or_else(|| {
            eprintln!(
                "file input will be required, but a default is currently provided for testing"
            );
            PATH
        });

        let labels: Vec<String> = matches
            .value_of("status_labels")
            .map(|labels_string| labels_string.split(",").collect())
            .unwrap_or(STATUS_LABELS.to_vec())
            .iter()
            .map(|s| s.to_string())
            .collect();

        Ok(Config {
            file_path: file.to_string(),
            status_labels: labels,
        })
    }
}
