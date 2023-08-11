use thirtyfour::prelude::*;
use tokio::time::Duration;
use sugang::chrome_manager::*;
use sugang::redirec_sugang_canvas::*;
use sugang::image_manager::*;
use crate::download_screenshot::screenshot_canvas;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    one_cycle().await?;

    Ok(())
}

// 659 16

async fn one_cycle() -> WebDriverResult<()> {
    let driver: WebDriver = start_driver().await?;
    open_wise(&driver).await?;
    auto_login(&driver).await?;
    click_to_open_tab(&driver).await?;

    match await_canvas(&driver).await{
        Ok(_) => println!("no problem"),
        Err(WebDriverError::CmdError(_)) => println!("No problem"),
        _ => println!("ERR occur"),
    }
    screenshot_canvas(&driver).await?;

    tokio::time::sleep(Duration::from_secs(1000)).await;

    terminate_driver(driver).await?;
    Ok(())
}

fn debugging_click(driver: &WebDriver){

}