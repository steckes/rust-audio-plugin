use my_plugin::MyPlugin;
use nih_plug::prelude::*;

fn main() {
    nih_export_standalone::<MyPlugin>();
}
