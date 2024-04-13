slint::include_modules!();

use frontend::Frontend;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn start() {
    let ui = AppWindow::new().unwrap();

    let frontend = Frontend::new();

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui_handle = ui_handle.clone();
            frontend.process_counter(ui_handle.unwrap().get_counter().into(), move |counter| {
                let ui = ui_handle.unwrap();
                ui.set_counter(counter.into());
            });
        }
    });

    ui.run().unwrap();
}
