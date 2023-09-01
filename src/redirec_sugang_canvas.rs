use thirtyfour::By;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;
use crate::web_exception_handler::alert_handler;
use crate::user_info::*;
use tokio::time::Duration;

pub async fn open_wise(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://sugang.uos.ac.kr/uosdoc/login_sugang.jsp").await?;

    Ok(())
}

pub async fn login_pass(driver: &WebDriver) -> WebDriverResult<()> {
    let username_element = driver.find(By::Name("strLoginId")).await?;
    let password_element = driver.find(By::Name("strLoginPw")).await?;
    let submit_button = driver.find(By::Id("loginImg")).await?;

    username_element.send_keys(ID).await?;
    password_element.send_keys(PASSWORD).await?;
    submit_button.click().await?;

    Ok(())
}

pub async fn click_sugang_menu(driver: &WebDriver) -> WebDriverResult<()> {
    
    let menu_down = driver.query(By::Id("TopMenu_수강")).first().await?;
    menu_down.wait_until().displayed().await?;
    menu_down.click().await?;

    let sugang_button = driver.query(By::Id("Menu_STUD00230")).first().await?;
    sugang_button.wait_until().displayed().await?;
    sugang_button.click().await?;

    Ok(())
}

pub async fn await_first_canvas(driver: &WebDriver) -> WebDriverResult<()> {
    driver.enter_frame(0).await?;
    let canvas_view = driver.query(By::Id("UcrTlsnAply_STUD00230")).first().await?;
    canvas_view.wait_until().displayed().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    alert_handler(driver).await?;
    driver.enter_default_frame().await?;
    Ok(())
}