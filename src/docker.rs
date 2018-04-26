use serde_json;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use tar::Archive;
use tempdir::TempDir;

pub fn decompress_and_squash(docker_archive_path: PathBuf) -> Result<(), io::Error> {
    println!("Decompressing the image...");

    let docker_archive_file = File::open(docker_archive_path)?;
    let mut docker_archive = Archive::new(docker_archive_file);

    let tmp_archive_dir = TempDir::new("archive")?;
    docker_archive.unpack(&tmp_archive_dir)?;
    println!("Decompressing the image done!");

    let layers = get_layers_from_manifest(
        tmp_archive_dir.path().join("manifest.json"),
        PathBuf::from(tmp_archive_dir.path()),
    )?;
    println!("{:?}", layers);

    let root_dir = TempDir::new("rootfs")?;
    squash_layers(layers, root_dir.path().to_path_buf())?;

    Ok(())
}

type Manifest = Vec<Layer>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Layer {
    //config: String,
    // repo_tags: Vec<String>,
    layers: Vec<String>,
}

fn get_layers_from_manifest(
    manifest_path: PathBuf,
    tmp_path: PathBuf,
) -> Result<(Vec<PathBuf>), io::Error> {
    let mut f = File::open(manifest_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let manifest: Manifest = serde_json::from_str(&contents)?;

    let mut layer_paths: Vec<PathBuf> = Vec::with_capacity(manifest.len());

    for layers in manifest.iter() {
        for layer in layers.layers.iter() {
            layer_paths.push(tmp_path.join(layer));
        }
    }

    // Reverse the Vec
    layer_paths.reverse();

    Ok(layer_paths)
}

fn squash_layers(layers: Vec<PathBuf>, root_dir: PathBuf) -> Result<(), io::Error> {
    println!("Squashing the layers...");

    for layer_path in layers.iter() {
        let layer_file = File::open(layer_path)?;
        let mut layer = Archive::new(layer_file);
        layer.unpack(&root_dir)?;
    }
    Ok(())
}
