use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use crate::image_manager::image_screenshot::screenshot_canvas;
use crate::image_manager::image_analyzer::*;
use crate::image_manager::pass_iframe::*;
use crate::web_exception_handler::alert_handler;
use thirtyfour::error::WebDriverError;
use fantoccini::error::CmdError::Standard;
use tokio::time::Duration;


pub async fn initial_apply_canvas(driver: &WebDriver, target_index: Vec<i32>) -> WebDriverResult<Vec<(i64, i64)>>{
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

pub async fn one_cycle_apply(driver: &WebDriver, targets: &Vec<(i64,i64)>)->WebDriverResult<()>{
    for target in targets{
        let click_apply = click_apply_button(driver, target).await;

        match click_apply{
            Ok(_) => (),
            Err(WebDriverError::CmdError(Standard(wd))) => {
                if wd.error() == "unexpected alert open" {
                    alert_handler(driver).await?;
                    click_apply_button(driver, target).await?;
                } else{
                    return Err(WebDriverError::CustomError("Unexpected".to_owned()));
                }
            }
            _ => {
                return Err(WebDriverError::CustomError("Unexpected".to_owned()));
            }
        }
        alert_handler(driver).await?;
    }
    Ok(())
}

pub async fn click_apply_button(driver: &WebDriver, target: &(i64, i64))->WebDriverResult<()>{
    driver.action_chain()
                .move_to(target.0, target.1)
                .click()
                .perform()
                .await?;
    Ok(())
}

pub async fn click_reload_button(driver: &WebDriver, location: &(i64,i64))->WebDriverResult<()>{
    driver.action_chain()
                .move_to(location.0, location.1)
                .click()
                .perform()
                .await?;
    Ok(())
}

pub async fn iteration(driver: &WebDriver, targets: &Vec<(i64,i64)>)->WebDriverResult<()>{
    let reload_button = targets[0];
    let mut count = 0;
    let count_unit = targets.len() + 1;
    loop {
        if count > 50 {
            break;
        }

        one_cycle_apply(&driver, &targets).await?;
        click_reload_button(&driver, &reload_button).await?;

        tokio::time::sleep(Duration::from_secs(1)).await;
        count += count_unit;
    }

    Ok(())
}