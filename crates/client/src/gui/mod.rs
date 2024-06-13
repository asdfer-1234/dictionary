use std::rc::Rc;

use dictionary::IndexedDictionary;
use gtk4::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

use relm4::SimpleComponent;

use crate::file::read_indexed_dictionaries;
use crate::Config;

mod entry;
use entry::Entry;

#[derive(Debug)]
struct App {
    config: Config,
    dictionaries: Vec<Box<dyn IndexedDictionary>>,
    entry: gtk::EntryBuffer,
    results: FactoryVecDeque<Entry>,
}

#[derive(Debug)]
enum AppInput {
    Search,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppInput;
    type Output = ();
    type Init = Config;

    view! {
        gtk4::Window{
            set_title: Some("국립국어대사전"),
            set_default_width: 500,
            set_default_height: 500,
            gtk4::Box{
                set_css_classes: &["content"],
                set_margin_all: 10,
                set_orientation: gtk4::Orientation::Vertical,
                set_spacing: 5,
                #[name="search_box"]
                gtk4::Entry{
                    set_placeholder_text: Some("검색"),
                    set_buffer: &model.entry,
                    connect_changed => AppInput::Search,
                    connect_activate => AppInput::Search,
                },

                gtk4::ScrolledWindow{
                    set_vexpand: true,
                    set_hscrollbar_policy: gtk4::PolicyType::Never,
                    #[local_ref]
                    results_box -> gtk4::Box{
                        set_orientation: gtk4::Orientation::Vertical,
                        set_spacing: 5,
                    },
                },

            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let results = FactoryVecDeque::builder().launch_default().detach();
        let model = App {
            dictionaries: read_indexed_dictionaries(&init.dictionary_directory),
            config: init,
            results,
            entry: gtk::EntryBuffer::default(),
        };
        let results_box = model.results.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>) {
        match input {
            AppInput::Search => {
                let query = &self.entry.text();
                let mut result_guard = self.results.guard();
                result_guard.clear();
                for i in &self.dictionaries {
                    for entry in i.search(&query) {
                        result_guard.push_back(entry.clone_box());
                    }
                }
            }
        }
    }
}

pub fn run(config: &Config) {
    let app = RelmApp::new("net.asdfer.dictionary");
    app.set_global_css(include_str!("style.css"));
    app.run::<App>(config.clone());
}
