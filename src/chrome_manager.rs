use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::DesiredCapabilities;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    let _binary_path = "./chromedriver";
    let port = 9515;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&format!("http://localhost:{}", port), caps).await?;

    Ok(driver)
}

pub async fn terminate_driver(driver: WebDriver) -> WebDriverResult<()> {
    driver.quit().await?;
    Ok(())
}