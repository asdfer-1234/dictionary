use gtk4::prelude::*;
use relm4::prelude::*;

use relm4::factory::FactoryComponent;

#[derive(Debug)]
pub struct Entry {
    entry: Box<dyn dictionary::Entry>,
}

#[relm4::factory(pub)]
impl FactoryComponent for Entry {
    type Init = Box<dyn dictionary::Entry>;
    type ParentWidget = gtk::Box;
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Frame{

            gtk::Box{
                set_margin_all: 10,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                #[name(word_label)]
                gtk::Label{
                    set_css_classes: &["word"],
                    set_label: &self.entry.word(),
                    set_halign: gtk::Align::Fill,
                    set_xalign: 0.0,
                },

                #[name(etymology_label)]
                gtk::Label{
                    set_css_classes: &["etymology"],
                    set_label: &match &self.entry.etymologies(){
                        Some(x) => x.to_string(),
                        None => String::from(""),
                    },
                    set_halign: gtk::Align::Fill,
                    set_xalign: 0.0,
                },


                #[name(definition_label)]
                gtk::Label{
                    set_css_classes: &["definition"],
                    set_label: match &self.entry.definition(){
                        Some(x) => x,
                        None => "Definition not provided",
                    },
                    set_halign: gtk::Align::Fill,
                    set_xalign: 0.0,
                    set_wrap: true,
                },
            }
        },
    }

    fn init_model(entry: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { entry }
    }
}
