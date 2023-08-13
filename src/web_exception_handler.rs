use thirtyfour::error::*;
use thirtyfour::WebDriver;


pub async fn alert_handler(driver: &WebDriver) -> WebDriverResult<()> {
    let close_alert = driver.accept_alert().await;

    match close_alert{
        Ok(_) => Ok(()),
        Err(WebDriverError::NoSuchAlert(_)) => Ok(()),
        _ => Err(WebDriverError::CustomError("Unexpected".to_owned())),
    }
}