use crate::image_manager::pass_iframe::*;
use crate::redirec_sugang_canvas::await_first_canvas;
use crate::redirec_sugang_canvas::click_sugang_menu;
use crate::redirec_sugang_canvas::login_pass;
use crate::redirec_sugang_canvas::open_wise;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;

pub async fn initialize(driver: &WebDriver)-> WebDriverResult<()>{
    open_wise(driver).await?;
    login_pass(driver).await?;
    click_sugang_menu(driver).await?;

    match await_first_canvas(driver).await{
        Ok(_) => println!("no problem"),
        Err(WebDriverError::CmdError(_)) => println!("No problem"),
        _ => println!("ERR occur"),
    }
    remove_first_canvas(driver).await?;

    Ok(())
}