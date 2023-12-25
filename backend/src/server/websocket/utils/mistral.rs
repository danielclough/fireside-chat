use crate::mistral::infer::mistral;
use crate::mistral::types::inference_args::InferenceArgs;
use crate::mistral::types::load_model::ModelTokenizerDevice;
use crate::mistral::types::prompt_template::TemplateGenerator;
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
    let prompt = TemplateGenerator::generate(&text_from_chat, history_split, &model_tokenizer_device.model_config, inference_args.load_context);

    println!("\nPrompt:\n{}\n\n", prompt);

    // Produce response from Mistral
    let bot_response = mistral(prompt, &model_tokenizer_device, &inference_args)
        .unwrap_or("Bot is away at the moment, try again later.".to_string());

    // Parse Bot Response with Regex
    let regex = Regex::new(r"(<\|im_end\|>|<\|/im_end\|>|\|im_end\||<\||assistant\n|\[INST\]|\[\/INST\]|\[inst\]|\[\/inst\]|<\/s>|<\/S>|\n\n### User:)")
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
    format!("{}", split_bot_response.clone())
}
