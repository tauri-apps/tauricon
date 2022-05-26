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

    let size = &path[..scale_index];

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

        let dimensions = (&sizes[0..=1])
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