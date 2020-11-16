#![recursion_limit = "512"]
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
struct Model {}

#[derive(Clone, Debug)]
enum Message {
    Exit,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("de.fabianlars.wtools"), ApplicationFlags::empty())>
                <Window default_width=1280 default_height=720 title="wtools by FabianLars" window_position=WindowPosition::Center on destroy=|_| Message::Exit>
                    <Notebook show_tabs=true>
                        <Box Notebook::tab_label="Profile">
                            <Label label="Profile" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                        <Box Notebook::tab_label="Delete">
                            <Label label="Delete" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                        <Box Notebook::tab_label="List">
                            <Label label="List" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                        <Box Notebook::tab_label="Move">
                            <Label label="Move" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                        <Box Notebook::tab_label="Purge">
                            <Label label="Purge" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                        <Box Notebook::tab_label="Upload">
                            <Label label="Upload" />
                            <Box><CheckButton active=true /></Box>
                        </Box>
                    </Notebook>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
