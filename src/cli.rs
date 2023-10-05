pub use clap::{Args, Parser};

const LONG_ABOUT: &str = "Web Crawler to export all the exercise's solutions from \
https://hub.driven.com.br . It will convert the exercise's statements into markdown, \
and extract the code in each editor via the OS clipboard. Hence its important to \
not copy anything to the clipboard during execution to avoid any interference. \
Directories for each module will be created in the current working dir and within those \
a dir per exercise will be created as well, containing a .js and a .md file.";

const EMAIL_HELP: &str = "Email to use in authentication. \
Password will be read from DRIVEN_PWD env variable or prompt at runtime.";

const HEADLESS_HELP: &str = "If set, the browser will run in headless mode.";

const WAIT_HELP: &str = "The max wait time in seconds in searches for web elements. \
Greater values are requires in slower connections. Default is probably fine.";

const PORT_HELP: &str = "The port in which chromedriver is listening on. \
Download chromedriver binary for your platform at https://chromedriver.chromium.org/downloads .";

#[derive(Parser)]
#[command(version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[arg(long, help = EMAIL_HELP)]
    pub email: Option<String>,

    #[arg(long, help = HEADLESS_HELP)]
    pub headless: bool,

    #[arg(long, default_value_t = 5, help = WAIT_HELP)]
    pub wait: u64,

    #[arg(long, default_value_t = 9515, help = PORT_HELP)]
    pub port: u16,
}
