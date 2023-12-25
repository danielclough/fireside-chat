enum TemplateFormat {
    ChatML,
    MistralInstruct,
    SolarInstruct,
    Amazon,
}
#[derive(Clone, Debug)]
pub struct TemplateGenerator {
    pub model_config: String,
}

impl TemplateGenerator {
    pub fn generate(last_message: &str, history_vec: Vec<String>, model_config: &Option<String>, load_context: bool) -> String {
        Self::generate_prompt(last_message, history_vec, Self::match_format(model_config.to_owned()), load_context)
    }
    fn match_format(model_config: Option<String>) -> Option<TemplateFormat> {
        let pt: Option<&str> = model_config.as_deref();
        match pt {
            Some("ChatML") => {
                Some(TemplateFormat::ChatML)
            },
            Some("Amazon") => {
                Some(TemplateFormat::Amazon)
            }
            Some("MistralInstruct") => Some(TemplateFormat::MistralInstruct),
            Some("SolarInstruct") => Some(TemplateFormat::SolarInstruct),
            _ => None,
        }
    }
    fn generate_prompt(
        last_message: &str,
        history_vec: Vec<String>,
        template_format: Option<TemplateFormat>,
        load_context: bool,
    ) -> String {
        let mut context = "".to_string();
        if load_context {
            context = get_context();
        }

        match template_format {
            Some(TemplateFormat::ChatML) => {
                let prompt_starter = format!("<|im_start|>system
{}
You are CandleChat, a large language model trained by Daniel Clough. Write out your reasoning step-by-step to be sure you get the right answers!
<|im_end|>
<|im_start|>user
Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.
<|im_end|>
<|im_start|>assistant
Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say 'I don't know'.
<|im_end|>
", context);

                if history_vec.is_empty() {
                    format!(
                        "{}
<|im_start|>user
{}
<|im_end|>
<|im_start|>assistant
",
                        prompt_starter, last_message
                    )
                } else {
                    format!(
                        "{}
<|im_start|>user
{}
<|im_end|>
<|im_start|>assistant
{}
<|im_end|>
<|im_start|>user
{}
<|im_end|>
<|im_start|>assistant",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::MistralInstruct) => {
                let prompt_starter = format!( "<s>
[INST]{}
You are CandleChat, a large language model trained by Daniel Clough. Write out your reasoning step-by-step to be sure you get the right answers! Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.[/INST]
Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say 'I don't know'.</s>", context);

                if history_vec.is_empty() {
                    format!("{}[INST]{}[/INST]", prompt_starter, last_message)
                } else {
                    format!(
                        "{}[INST]{}[/INST]{}[INST]{}[/INST]",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::SolarInstruct) => {
                let prompt_starter = format!("<s> ### User:
{}
You are CandleChat, a large language model trained by Daniel Clough. Write out your reasoning step-by-step to be sure you get the right answers! Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.

### Assistant:
Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say 'I don't know'.</s>

", context);

                if history_vec.is_empty() {
                    format!("{}
### User:
{}

### Assistant:", prompt_starter, last_message)
                } else {
                    format!(
                        "{}
### User:
{}

### Assistant:
{}

### User:
{}

### Assistant:",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::Amazon) => {
                let prompt_starter = "<|prompter|>You are CandleChat, a large language model trained by Daniel Clough. Write out your reasoning step-by-step to be sure you get the right answers! Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.</s><|assistant|>Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say 'I don't know'.</s>".to_string();

                if history_vec.is_empty() {
                    format!("{}<|prompter|>{}</s>", prompt_starter, last_message)
                } else {
                    format!(
                        "<|prompter|>{}</s><|assistant|>{}</s><|prompter|>{}</s><|assistant|>{}</s>",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            None => last_message.to_string(),
        }
    }
}

use std::fs;
use std::io::Read;

fn get_context() -> String {
    let directory = "./context";
    let mut file_contents: Vec<String> = Vec::new();

    for entry in fs::read_dir(directory).expect("Read dir") {
        let entry = entry.expect("File");
        let path = entry.path();

        println!("Reading:\n{:?}", path);
        
        if path.is_file() {
            if path.to_string_lossy().contains(".pdf") {
                let bytes = std::fs::read(path).expect("pdf works");
                let contents = pdf_extract::extract_text_from_mem(&bytes).expect("pdf works");
                file_contents.push(remove_special(contents));
            } else {
                let mut file = fs::File::open(&path).expect("Open File");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("File to string");
                file_contents.push(remove_special(contents));
            }
        }
    }

    file_contents.join("/n/n")
}

fn remove_special(content: String) -> String {
    content.replace(|c: char| !c.is_ascii_alphanumeric() && c != ' ' && c != '.' && c != ',' && c != '-' && c != '(' && c != ')' && c != '/' && c != '\n', "")
}