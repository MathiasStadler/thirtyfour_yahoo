use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
// use std::io::prelude::*;

// use csv::Writer;
// FROM HERE 
// https://dev.to/stevepryde/using-selenium-with-rust-aca
use thirtyfour::prelude::*;
use tokio;

// use thirtyfour::{
//    // prelude::{ElementWaitable, WebDriverError},
//    prelude::WebDriverError,
//    By,
//    DesiredCapabilities,
//    ChromeCapabilities,
//    WebDriver,
//    WebElement,
// };

use url::Url;

//path of webpage
#[allow(dead_code)]
const WEB_XPATH:&[&[&str]] = &[
];

/// .
///
/// # Errors
///
/// This function will return an error if .
fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    color_eyre::install()?;

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}

pub async fn run() -> color_eyre::Result<(),Box<dyn Error>> {
    
    // old
    // let _place: &str ="Place";
    let _driver = initialize_driver().await?;
    let url = Url::parse("https://finance.yahoo.com")?;

    _driver.goto(url).await?;
    thread::sleep(Duration::from_secs(2));

    // old
    // search_location(&_driver, _place).await?;
    // thread::sleep(Duration::from_secs(2));

    // scrape_all(_driver.clone()).await?;
    screenshot_browser(_driver.clone()).await?;
    // save_result_table(_driver.clone()).await?;
    // close_browser(_driver.clone()).await?;
    

    Ok(())
}

async fn screenshot_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    //screenshot of browser windows
    // FROM HERE
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let screenshot = _driver.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html
    let mut file = File::create("screenshot.png")?;
    file.write_all(&screenshot)?;

    // println!("Screenshot of browser windows => {:?} ",screenshot);
    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    // let caps = DesiredCapabilities::chrome();
    
    let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    caps.add_arg("--headless")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}
