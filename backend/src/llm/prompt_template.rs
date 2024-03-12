enum TemplateFormat {
    ChatML,
    MistralInstruct,
    SolarInstruct,
    Teknium,
    TekniumOld,
    Amazon,
    Zephyr,
    PhiQA,
    PhiChat,
    PhiCode,
    ShareGPT,
    Alpaca,
}
#[derive(Clone, Debug)]
pub struct TemplateGenerator {
    pub template_format: String,
}

impl TemplateGenerator {
    pub fn generate(
        last_message: &str,
        history_vec: Vec<String>,
        template_format: &Option<String>,
        load_context: bool,
        role: Option<String>,
    ) -> String {
        Self::generate_prompt(
            last_message,
            history_vec,
            Self::match_format(template_format.to_owned()),
            load_context,
            role,
        )
    }
    fn match_format(template_format: Option<String>) -> Option<TemplateFormat> {
        let pt: Option<&str> = template_format.as_deref();
        match pt {
            Some("ChatML") => Some(TemplateFormat::ChatML),
            Some("MistralInstruct") => Some(TemplateFormat::MistralInstruct),
            Some("SolarInstruct") => Some(TemplateFormat::SolarInstruct),
            Some("Teknium") => Some(TemplateFormat::Teknium),
            Some("TekniumOld") => Some(TemplateFormat::TekniumOld),
            Some("Amazon") => Some(TemplateFormat::Amazon),
            Some("Zephyr") => Some(TemplateFormat::Zephyr),
            Some("PhiQA") => Some(TemplateFormat::PhiQA),
            Some("PhiChat") => Some(TemplateFormat::PhiChat),
            Some("PhiCode") => Some(TemplateFormat::PhiCode),
            Some("ShareGPT") => Some(TemplateFormat::ShareGPT),
            Some("Alpaca") => Some(TemplateFormat::Alpaca),
            _ => None,
        }
    }
    fn generate_prompt(
        last_message: &str,
        history_vec: Vec<String>,
        template_format: Option<TemplateFormat>,
        load_context: bool,
        role: Option<String>,
    ) -> String {
        let mut context = String::new();
        let template_primer_prompt = r#"Write out your reasoning step-by-step to be sure you get the right answers! Always respond with concise messages with correct grammar. Avoid garbled content and words that run into one another. If you don't know the answer to a question say "I don't know.""#;
        let template_primer_response = r#"Understood! I will always respond with concise messages and correct grammar. If I don't know the answer to a question, I will say "I don't know.""#;
        // Load txt or pdf from .config/context/
        if load_context {
            context = get_context();
        }
        // Add role
        if role.is_some() && (!role.clone().unwrap().is_empty()) {
            let role = role.unwrap();

            let role_list = get_default_list();
            let mut current = role_list
                .human
                .iter()
                .filter(|x| x.role == role)
                .collect::<Vec<&RoleListEntry>>();
            if current.is_empty() {
                current = role_list
                    .computer
                    .iter()
                    .filter(|x| x.role == role)
                    .collect::<Vec<&RoleListEntry>>();
            }
            context += &current[0].prompt;
        } else {
            context += "You are Fire, a chat service produced by Daniel Clough.";
        }

        match template_format {
            Some(TemplateFormat::ChatML) => {
                let prompt_starter = format!(
                    "<|im_start|>system
{}
<|im_end|>
<|im_start|>user
{}
<|im_end|>
<|im_start|>assistant
{}
<|im_end|>",
                    context, template_primer_prompt, template_primer_response
                );

                if history_vec.is_empty() {
                    format!(
                        "{}<|im_start|>user
{}
<|im_end|>
<|im_start|>assistant",
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
                let prompt_starter = format!(
                    "<s>[INST]{}{}[/INST]
{}</s>",
                    context, template_primer_prompt, template_primer_response
                );

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
                let prompt_starter = format!(
                    "<s>### User:
{} {}
### Assistant:
{}.</s>",
                    context, template_primer_prompt, template_primer_response
                );

                if history_vec.is_empty() {
                    format!(
                        "{}
### User:
{}
### Assistant:",
                        prompt_starter, last_message
                    )
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
            Some(TemplateFormat::TekniumOld) => {
                if history_vec.is_empty() {
                    format!(
                        "USER: {}
ASSISTANT:",
                        last_message
                    )
                } else {
                    format!(
                        "USER: {}
ASSISTANT: {}
USER: {}
ASSISTANT:",
                        history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::Teknium) => {
                let prompt_starter = format!(
                    "{}
USER: {}
ASSISTANT: {}",
                    context, template_primer_prompt, template_primer_response
                );

                if history_vec.is_empty() {
                    format!(
                        "{}
USER: {}
ASSISTANT:",
                        prompt_starter, last_message
                    )
                } else {
                    format!(
                        "{}
USER: {}
ASSISTANT: {}
USER: {}
ASSISTANT:",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::Amazon) => {
                let prompt_starter = format!(
                    "<|prompter|>{}{}</s><|assistant|>{}</s>",
                    context, template_primer_prompt, template_primer_response
                );

                if history_vec.is_empty() {
                    format!("{}<|prompter|>{}</s>", prompt_starter, last_message)
                } else {
                    format!(
                        "{}<|prompter|>{}</s><|assistant|>{}</s><|prompter|>{}</s>",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::Zephyr) => {
                let prompt_starter = format!(
                    "<|system|>
{}</s>
<|user|>
{}</s>
<|assistant|>
{}</s>",
                    context, template_primer_prompt, template_primer_response
                );

                if history_vec.is_empty() {
                    format!(
                        "{}
<|user|>
{}</s>
<|assistant|>",
                        prompt_starter, last_message
                    )
                } else {
                    format!(
                        "{}
<|user|>
{}</s>
<|assistant|>
{}</s>
<|user|>
{}</s>
<|assistant|>",
                        prompt_starter, history_vec[0], history_vec[1], last_message
                    )
                }
            }
            Some(TemplateFormat::PhiQA) => {
                format!("Instruct: {}\nOutput:", last_message)
            }
            Some(TemplateFormat::PhiChat) => {
                format!("Alice: {}\nBob:", last_message)
            }
            Some(TemplateFormat::PhiCode) => {
                format!("\"\"\"\n{}\n\"\"\"\n", last_message)
            }
            Some(TemplateFormat::ShareGPT) => {
                format!("USER: {}\nASSISTANT:", last_message)
            }
            Some(TemplateFormat::Alpaca) => {
                format!("### Instruction:\n{}\n### Response:", last_message)
            }
            None => last_message.to_string(),
        }
    }
}

use std::fs;
use std::io::Read;

use common::llm::role_list::RoleListEntry;

use crate::server::rest::role_list::get_default_list;
use crate::utilities::config_path::context_file_dir;

fn get_context() -> String {
    let directory = context_file_dir();
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

    file_contents.join("/n")
}

fn remove_special(content: String) -> String {
    content.replace(
        |c: char| {
            !c.is_ascii_alphanumeric()
                && c != ' '
                && c != '.'
                && c != ','
                && c != '-'
                && c != '('
                && c != ')'
                && c != '/'
                && c != '\n'
        },
        "",
    )
}
