use thirtyfour::ElementRect;
use image::Rgba;
use image::GenericImageView;

const STEP0_WIDTH: f64 = 0.9565;
const STEP0_HEIGHT: f64 = 0.064;

const STEP_APPLY_WIDTH: f64 = 0.071;
const STEP_APPLY_HEIGHT_DEFAULT: f64 = 0.3148;
const STEP_APPLY_HEIGHT_ADD: f64 = 0.03;

const STEP_QUERY_WIDTH: f64 = 0.9055;
const STEP_QUERY_HEIGHT: f64 = 0.2429;


pub fn analysis_by_color()->(u32, u32){
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

pub fn get_button_location() -> (i64, i64){
    let screenshot_path = "screenshot.png";
    let screenshot_image = image::open(screenshot_path);

    let binding = screenshot_image.expect("REASON");

    (((binding.width()) as f64 * STEP0_WIDTH) as i64, ((binding.height()) as f64 * STEP0_HEIGHT) as i64)

}

pub fn get_absolute_pixel(canvas_element: ElementRect, x: u32, y: u32)->(f64, f64){

    let abs_x = canvas_element.x + x as f64;
    let abs_y = canvas_element.y + y as f64;
    println!("canvas = {:?}, {:?}", abs_x, abs_y);
    (abs_x, abs_y)
}

pub fn query_button_location() -> (i64, i64){
    let screenshot_path = "screenshot2.png";
    let screenshot_image = image::open(screenshot_path);

    let binding = screenshot_image.expect("REASON");

    (((binding.width()) as f64 * STEP_QUERY_WIDTH) as i64, ((binding.height()) as f64 * STEP_QUERY_HEIGHT) as i64)

}
pub fn apply_button_location(index: i32) -> (i64, i64){
    let screenshot_path = "screenshot2.png";
    let screenshot_image = image::open(screenshot_path);

    let binding = screenshot_image.expect("REASON");

    (((binding.width()) as f64 * STEP_APPLY_WIDTH) as i64, ((binding.height()) as f64 * (STEP_APPLY_HEIGHT_DEFAULT + (index as f64 * STEP_APPLY_HEIGHT_ADD))) as i64)

}