extern crate anyhow;

use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};

fn main() -> anyhow::Result<()> {
    //    Set-up model
    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPT2,
        max_length: Some(30),
        do_sample: false,
        num_beams: 1,
        temperature: 1.0,
        num_return_sequences: 1,
        ..Default::default()
    };
    let model = TextGenerationModel::new(generate_config)?;

    let input_context = "The dog";
    // let second_input_context = "The cat was";
    let output = model.generate(&[input_context], None);

    for sentence in output {
        println!("{sentence:?}");
    }
    Ok(())
}