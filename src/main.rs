use clap::Parser;
use encoding_rs::Encoding;
use qrcode::{render::unicode::Dense1x2, QrCode, Version};
use std::fmt;

#[derive(Debug)]
enum CliError {
    Usage(String),
    Image(String),
    QrCode(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CliError::Usage(s) => s,
            CliError::Image(s) => s,
            CliError::QrCode(s) => s,
        };
        f.write_str(s)
    }
}

impl std::error::Error for CliError {}

type Result<T> = std::result::Result<T, CliError>;

macro_rules! usage_error {
    ($($arg:tt)*) => {
        CliError::Usage(format!($($arg)*))
    };
}

impl From<image::ImageError> for CliError {
    fn from(e: image::ImageError) -> Self {
        Self::Image(format!("{e}"))
    }
}

impl From<rqrr::DeQRError> for CliError {
    fn from(e: rqrr::DeQRError) -> Self {
        Self::QrCode(format!("{e}"))
    }
}

impl From<qrcode::types::QrError> for CliError {
    fn from(e: qrcode::types::QrError) -> Self {
        Self::QrCode(format!("{e}"))
    }
}

#[derive(clap::Args)]
struct DecodeOpts {
    /// Character encoding to use.
    #[clap(short = 'e', long = "encoding", default_value = "UTF-8")]
    encoding: String,

    /// Path to the image to decode.
    image: std::path::PathBuf,
}

struct EcLevel(qrcode::EcLevel);

impl std::str::FromStr for EcLevel {
    type Err = CliError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(match s {
            "L" => qrcode::EcLevel::L,
            "M" => qrcode::EcLevel::M,
            "Q" => qrcode::EcLevel::Q,
            "H" => qrcode::EcLevel::H,
            _ => return Err(usage_error!("illegal level: {s}")),
        }))
    }
}

#[derive(clap::Args)]
struct EncodeOpts {
    /// Generates Micro QR Code. (requires --version)
    #[clap(short = 'm', long = "micro", requires = "version")]
    micro: bool,

    /// The version of the generated image. (1 to 40 for normal, 1 to 4 for micro)
    #[clap(short = 'v', long = "version")]
    version: Option<i16>,

    /// The error correction level. (L/M/Q/H)
    #[clap(short = 'l', long = "level", default_value = "L")]
    level: EcLevel,

    /// Data to be encoded.
    data: String,
}

#[derive(clap::Parser)]
#[clap(about, version)]
enum Command {
    /// Decodes QR Code from an image file
    Decode(DecodeOpts),

    /// Encodes QR Code from a string
    Encode(EncodeOpts),
}

fn main() {
    let command = Command::parse();
    let res = match command {
        Command::Decode(opts) => decode(opts),
        Command::Encode(opts) => encode(opts),
    };

    if let Err(e) = res {
        let (msg, code) = match e {
            CliError::Usage(msg) => (msg, 1),
            CliError::Image(msg) => (msg, 2),
            CliError::QrCode(msg) => (msg, 3),
        };
        eprintln!("error: {msg}");
        std::process::exit(code);
    }
}

fn decode(opts: DecodeOpts) -> Result<()> {
    let encoding = match Encoding::for_label(opts.encoding.as_bytes()) {
        Some(encoding) => encoding,
        None => return Err(usage_error!("unsupported encoding: {}", opts.encoding)),
    };

    let img = image::open(&opts.image)?.to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);

    for grid in img.detect_grids() {
        let mut content = vec![];
        let meta = grid.decode_to(&mut content)?;

        let (content, _, has_error) = encoding.decode(content.as_slice());
        if has_error {
            eprintln!("warning: failed to decode content");
        }

        println!("# Version: {}", meta.version.to_size());
        println!("# ECC Level: {}", meta.ecc_level);
        println!("# Mask: {}", meta.mask);
        println!("{content}");
    }

    Ok(())
}

fn encode(opts: EncodeOpts) -> Result<()> {
    let version = match opts.version {
        Some(version) => match version {
            1..=4 if opts.micro => Some(Version::Micro(version)),
            1..=40 => Some(Version::Normal(version)),
            _ => return Err(usage_error!("unsupported version: {version}")),
        },
        None => None,
    };

    let code = match version {
        Some(version) => QrCode::with_version(&opts.data, version, opts.level.0)?,
        None => QrCode::with_error_correction_level(&opts.data, opts.level.0)?,
    };

    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{image}");

    Ok(())
}
