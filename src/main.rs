#[macro_use]
extern crate glib;
use cairo::Context;
use gio::prelude::*;
use gio::subclass::prelude::*;
use glib::subclass;
use glib::translate::*;
use gtk::subclass::prelude::*;
use gtk::CellRendererState;

fn main() {
    let _renderer = MyRenderer::new();
    println!("Hello, world!");
}

pub struct MyRendererPrivate {}

glib_wrapper! {
    pub struct MyRenderer(
        Object<subclass::simple::InstanceStruct<MyRendererPrivate>,
        subclass::simple::ClassStruct<MyRendererPrivate>,
        MyRendererClass>)
        @extends gtk::CellRenderer;

    match fn {
        get_type => || MyRendererPrivate::get_type().to_glib(),
    }
}

impl MyRenderer {
    pub fn new() -> Self {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create MyRenderer")
            .downcast()
            .expect("Created MyRenderer is of wrong type")
    }
}

// https://gitlab.gnome.org/GNOME/rhythmbox/-/blob/master/widgets/rb-cell-renderer-rating.c
// https://github.com/gtk-rs/examples/blob/master/src/bin/basic_subclass.rs
impl ObjectImpl for MyRendererPrivate {
    glib_object_impl!();
}
impl ObjectSubclass for MyRendererPrivate {
    const NAME: &'static str = "MyRenderer";
    type ParentType = gtk::CellRenderer;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib::glib_object_subclass!();
}
impl CellRendererImpl for MyRendererPrivate {
    fn render<P: IsA<gtk::Widget>>(
        &self,
        _renderer: &gtk::CellRenderer,
        _cr: &Context,
        _widget: &P,
        _background_area: &gtk::Rectangle,
        _cell_area: &gtk::Rectangle,
        _flags: CellRendererState,
    ) {
        println!("render!!");
    }
}
