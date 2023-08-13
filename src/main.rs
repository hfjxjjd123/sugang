use thirtyfour::prelude::*;
use tokio::time::Duration;
use sugang::chrome_manager::*;
use sugang::init_page::initialize;
use sugang::iter_apply::*;
use sugang::web_exception_handler::alert_handler;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let num_of_basket: Vec<i32> = vec![1,2,6];

    let driver: WebDriver = start_driver().await?;
    initialize(&driver).await?;

    let targets: Vec<(i64, i64)> = initial_apply_canvas(&driver, num_of_basket).await?;
    let reload_button = targets[0];

    //성공했다면 성공했을시 로직
    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }

        for target in &targets{
            driver.action_chain()
                .move_to(target.0, target.1)
                .click()
                .perform()
                .await?;
            //기다렸다가 실행하는 로직 or alert 핸들링하기
            //alert가 없는데 alert 핸들러 실행하는 로직 수정
            alert_handler(&driver).await?;
        }
        //click reload
        driver.action_chain()
                .move_to(reload_button.0, reload_button.1)
                .click()
                .perform()
                .await?;

        tokio::time::sleep(Duration::from_secs(2)).await;
        count += 1;
    }
    

    tokio::time::sleep(Duration::from_secs(10)).await;
    terminate_driver(driver).await?;
    Ok(())
}


