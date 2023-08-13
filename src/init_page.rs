use crate::image_manager::pass_iframe::*;
use crate::redirec_sugang_canvas::await_first_canvas;
use crate::redirec_sugang_canvas::click_sugang_menu;
use crate::redirec_sugang_canvas::login_pass;
use crate::redirec_sugang_canvas::open_wise;
use thirtyfour::error::WebDriverResult;
use thirtyfour::WebDriver;
use thirtyfour::prelude::*;
use crate::web_exception_handler::alert_handler;
use fantoccini::error::CmdError::Standard;
use crate::iter_apply::initial_apply_canvas;


pub async fn initialize(driver: &WebDriver)-> WebDriverResult<()>{
    open_wise(driver).await?;
    login_pass(driver).await?;
    click_sugang_menu(driver).await?;

    match await_first_canvas(driver).await{
        Ok(_) => (),
        Err(WebDriverError::CmdError(Standard(wd))) => {
            if wd.error() == "unexpected alert open" {
                alert_handler(driver).await?;
                await_first_canvas(driver).await?;
            } else{
                return Err(WebDriverError::CustomError("Unexpected".to_owned()));
            }
        }
        _ => {
            return Err(WebDriverError::CustomError("Unexpected".to_owned()));
        }
    }
    remove_first_canvas(driver).await?;

    Ok(())
}

pub async fn analyze_elements_location(driver: &WebDriver, num_of_basket: Vec<i32>)->WebDriverResult<Vec<(i64,i64)>>{
    let targets: Vec<(i64, i64)> = initial_apply_canvas(&driver, num_of_basket).await?;

    Ok(targets)
}