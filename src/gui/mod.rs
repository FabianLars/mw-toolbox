use iced::{
    button, text_input, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Radio, Row, Settings, Text, TextInput,
};
use serde::{Deserialize, Serialize};

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
    chosen_command: ChosenCommand,
    apply_button: button::State,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChosenCommand {
    Delete,
    List,
    Update,
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
        ChosenCommand::Update,
    ];
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    LoginNameChanged(String),
    LoginPasswordChanged(String),
    CommandSelected(ChosenCommand),
    ApplyPressed,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> (App, Command<Message>) {
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
                    Message::ApplyPressed => (),
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
                apply_button,
                ..
            }) => Column::new()
                .push(
                    Row::new()
                        .push(
                            TextInput::new(
                                ln_input,
                                "Fandom Username",
                                ln_input_value,
                                Message::LoginNameChanged,
                            )
                            .padding(10)
                            .size(30),
                        )
                        .push(
                            TextInput::new(
                                lp_input,
                                "Fandom Password",
                                lp_input_value,
                                Message::LoginPasswordChanged,
                            )
                            .padding(10)
                            .size(30),
                        ),
                )
                .push(ChosenCommand::ALL.iter().cloned().fold(
                    Row::new().padding(10),
                    |row, cmd| {
                        row.push(Radio::new(
                            cmd,
                            &format!("{:?}", cmd),
                            Some(chosen_command.to_owned()),
                            Message::CommandSelected,
                        ))
                    },
                ))
                .into(),
        }
    }
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
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories::ProjectDirs::from("de", "FabianLars", "wtools")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("wtools.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use tokio::prelude::*;

        let mut contents = String::new();

        let mut file = tokio::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        use tokio::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }

        {
            let mut file = tokio::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        tokio::time::delay_for(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
