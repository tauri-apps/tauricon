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

pub fn parse_icon_path(path: &str) -> anyhow::Result<ImageInfo> {
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

    let scale_index = path.chars().position(|x| x == '@').unwrap_or(1);

    let scale_string: String = path[scale_index..]
        .chars()
        .filter(|x| x.is_digit(10))
        .collect();

    let scale = scale_string
        .parse::<u32>()
        .context("Failed to parse scale into a number")?;

    Ok(ImageInfo {
        width: 0,
        height: 0,
        scale,
        format: image_type,
    })
}
