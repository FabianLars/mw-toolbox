use iced::{button, scrollable, text_input, Align, Application, Button, Column, Command, Container, Element, HorizontalAlignment, Length, Radio, Row, Scrollable, Settings, Text, TextInput, Checkbox};
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

    lockfile: String,
    file_button: button::State,
    execute_button: button::State,
    selected_files: PathType,
    upload_scrollable: scrollable::State,
    saving: bool,
    needs_saving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Account,
    Delete,
    List,
    Upload,
}

impl Default for Tab {
    fn default() -> Self { Tab::Account }
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
        let needs_saving = match self {
            App::Loading => false,
            App::Loaded(state) => state.needs_saving,
        };

        format!(
            "wtools by FabianLars{}",
            if needs_saving { " - *Unsaved Changes" } else { "" }
        )
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
                let mut saved = false;

                match message {
                    Message::WikiUrlChanged(value) => {
                        state.wiki_url_input_value = value;
                        if state.is_persistent { state.needs_saving = true; }
                    }
                    Message::LoginNameChanged(value) => {
                        state.ln_input_value = value;
                        if state.is_persistent { state.needs_saving = true; }
                    }
                    Message::LoginPasswordChanged(value) => {
                        state.lp_input_value = value;
                        if state.is_persistent { state.needs_saving = true; }
                    }
                    Message::TabSelected(selected) => {
                        state.active_tab = selected;
                    }
                    Message::CheckboxPersistentLogin(toggle) => {
                        state.is_persistent = toggle;
                        if state.is_persistent { state.needs_saving = true; }
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
                        saved = true;
                        state.needs_saving = false;
                    }
                    _ => {}
                }

                if state.needs_saving && !saved && state.is_persistent {
                    state.needs_saving = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            ln_input_value: state.ln_input_value.clone(),
                            lp_input_value: state.lp_input_value.clone(),
                            wikiurl: state.wiki_url_input_value.clone(),
                            is_persistent: state.is_persistent.clone(),
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
                file_button,
                execute_button,
                selected_files,
                upload_scrollable,
                ..
            }) => {
                let navbar = Row::new().padding(10)
                        .push(
                            Button::new(
                                btn_account,
                                Container::new(Text::new("Account")).padding(5),
                            ).on_press(Message::TabSelected(Tab::Account))
                        )
                        .push(
                            Button::new(
                                btn_delete,
                                Container::new(Text::new("Delete")).padding(5),
                            ).on_press(Message::TabSelected(Tab::Delete))
                        )
                        .push(
                            Button::new(
                                btn_list,
                                Container::new(Text::new("List")).padding(5),
                            ).on_press(Message::TabSelected(Tab::List))
                        )
                        .push(
                            Button::new(
                                btn_upload,
                                Container::new(Text::new("Upload")).padding(5),
                            ).on_press(Message::TabSelected(Tab::Upload))
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
                    Tab::Account => Column::new().padding(10).spacing(10)
                        .push(
                            TextInput::new(wiki_url_input, "Fandom Wiki URL (api.php)", wiki_url_input_value, Message::WikiUrlChanged)
                                .size(40)
                        )
                        .push(Row::new()
                            .push(
                                TextInput::new(ln_input, "Fandom Username", ln_input_value, Message::LoginNameChanged).size(40)
                            )
                            .push(
                                TextInput::new(lp_input, "Fandom Password", lp_input_value, Message::LoginPasswordChanged).size(40).password()
                            )
                        )
                        .push(Checkbox::new(
                        *is_persistent, "Stay logged in (stored locally on your device)", Message::CheckboxPersistentLogin
                        )),

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
                    .push(
                        navbar
                            .height(Length::FillPortion(1))
                            .width(Length::Fill)
                    )
                    .push(
                        tab_container
                            .height(Length::FillPortion(10))
                    );

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
    wikiurl: String,
    lp_input_value: String,
    is_persistent: bool
}

impl SavedState {
    async fn load() -> Result<SavedState, ()> {
        let loginname = storage::get_secure(&base64::encode("lgname"), "lgnamead")
            .await.unwrap_or(String::new());
        Ok(Self {
            lp_input_value: storage::get_secure(&base64::encode("wk_botpw"), &loginname)
                .await.unwrap_or(String::new()),
            ln_input_value: loginname,
            wikiurl: storage::get_secure(&base64::encode("wikiurl"), "wikiurlad")
                .await.unwrap_or(String::from("https://leagueoflegends.fandom.com/de/api.php")),
            is_persistent: storage::get_secure(&base64::encode("is_persistent"), "is_persistentad")
                .await.unwrap_or(String::from("false")).parse::<bool>().unwrap_or(false),
            lockfile: storage::get_secure(&base64::encode("lockfile"), "lockfilead")
                .await.unwrap_or(String::new()),
        })
    }

    async fn save(self) -> Result<(), ()> {
        storage::insert_secure(&base64::encode("lgname"), &self.ln_input_value, "lgnamead")
            .await
            .map_err(|e| println!("{:?}", e))?;
        storage::insert_secure(&base64::encode("wk_botpw"), &self.lp_input_value, &self.ln_input_value)
            .await
            .map_err(|_| ())?;
        storage::insert_secure(&base64::encode("lockfile"), &self.lockfile, "lockfilead")
            .await
            .map_err(|_| ())?;
        storage::insert_secure(&base64::encode("wikiurl"), &self.wikiurl, "wikiurlad")
            .await
            .map_err(|_| ())?;
        storage::insert_secure(&base64::encode("is_persistent"), &self.is_persistent.to_string(), "is_persistentad")
            .await
            .map_err(|_| ())?;

        // This is a simple way to save at most once every couple seconds
        tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
