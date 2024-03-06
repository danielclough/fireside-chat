#[cfg(feature = "accelerate")]
extern crate accelerate_src;

#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

use std::borrow::BorrowMut;

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::generation::LogitsProcessor;

use tokenizers::tokenizer::Tokenizer;

use crate::llm::load_model::Model;

use super::load_model::Cache;

pub struct TextGeneration {
    model: Model,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        model: &Model,
        tokenizer: &Tokenizer,
        device: &Device,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        repeat_penalty: f32,
        repeat_last_n: usize,
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model: model.clone(),
            tokenizer: TokenOutputStream::new(tokenizer.clone()),
            device: device.clone(),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
        }
    }

    pub fn run(&mut self, prompt: &str, sample_len: usize, cache: Option<Cache>) -> Result<String> {
        use std::io::Write;
        self.tokenizer.clear();
        // Vec for response
        let mut response: Vec<String> = vec![];

        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();

        if tokens.is_empty() {
            anyhow::bail!("Empty prompts are not supported.")
        }
        // // Uncomment to print prompt tokens while generating.
        // for &t in tokens.iter() {
        //     if let Some(t) = self.tokenizer.next_token(t)? {
        //         // print prompt tokens
        //         print!("{t}");
        //     }
        // }
        // std::io::stdout().flush()?;

        let mut generated_tokens = 0usize;

        // Consider bailing action for Mistral 0 is placeholder.

        let eos_token = match self.model {
            // Model::Llama(_) => todo!(),
            // Model::Llama2(_) => todo!(),
            // Model::QLlama2(_) => todo!(),
            // Model::Mistral(_) => todo!(),
            // Model::QMistral(_) => todo!(),
            // Model::MixFormer(_) => todo!(),
            Model::Phi(_) | Model::QuantizedPhi(_) => {
                match self.tokenizer.get_token("<|endoftext|>") {
                    Some(token) => token,
                    None => anyhow::bail!("cannot find the endoftext token"),
                }
            }
            _ => self.tokenizer.get_token("</s>").unwrap_or(0),
        };

        let start_gen = std::time::Instant::now();
        let mut index_pos = 0;

        for index in 0..sample_len {
            let (context_size, context_index) = match &mut self.model {
                Model::Llama(_m) => (tokens.len(), 0),
                _ => {
                    if index > 0 {
                        (1, index_pos)
                    } else {
                        (tokens.len(), 0)
                    }
                }
            };
            let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            // println!("Input: \n{}\n\n",input);

            let logits = match &mut self.model {
                Model::Mistral(m) => m
                    .forward(&input, context_index)
                    .map_err(|x| println!("Error: {x}")),
                Model::QMistral(m) => m
                    .forward(&input, context_index)
                    .map_err(|x| println!("Error: {x}")),
                Model::MixFormer(m) => m.forward(&input).map_err(|x| println!("Error: {x}")),
                Model::Phi(m) => m.forward(&input).map_err(|x| println!("Error: {x}")),
                Model::QuantizedPhi(m) => m.forward(&input).map_err(|x| println!("Error: {x}")),
                Model::Llama(m) => {
                    let cache = match cache {
                        Some(Cache::LlamaCache(ref c)) => Some(c),
                        _ => None,
                    };
                    m.forward(
                        &input,
                        context_index,
                        cache.unwrap().to_owned().borrow_mut(),
                    )
                    .map_err(|x| println!("Error: {x}"))
                }
                Model::Llama2(m) => {
                    let cache = match cache {
                        Some(Cache::Llama2Cache(ref c)) => Some(c),
                        _ => None,
                    };
                    m.forward(
                        &input,
                        context_index,
                        cache.unwrap().to_owned().borrow_mut(),
                    )
                    .map_err(|x| println!("Error: {x}"))
                }
                Model::QLlama2(m) => {
                    let cache = match cache {
                        Some(Cache::Llama2Cache(ref c)) => Some(c),
                        _ => None,
                    };
                    m.forward(
                        &input,
                        context_index,
                        cache.unwrap().to_owned().borrow_mut(),
                    )
                    .map_err(|x| println!("Error: {x}"))
                }
                Model::NoModel(_) => Tensor::randn(0f32, 1., (3, 4), &self.device)
                    .map_err(|x| println!("Error: {x}")),
            };

            let logits = match &mut self.model {
                Model::Mistral(_) => logits
                    .unwrap()
                    .squeeze(0)?
                    .squeeze(0)?
                    .to_dtype(candle_core::DType::F32)?,
                _ => logits.unwrap().squeeze(0)?.squeeze(0)?,
            };
            let logits = if self.repeat_penalty == 1.0 {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            index_pos += ctxt.len();

            let next_token = self.logits_processor.sample(&logits)?;
            generated_tokens += 1;
            tokens.push(next_token);

            if next_token == eos_token {
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                // push response tokens to response vector.
                response.push(t);

                // // Uncomment if you want to watch generation of response tokens.
                // print!("{t}");
                // std::io::stdout().flush()?;
            }
        }

        let dt = start_gen.elapsed();
        if let Some(rest) = self.tokenizer.decode_rest().map_err(E::msg)? {
            print!("{rest}");
        }

        std::io::stdout().flush()?;
        tracing::debug!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            (generated_tokens as f64) / dt.as_secs_f64()
        );

        // convert response to string and return string
        let response_string = response.join("");

        Ok(response_string)
    }
}
