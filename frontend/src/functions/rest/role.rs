use common::llm::inference::{InferenceArgsForInput, InferenceArgsForJson};
use gloo_net::http::Request;

use common::llm::role_list::RoleList;
use crate::functions::get_path::get_llm_path;

pub async fn patch_role_list(inference_args: InferenceArgsForJson, ipv4: String) -> InferenceArgsForInput {
    let path = get_llm_path("role-list",ipv4);

    Request::patch(&path)
        .header("Content-Type", "application/json")
        .json(&inference_args)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn get_role_list(ipv4: String) -> RoleList {
    let path = get_llm_path("role-list",ipv4);

    let response = Request::get(&path).send().await;

    if response.is_ok() {
        response
            .expect("Load role list from API")
            .json()
            .await
            .unwrap()
    } else {
        RoleList { human: vec![], computer: vec![]  }
    }
}
