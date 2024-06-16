use leptos::*;
use leptos_dom::logging::console_log;

use crate::service::project_handler::ProjectHandler;
use crate::service::projects::Project;
use crate::service::storage::save_storage;

#[component]
pub fn CardComponent(
    child: Project,
    active_projects: ReadSignal<Vec<Project>>,
    nearly_expired_projects: ReadSignal<Vec<Project>>,
    close_to_expired_projects: ReadSignal<Vec<Project>>,
    set_active_projects: WriteSignal<Vec<Project>>,
    set_nearly_expired_projects: WriteSignal<Vec<Project>>,
    set_close_to_expired_projects: WriteSignal<Vec<Project>>,
) -> impl IntoView {
    let (edit_mode, set_edit_mode) = create_signal(0);
    let (edit_project, set_edit_project) = create_signal("".to_string());
    let child_stored = store_value(child.clone());
    view! {
        <div class="flex items-center justify-between mb-2">
            <p>{child_stored.with_value(|f| f.clone().name)}</p>

            <Show
                when=move || { edit_mode() == 1 }
                fallback=move|| view! { <button on:click=move |_| { set_edit_mode(1); }
                    class="bg-blue-600 text-white px-2 py-1 rounded">Edit</button> }
            >
            <input class="text-black" type="text" on:input=move |ev| {
                        // event_target_value is a Leptos helper function
                        // it functions the same way as event.target.value
                        // in JavaScript, but smooths out some of the typecasting
                        // necessary to make this work in Rust
                        set_edit_project.set(event_target_value(&ev));
                    }/>

            <button on:click=move |_| {

                let project = child_stored.with_value(|f| f.clone());
                let project2 = project.clone();
                let new_version = edit_project();
                spawn_local(async move {
                    save_storage(project, new_version).await;
                    console_log("Saved cucc");
                });
                set_edit_mode(0);

                if let Some(pos) = active_projects().iter().position(|i| i.name==project2.name){
                    set_active_projects.update(|v| if pos < v.len() {v.remove(pos);} );
                }


                if let Some(pos) = nearly_expired_projects().iter().position(|i| i.name==project2.name){
                    set_nearly_expired_projects.update(|v| if pos < v.len() {v.remove(pos);} );
                }


                if let Some(pos) = close_to_expired_projects().iter().position(|i| i.name==project2.name){
                    set_close_to_expired_projects.update(|v| if pos < v.len() {v.remove(pos);} );
                }

                create_effect(move |_| {
                    spawn_local(async move {
                        ProjectHandler::new()
                            .await
                            .get_active_projects()
                            .await
                            .into_iter()
                            .for_each(|project| {
                                if active_projects().iter().any(|i| i.name!=project.name){
                                    set_active_projects.update(|v| v.push(project));

                                }
                            });
                    });
                });

                create_effect(move |_| {
                    spawn_local(async move {
                        ProjectHandler::new()
                            .await
                            .get_nearly_expired_projects()
                            .await
                            .into_iter()
                            .for_each(|project| {
                                if nearly_expired_projects().iter().any(|i| i.name!=project.name){
                                    set_nearly_expired_projects.update(|v| v.push(project));
                                }

                            });
                    });
                });

                create_effect(move |_| {
                    spawn_local(async move {
                        ProjectHandler::new()
                            .await
                            .get_close_to_expired_projects()
                            .await
                            .into_iter()
                            .for_each(|project| {
                                if close_to_expired_projects().iter().any(|i| i.name!=project.name){
                                    set_close_to_expired_projects.update(|v| v.push(project));
                                }
                            });
                    });
                });
            }

            class="bg-blue-600 text-white px-2 py-1 rounded">Done</button>
            </Show>


        </div>
    }
}
