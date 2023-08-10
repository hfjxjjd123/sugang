use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::fs::File;
use std::io::Write;
use image::*;
use thirtyfour::ElementRect;

const STEP0_WIDTH: f64 = 0.9565;
const STEP0_HEIGHT: f64 = 0.064;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // one_cycle().await?;

    let (x,y) = get_button_location();
    println!("{:?}, {:?}", x, y);

    Ok(())
}

// 660/690 16/250

async fn one_cycle() -> WebDriverResult<()> {
    let driver: WebDriver = start_driver().await?;
    open_wise(&driver).await?;
    auto_login(&driver).await?;
    open_sugang_tab(&driver).await?;

    match start_canvas(&driver).await{
        Ok(_) => println!("no problem"),
        Err(WebDriverError::CmdError(_)) => println!("No problem"),
        _ => println!("ERR occur"),
    }
    screenshot_canvas(&driver).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    terminate_driver(driver).await?;
    Ok(())
}

async fn start_driver() -> WebDriverResult<WebDriver> {
    let _binary_path = "./chromedriver";
    let port = 9515;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&format!("http://localhost:{}", port), caps).await?;

    Ok(driver)
}

async fn open_wise(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://sugang.uos.ac.kr/uosdoc/login_sugang.jsp").await?;

    Ok(())
}

async fn terminate_driver(driver: WebDriver) -> WebDriverResult<()> {
    driver.quit().await?;
    Ok(())
}

async fn auto_login(driver: &WebDriver) -> WebDriverResult<()> {
    let username_element = driver.find(By::Name("strLoginId")).await?;
    let password_element = driver.find(By::Name("strLoginPw")).await?;
    let submit_button = driver.find(By::Id("loginImg")).await?;

    username_element.send_keys("hfjxjjd123").await?;
    password_element.send_keys("sksms016526!").await?;
    submit_button.click().await?;

    Ok(())
}

async fn open_sugang_tab(driver: &WebDriver) -> WebDriverResult<()> {
    
    let menu_down = driver.query(By::Id("TopMenu_수강")).first().await?;
    menu_down.wait_until().displayed().await?;
    menu_down.click().await?;

    let sugang_button = driver.query(By::Id("Menu_STUD00230")).first().await?;
    sugang_button.wait_until().displayed().await?;
    sugang_button.click().await?;

    Ok(())
}

async fn alert_handler(driver: &WebDriver) -> WebDriverResult<()> {
    driver.accept_alert().await?;
    Ok(())
}

async fn start_canvas(driver: &WebDriver) -> WebDriverResult<()> {
    
    let canvas_view = driver.query(By::Id("UcrTlsnAply_STUD00230_mdi_div")).first().await?;
    canvas_view.wait_until().displayed().await?;
    alert_handler(driver).await?;

    Ok(())
}

async fn screenshot_canvas(driver: &WebDriver) -> WebDriverResult<()> {

    let iframe1 = driver.query(By::Id("ContentFrame")).first().await?;
    iframe1.wait_until().displayed().await?;
    driver.enter_frame(0).await?;
    driver.enter_frame(0).await?;
    driver.enter_frame(0).await?;

    let canvas_intro = driver.find(By::Id("TmtViewer")).await?;
    let screenshot_data = canvas_intro.screenshot_as_png().await?;
    let canvas_location = canvas_intro.rect().await?;

    let mut file = File::create("screenshot.png")?;
    file.write_all(&screenshot_data)?;


    // Analyze the image to determine the X and Y coordinates of the element to click.
    // let (x, y) = analysis_first_canvas();
    // if x==0 && y==0{
    //     return Err(WebDriverError::CustomError("image analysis failed".to_owned()));
    // }
    let (x,y) = get_button_location();
    let (absolute_x, absolute_y) = get_absolute_pixel(canvas_location, x, y);

    // Simulate a click at the identified X and Y coordinates.
    driver
    .action_chain()
    .move_to(absolute_x , absolute_y)
    .click()
    .perform()
    .await?;

    Ok(())
}

fn analysis_first_canvas()->(u32, u32){
    let screenshot_path = "screenshot.png";
    let screenshot_image = image::open(screenshot_path);

    match screenshot_image{
        Ok(_) => {
            let target_color = Rgba([0xfa, 0xfa, 0xfa, 0xff]);

            for (x, y, pixel) in screenshot_image.expect("REASON").pixels() {
                if pixel == target_color {
                    return (x, y);
                }
            }
            return (0, 0);
        }
        _ => {
            return (0, 0);
        }
    }
}

fn get_button_location() -> (i64, i64){
    let screenshot_path = "screenshot.png";
    let screenshot_image = image::open(screenshot_path);

    let binding = screenshot_image.expect("REASON");

    (((binding.width()) as f64 * STEP0_WIDTH) as i64, ((binding.height()) as f64 * STEP0_HEIGHT) as i64)

}

fn get_absolute_pixel(canvas_element: ElementRect, x: u32, y: u32)->(i64, i64){

    let abs_x = (canvas_element.x) as i64 + x as i64;
    let abs_y = (canvas_element.y) as i64 + y as i64;

    (abs_x, abs_y)
}

