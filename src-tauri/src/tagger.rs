use std::collections::HashMap;

use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use specta::Type;
use wdtagger::{
    config::ModelConfig,
    file::{ConfigFile, HfFile, TagCSVFile, TaggerModelFile},
    pipeline::{TaggingPipeline, TaggingResult},
    processor::ImagePreprocessor,
    tagger::{Device, TaggerModel},
    tags::LabelTags,
};

pub fn get_device() -> Vec<Device> {
    return vec![Device::Cpu];
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ModelArgs {
    pub repo_id: String,
    pub model_file: String,
    pub config_file: String,
    pub tag_csv_file: String,
}

pub fn load_pipeline(model_args: ModelArgs) -> anyhow::Result<TaggingPipeline> {
    let device = get_device();

    // define files
    let model_file = TaggerModelFile::custom(&model_args.repo_id, None, &model_args.model_file);
    let config_file = ConfigFile::custom(&model_args.repo_id, None, &model_args.config_file);
    let tag_csv_file = TagCSVFile::custom(&model_args.repo_id, None, &model_args.tag_csv_file);

    // pre-download files
    let model_file_path = model_file.get()?;
    let config_file_path = config_file.get()?;
    let tag_csv_file_path = tag_csv_file.get()?;

    // load model
    TaggerModel::use_devices(device)?; // do once
    let model = TaggerModel::load(&model_file_path)?;
    let config = ModelConfig::load(&config_file_path)?;
    let preprocessor = ImagePreprocessor::from_config(&config)?;
    let label_tags = LabelTags::load(&tag_csv_file_path)?;

    let pipe = TaggingPipeline::new(model, preprocessor, label_tags, &0f32);

    Ok(pipe)
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SingleInferenceArgs {
    pub model_args: ModelArgs,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct InferenceResult {
    pub rating: HashMap<String, f32>,
    pub character: HashMap<String, f32>,
    pub general: HashMap<String, f32>,
}

impl From<TaggingResult> for InferenceResult {
    fn from(result: TaggingResult) -> Self {
        let rating = result.rating.into_iter().collect();
        let character = result.character.into_iter().collect();
        let general = result.general.into_iter().collect();

        InferenceResult {
            rating,
            character,
            general,
        }
    }
}

#[derive(Debug, thiserror::Error, Serialize, Type)]
pub enum TaggerError {
    #[error(transparent)]
    Io(
        #[serde(skip)]
        #[from]
        std::io::Error,
    ),

    #[error(transparent)]
    Image(
        #[serde(skip)]
        #[from]
        image::ImageError,
    ),

    #[error("Tagger error: {0}")]
    Tagger(String),
}

#[tauri::command]
#[specta::specta]
pub fn inference_single_image(args: SingleInferenceArgs) -> Result<InferenceResult, TaggerError> {
    let model_args = args.model_args;
    let pipe = load_pipeline(model_args).map_err(|e| TaggerError::Tagger(e.to_string()))?;

    let image_path = args.image_path;
    let img = image::open(&image_path)?;
    let result = pipe
        .predict(img)
        .map_err(|e| TaggerError::Tagger(e.to_string()))?;

    Ok(result.into())
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct BatchleInferenceArgs {
    pub model_args: ModelArgs,
    pub image_paths: Vec<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn inference_batch_images(
    args: BatchleInferenceArgs,
) -> Result<Vec<InferenceResult>, TaggerError> {
    let model_args = args.model_args;
    let pipe = load_pipeline(model_args).map_err(|e| TaggerError::Tagger(e.to_string()))?;

    tauri::async_runtime::spawn(async move {
        args.image_paths
            .iter()
            .map(|image_path| {
                tokio::task::block_in_place(|| {
                    let img = image::open(&image_path)?;
                    let result = pipe
                        .predict(img)
                        .map_err(|e| TaggerError::Tagger(e.to_string()))?;
                    Ok(result.into())
                })
            })
            .collect::<Result<Vec<_>, _>>()
    })
    .await
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_pipeline() {
        let model_args = ModelArgs {
            repo_id: "SmilingWolf/wd-swinv2-tagger-v3".to_string(),
            model_file: "model.onnx".to_string(),
            config_file: "config.json".to_string(),
            tag_csv_file: "selected_tags.csv".to_string(),
        };

        let result = load_pipeline(model_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inference_single_image() {
        let model_args = ModelArgs {
            repo_id: "SmilingWolf/wd-swinv2-tagger-v3".to_string(),
            model_file: "model.onnx".to_string(),
            config_file: "config.json".to_string(),
            tag_csv_file: "selected_tags.csv".to_string(),
        };

        let args = SingleInferenceArgs {
            model_args,
            image_path: "tests/sample_01.jpg".to_string(),
        };

        let result = inference_single_image(args);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_inference_batch_images() {
        let model_args = ModelArgs {
            repo_id: "SmilingWolf/wd-swinv2-tagger-v3".to_string(),
            model_file: "model.onnx".to_string(),
            config_file: "config.json".to_string(),
            tag_csv_file: "selected_tags.csv".to_string(),
        };

        let args = BatchleInferenceArgs {
            model_args,
            image_paths: vec![
                "tests/sample_01.jpg".to_string(),
                "tests/sample_01.jpg".to_string(),
                "tests/sample_01.jpg".to_string(),
            ],
        };

        let result = inference_batch_images(args).await;
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 3);
    }
}
