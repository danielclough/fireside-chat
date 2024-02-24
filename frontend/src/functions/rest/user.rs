use common::database::user::{NewUser, UserForJson};
use gloo_net::http::Request;
use leptos::{spawn_local, Signal, SignalGet, SignalSet, WriteSignal};

use crate::functions::get_path::get_database_path;

pub async fn get_active_user() -> UserForJson {
    let path = get_database_path("users/active/true");
    Request::get(&path)
        .send()
        .await
        .expect("Load active user from API")
        .json()
        .await
        .unwrap_or(vec![UserForJson {
            id: 0,
            name: "None".to_string(),
            active: true,
        }])
        .first()
        .unwrap()
        .to_owned()
}
pub async fn check_user_exists(name: String) -> UserForJson {
    let slug = format!("user/name/{}", name);
    let path = get_database_path(&slug);

    Request::get(&path)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn post_new_user(set_args_for_json: NewUser) -> UserForJson {
    let path = get_database_path("user");

    Request::post(&path)
        .header("Content-Type", "application/json")
        .json(&set_args_for_json)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn patch_existing_user(set_args_for_json: UserForJson) -> UserForJson {
    let path = format!("user/id/{}", set_args_for_json.id);
    let path = get_database_path(&path);

    Request::patch(&path)
        .header("Content-Type", "application/json")
        .json(&set_args_for_json)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}


pub fn switch_users(user: Signal<UserForJson>, set_user: WriteSignal<UserForJson>, input_string: String) -> UserForJson {
    // Create User structs
    let old_user = user.get();
    spawn_local(async move {
        // Update "old" user (active: false)
        if old_user.clone().id != 0 {
            _ = patch_existing_user(old_user.clone()).await;
        };

        spawn_local(async move {
            // Update "new" user  (active: true OR create new)
            let new_user_exists: UserForJson = if old_user.clone().id != 0 {
                    check_user_exists(input_string.clone()).await
            } else {
                old_user.clone()
            };

            let new_user = if new_user_exists.id == 0 {
                let new_user = NewUser {
                    name: input_string,
                    active: true,
                };
                post_new_user(new_user).await
            } else {
                let new_user = UserForJson {
                    id: new_user_exists.id,
                    name: new_user_exists.name,
                    active: true,
                };
                patch_existing_user(new_user).await
            };
            // set user
            spawn_local(async move {
                set_user.set(new_user);
            });
        });
    });
    user.get()
}