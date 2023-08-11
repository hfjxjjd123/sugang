use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use crate::image_manager::image_screenshot::screenshot_canvas;
use crate::image_manager::image_analyzer::*;
use crate::image_manager::pass_iframe::*;

pub async fn initial_apply_canvas(driver: &WebDriver) -> WebDriverResult<()>{
    
    let iframe1_location = pass_iframe1(&driver).await?;
    driver.enter_frame(0).await?;
    let iframe2_location = pass_iframe2(&driver).await?;
    driver.enter_frame(0).await?;

    let apply_canvas = screenshot_canvas(&driver, "TmtViewer", "screenshot2.png").await?;

    // let query_button = query_button_location();
    let apply_button2 = apply_button_location(2);


    let final_x:i64 = (apply_canvas.x + apply_button2.0 as f64 + iframe1_location.x + iframe2_location.x) as i64;
    let final_y:i64 = (apply_canvas.y + apply_button2.1 as f64 + iframe1_location.y + iframe2_location.y) as i64;

    println!("{},{}", final_x, final_y);


    driver
    .action_chain()
    .move_to(final_x, final_y)
    .click()
    .perform()
    .await?;

    Ok(())

}