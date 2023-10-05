use clap::Parser;
use cli::Cli;
use std::io::Write;
use thirtyfour::prelude::*;

mod cli;
mod scrapper;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let cli = Cli::parse();

    let email = cli.email.unwrap_or_else(|| {
        let mut input = String::new();
        print!("Email: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read email from stdin");

        input
    });

    let password = std::env::var("DRIVEN_PWD").unwrap_or_else(|_| {
        rpassword::prompt_password("Password: ").expect("Failed to read password from stdin")
    });

    let driver = scrapper::setup_driver(cli.headless, cli.port, cli.wait).await?;
    scrapper::login(&driver, email, password).await?;
    scrapper::loop_over_modules(&driver).await?;

    driver.quit().await?;

    Ok(())
}
