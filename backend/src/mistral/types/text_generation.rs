use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::generation::LogitsProcessor;

use tokenizers::tokenizer::Tokenizer;

use super::load_model::Model;

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

    pub fn run(&mut self, prompt: &str, sample_len: usize) -> Result<String> {
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

        // // Uncomment to print prompt tokens while generating.
        // for &t in tokens.iter() {
        //     if let Some(t) = self.tokenizer.next_token(t)? {
        //         // print prompt tokens
        //         print!("{t}");
        //     }
        // }
        // std::io::stdout().flush()?;

        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("</s>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the </s> token"),
        };
        let start_gen = std::time::Instant::now();
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            let logits = match &mut self.model {
                Model::Mistral(m) => m.forward(&input, start_pos)?,
                Model::Quantized(m) => m.forward(&input, start_pos)?,
            };
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
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

            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;
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
