use crate::llm::infer::start;
use crate::llm::inference_args::InferenceArgs;
use crate::llm::load_model::ModelTokenizerDevice;
use crate::llm::prompt_template::TemplateGenerator;
use crate::server::types::AppState;
use regex::Regex;

pub fn create_bot_msg(
    text_from_chat: String,
    conversation_history: &mut Vec<String>,
    model_tokenizer_device: ModelTokenizerDevice,
    inference_args: InferenceArgs,
) -> String {
    let history_split = conversation_history
        .iter()
        .rev()
        .take(2)
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // Create prompt

    let role = inference_args.role.as_ref().map(|r| r.to_string());
    let prompt = TemplateGenerator::generate(
        &text_from_chat,
        history_split,
        &model_tokenizer_device.template,
        inference_args.load_context,
        role,
    );

    println!(
        "\n{:?}\nPrompt:\n{}\n\n",
        &model_tokenizer_device.template, prompt
    );

    // Produce response from Mistral
    let bot_response = start(prompt, model_tokenizer_device.clone(), &inference_args)
        .unwrap_or("BOT_ERROR".to_string());

    // Parse Bot Response with Regex
    let regex = Regex::new(r#"(<\|im_end\|>|<\|/im_end\|>|\|im_end\||<\||assistant\n|\[INST\]|\[\/INST\]|\[inst\]|\[\/inst\]|<\/s>|<\/S>|\n\n### User:|\"\n\n|\nB:)"#)
        .expect("Invalid regex");
    let split_bot_response = regex
        .split(&bot_response)
        .take(1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    // add to conversation_history
    conversation_history.push(text_from_chat);
    conversation_history.push(split_bot_response.clone());

    // Send bot message
    split_bot_response.clone().to_string()
}

pub fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}
