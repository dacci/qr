use clap::Parser;
use encoding_rs::Encoding;
use qrcode::{render::unicode::Dense1x2, QrCode, Version};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::process::{ExitCode, Termination};

#[derive(Debug)]
enum Error {
    Failure(String),
    Image(image::ImageError),
    Decode(rqrr::DeQRError),
    Io(std::io::Error),
    Encode(qrcode::types::QrError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Failure(msg) => f.write_str(msg),
            Error::Image(cause) => write!(f, "{cause}"),
            Error::Decode(cause) => write!(f, "{cause}"),
            Error::Io(cause) => write!(f, "{cause}"),
            Error::Encode(cause) => write!(f, "{cause}"),
        }
    }
}

impl std::error::Error for Error {}

impl Termination for Error {
    fn report(self) -> ExitCode {
        match self {
            Error::Failure(_) => ExitCode::FAILURE,
            Error::Image(_) => 2.into(),
            Error::Decode(_) => 3.into(),
            Error::Io(_) => 4.into(),
            Error::Encode(_) => 5.into(),
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Failure(msg)
    }
}

impl From<image::ImageError> for Error {
    fn from(cause: image::ImageError) -> Self {
        Self::Image(cause)
    }
}

impl From<rqrr::DeQRError> for Error {
    fn from(cause: rqrr::DeQRError) -> Self {
        Self::Decode(cause)
    }
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

impl From<qrcode::types::QrError> for Error {
    fn from(cause: qrcode::types::QrError) -> Self {
        Self::Encode(cause)
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

fn main() -> ExitCode {
    let command = Command::parse();
    let res = match command {
        Command::Decode(args) => decode(args),
        Command::Encode(args) => encode(args),
    };

    if let Err(e) = res {
        eprintln!("Error: {e}");
        e.report()
    } else {
        ExitCode::SUCCESS
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
        None => return Err(format!("Unsupported encoding: {}", args.encoding).into()),
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s {
            "L" => qrcode::EcLevel::L,
            "M" => qrcode::EcLevel::M,
            "Q" => qrcode::EcLevel::Q,
            "H" => qrcode::EcLevel::H,
            _ => return Err(format!("Illegal level: {s}").into()),
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

    /// Path to a file contains data to be encoded.
    #[clap(short, long, required_unless_present = "data")]
    file: Option<String>,

    /// Data to be encoded.
    #[clap(required_unless_present = "file")]
    data: Option<String>,
}

fn encode(args: EncodeArgs) -> Result<()> {
    let version = match args.version {
        Some(version) => match version {
            1..=4 if args.micro => Some(Version::Micro(version)),
            1..=40 => Some(Version::Normal(version)),
            _ => return Err(format!("Unsupported version: {version}").into()),
        },
        None => None,
    };

    let data = if let Some(path) = &args.file {
        let mut src: Box<dyn Read> = match path.as_str() {
            "-" => Box::new(std::io::stdin()),
            path => Box::new(File::open(path)?),
        };
        let mut data = Vec::new();
        src.read_to_end(&mut data)?;
        data
    } else if let Some(data) = &args.data {
        data.as_bytes().to_vec()
    } else {
        unreachable!()
    };

    let code = match version {
        Some(version) => QrCode::with_version(&data, version, args.level.0)?,
        None => QrCode::with_error_correction_level(&data, args.level.0)?,
    };

    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{image}");

    Ok(())
}
