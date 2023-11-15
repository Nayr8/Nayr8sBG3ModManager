use std::path::PathBuf;
use yew::prelude::*;
use models::FileEntry;
use crate::bindings::FileBrowser;
use location::FileExplorerLocation;
use files::Files;
use current_file::CurrentFile;
use file_nav::FileNav;
use add_new_mod::AddNewModMenu;
use crate::components::Button;

mod location;
mod files;
mod current_file;
mod add_new_mod;
mod file_nav;

#[derive(Properties, PartialEq)]
pub struct FileExplorerProps {
    pub file_explorer_open: UseStateHandle<bool>,
    pub selected_mod: UseStateHandle<Option<usize>>,
}
#[function_component(FileExplorer)]
pub fn file_explorer(props: &FileExplorerProps) -> Html {
    let current_path = use_state(|| std::sync::Arc::new(PathBuf::new()));
    let current_entries = use_state(|| Vec::<FileEntry>::new());
    let navigation_enabled_state = use_state(|| (false, false));

    use_effect_with_deps(|navigation_enabled_state| {
        FileBrowser::get_navigation_enabled_state(navigation_enabled_state.clone());
    }, navigation_enabled_state.clone());

    let current_file: UseStateHandle<Option<FileEntry>> = use_state(|| None);

    use_effect_with_deps(|(current_path, current_entries)| {
        FileBrowser::read_current_dir_into((*current_path).clone(), (*current_entries).clone());
    }, (current_path.clone(), current_entries.clone()).clone());

    let current_directory_str = current_path.to_string_lossy().to_string();

    let add_mod_menu = use_state(|| false);

    if *add_mod_menu {
        html! {
            <AddNewModMenu current_file={current_file.clone()} add_mod_menu={add_mod_menu.clone()} file_explorer_open={props.file_explorer_open.clone()} />
        }
    } else {
        let cancel = {
            let file_explorer_open = props.file_explorer_open.clone();
            let selected_mod = props.selected_mod.clone();
            move |_: MouseEvent| {
                file_explorer_open.set(false);
                selected_mod.set(None);
            }
        };
        let go_back = {
            let current_path = current_path.clone();
            let current_entries = current_entries.clone();
            let navigation_enabled_state = navigation_enabled_state.clone();
            move |_: MouseEvent| {
                FileBrowser::go_back();
                FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
                FileBrowser::get_navigation_enabled_state(navigation_enabled_state.clone());
            }
        };
        let go_forward = {
            let current_path = current_path.clone();
            let current_entries = current_entries.clone();
            let navigation_enabled_state = navigation_enabled_state.clone();
            move |_: MouseEvent| {
                FileBrowser::go_forward();
                FileBrowser::read_current_dir_into(current_path.clone(), current_entries.clone());
                FileBrowser::get_navigation_enabled_state(navigation_enabled_state.clone());
            }
        };
        html! {
            <div class="file-explorer">
                <div class="file-nav-top">
                    <Button disabled={!navigation_enabled_state.0} onclick={go_back}>
                        if navigation_enabled_state.0 { <img src="public/file_browser_back.png" /> }
                        else { <img src="public/file_browser_back_disabled.png" /> }
                    </Button>
                    <Button disabled={!navigation_enabled_state.1} onclick={go_forward}>
                        if navigation_enabled_state.1 { <img src="public/file_browser_forward.png" /> }
                        else { <img src="public/file_browser_forward_disabled.png" /> }
                    </Button>
                </div>
                <FileNav
                    current_path={current_path.clone()}
                    current_entries={current_entries.clone()}
                    navigation_enabled_state={navigation_enabled_state.clone()} />
                <div class="file-cancel">
                    <Button onclick={cancel}>{"Return to mod list"}</Button>
                </div>
                <FileExplorerLocation current_directory={current_directory_str} />
                <Files
                    current_path={current_path.clone()}
                    current_entries={current_entries.clone()}
                    current_file={current_file.clone()}
                    navigation_enabled_state={navigation_enabled_state.clone()} />
                <CurrentFile
                    current_file={current_file.clone()}
                    add_mod_menu={add_mod_menu.clone()} />
            </div>
        }
    }
}
