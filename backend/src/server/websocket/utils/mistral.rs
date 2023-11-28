use crate::mistral::infer::mistral;
use crate::mistral::types::inference_args::InferenceArgs;
use crate::mistral::types::load_model::ModelTokenizerDevice;
use regex::Regex;

pub fn create_bot_msg(
    text_from_chat: String,
    conversation_history: &mut Vec<String>,
    model_tokenizer_device: ModelTokenizerDevice,
    inference_args: InferenceArgs,
) -> String {
    // Create prompt
    let prompt = format!(
        "<s>[INST] Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.[/INST] {} </s>
[INST] {} [/INST]",
        conversation_history
            .iter()
            .rev()
            .take(2)
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        text_from_chat.clone()
    );

    tracing::debug!("{}", prompt);

    // Produce response from Mistral
    let bot_response = mistral(prompt, &model_tokenizer_device, &inference_args)
        .unwrap_or("Bot is away at the moment, try again later.".to_string());

    // Parse Bot Response with Regex
    let regex = Regex::new(r"(\[INST\]|\[\/INST\]|\[inst\]|\[\/inst\])").expect("Invalid regex");
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
    format!("Bot: {}", split_bot_response.clone())
}
