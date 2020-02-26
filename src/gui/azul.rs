use azul::{
    prelude::*,
    widgets::{button::Button, label::Label},
};

struct DataModel {
    counter: usize,
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Update counter")
            .dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div).with_child(label).with_child(button)
    }
}

// View updates Model
fn update_counter(
    app_state: &mut AppState<DataModel>,
    _event: &mut CallbackInfo<DataModel>,
) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    Redraw
}

pub fn start() {
    let mut app = App::new(DataModel { counter: 0 }, AppConfig::default()).unwrap();
    let window = app
        .create_window(WindowCreateOptions::default(), css::native())
        .unwrap();
    app.run(window).unwrap();
}
