use thirtyfour::prelude::*;
use tokio::time::Duration;
use sugang::chrome_manager::*;
use sugang::init_page::*;
use sugang::iter_apply::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let num_of_basket: Vec<i32> = vec![1,2];
    let driver: WebDriver = start_driver().await?;
    
    initialize(&driver).await?;
    let elements = analyze_elements_location(&driver, num_of_basket).await?;
    iteration(&driver, &elements).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    terminate_driver(driver).await?;

    let num_of_basket: Vec<i32> = vec![2];
    let driver: WebDriver = start_driver().await?;
    
    initialize(&driver).await?;
    let elements = analyze_elements_location(&driver, num_of_basket).await?;
    iteration(&driver, &elements).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    terminate_driver(driver).await?;
    Ok(())
    
}


