use druid::{
    commands,
    widget::{Align, Button, Flex, TextBox},
    AppDelegate, AppLauncher, Command, DelegateCtx, Env, FileDialogOptions, FileSpec, Handled,
    LocalizedString, Target, Widget, WindowDesc,
};

struct Delegate;

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("open-save-demo").with_placeholder("Opening/Saving Demo"));
    let data = "Type here.".to_owned();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<String> {
    let rs = FileSpec::new("Rust source", &["rs"]);
    let txt = FileSpec::new("Text file", &["txt"]);
    let other = FileSpec::new("Bogus file", &["foo", "bar", "baz"]);
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![rs, txt, other])
        .default_type(txt);
    let open_dialog_options = save_dialog_options.clone();

    let input = TextBox::new();
    let save = Button::new("Save").on_click(move |ctx, _, _| {
        ctx.submit_command(Command::new(
            druid::commands::SHOW_SAVE_PANEL,
            save_dialog_options.clone(),
            Target::Auto,
        ))
    });
    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(Command::new(
            druid::commands::SHOW_OPEN_PANEL,
            open_dialog_options.clone(),
            Target::Auto,
        ))
    });

    let mut col = Flex::column();
    col.add_child(input);
    col.add_spacer(8.0);
    col.add_child(save);
    col.add_child(open);
    Align::centered(col)
}

impl AppDelegate<String> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut String,
        _env: &Env,
    ) -> Handled {
        if let Some(Some(file_info)) = cmd.get(commands::SAVE_FILE) {
            if let Err(e) = std::fs::write(file_info.path(), &data[..]) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    let first_line = s.lines().next().unwrap_or("");
                    *data = first_line.to_owned();
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}
