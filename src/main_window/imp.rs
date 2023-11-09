
use crate::first_menu_bar::FirstMenuBar;
use gtk::{
    glib::{self},
    subclass::prelude::*,
    ShortcutTrigger, Shortcut
};

use gtk4 as gtk;

/// The private struct, which can hold widgets and other data.
#[derive(Debug, gtk::CompositeTemplate)]
#[template(file = "main_window.ui")]
pub struct MainWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub menubar: TemplateChild<FirstMenuBar>,
    #[template_child]
    pub add_ss: TemplateChild<gtk::Button>,
    #[template_child]
    pub image: TemplateChild<gtk::Image>,

    pub shortcut_screen: Shortcut
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            menubar: Default::default(),
            add_ss: Default::default(),
            image: Default::default(),
            shortcut_screen: Shortcut::new(
                ShortcutTrigger::parse_string("<Ctrl>a"),
                Some(gtk::NamedAction::new("win.new_screen"))
            )
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct UtilityCallbacks {}

#[gtk::template_callbacks(functions)]
impl UtilityCallbacks {}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().delay_action_setup();
        self.obj().screen_action_setup("<Ctrl>a");
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}