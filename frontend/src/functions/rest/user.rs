use common::database::user::{NewUser, UserForJson};
use gloo_net::http::Request;
use leptos::{spawn_local, Signal, SignalGet, SignalUpdate, WriteSignal};

use crate::functions::get_path::get_database_path;

pub async fn get_user_by_name(name: String, database_url: String) -> UserForJson {
    let route: String = format!("user/name/{}", name);
    let path = get_database_path(&route, database_url);

    let default = UserForJson {
        id: 0,
        name: "".to_string(),
        active: true,
    };
    if name.as_str() == "" {
        default
    } else {
        let response = Request::get(&path).send().await;
        if response.is_ok() {
            response.unwrap().json().await.unwrap_or(default).to_owned()
        } else {
            default
        }
    }
}
pub async fn _get_active_user(database_url: String) -> UserForJson {
    let path = get_database_path("users/active/true", database_url);

    let default_vec = vec![UserForJson {
        id: 0,
        name: "Database Error".to_string(),
        active: true,
    }];
    let response = Request::get(&path).send().await;
    if response.is_ok() {
        response
            .unwrap()
            .json()
            .await
            .unwrap_or(default_vec)
            .first()
            .unwrap()
            .to_owned()
    } else {
        default_vec.first().unwrap().to_owned()
    }
}
pub async fn check_user_exists(name: String, database_url: String) -> UserForJson {
    let slug = format!("user/name/{}", name);
    let path = get_database_path(&slug, database_url);

    Request::get(&path)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn post_new_user(set_args_for_json: NewUser, database_url: String) -> UserForJson {
    let path = get_database_path("user", database_url);

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

pub async fn patch_existing_user(
    set_args_for_json: UserForJson,
    database_url: String,
) -> UserForJson {
    let path = format!("user/id/{}", set_args_for_json.id);
    let path = get_database_path(&path, database_url);

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

pub fn switch_users(
    old_user: Signal<UserForJson>,
    set_user: WriteSignal<UserForJson>,
    new_user_name: String,
    database_url: Signal<String>,
) {
    let old_user_exists = old_user.get().id != 0;

    spawn_local(async move {
        // Update "old" user (active: false)
        if old_user_exists {
            _ = patch_existing_user(
                UserForJson {
                    id: old_user.get().id,
                    name: old_user.get().name,
                    active: false,
                },
                database_url.get(),
            )
            .await;
        };

        let new_user_exists = if new_user_name.clone().as_str() != "" {
            let new_user = check_user_exists(new_user_name.clone(), database_url.get()).await;
            if new_user.clone().id != 0 {
                Some(new_user)
            } else {
                None
            }
        } else {
            None
        };

        let new_user = if new_user_exists.is_some() {
            let new_user = UserForJson {
                id: new_user_exists.clone().unwrap().id,
                name: new_user_exists.unwrap().name,
                active: true,
            };
            patch_existing_user(new_user.clone(), database_url.get()).await
        } else {
            let new_user = NewUser {
                name: new_user_name,
                active: true,
            };
            post_new_user(new_user, database_url.get()).await
        };

        set_user.update(|x| *x = new_user);
    })
}
