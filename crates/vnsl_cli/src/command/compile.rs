use std::{fs, path::Path};

use anyhow::Result;
use vnsl_core::model::VnslScene;

use crate::model::CompileOptions;

pub fn compile_json(scenes: Vec<VnslScene>, options: CompileOptions) -> Result<()> {
    let output = options.output.clone().unwrap_or("./".to_string());
    let output_path = Path::new(&output);
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    let json_scenes: Result<Vec<(VnslScene, String)>> =
        scenes.into_iter().map(transform_scene_json).collect();

    for scene in json_scenes? {
        let name = scene.0.name;
        let output = output_path.join(format!("{}.json", name));
        fs::write(output, scene.1)?;
    }
    Ok(())
}

fn transform_scene_json(scene: VnslScene) -> Result<(VnslScene, String)> {
    let result = serde_json::to_string_pretty(&scene)?;
    Ok((scene, result))
}
