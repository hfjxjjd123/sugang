use thirtyfour::prelude::*;
use tokio::time::Duration;
use sugang::chrome_manager::*;
use sugang::init_page::initialize;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver: WebDriver = start_driver().await?;
    //while initialize 값이 에러가 아닐 동안 계속 트라이

    initialize(&driver).await?;
    one_cycle().await?;
    terminate_driver(driver).await?;
    Ok(())
}

async fn one_cycle() -> WebDriverResult<()> {
    
    tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}