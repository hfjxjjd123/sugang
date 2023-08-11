use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;


pub async fn alert_handler(driver: &WebDriver) -> WebDriverResult<()> {
    driver.accept_alert().await?;
    Ok(())
}