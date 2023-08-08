use thirtyfour::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    
    one_cycle().await?;

    Ok(())
}

async fn one_cycle() -> WebDriverResult<()> {
    let driver: WebDriver = start_driver().await?;
    open_wise(&driver).await?;
    terminate_driver(driver).await?;

    Ok(())
}

async fn start_driver() -> WebDriverResult<WebDriver> {
    let binary_path = "./chromedriver";
    let port = 9515;
    let mut caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&format!("http://localhost:{}", port), caps).await?;

    Ok(driver)
}

async fn open_wise(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://sugang.uos.ac.kr/uosdoc/login_sugang.jsp").await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok(())
}

async fn terminate_driver(driver: WebDriver) -> WebDriverResult<()> {
    driver.quit().await?;
    Ok(())
}