use std::fs::File;
use thirtyfour::By;
use std::io::Write;
use thirtyfour::error::WebDriverResult;
use thirtyfour::ElementRect;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;

pub async fn screenshot_canvas(driver: &WebDriver, id: &str, image_name: &str) -> WebDriverResult<ElementRect>{
    let canvas_intro = driver.query(By::Id(id)).first().await?;
    let canvas_location = canvas_intro.rect().await?;
    let screenshot_data = canvas_intro.screenshot_as_png().await?;

    let mut file = File::create(image_name)?;
    file.write_all(&screenshot_data)?;

    Ok(canvas_location)
}