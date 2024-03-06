#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use crate::llm::{inference_args::InferenceArgs, load_model::ModelTokenizerDevice};
use anyhow::Result;

use crate::llm::text_generation::TextGeneration;

pub fn start(
    prompt: String,
    model_tokenizer_device: ModelTokenizerDevice,
    inference_args: &InferenceArgs,
) -> Result<String> {
    // AVX, or Advanced Vector Extensions,
    //   an instruction set architecture extension for x86 microprocessors from Intel and AMD.
    // ARM NEON
    //   an advanced SIMD (Single Instruction, Multiple Data) architecture extension
    //   for the ARM Cortex-A series processors.
    // SIMD128
    //   a SIMD (Single Instruction, Multiple Data) with 128-bit registers.
    // F16C (aka. CVT16)
    //   an x86 instruction set architecture extension providing support for
    //   converting between half-precision and standard IEEE single-precision floating-point formats.
    tracing::debug!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle_core::utils::with_avx(),
        candle_core::utils::with_neon(),
        candle_core::utils::with_simd128(),
        candle_core::utils::with_f16c()
    );
    tracing::debug!(
        "temp: {:.2} repeat-penalty: {:.2} repeat-last-n: {}",
        inference_args.temperature.unwrap_or(0.0),
        inference_args.repeat_penalty,
        inference_args.repeat_last_n
    );

    let mut pipeline = TextGeneration::new(
        &model_tokenizer_device.model,
        &model_tokenizer_device.tokenizer.clone().unwrap(),
        &model_tokenizer_device.device,
        inference_args.seed,
        inference_args.temperature,
        inference_args.top_p,
        inference_args.repeat_penalty,
        inference_args.repeat_last_n,
    );

    // Run pipeline and return response
    let response = pipeline.run(
        &prompt,
        inference_args.sample_len,
        model_tokenizer_device.model_cache.clone(),
    )?;

    println!("{:?}", response);
    Ok(response)
}
