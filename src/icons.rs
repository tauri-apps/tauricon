use anyhow::Context;
use image::ImageFormat;

#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub format: IconFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconFormat {
    Png,
    Ico,
    Icns,
}

impl From<IconFormat> for String {
    fn from(format: IconFormat) -> Self {
        match format {
            IconFormat::Png => "png".to_string(),
            IconFormat::Ico => "ico".to_string(),
            IconFormat::Icns => "icns".to_string(),
        }
    }
}

impl From<IconFormat> for ImageFormat {
    fn from(format: IconFormat) -> Self {
        match format {
            IconFormat::Ico => ImageFormat::Ico,
            _ => ImageFormat::Png,
        }
    }
}

pub fn parse_icon_path(path: &str) -> anyhow::Result<ImageInfo> {
    let path = path.split('/').last().unwrap_or(path);

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

    let scale_index = path.chars().position(|x| x == '@').unwrap_or(0);

    debug!("{}", scale_index);

    let scale_string: String = {
        if scale_index == 0 {
            "".to_string()
        } else {
            path[scale_index..]
                .chars()
                .filter(|x| x.is_digit(10))
                .collect()
        }
    };

    let scale = if scale_string.is_empty() {
        1
    } else {
        scale_string
            .parse::<u32>()
            .context("Failed to parse scale into a number")?
    };

    if image_type == IconFormat::Ico {
        return Ok(ImageInfo {
            width: 256,
            height: 256,
            scale,
            format: image_type,
        });
    }

    let size = if scale_index == 0 {
        let until = path.len() - end.len() - 1;

        &path[..until]
    } else {
        &path[..scale_index]
    };

    if size == "icon" {
        return Ok(ImageInfo {
            width: 1240,
            height: 1240,
            scale,
            format: image_type,
        });
    }

    let (width, height) = {
        let sizes: Vec<&str> = size.split('x').collect();

        debug!("Parseing size: {:?}", sizes);

        let dimensions = (&sizes[0..2])
            .iter()
            .map(|x| {
                x.parse::<u32>()
                    .context("Failed to parse dimension into a number")
            })
            .collect::<anyhow::Result<Vec<u32>>>()?;

        if dimensions.len() != 2 {
            panic!("Juliette you fucked up :)");
        }

        (dimensions[0], dimensions[1])
    };

    Ok(ImageInfo {
        width,
        height,
        scale,
        format: image_type,
    })
}
