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

#[derive(Debug, Eq, Hash, PartialEq)]
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
#[derive(Debug, Eq, Hash, PartialEq)]
enum AudioResType {
    TapHitSound,
    DragHitSound,
    FlickHitSound,
}
#[derive(Debug, Eq, Hash, PartialEq)]
enum ResType {
    Image(ImageResType),
    Audio(AudioResType),
}
fn res_name_parser(res: &HashMap<String, String>) -> HashMap::<ResType, String> {
    let mut res_urls = HashMap::<ResType, String>::new();

    for (i, url) in res {
        let i_lowercase = i.to_lowercase();
        if ["clickraw", "clickraw.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::HitFX), url.clone());
        } else if ["tap", "tap.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::Tap), url.clone());
        } else if ["taphl", "taphl.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::TapHL), url.clone());
        } else if ["holdend", "holdend.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::HoldEnd), url.clone());
        } else if ["hold", "hold.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::Hold), url.clone());
        } else if ["holdhl", "holdhl.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::HoldHL), url.clone());
        } else if ["holdhead", "holdhead.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::HoldHead), url.clone());
        } else if ["holdheadhl", "holdheadhl.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::HoldHeadHL), url.clone());
        } else if ["drag", "drag.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::Drag), url.clone());
        } else if ["draghl", "draghl.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::DragHL), url.clone());
        } else if ["flick", "flick.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::Flick), url.clone());
        } else if ["flickhl", "flickhl.png"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Image(ImageResType::FlickHL), url.clone());
        } else if ["hitsong0", "hitsong0.ogg"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Audio(AudioResType::TapHitSound), url.clone());
        } else if ["hitsong1", "hitsong1.ogg"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Audio(AudioResType::DragHitSound), url.clone());
        } else if ["hitsong2", "hitsong2.ogg"].contains(&i_lowercase.as_str()) {
            res_urls.insert(ResType::Audio(AudioResType::FlickHitSound), url.clone());
        }
    }

    res_urls
}

fn download_res(res_urls: HashMap::<ResType, String>) {
    for (res_type, url) in res_urls {
        // download
    }
}

#[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// from prpr/src/core/resource.rs, may be replaced in the future
struct ResPackInfo {
    name: String,
    author: String,

    hit_fx: (u32, u32),
    #[serde(default = "default_duration")]
    hit_fx_duration: f32,
    #[serde(default = "default_scale")]
    hit_fx_scale: f32,
    #[serde(default)]
    hit_fx_rotate: bool,
    #[serde(default)]
    hide_particles: bool,
    #[serde(default = "default_tinted")]
    hit_fx_tinted: bool,

    hold_atlas: (u32, u32),
    #[serde(rename = "holdAtlasMH")]
    hold_atlas_mh: (u32, u32),

    #[serde(default)]
    hold_keep_head: bool,
    #[serde(default)]
    hold_repeat: bool,
    #[serde(default)]
    hold_compact: bool,

    #[serde(default = "default_perfect")]
    color_perfect: u32,
    #[serde(default = "default_good")]
    color_good: u32,

    #[serde(default)]
    description: String,
}
fn generate_respack_info(meta: PTRespackMeta) -> ResPackInfo {
    let mut respack_info = ResPackInfo::new();

    respack_info.name = meta.name;
    respack_info.author = meta.author;
    // respack_info.description = "Generated from {}".to_string();

    respack_info
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