
use image::{
    error::{ParameterError, ParameterErrorKind},
    ImageBuffer, ImageError, Rgba,
};
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use slint::{Timer, TimerMode};

slint::slint!{
    export component MainWindow {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
fn main() {
    let window = MainWindow::new().unwrap();
    let _s = grab_selfie(&window);
    window.run().unwrap()
}

pub fn grab_selfie(main_window: &MainWindow) -> Timer {
    let sc_timer = Timer::default();
    let sc_handle = main_window.as_weak();

    sc_timer.start(
        TimerMode::SingleShot,
        std::time::Duration::from_millis(1000 * 3),
        move || {
            let ui = sc_handle.unwrap();
            ui.window().request_redraw();
            let pixel_buffer = ui.window().take_snapshot().unwrap();
            save_screenshot(pixel_buffer);
        },
    );

    sc_timer
}

fn save_screenshot(screenshot: SharedPixelBuffer<Rgba8Pixel>) {

    let (width, height) = (screenshot.width(), screenshot.height());
    let raw_pixels: &[u8] = screenshot.as_bytes();

    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_pixels)
        .ok_or_else(|| {
            ImageError::Parameter(ParameterError::from_kind(
                ParameterErrorKind::DimensionMismatch,
            ))
        })
        .unwrap();

    let _ = img.save("screenshot.png");
}