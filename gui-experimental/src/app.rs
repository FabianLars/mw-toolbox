use std::path::PathBuf;

use iced::{
    button, futures::TryFutureExt, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings,
    Text, TextInput,
};
use native_dialog::{Dialog, OpenMultipleFile};

use wtools::{api, PathType, WikiClient};

use crate::style;

pub fn start() {
    App::run(Settings::default()).unwrap()
}

#[derive(Debug, Default)]
struct App {
    loading: bool,
    wk_client: WikiClient,
    state: State,
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
    Loaded(Result<SavedState, ()>),
    Saved(Result<(), ()>),
    TabSelected(Tab),
    LoginNameChanged(String),
    LoginPasswordChanged(String),
    WikiUrlChanged(String),
    CheckboxPersistentLogin(bool),
    LoginButtonPressed,
    LoggedIn(Result<WikiClient, ()>),
    FileButtonPressed,
    FilesSelected(Result<PathType, ()>),
    ExecuteButtonPressed,
    Executed(Result<(), ()>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                loading: true,
                ..App::default()
            },
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("wtools by FabianLars")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self.loading {
            true => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        self.loading = false;
                        self.state = State {
                            ln_input_value: state.ln_input_value,
                            lp_input_value: state.lp_input_value,
                            is_persistent: state.is_persistent,
                            wiki_url_input_value: state.wikiurl,
                            ..State::default()
                        };
                    }
                    Message::Loaded(Err(_)) => {
                        self.state = State::default();
                    }
                    _ => {}
                }

                Command::none()
            }
            false => {
                match message {
                    Message::WikiUrlChanged(value) => {
                        self.state.wiki_url_input_value = value;
                    }
                    Message::LoginNameChanged(value) => {
                        self.state.ln_input_value = value;
                    }
                    Message::LoginPasswordChanged(value) => {
                        self.state.lp_input_value = value;
                    }
                    Message::TabSelected(selected) => {
                        self.state.active_tab = selected;
                    }
                    Message::CheckboxPersistentLogin(toggle) => {
                        self.state.is_persistent = toggle;
                    }
                    Message::LoginButtonPressed => {
                        return Command::perform(
                            WikiClient::new_logged_in(
                                self.state.wiki_url_input_value.clone(),
                                self.state.ln_input_value.clone(),
                                self.state.lp_input_value.clone(),
                            )
                            .map_err(|_| ()),
                            Message::LoggedIn,
                        );
                    }
                    Message::FileButtonPressed => {
                        if let Tab::Upload = self.state.active_tab {
                            return Command::perform(file_dialog(), Message::FilesSelected);
                        }
                    }
                    Message::FilesSelected(files) => {
                        if let Tab::Upload = self.state.active_tab {
                            self.state.selected_files = match files {
                                Ok(p) => p,
                                Err(_) => PathType::default(),
                            }
                        }
                    }
                    Message::ExecuteButtonPressed => {
                        if let Tab::Upload = self.state.active_tab {
                            return Command::perform(
                                api::upload::upload(
                                    self.wk_client.clone(),
                                    self.state.selected_files.clone(),
                                )
                                .map_err(|_| ()),
                                Message::Executed,
                            );
                        }
                    }
                    Message::LoggedIn(res) => {
                        if let Ok(client) = res {
                            self.wk_client = client;

                            return match self.state.is_persistent {
                                true => Command::perform(
                                    SavedState {
                                        ln_input_value: self.state.ln_input_value.clone(),
                                        lp_input_value: self.state.lp_input_value.clone(),
                                        wikiurl: self.state.wiki_url_input_value.clone(),
                                        is_persistent: self.state.is_persistent,
                                    }
                                    .save(),
                                    Message::Saved,
                                ),
                                false => Command::perform(
                                    SavedState {
                                        ln_input_value: String::new(),
                                        lp_input_value: String::new(),
                                        wikiurl: self.state.wiki_url_input_value.clone(),
                                        is_persistent: self.state.is_persistent,
                                    }
                                    .save(),
                                    Message::Saved,
                                ),
                            };
                        }
                    }
                    Message::Saved(_) => {
                        self.state.saving = false;
                    }
                    Message::Executed(res) => println!("{:?}", res),
                    _ => {}
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.loading {
            true => loading_message(),
            false => {
                let navbar = Row::new()
                    .padding(10)
                    .push(
                        Button::new(
                            &mut self.state.btn_account,
                            Container::new(Text::new("Account")).padding(5),
                        )
                        .on_press(Message::TabSelected(Tab::Account)),
                    )
                    .push(
                        Button::new(
                            &mut self.state.btn_delete,
                            Container::new(Text::new("Delete")).padding(5),
                        )
                        .on_press(Message::TabSelected(Tab::Delete)),
                    )
                    .push(
                        Button::new(
                            &mut self.state.btn_list,
                            Container::new(Text::new("List")).padding(5),
                        )
                        .on_press(Message::TabSelected(Tab::List)),
                    )
                    .push(
                        Button::new(
                            &mut self.state.btn_upload,
                            Container::new(Text::new("Upload")).padding(5),
                        )
                        .on_press(Message::TabSelected(Tab::Upload)),
                    );

                let mut text_files = String::new();
                match &self.state.selected_files {
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

                let tab_container = Container::new(match &self.state.active_tab {
                    Tab::Account => Column::new()
                        .padding(10)
                        .spacing(10)
                        .push(
                            TextInput::new(
                                &mut self.state.wiki_url_input,
                                "Fandom Wiki URL (api.php)",
                                &self.state.wiki_url_input_value,
                                Message::WikiUrlChanged,
                            )
                            .size(40),
                        )
                        .push(
                            Row::new()
                                .push(
                                    TextInput::new(
                                        &mut self.state.ln_input,
                                        "Fandom Username",
                                        &self.state.ln_input_value,
                                        Message::LoginNameChanged,
                                    )
                                    .size(40),
                                )
                                .push(
                                    TextInput::new(
                                        &mut self.state.lp_input,
                                        "Fandom Password",
                                        &self.state.lp_input_value,
                                        Message::LoginPasswordChanged,
                                    )
                                    .size(40)
                                    .password(),
                                ),
                        )
                        .push(
                            Row::new()
                                .push(Checkbox::new(
                                    self.state.is_persistent,
                                    "Stay logged in (stored locally on your device)",
                                    Message::CheckboxPersistentLogin,
                                ))
                                .push(
                                    Button::new(&mut self.state.login_button, Text::new("Login"))
                                        .on_press(Message::LoginButtonPressed),
                                ),
                        ),

                    Tab::Upload => Column::new()
                        .push(
                            Container::new(
                                Scrollable::new(&mut self.state.upload_scrollable)
                                    .push(Text::new(text_files)),
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
                                        Button::new(
                                            &mut self.state.file_button,
                                            Text::new("Select File(s)"),
                                        )
                                        .on_press(Message::FileButtonPressed),
                                    )
                                    .push(
                                        Button::new(
                                            &mut self.state.execute_button,
                                            Text::new("Execute"),
                                        )
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
        .map_err(|_| ())?;

    let mut temp: Vec<PathBuf> = Vec::new();
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

        let s = Self {
            lp_input_value,
            ln_input_value,
            wikiurl,
            is_persistent,
        };
        Ok(s)
    }

    async fn save(self) -> Result<(), ()> {
        storage::insert_multiple(&[
            (
                "d7f0942b",
                storage::encrypt(&self.ln_input_value)
                    .map_err(|e| println!("Error encrypting password: {}", e))?
                    .as_slice(),
            ),
            (
                "b9c95dde",
                storage::encrypt(&self.lp_input_value)
                    .map_err(|e| println!("Error encrypting name: {}", e))?
                    .as_slice(),
            ),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
        .map_err(|e| println!("Error saving app data: {}", e))
    }
}
