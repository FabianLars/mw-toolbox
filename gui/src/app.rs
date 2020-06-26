use iced::{
    button, scrollable, text_input, Align, Application, Button, Column, Command, Container,
    Element, HorizontalAlignment, Length, Radio, Row, Scrollable, Settings, Text, TextInput,
};
use native_dialog::{Dialog, OpenMultipleFile};
use serde::{Deserialize, Serialize};

use wtools::{commands::upload, storage, Config, PathType};

use crate::style;

pub fn start() {
    App::run(Settings::default())
}

#[derive(Debug)]
enum App {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    ln_input: text_input::State,
    ln_input_value: String,
    lp_input: text_input::State,
    lp_input_value: String,
    lockfile: String,
    chosen_command: ChosenCommand,
    file_button: button::State,
    execute_button: button::State,
    selected_files: PathType,
    upload_scrollable: scrollable::State,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChosenCommand {
    Delete,
    List,
    Upload,
}

impl Default for ChosenCommand {
    fn default() -> Self {
        ChosenCommand::List
    }
}

impl ChosenCommand {
    pub const ALL: [ChosenCommand; 3] = [
        ChosenCommand::Delete,
        ChosenCommand::List,
        ChosenCommand::Upload,
    ];
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(std::result::Result<SavedState, ()>),
    Saved(std::result::Result<(), ()>),
    LoginNameChanged(String),
    LoginPasswordChanged(String),
    CommandSelected(ChosenCommand),
    FileButtonPressed,
    FilesSelected(Result<PathType, ()>),
    ExecuteButtonPressed,
    Executed(Result<(), ()>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            App::Loading => false,
            App::Loaded(state) => state.dirty,
        };

        format!(
            "wtools by FabianLars{}",
            if dirty { " - *Unsaved Changes" } else { "" }
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            App::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = App::Loaded(State {
                            ln_input_value: state.ln_input_value,
                            lockfile: state.lockfile,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = App::Loaded(State::default());
                    }
                    _ => {}
                }

                Command::none()
            }
            App::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::LoginNameChanged(value) => {
                        state.ln_input_value = value;
                    }
                    Message::LoginPasswordChanged(value) => {
                        state.lp_input_value = value;
                    }
                    Message::CommandSelected(selected) => {
                        state.chosen_command = selected;
                    }
                    Message::FileButtonPressed => {
                        if let ChosenCommand::Upload = state.chosen_command {
                            return Command::perform(file_dialog(), Message::FilesSelected);
                        }
                    }
                    Message::FilesSelected(files) => {
                        if let ChosenCommand::Upload = state.chosen_command {
                            state.selected_files = match files {
                                Ok(p) => p,
                                Err(_) => PathType::default(),
                            }
                        }
                    }
                    Message::ExecuteButtonPressed => {
                        if let ChosenCommand::Upload = state.chosen_command {
                            return Command::perform(
                                upload::from_gui(
                                    Config::new(
                                        state.ln_input_value.clone(),
                                        state.lp_input_value.clone(),
                                    )
                                    .with_pathtype(state.selected_files.clone()),
                                ),
                                Message::Executed,
                            );
                        }
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }
                    _ => {}
                }

                if !saved {
                    state.dirty = true;
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            ln_input_value: state.ln_input_value.clone(),
                            lockfile: state.lockfile.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            App::Loading => loading_message(),
            App::Loaded(State {
                ln_input,
                ln_input_value,
                lp_input,
                lp_input_value,
                chosen_command,
                file_button,
                execute_button,
                selected_files,
                upload_scrollable,
                ..
            }) => {
                let navbar = Container::new(
                    Column::new()
                        .push(
                            Row::new()
                                .padding(10)
                                .spacing(10)
                                .push(
                                    TextInput::new(
                                        ln_input,
                                        "Fandom Username",
                                        ln_input_value,
                                        Message::LoginNameChanged,
                                    )
                                    .size(40),
                                )
                                .push(
                                    TextInput::new(
                                        lp_input,
                                        "Fandom Password",
                                        lp_input_value,
                                        Message::LoginPasswordChanged,
                                    )
                                    .size(40)
                                    .password(),
                                ),
                        )
                        // TODO: Tabber instead of RadioButtons
                        .push(ChosenCommand::ALL.iter().cloned().fold(
                            Row::new().padding(10),
                            |row, cmd| {
                                row.push(
                                    Radio::new(
                                        cmd,
                                        &format!("{:?}", cmd),
                                        Some(chosen_command.to_owned()),
                                        Message::CommandSelected,
                                    )
                                    .width(Length::FillPortion(1)),
                                )
                            },
                        )),
                );

                let mut text_files = String::new();
                match selected_files {
                    PathType::File(x) => text_files.push_str(match x.file_name() {
                        Some(s) => s.to_str().unwrap(),
                        None => "",
                    }),
                    PathType::Files(x) => {
                        for f in x {
                            text_files.push_str(f.file_name().unwrap().to_str().unwrap());
                            text_files.push_str("\n");
                        }
                    }
                    PathType::Folder(x) => {
                        for f in std::fs::read_dir(x).unwrap() {
                            text_files.push_str(f.unwrap().file_name().to_str().unwrap());
                            text_files.push_str("\n");
                        }
                    }
                }

                let cmd_container = Container::new(match chosen_command {
                    ChosenCommand::Upload => Column::new()
                        .push(
                            Container::new(
                                Scrollable::new(upload_scrollable).push(Text::new(text_files)),
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Align::Center),
                        )
                        .push(
                            Container::new(
                                Row::new()
                                    .padding(10)
                                    .spacing(20)
                                    .push(
                                        Button::new(file_button, Text::new("Select File(s)"))
                                            .on_press(Message::FileButtonPressed),
                                    )
                                    .push(
                                        Button::new(execute_button, Text::new("Execute"))
                                            .on_press(Message::ExecuteButtonPressed),
                                    ),
                            )
                            .width(Length::Fill)
                            .height(Length::Shrink)
                            .align_x(Align::Center),
                        ),

                    _ => Column::new(),
                });

                let content = Column::new()
                    .push(
                        navbar
                            .height(Length::FillPortion(2))
                            .style(style::Theme::Dark),
                    )
                    .push(
                        cmd_container
                            .height(Length::FillPortion(10))
                            .style(style::Theme::Dark),
                    );

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

async fn file_dialog() -> Result<PathType, ()> {
    let dialog = OpenMultipleFile {
        dir: None,
        filter: None,
    };
    let result = tokio::task::spawn_blocking(|| dialog.show().unwrap())
        .await
        .unwrap();

    let mut temp: Vec<std::path::PathBuf> = Vec::new();
    for f in result {
        temp.push(std::path::PathBuf::from(f));
    }
    Ok(PathType::Files(temp))
}

fn loading_message() -> Element<'static, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    ln_input_value: String,
    lockfile: String,
}

impl SavedState {
    async fn load() -> Result<SavedState, ()> {
        Ok(Self {
            ln_input_value: storage::get_secure(&base64::encode("lgname"), "lgnamead")
                .await
                .map_err(|_| ())?,
            lockfile: storage::get_secure(&base64::encode("lockfile"), "lockfilead")
                .await
                .map_err(|_| ())?,
        })
    }

    async fn save(self) -> Result<(), ()> {
        storage::insert_secure(&base64::encode("lgname"), &self.ln_input_value, "lgnamead")
            .await
            .map_err(|_| ())?;
        storage::insert_secure(&base64::encode("lockfile"), &self.lockfile, "lockfilead")
            .await
            .map_err(|_| ())?;

        // This is a simple way to save at most once every couple seconds
        tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
