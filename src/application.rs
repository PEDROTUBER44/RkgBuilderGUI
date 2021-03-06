use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::RkgbuilderguiWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RkgbuilderguiApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for RkgbuilderguiApplication {
        const NAME: &'static str = "RkgbuilderguiApplication";
        type Type = super::RkgbuilderguiApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for RkgbuilderguiApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for RkgbuilderguiApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = RkgbuilderguiWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for RkgbuilderguiApplication {}
    }

glib::wrapper! {
    pub struct RkgbuilderguiApplication(ObjectSubclass<imp::RkgbuilderguiApplication>)
        @extends gio::Application, gtk::Application, 
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RkgbuilderguiApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create RkgbuilderguiApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("rkgbuildergui")
            .version(VERSION)
            .authors(vec!["PEDROTUBER44".into()])
            .build();

        dialog.present();
    }
}
