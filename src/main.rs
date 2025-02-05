use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

const PTRESPACK_META_URL: &str = "https://pgres4pt.realtvop.top";

#[derive(Debug, Deserialize)]
struct PTRespackMeta {
    name: String,
    author: String,
    // includes_hit_songs: bool,
    res: HashMap<String, String>,
}

async fn fetch_meta(url: &str) -> Result<PTRespackMeta, Error> {
    let client = reqwest::Client::new();
    
    let response = client.get(url)
        .send()
        .await?;
    
    let meta = response.json::<PTRespackMeta>().await?;
    Ok(meta)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum ImageResType {
    HitFX,
    Tap,
    TapHL,
    HoldEnd,
    Hold,
    HoldHL,
    HoldHead,
    HoldHeadHL,
    Drag,
    DragHL,
    Flick,
    FlickHL,
}
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum AudioResType {
    TapHitSound,
    DragHitSound,
    FlickHitSound,
}
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum ResType {
    Image(ImageResType),
    Audio(AudioResType),
}
const IMAGE_RES_MAPPINGS: [(&[&str], ImageResType); 12] = [
    (&["clickraw", "clickraw.png"], ImageResType::HitFX),
    (&["tap", "tap.png"], ImageResType::Tap),
    (&["taphl", "taphl.png"], ImageResType::TapHL),
    (&["holdend", "holdend.png"], ImageResType::HoldEnd),
    (&["hold", "hold.png"], ImageResType::Hold),
    (&["holdhl", "holdhl.png"], ImageResType::HoldHL),
    (&["holdhead", "holdhead.png"], ImageResType::HoldHead),
    (&["holdheadhl", "holdheadhl.png"], ImageResType::HoldHeadHL),
    (&["drag", "drag.png"], ImageResType::Drag),
    (&["draghl", "draghl.png"], ImageResType::DragHL),
    (&["flick", "flick.png"], ImageResType::Flick),
    (&["flickhl", "flickhl.png"], ImageResType::FlickHL),
];

const AUDIO_RES_MAPPINGS: [(&[&str], AudioResType); 3] = [
    (&["hitsong0", "hitsong0.ogg"], AudioResType::TapHitSound),
    (&["hitsong1", "hitsong1.ogg"], AudioResType::DragHitSound),
    (&["hitsong2", "hitsong2.ogg"], AudioResType::FlickHitSound),
];

fn res_name_parser(res: &HashMap<String, String>) -> HashMap<ResType, String> {
    let mut res_urls = HashMap::<ResType, String>::new();
    
    for (name, url) in res {
        let name_lower = name.to_lowercase();
        
        if let Some((_, img_type)) = IMAGE_RES_MAPPINGS
            .iter()
            .find(|(names, _)| names.contains(&name_lower.as_str())) {
            res_urls.insert(ResType::Image(img_type.clone()), url.clone());
        }
        
        if let Some((_, audio_type)) = AUDIO_RES_MAPPINGS
            .iter()
            .find(|(names, _)| names.contains(&name_lower.as_str())) {
            res_urls.insert(ResType::Audio(audio_type.clone()), url.clone());
        }
    }

    res_urls
}

fn download_res(res_urls: HashMap::<ResType, String>) {
    for (_res_type, _url) in res_urls {
        // download
    }
}

#[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// from prpr/src/core/resource.rs, may be replaced in the future
struct ResPackInfo {
    name: String,
    author: String,

    // hit_fx: (u32, u32),
    // #[serde(default = "default_duration")]
    // hit_fx_duration: f32,
    // #[serde(default = "default_scale")]
    // hit_fx_scale: f32,
    // #[serde(default)]
    // hit_fx_rotate: bool,
    // #[serde(default)]
    // hide_particles: bool,
    // #[serde(default = "default_tinted")]
    // hit_fx_tinted: bool,

    // hold_atlas: (u32, u32),
    // #[serde(rename = "holdAtlasMH")]
    // hold_atlas_mh: (u32, u32),

    // #[serde(default)]
    // hold_keep_head: bool,
    // #[serde(default)]
    // hold_repeat: bool,
    // #[serde(default)]
    // hold_compact: bool,

    // #[serde(default = "default_perfect")]
    // color_perfect: u32,
    // #[serde(default = "default_good")]
    // color_good: u32,

    // #[serde(default)]
    description: String,
}
fn generate_respack_info(meta: PTRespackMeta) -> ResPackInfo {
    ResPackInfo {
        name: meta.name,
        author: meta.author,
        description: String::new(),
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    match runtime.block_on(fetch_meta(PTRESPACK_META_URL)) {
        Ok(meta) => {
            println!("Fetched metadata:\n{:#?}", meta);
            // res_name_parser(&meta.res);
            println!("Parsed res names:\n{:#?}", res_name_parser(&meta.res));
        },
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}