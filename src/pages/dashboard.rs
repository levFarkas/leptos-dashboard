use leptos::*;

use crate::service::{aws_eks_versions::get_versions, project_handler::ProjectHandler};

#[component]
pub fn DashBoard() -> impl IntoView {
    let (_versions, set_versions) = create_signal(Vec::new());
    let (active_projects, set_active_projects) = create_signal(Vec::new());
    let (nearly_expired_projects, set_nearly_expired_projects) = create_signal(Vec::new());
    let (close_to_expired_projects, set_close_to_expired_projects) = create_signal(Vec::new());

    create_effect(move |_| {
        spawn_local(async move {
            get_versions().await.into_iter().for_each(|versions| {
                for version in versions.clone() {
                    set_versions.update(|v| v.push(version));
                }
            });
        });
    });
    create_effect(move |_| {
        spawn_local(async move {
            ProjectHandler::new()
                .get_active_projects()
                .await
                .into_iter()
                .for_each(|project| {
                    set_active_projects.update(|v| v.push(project));
                });
        });
    });

    create_effect(move |_| {
        spawn_local(async move {
            ProjectHandler::new()
                .get_nearly_expired_projects()
                .await
                .into_iter()
                .for_each(|project| {
                    set_nearly_expired_projects.update(|v| v.push(project));
                });
        });
    });

    create_effect(move |_| {
        spawn_local(async move {
            ProjectHandler::new()
                .get_close_to_expired_projects()
                .await
                .into_iter()
                .for_each(|project| {
                    set_close_to_expired_projects.update(|v| v.push(project));
                });
        });
    });

    view! {

        // <div class="flex flex-row justify-evenly items-center">
        // {
        //     move || names().into_iter()
        //         .map(|name| view! {
        //             <div class="flex-1 max-w-40 min-h-80 border-8 rounded-lg bg-clip-padding p-4">
        //             <div class="max-w-40 min-h-10 bg-cyan-700 rounded-lg padding-10 mb-6">
        //                 Header
        //             </div>
        //             <button
        //                 class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg min-h-24 w-28 break-words mb-2"
        //             >
        //             {name}
        //             </button>
        //             </div>
        //             }
        //         )
        //         .collect::<Vec<_>>()}

        // </div>


        <main class="my-0 mx-auto text-center">
            <div class="bg-gray-900 text-white font-sans">
                <div class="flex h-screen">
                    <div class="w-1/5 bg-blue-900 p-4">
                        <h1 class="text-lg font-bold mb-4">DevOps menu</h1>
                        <ul class="space-y-2">
                            <li><a href="#" class="block hover:bg-blue-800 py-2 px-4 rounded">Home</a></li>
                            <li><a href="#" class="block hover:bg-blue-800 py-2 px-4 rounded">Analytics</a></li>
                            <li><a href="#" class="block hover:bg-blue-800 py-2 px-4 rounded">Settings</a></li>
                            <li><a href="#" class="block hover:bg-blue-800 py-2 px-4 rounded">Logout</a></li>
                        </ul>
                    </div>
                    <div class="flex flex-1">
                        <div class="w-1/3 bg-gray-800 p-4">
                            <h2 class="text-lg font-bold mb-4">Active</h2>
                            <div class="bg-gray-700 p-4 rounded">
                            <For
                                each=active_projects
                                key=|state| state.name()
                                let:child
                            >
                                    <div class="flex items-center justify-between mb-2">
                                        <p>{child.name()        }</p>
                                        <button class="bg-blue-600 text-white px-2 py-1 rounded">Edit</button>
                                    </div>
                            </For>
                            </div>
                        </div>
                        <div class="w-1/3 bg-gray-800 p-4">
                            <h2 class="text-lg font-bold mb-4">About to expire</h2>
                            <div class="bg-gray-700 p-4 rounded">
                            <For
                                each=nearly_expired_projects
                                key=|state| state.name()
                                let:child
                            >
                                    <div class="flex items-center justify-between mb-2">
                                        <p>{child.name()        }</p>
                                        <button class="bg-blue-600 text-white px-2 py-1 rounded">Edit</button>
                                    </div>
                            </For>
                            </div>
                        </div>
                        <div class="w-1/3 bg-gray-800 p-4">
                            <h2 class="text-lg font-bold mb-4">Very close to expire</h2>
                            <div class="bg-gray-700 p-4 rounded">
                            <For
                                each=close_to_expired_projects
                                key=|state| state.name()
                                let:child
                            >
                                    <div class="flex items-center justify-between mb-2">
                                        <p>{child.name()        }</p>
                                        <button class="bg-blue-600 text-white px-2 py-1 rounded">Edit</button>
                                    </div>
                            </For>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>

    }
}
