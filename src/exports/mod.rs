use arcdps::imgui::{Ui, Window};

pub fn imgui(_ui: &Ui, not_charsel_or_loading: bool) {
    if !not_charsel_or_loading {
        return;
    }

    let window = Window::new("Premium Pass");
    window.build(_ui, || {
        _ui.text("Hello world!");
        _ui.separator();

        let mouse_pos = _ui.io().mouse_pos;
        _ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));
    });
}
