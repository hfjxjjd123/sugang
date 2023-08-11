use std::fs::File;
use crate::image_manager::image_analyzer::*;
use thirtyfour::By;
use std::io::Write;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;

pub async fn screenshot_canvas(driver: &WebDriver) -> WebDriverResult<()> {

    let iframe1 = driver.query(By::Id("ContentFrame")).first().await?;
    iframe1.wait_until().displayed().await?;
    let iframe1_location = iframe1.rect().await?;
    println!("iframe1 = {:?}, {:?} : width, height {:?}, {:?}", iframe1_location.x, iframe1_location.y, iframe1_location.width, iframe1_location.height);
    driver.enter_frame(0).await?;
    

    let iframe2_selector = By::Css("iframe.window_iframe[src*='UcrTlsnAply']");
    let iframe2 = driver.find(iframe2_selector).await?;
    let iframe2_location = iframe2.rect().await?;
    println!("iframe2 = {:?}, {:?}", iframe2_location.x, iframe2_location.y);
    driver.enter_frame(0).await?;

    //iframe3
    let iframe3 = driver.find(By::Id("UcrTlsnAplySignPop")).await?;
    let iframe3_location = iframe3.rect().await?;
    println!("iframe3 = {:?}, {:?}", iframe3_location.x, iframe3_location.y);
    driver.enter_frame(0).await?;
    //Error: NoSuchElement("Id(TmtViewer)") 가능
    let canvas_intro = driver.query(By::Id("TmtViewer")).first().await?;
    let canvas_location = canvas_intro.rect().await?;
    let screenshot_data = canvas_intro.screenshot_as_png().await?;

    let mut file = File::create("screenshot.png")?;
    file.write_all(&screenshot_data)?;


    // Analyze the image to determine the X and Y coordinates of the element to click.
    // let (x, y) = analysis_first_canvas();
    // if x==0 && y==0{
    //     return Err(WebDriverError::CustomError("image analysis failed".to_owned()));
    // }
    let (x,y) = get_button_location();
    let (absolute_x, absolute_y) = get_absolute_pixel(canvas_location, x.try_into().unwrap(), y.try_into().unwrap());

    // Simulate a click at the identified X and Y coordinates.
    // driver
    // .action_chain()
    // .move_to(absolute_x , absolute_y)
    // .click()
    // .perform()
    // .await?;

    let script = format!(
        r#"
        const canvas = arguments[0];
        const x = {};
        const y = {};
        const clickEvent = new MouseEvent("click", {{ clientX: x, clientY: y }});
        canvas.dispatchEvent(clickEvent);
        "#,
        absolute_x, absolute_y
    );
    driver.execute(&script, vec![canvas_intro.to_json()?]).await?;

    Ok(())
}