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

#[derive(clap::Parser)]
#[clap(about, version)]
enum Command {
    /// Decodes QR Code from an image file
    Decode(DecodeArgs),

    /// Encodes QR Code from a string
    Encode(EncodeArgs),
}

fn main() {
    let command = Command::parse();
    let res = match command {
        Command::Decode(args) => decode(args),
        Command::Encode(args) => encode(args),
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

#[derive(clap::Args)]
struct DecodeArgs {
    /// Character encoding to use.
    #[clap(short, long, default_value = "UTF-8")]
    encoding: String,

    /// Path to the image to decode.
    image: std::path::PathBuf,
}

fn decode(args: DecodeArgs) -> Result<()> {
    let encoding = match Encoding::for_label(args.encoding.as_bytes()) {
        Some(encoding) => encoding,
        None => return Err(usage_error!("unsupported encoding: {}", args.encoding)),
    };

    let img = image::open(&args.image)?.to_luma8();
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
struct EncodeArgs {
    /// Generates Micro QR Code. (requires --version)
    #[clap(short, long, requires = "version")]
    micro: bool,

    /// The version of the generated image. (1 to 40 for normal, 1 to 4 for micro)
    #[clap(short, long)]
    version: Option<i16>,

    /// The error correction level. (L/M/Q/H)
    #[clap(short, long, default_value = "L")]
    level: EcLevel,

    /// Data to be encoded.
    data: String,
}

fn encode(args: EncodeArgs) -> Result<()> {
    let version = match args.version {
        Some(version) => match version {
            1..=4 if args.micro => Some(Version::Micro(version)),
            1..=40 => Some(Version::Normal(version)),
            _ => return Err(usage_error!("unsupported version: {version}")),
        },
        None => None,
    };

    let code = match version {
        Some(version) => QrCode::with_version(&args.data, version, args.level.0)?,
        None => QrCode::with_error_correction_level(&args.data, args.level.0)?,
    };

    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{image}");

    Ok(())
}
