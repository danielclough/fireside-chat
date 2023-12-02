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
    let history_split = conversation_history
        .iter()
        .rev()
        .take(2)
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // Create prompt
    let prompt_starter = "<s>[INST] Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.[/INST] Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say 'I don't know'.</s>".to_string();

    let prompt: String;

    println!("{:?}", history_split);

    if history_split.is_empty() {
        prompt = format!(
            "{}
[INST] {} [/INST] ",
            prompt_starter,
            text_from_chat.clone()
        )
    } else {
        prompt = format!(
            "{} [INST] {} [/INST] {} 
[INST] {} [/INST] ",
            prompt_starter,
            history_split[0],
            history_split[1],
            text_from_chat.clone()
        );
    };

    println!("{}", prompt);

    // Produce response from Mistral
    let bot_response = mistral(prompt, &model_tokenizer_device, &inference_args)
        .unwrap_or("Bot is away at the moment, try again later.".to_string());

    // Parse Bot Response with Regex
    let regex = Regex::new(r"(\[INST\]|\[\/INST\]|\[inst\]|\[\/inst\]|<\\s>|<\\S>|\\n)")
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
    format!("Bot: {}", split_bot_response.clone())
}
