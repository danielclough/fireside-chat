use anyhow::Result;

use super::types::{ InferenceArgs, TextGeneration, ModelTokenizerDevice };

pub fn mistral(prompt: String, model_args: &ModelTokenizerDevice) -> Result<String> {
    // Load Config for each run.
    // Consider moving if config should be loaded when the project starts.
    let config_string = std::fs
        ::read_to_string("./config_inference.yaml")
        .expect("Load config_inference.yaml");
    let inference_args: InferenceArgs = serde_yaml
        ::from_str(config_string.as_str())
        .expect("config_inference.yaml to struct");

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
        model_args.model.clone(),
        model_args.tokenizer.clone(),
        inference_args.seed,
        inference_args.temperature,
        inference_args.top_p,
        inference_args.repeat_penalty,
        inference_args.repeat_last_n,
        &model_args.device
    );

    // Run pipeline and return response
    let response = pipeline.run(&prompt, inference_args.sample_len)?;
    Ok(response)
}
