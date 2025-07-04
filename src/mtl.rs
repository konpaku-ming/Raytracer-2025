use crate::texture::MappedTexture;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct MtlInfo {
    pub name: String,
    pub map_kd: Option<String>,   // 漫反射贴图
    pub map_bump: Option<String>, // 法线贴图
    pub map_d: Option<String>,    // Alpha 贴图
}

pub fn parse_mtl_file(path: &str) -> HashMap<String, MtlInfo> {
    let file = File::open(path).expect("无法打开MTL文件");
    let reader = BufReader::new(file);

    let mut materials = HashMap::new();
    let mut current = MtlInfo {
        name: String::new(),
        map_kd: None,
        map_bump: None,
        map_d: None,
    };

    for line in reader.lines().filter_map(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.get(0).copied() {
            Some("newmtl") => {
                if !current.name.is_empty() {
                    materials.insert(current.name.clone(), current);
                }
                current = MtlInfo {
                    name: tokens[1].to_string(),
                    map_kd: None,
                    map_bump: None,
                    map_d: None,
                };
            }
            Some("map_Kd") => current.map_kd = tokens.get(1).map(|s| s.to_string()),
            Some("map_bump") | Some("bump") => {
                current.map_bump = tokens.get(1).map(|s| s.to_string())
            }
            Some("map_d") => current.map_d = tokens.get(1).map(|s| s.to_string()),
            _ => {}
        }
    }

    if !current.name.is_empty() {
        materials.insert(current.name.clone(), current);
    }

    materials
}

pub fn make_mapped_texture_from_mtl(material: &MtlInfo) -> MappedTexture {
    MappedTexture::new(
        material.map_kd.as_deref().unwrap_or("default_diffuse.png"),
        material.map_bump.as_deref(),
        material.map_d.as_deref(),
    )
}
