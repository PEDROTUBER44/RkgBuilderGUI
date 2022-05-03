use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/rocket/RkgBuilderGUI/window.ui")]
    pub struct RkgbuilderguiWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RkgbuilderguiWindow {
        const NAME: &'static str = "RkgbuilderguiWindow";
        type Type = super::RkgbuilderguiWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RkgbuilderguiWindow {}
    impl WidgetImpl for RkgbuilderguiWindow {}
    impl WindowImpl for RkgbuilderguiWindow {}
    impl ApplicationWindowImpl for RkgbuilderguiWindow {}
}

glib::wrapper! {
    pub struct RkgbuilderguiWindow(ObjectSubclass<imp::RkgbuilderguiWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RkgbuilderguiWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create RkgbuilderguiWindow")
    }
}
