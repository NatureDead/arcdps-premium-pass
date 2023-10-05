use arcdps::imgui::{Ui, Window};

use crate::itemcode::*;

pub fn imgui(ui: &Ui, not_charsel_or_loading: bool) {
    if !not_charsel_or_loading {
        return;
    }

    let item_code = decode_chat_code_for_item_or_skin("[&AgFeNAEA]");
    let mut chat_code = generate_chat_code_for_item(item_code, 3, None, None, None);

    let window = Window::new("Premium Pass");
    window.build(ui, || {
        ui.text("chat code: ");
        ui.same_line();
        ui.input_text("", &mut chat_code).build();

        if ui.button("copy") {
            ui.set_clipboard_text(chat_code.clone());
        }
        
        if ui.button("copy and send") {
            // ui.set_clipboard_text(chat_code.clone());
            

            // let mut isPressed =  ui.io().index(Key::Enter);

            // ui.io().keys_down[] = true;
        }
    });
}
