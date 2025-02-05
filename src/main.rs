use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use bytes::Bytes;
use image::{ImageBuffer, GenericImageView, DynamicImage, ImageEncoder};
use std::env;

const PTRESPACK_META_URL: &str = "https://pgres4pt.realtvop.top/fish";

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
    CombinedHold,
    CombinedHoldHL,
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

fn get_output_dir(name: &str) -> std::path::PathBuf {
    Path::new("output").join(name)
}

async fn ensure_directories(name: &str) -> std::io::Result<()> {
    fs::create_dir_all(get_output_dir(name))?;
    Ok(())
}

async fn download_file(client: &reqwest::Client, url: &str) -> Result<bytes::Bytes, Error> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

async fn save_file(path: &Path, contents: bytes::Bytes) -> std::io::Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(&contents).await?;
    Ok(())
}

fn get_filename(res_type: &ResType) -> &'static str {
    match res_type {
        ResType::Image(img_type) => match img_type {
            ImageResType::HitFX => "hit_fx.png",
            ImageResType::Tap => "click.png",
            ImageResType::TapHL => "click_mh.png",
            ImageResType::Drag => "drag.png",
            ImageResType::DragHL => "drag_mh.png",
            ImageResType::Flick => "flick.png",
            ImageResType::FlickHL => "flick_mh.png",
            ImageResType::CombinedHold => "hold.png",
            ImageResType::CombinedHoldHL => "hold_mh.png",
            
            _ => "",
        },
        ResType::Audio(audio_type) => match audio_type {
            AudioResType::TapHitSound => "click.ogg",
            AudioResType::DragHitSound => "drag.ogg",
            AudioResType::FlickHitSound => "flick.ogg",
        },
    }
}

struct DownloadResult {
    res_type: ResType,
    content: Bytes,
}

fn hit_fx_convector(image_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(image_data)?;
    
    let orig_width = img.width();
    let orig_height = img.height();
    
    let frame_width = orig_width;
    let frame_height = orig_height / 30;
    
    let new_width = frame_width * 5;
    let new_height = frame_height * 6;

    let mut new_image = ImageBuffer::new(new_width, new_height);
    
    for i in 0..30 {
        let old_y = (i as u32) * frame_height;
        
        let new_x = ((i as u32) % 5) * frame_width;
        let new_y = ((i as u32) / 5) * frame_height;

        for y in 0..frame_height {
            for x in 0..frame_width {
                let pixel = img.get_pixel(x, old_y + y);
                new_image.put_pixel(new_x + x, new_y + y, pixel);
            }
        }
    }

    let mut output = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut output);
    encoder.write_image(
        new_image.as_raw(),
        new_width,
        new_height,
        image::ColorType::Rgba8
    )?;
    Ok(output)
}

fn combine_hold_images(holdend: &[u8], hold: &[u8], holdhead: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let end_img = image::load_from_memory(holdend)?;
    let hold_img = image::load_from_memory(hold)?;
    let head_img = image::load_from_memory(holdhead)?;

    let width = end_img.width().max(hold_img.width()).max(head_img.width());
    let height = end_img.height() + hold_img.height() + head_img.height();

    let mut combined = ImageBuffer::new(width, height);

    let x_offset = (width - end_img.width()) / 2;
    for y in 0..end_img.height() {
        for x in 0..end_img.width() {
            combined.put_pixel(x_offset + x, y, end_img.get_pixel(x, y));
        }
    }

    let x_offset = (width - hold_img.width()) / 2;
    let y_offset = end_img.height();
    for y in 0..hold_img.height() {
        for x in 0..hold_img.width() {
            combined.put_pixel(x_offset + x, y_offset + y, hold_img.get_pixel(x, y));
        }
    }

    let x_offset = (width - head_img.width()) / 2;
    let y_offset = end_img.height() + hold_img.height();
    for y in 0..head_img.height() {
        for x in 0..head_img.width() {
            combined.put_pixel(x_offset + x, y_offset + y, head_img.get_pixel(x, y));
        }
    }

    let mut output = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut output);
    encoder.write_image(
        combined.as_raw(),
        width,
        height,
        image::ColorType::Rgba8
    )?;
    Ok(output)
}

async fn download_res(res_urls: HashMap<ResType, String>) -> Result<Vec<DownloadResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut downloaded = Vec::new();

    for (res_type, url) in res_urls {
        let bytes = download_file(&client, &url).await?;
        downloaded.push(DownloadResult {
            res_type,
            content: bytes,
        });
    }

    Ok(downloaded)
}

async fn save_res(downloads: Vec<DownloadResult>, meta: PTRespackMeta) -> Result<(), Box<dyn std::error::Error>> {
    ensure_directories(&meta.name).await?;
    let output_dir = get_output_dir(&meta.name);
    let mut hold_components = HashMap::new();

    for res in &downloads {
        let filename = get_filename(&res.res_type);
        let filepath = output_dir.join(filename);
        
        match &res.res_type {
            ResType::Image(ImageResType::HitFX) => {
                let processed_data = hit_fx_convector(&res.content)?;
                save_file(&filepath, Bytes::from(processed_data)).await?;
            },
            ResType::Image(img_type) => {
                match img_type {
                    ImageResType::HoldEnd | ImageResType::Hold | ImageResType::HoldHead |
                    ImageResType::HoldHL | ImageResType::HoldHeadHL => {
                        hold_components.insert(img_type.clone(), res.content.clone());
                    },
                    _ => { save_file(&filepath, res.content.clone()).await? },
                }
            },
            _ => {
                save_file(&filepath, res.content.clone()).await?;
            }
        }
    }

    if let (Some(end), Some(hold), Some(head)) = (
        hold_components.get(&ImageResType::HoldEnd),
        hold_components.get(&ImageResType::Hold),
        hold_components.get(&ImageResType::HoldHead)
    ) {
        let combined = combine_hold_images(end, hold, head)?;
        save_file(
            &output_dir.join(get_filename(&ResType::Image(ImageResType::CombinedHold))),
            Bytes::from(combined)
        ).await?;
    }

    if let (Some(end), Some(hold), Some(head)) = (
        hold_components.get(&ImageResType::HoldEnd),
        hold_components.get(&ImageResType::HoldHL),
        hold_components.get(&ImageResType::HoldHeadHL)
    ) {
        let combined = combine_hold_images(end, hold, head)?;
        save_file(
            &output_dir.join(get_filename(&ResType::Image(ImageResType::CombinedHoldHL))),
            Bytes::from(combined)
        ).await?;
    }

    let res_info = generate_respack_info(meta, &hold_components)?;
    let yaml = serde_yaml::to_string(&res_info)?;
    save_file(
        &output_dir.join("info.yml"),
        Bytes::from(yaml.into_bytes())
    ).await?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResPackInfo {
    name: String,
    author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    hit_fx: Option<(u32, u32)>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    hold_atlas: Option<(u32, u32)>,
    #[serde(rename = "holdAtlasMH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hold_atlas_mh: Option<(u32, u32)>,

    description: String,
}

fn get_image_dimensions(data: &[u8]) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let img = image::load_from_memory(data)?;
    Ok((img.width(), img.height()))
}

fn generate_respack_info(meta: PTRespackMeta, hold_components: &HashMap<ImageResType, Bytes>) -> Result<ResPackInfo, Box<dyn std::error::Error>> {
    let hold_atlas = if hold_components.contains_key(&ImageResType::HoldEnd) 
        && hold_components.contains_key(&ImageResType::HoldHead) {
        let (_, end_height) = get_image_dimensions(hold_components.get(&ImageResType::HoldEnd).unwrap())?;
        let (_, head_height) = get_image_dimensions(hold_components.get(&ImageResType::HoldHead).unwrap())?;
        Some((end_height, head_height))
    } else {
        None
    };

    let hold_atlas_mh = if hold_components.contains_key(&ImageResType::HoldEnd) 
        && hold_components.contains_key(&ImageResType::HoldHeadHL) {
        let (_, end_height) = get_image_dimensions(hold_components.get(&ImageResType::HoldEnd).unwrap())?;
        let (_, head_hl_height) = get_image_dimensions(hold_components.get(&ImageResType::HoldHeadHL).unwrap())?;
        Some((end_height, head_hl_height))
    } else {
        None
    };

    Ok(ResPackInfo {
        name: meta.name,
        author: meta.author,
        hit_fx: if hold_components.contains_key(&ImageResType::HitFX) { Some((5, 6)) } else { None },
        hold_atlas,
        hold_atlas_mh,
        description: String::new(),
    })
}

pub async fn load_pt_online_respack(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let meta = fetch_meta(url).await?;
    let res_urls = res_name_parser(&meta.res);
    let downloaded = download_res(res_urls).await?;
    save_res(downloaded, meta).await?;
    Ok(())
}

fn main() {
    let url = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: ptonlineres2prpr <url>");
        eprintln!("No URL provided, using example: {}", PTRESPACK_META_URL);
        PTRESPACK_META_URL.to_string()
    });

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    if let Err(e) = runtime.block_on(load_pt_online_respack(&url)) {
        eprintln!("Error occurred: {}", e);
    }
}