use baka::setting::read_plugin;
use baka::{
    setting::{read_project_setting, read_root_setting},
};

fn main() {
    // TEST
    read_root_setting();
    read_project_setting();
    read_plugin();

}
