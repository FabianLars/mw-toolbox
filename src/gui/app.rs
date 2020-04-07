use iced::{
    button, scrollable, text_input, Align, Application, Button, Column, Command, Container,
    Element, HorizontalAlignment, Length, Radio, Row, Scrollable, Settings, Text, TextInput,
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
    file_button: button::State,
    folder_button: button::State,
    execute_button: button::State,
    selected_files: super::super::UploadInput,
    upload_scrollable: scrollable::State,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChosenCommand {
    Delete,
    List,
    Update,
    Upload,
}

impl Default for ChosenCommand {
    fn default() -> Self {
        ChosenCommand::List
    }
}

impl ChosenCommand {
    pub const ALL: [ChosenCommand; 4] = [
        ChosenCommand::Delete,
        ChosenCommand::List,
        ChosenCommand::Update,
        ChosenCommand::Upload,
    ];
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    LoginNameChanged(String),
    LoginPasswordChanged(String),
    CommandSelected(ChosenCommand),
    FileButtonPressed,
    FolderButtonPressed,
    ExecuteButtonPressed,
    Executed(Result<(), ExecuteError>),
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
                        let result =
                            nfd::open_file_multiple_dialog(None, None).unwrap_or_else(|e| {
                                panic!(e);
                            });

                        match result {
                            nfd::Response::Okay(file_path) => {
                                state.selected_files = super::super::UploadInput::File(
                                    std::path::PathBuf::from(file_path),
                                )
                            }
                            nfd::Response::OkayMultiple(files) => {
                                let mut temp: Vec<std::path::PathBuf> = Vec::new();
                                for f in files {
                                    temp.push(std::path::PathBuf::from(f));
                                }
                                state.selected_files = super::super::UploadInput::Files(temp)
                            }
                            nfd::Response::Cancel => println!("User canceled"),
                        }
                    }
                    Message::FolderButtonPressed => {
                        let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
                            panic!(e);
                        });

                        match result {
                            nfd::Response::Okay(folder_path) => {
                                state.selected_files = super::super::UploadInput::Folder(
                                    std::path::PathBuf::from(folder_path),
                                )
                            }
                            nfd::Response::Cancel => println!("User canceled"),
                            _ => (),
                        }
                    }
                    Message::ExecuteButtonPressed => {
                        if state.chosen_command == ChosenCommand::Upload {
                            return Command::perform(
                                crate::commands::upload::from_gui(super::super::UploadProps {
                                    input: state.selected_files.clone(),
                                    loginname: state.ln_input_value.clone(),
                                    loginpassword: state.lp_input_value.clone(),
                                }),
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
                folder_button,
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
                        )),
                );

                let mut text_files = String::new();
                match selected_files {
                    super::super::UploadInput::File(x) => text_files.push_str(
                        x.file_name()
                            .unwrap_or(std::ffi::OsStr::new(""))
                            .to_str()
                            .expect("file to gui text"),
                    ),
                    super::super::UploadInput::Files(x) => {
                        for f in x {
                            text_files.push_str(
                                f.file_name().unwrap().to_str().expect("files to gui text"),
                            );
                            text_files.push_str("\n");
                        }
                    }
                    super::super::UploadInput::Folder(x) => {
                        for f in std::fs::read_dir(x).expect("read folder for gui text") {
                            text_files.push_str(
                                f.unwrap()
                                    .file_name()
                                    .to_str()
                                    .expect("folder -> file name to str (gui)"),
                            );
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
                                        Button::new(folder_button, Text::new("Select Folder"))
                                            .on_press(Message::FolderButtonPressed),
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
                            .style(super::style::Theme::Dark),
                    )
                    .push(
                        cmd_container
                            .height(Length::FillPortion(10))
                            .style(super::style::Theme::Dark),
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
pub enum ExecuteError {
    Upload,
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
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;

        Ok(())
    }
}
