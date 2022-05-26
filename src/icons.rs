use anyhow::Context;

pub struct ImageInfo {
    width: u32,
    height: u32,
    scale: u32,
    format: IconFormat,
}

pub enum IconFormat {
    Png,
    Ico,
    Icns,
}

pub fn parse_icon_path(path: &String) -> anyhow::Result<ImageInfo> {
    let end = path
        .split('.')
        .last()
        .context("Icon must have a file type")?;

    let image_type = match end {
        "png" => IconFormat::Png,
        "ico" => IconFormat::Ico,
        "icns" => IconFormat::Icns,
        _ => {
            return Err(anyhow::anyhow!(
                "Icon must have a file type of png, ico or icns"
            ))
        }
    };

    Ok(ImageInfo {
        width: 0,
        height: 0,
        scale: 1,
        format: IconFormat::Png,
    })
}
