use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TopBarProps {
    pub file_explorer_open: UseStateHandle<bool>,
    pub selected_mod: UseStateHandle<Option<usize>>,
}

#[function_component(TopBar)]
pub fn top_bar(props: &TopBarProps) -> Html {
    let toggle_file_explorer = {
        let file_explorer_open = props.file_explorer_open.clone();
        let selected_mod = props.selected_mod.clone();
        move |_| {
            file_explorer_open.set(!*file_explorer_open);
            selected_mod.set(None);
        }
    };
    html! {
        <div class="top-bar">
            if *props.file_explorer_open {
                <div class="top-bar-element" onclick={toggle_file_explorer}>{"Back"}</div>
            } else {
                <div class="top-bar-element" onclick={toggle_file_explorer}>{"Add Mod"}</div>

                if let Some(_mod_index) = *props.selected_mod {
                    <div class="top-bar-element">{"Remove Mod"}</div>
                } else {
                    <div class="top-bar-element top-bar-element-disabled">{"Remove Mod"}</div>
                }
            }
        </div>
    }
}