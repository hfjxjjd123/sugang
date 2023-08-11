use thirtyfour::prelude::*;
use tokio::time::Duration;
use sugang::chrome_manager::*;
use sugang::init_page::initialize;
use sugang::iter_apply::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver: WebDriver = start_driver().await?;
    //while initialize 값이 에러가 아닐 동안 계속 트라이
    initialize(&driver).await?;
    initial_apply_canvas(&driver).await?;

    tokio::time::sleep(Duration::from_secs(10)).await;
    terminate_driver(driver).await?;
    Ok(())
}