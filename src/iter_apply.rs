use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use crate::image_manager::image_screenshot::screenshot_canvas;

pub async fn initial_apply_canvas(driver: &WebDriver) -> WebDriverResult<()>{
    
    driver.enter_frame(0).await?;
    driver.enter_frame(0).await?;
    
    screenshot_canvas(&driver, "TmtViewer", "screenshot2.png").await?;

    Ok(())

}