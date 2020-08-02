use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Text, TextInput,
};
use native_dialog::{Dialog, OpenMultipleFile};

use wtools::{commands::upload, storage, Config, PathType};

use crate::style;

pub fn start() {
    App::run(Settings::default())
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum App {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    active_tab: Tab,
    btn_account: button::State,
    btn_delete: button::State,
    btn_list: button::State,
    btn_upload: button::State,

    ln_input: text_input::State,
    ln_input_value: String,
    lp_input: text_input::State,
    lp_input_value: String,
    wiki_url_input: text_input::State,
    wiki_url_input_value: String,
    is_persistent: bool,
    login_button: button::State,

    lockfile: String,
    file_button: button::State,
    execute_button: button::State,
    selected_files: PathType,
    upload_scrollable: scrollable::State,
    saving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Account,
    Delete,
    List,
    Upload,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Account
    }
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(std::result::Result<SavedState, ()>),
    Saved(std::result::Result<(), ()>),
    TabSelected(Tab),
    LoginNameChanged(String),
    LoginPasswordChanged(String),
    WikiUrlChanged(String),
    CheckboxPersistentLogin(bool),
    LoginButtonPressed,
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
        String::from("wtools by FabianLars")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            App::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = App::Loaded(State {
                            ln_input_value: state.ln_input_value,
                            lp_input_value: state.lp_input_value,
                            lockfile: state.lockfile,
                            is_persistent: state.is_persistent,
                            wiki_url_input_value: state.wikiurl,
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
                match message {
                    Message::WikiUrlChanged(value) => {
                        state.wiki_url_input_value = value;
                    }
                    Message::LoginNameChanged(value) => {
                        state.ln_input_value = value;
                    }
                    Message::LoginPasswordChanged(value) => {
                        state.lp_input_value = value;
                    }
                    Message::TabSelected(selected) => {
                        state.active_tab = selected;
                    }
                    Message::CheckboxPersistentLogin(toggle) => {
                        state.is_persistent = toggle;
                    }
                    Message::LoginButtonPressed => {
                        println!("Login{:?}", &state.is_persistent);
                        return match state.is_persistent {
                            true => Command::perform(
                                SavedState {
                                    ln_input_value: state.ln_input_value.clone(),
                                    lp_input_value: state.lp_input_value.clone(),
                                    wikiurl: state.wiki_url_input_value.clone(),
                                    is_persistent: state.is_persistent,
                                    lockfile: state.lockfile.clone(),
                                }
                                .save(),
                                Message::Saved,
                            ),
                            false => Command::perform(
                                SavedState {
                                    ln_input_value: String::new(),
                                    lp_input_value: String::new(),
                                    wikiurl: state.wiki_url_input_value.clone(),
                                    is_persistent: state.is_persistent,
                                    lockfile: state.lockfile.clone(),
                                }
                                .save(),
                                Message::Saved,
                            ),
                        };
                    }
                    Message::FileButtonPressed => {
                        if let Tab::Upload = state.active_tab {
                            return Command::perform(file_dialog(), Message::FilesSelected);
                        }
                    }
                    Message::FilesSelected(files) => {
                        if let Tab::Upload = state.active_tab {
                            state.selected_files = match files {
                                Ok(p) => p,
                                Err(_) => PathType::default(),
                            }
                        }
                    }
                    Message::ExecuteButtonPressed => {
                        if let Tab::Upload = state.active_tab {
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
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            App::Loading => loading_message(),
            App::Loaded(State {
                active_tab,
                btn_account,
                btn_delete,
                btn_list,
                btn_upload,
                ln_input,
                ln_input_value,
                lp_input,
                lp_input_value,
                wiki_url_input,
                wiki_url_input_value,
                is_persistent,
                login_button,
                file_button,
                execute_button,
                selected_files,
                upload_scrollable,
                ..
            }) => {
                let navbar = Row::new()
                    .padding(10)
                    .push(
                        Button::new(btn_account, Container::new(Text::new("Account")).padding(5))
                            .on_press(Message::TabSelected(Tab::Account)),
                    )
                    .push(
                        Button::new(btn_delete, Container::new(Text::new("Delete")).padding(5))
                            .on_press(Message::TabSelected(Tab::Delete)),
                    )
                    .push(
                        Button::new(btn_list, Container::new(Text::new("List")).padding(5))
                            .on_press(Message::TabSelected(Tab::List)),
                    )
                    .push(
                        Button::new(btn_upload, Container::new(Text::new("Upload")).padding(5))
                            .on_press(Message::TabSelected(Tab::Upload)),
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

                let tab_container = Container::new(match active_tab {
                    Tab::Account => Column::new()
                        .padding(10)
                        .spacing(10)
                        .push(
                            TextInput::new(
                                wiki_url_input,
                                "Fandom Wiki URL (api.php)",
                                wiki_url_input_value,
                                Message::WikiUrlChanged,
                            )
                            .size(40),
                        )
                        .push(
                            Row::new()
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
                        .push(
                            Row::new()
                                .push(Checkbox::new(
                                    *is_persistent,
                                    "Stay logged in (stored locally on your device)",
                                    Message::CheckboxPersistentLogin,
                                ))
                                .push(
                                    Button::new(login_button, Text::new("Login"))
                                        .on_press(Message::LoginButtonPressed),
                                ),
                        ),

                    Tab::Upload => Column::new()
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
                    .push(navbar.height(Length::FillPortion(1)).width(Length::Fill))
                    .push(tab_container.height(Length::FillPortion(10)));

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(style::Theme::Dark)
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
        temp.push(f);
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

#[derive(Debug, Clone)]
struct SavedState {
    ln_input_value: String,
    lockfile: String,
    wikiurl: String,
    lp_input_value: String,
    is_persistent: bool,
}

impl SavedState {
    async fn load() -> Result<SavedState, ()> {
        let lp_input_value = storage::get_secure("b9c95dde").await.unwrap_or_default();
        let ln_input_value = storage::get_secure("d7f0942b").await.unwrap_or_default();
        let wikiurl = storage::get("wikiurl")
            .await
            .unwrap_or_else(|_| String::from("https://leagueoflegends.fandom.com/de/api.php"));
        let is_persistent = storage::get("is_persistent")
            .await
            .unwrap_or_else(|_| String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);
        let lockfile = storage::get("lockfile").await.unwrap_or_default();

        Ok(Self {
            lp_input_value,
            ln_input_value,
            wikiurl,
            is_persistent,
            lockfile,
        })
    }

    async fn save(self) -> Result<(), ()> {
        storage::insert_secure("d7f0942b", &self.ln_input_value)
            .await
            .map_err(|e| println!("loginname: {:?}", e))?;
        storage::insert_secure("b9c95dde", &self.lp_input_value)
            .await
            .map_err(|e| println!("botpw: {:?}", e))?;
        storage::insert("lockfile", &self.lockfile)
            .await
            .map_err(|e| println!("lockfile: {:?}", e))?;
        storage::insert("wikiurl", &self.wikiurl)
            .await
            .map_err(|e| println!("wikiurl: {:?}", e))?;
        storage::insert("is_persistent", &self.is_persistent.to_string())
            .await
            .map_err(|e| println!("persistent: {:?}", e))?;

        Ok(())
    }
}
