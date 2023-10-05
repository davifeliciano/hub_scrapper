use copypasta::{ClipboardContext, ClipboardProvider};
use std::{path::Path, time::Duration};
use thirtyfour::prelude::*;

pub async fn setup_driver(headless: bool, port: u16, wait: u64) -> WebDriverResult<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();

    if headless {
        caps.set_headless()?;
    }

    let driver_addr = format!("http://localhost:{}", port);
    let driver = WebDriver::new(&driver_addr, caps).await.expect(&format!(
        "Error connecting to chromedriver. Is the driver really running on port {}?",
        port
    ));

    driver
        .set_implicit_wait_timeout(Duration::from_secs(wait))
        .await?;

    Ok(driver)
}

pub async fn login(driver: &WebDriver, email: String, password: String) -> WebDriverResult<()> {
    driver
        .goto("https://hub.driven.com.br/login?redirect=%2Fcomputacao")
        .await?;

    let email_input = driver.find(By::Css("input[name='email']")).await?;
    let password_input = driver.find(By::Css("input[name='password']")).await?;

    email_input.send_keys(email).await?;
    password_input.send_keys(password + Key::Enter).await?;

    // Implicitly wait for page to load
    driver
        .find(By::Tag("h3"))
        .await
        .expect("Timeout waiting login redirection. Are your credentials right?");

    Ok(())
}

pub async fn loop_over_modules(driver: &WebDriver) -> WebDriverResult<()> {
    let mut ctx = ClipboardContext::new().unwrap();

    // TODO: Extract module_ids from page
    for (index, module_id) in (58..=63).enumerate() {
        let module_url = format!("https://hub.driven.com.br/computacao/modulo/{}", module_id);
        driver.goto(module_url).await?;

        // Implicitly wait for section progress bar to load, when h3 text will be set
        driver.find(By::Css("#single-spa-application\\:\\@hub\\/algorithms > div > div > div > div > section > div > svg")).await?;

        let module_h3 = driver.find(By::Css("h3")).await?;
        module_h3.wait_until().displayed().await?;
        let module_title = module_h3.text().await?;
        let module_title = format!("{:02} - {}", index + 1, module_title);
        let module_path = Path::new(&module_title);

        std::fs::create_dir(&module_path)
            .expect("Error creating output dir. Maybe there is a conflicting folder.");

        loop_over_sections(&driver, &mut ctx, &module_path).await?;
    }

    Ok(())
}

async fn get_exercise_sections_urls(driver: &WebDriver) -> WebDriverResult<Vec<String>> {
    let anchors = driver
        .find_all(By::Css(
            "main:last-child > section > section > div:last-child a",
        ))
        .await?;

    let mut exercises_urls: Vec<String> = vec![];

    for anchor in anchors {
        let href = anchor.attr("href").await.unwrap().unwrap();
        exercises_urls.push(format!("https://hub.driven.com.br{}", href));
    }

    Ok(exercises_urls)
}

async fn loop_over_sections(
    driver: &WebDriver,
    ctx: &mut ClipboardContext,
    module_path: &Path,
) -> WebDriverResult<()> {
    let exercises_urls = get_exercise_sections_urls(&driver).await?;

    for url in exercises_urls {
        driver
            .in_new_tab(|| async {
                driver.goto(url).await?;

                let section_h4 = driver.find(By::Css("h4")).await?;
                section_h4.wait_until().displayed().await?;
                let section_title = section_h4.text().await?;

                println!("Accessing section `{}`", section_title);

                loop_over_exercises(driver, ctx, module_path).await?;

                Ok(())
            })
            .await?;
    }

    Ok(())
}

async fn loop_over_exercises(
    driver: &WebDriver,
    ctx: &mut ClipboardContext,
    module_path: &Path,
) -> WebDriverResult<()> {
    let exercises_count = driver
        .find_all(By::Css(
            "#single-spa-application\\:\\@hub\\/algorithms nav > * > div",
        ))
        .await?
        .len();

    for exercise_index in 0..exercises_count {
        let nav_selector = format!(
            "#single-spa-application\\:\\@hub\\/algorithms nav > :nth-child({}) > div",
            2 * exercise_index + 1
        );
        driver.find(By::Css(&nav_selector)).await?.click().await?;

        let exercise_h3 = driver.find(By::Css("h3")).await?;
        exercise_h3.wait_until().displayed().await?;
        let exercise_title = exercise_h3.text().await?;

        println!(
            "Extracting exercise {} of {}: `{}`",
            exercise_index + 1,
            exercises_count,
            exercise_title
        );

        let exercise_title = exercise_title.replace("/", "|");
        let exercise_path = module_path.join(&exercise_title);

        std::fs::create_dir(&exercise_path).unwrap();

        let statement_html = driver
            .find(By::Css("div:has(> h3)"))
            .await?
            .inner_html()
            .await?;

        let statement_md = html2md::parse_html(&statement_html);

        std::fs::write(
            &exercise_path.join(format!("{}.md", &exercise_title)),
            statement_md,
        )
        .unwrap();

        let editor = driver.find(By::ClassName("ace_text-input")).await?;
        editor.send_keys(Key::Control + "a").await?;
        editor.send_keys(Key::Control + "c").await?;

        let code = ctx.get_contents().unwrap();
        std::fs::write(&exercise_path.join(format!("{}.js", &exercise_title)), code).unwrap();
    }

    Ok(())
}
