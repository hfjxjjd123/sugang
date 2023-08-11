use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use crate::image_manager::image_screenshot::screenshot_canvas;
use crate::image_manager::image_analyzer::*;
use crate::image_manager::pass_iframe::*;

pub async fn initial_apply_canvas(driver: &WebDriver, target_index: Vec<i32>) -> WebDriverResult<(i64, i64)>{
    let mut coordinates: Vec<(i64, i64)> = vec![];

    let iframe1_location = pass_iframe1(&driver).await?;
    driver.enter_frame(0).await?;
    let iframe2_location = pass_iframe2(&driver).await?;
    driver.enter_frame(0).await?;

    let apply_canvas = screenshot_canvas(&driver, "TmtViewer", "screenshot2.png").await?;

    for index in target_index{
        let apply_button = apply_button_location(index);
        let final_x:i64 = (apply_canvas.x + apply_button.0 as f64 + iframe1_location.x + iframe2_location.x) as i64;
        let final_y:i64 = (apply_canvas.y + apply_button.1 as f64 + iframe1_location.y + iframe2_location.y) as i64;

        coordinates.push((final_x, final_y));
    }

    let query_button = query_button_location();
    let query_button_x = (apply_canvas.x + query_button.0 as f64) as i64;
    let query_button_y = (apply_canvas.y + query_button.1 as f64) as i64;
    coordinates.push((query_button_x, query_button_y));

    Ok(coordinates)

}

