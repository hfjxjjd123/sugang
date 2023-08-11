use std::fs::File;
use crate::image_manager::image_analyzer::*;
use thirtyfour::By;
use std::io::Write;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;
use thirtyfour::ElementRect;

pub async fn remove_first_canvas(driver: &WebDriver) -> WebDriverResult<()> {

    //pass iframe
    let iframe1_location = pass_iframe1(&driver).await?;
    driver.enter_frame(0).await?;
    let iframe2_location = pass_iframe2(&driver).await?;
    driver.enter_frame(0).await?;
    let iframe3_location = pass_iframe3(&driver).await?;
    driver.enter_frame(0).await?;

    //Error: NoSuchElement("Id(TmtViewer)") 가능
    let canvas_location = screenshot_canvas(&driver, "TmtViewer", "screenshot.png").await?;

    let (x,y) = get_button_location();
    let (absolute_x, absolute_y) = get_absolute_pixel(canvas_location, x.try_into().unwrap(), y.try_into().unwrap());

    let final_x:i64 = (absolute_x + iframe1_location.x + iframe2_location.x + iframe3_location.x) as i64;
    let final_y:i64 = (absolute_y + iframe1_location.y + iframe2_location.y + iframe3_location.y) as i64;
    println!("{},{}", final_x, final_y);

    driver
    .action_chain()
    .move_to(final_x, final_y)
    .click()
    .perform()
    .await?;

    Ok(())
}

pub async fn screenshot_canvas(driver: &WebDriver, id: &str, image_name: &str) -> WebDriverResult<ElementRect>{
    let canvas_intro = driver.query(By::Id(id)).first().await?;
    let canvas_location = canvas_intro.rect().await?;
    let screenshot_data = canvas_intro.screenshot_as_png().await?;

    let mut file = File::create(image_name)?;
    file.write_all(&screenshot_data)?;

    Ok(canvas_location)
}

pub async fn pass_iframe1(driver: &WebDriver) -> WebDriverResult<ElementRect>{
    let iframe1 = driver.query(By::Id("ContentFrame")).first().await?;
    iframe1.wait_until().displayed().await?;
    let iframe1_location = iframe1.rect().await?;

    Ok(iframe1_location)
}
pub async fn pass_iframe2(driver: &WebDriver) -> WebDriverResult<ElementRect>{
    let iframe2_selector = By::Css("iframe.window_iframe[src*='UcrTlsnAply']");
    let iframe2 = driver.find(iframe2_selector).await?;
    let iframe2_location = iframe2.rect().await?;

    Ok(iframe2_location)
}
pub async fn pass_iframe3(driver: &WebDriver) -> WebDriverResult<ElementRect>{
    let iframe3 = driver.find(By::Id("UcrTlsnAplySignPop")).await?;
    let iframe3_location = iframe3.rect().await?;

    Ok(iframe3_location)
}