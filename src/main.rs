use clap::{load_yaml, App, ArgMatches};
use encoding_rs::Encoding;
use qrcode::{render::unicode::Dense1x2, EcLevel, QrCode, Version};
use rqrr::PreparedImage;
use std::process::exit;

enum CliError {
    Usage(String),
    Image(String),
    QrCode(String),
}

type Result<T> = std::result::Result<T, CliError>;

macro_rules! usage_error {
    ($($arg:tt)*) => {
        CliError::Usage(format!($($arg)*))
    };
}

impl From<image::ImageError> for CliError {
    fn from(e: image::ImageError) -> Self {
        Self::Image(format!("{}", e))
    }
}

impl From<rqrr::DeQRError> for CliError {
    fn from(e: rqrr::DeQRError) -> Self {
        Self::QrCode(format!("{}", e))
    }
}

impl From<qrcode::types::QrError> for CliError {
    fn from(e: qrcode::types::QrError) -> Self {
        Self::QrCode(format!("{}", e))
    }
}

fn main() {
    let cli_def = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_def)
        .name(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    if let Err(e) = main_impl(matches) {
        let (msg, code) = match e {
            CliError::Usage(msg) => (msg, 1),
            CliError::Image(msg) => (msg, 2),
            CliError::QrCode(msg) => (msg, 3),
        };
        eprintln!("error: {}", msg);
        exit(code);
    }
}

fn main_impl(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("decode", Some(sub_m)) => decode(sub_m),
        ("encode", Some(sub_m)) => encode(sub_m),
        _ => todo!(),
    }
}

fn decode(matches: &ArgMatches) -> Result<()> {
    let encoding = match matches.value_of("encoding") {
        Some(label) => match Encoding::for_label(label.as_bytes()) {
            Some(encoding) => encoding,
            None => return Err(usage_error!("unsupported encoding: {}", label)),
        },
        None => encoding_rs::UTF_8,
    };

    let name = matches.value_of("image").unwrap();
    let img = image::open(name)?.to_luma8();
    let mut img = PreparedImage::prepare(img);

    for grid in img.detect_grids() {
        let mut content = Vec::new();
        let meta = grid.decode_to(&mut content)?;

        let (content, _, has_error) = encoding.decode(content.as_slice());
        if has_error {
            eprintln!("warning: failed to decode content");
        }

        println!("# Version: {}", meta.version.to_size());
        println!("# ECC Level: {}", meta.ecc_level);
        println!("# Mask: {}", meta.mask);
        println!("{}", content);
    }

    Ok(())
}

fn encode(matches: &ArgMatches) -> Result<()> {
    let version = match matches.value_of("version") {
        Some(version) => match version.parse() {
            Ok(version) => match version {
                version @ 1..=4 if matches.is_present("micro") => Some(Version::Micro(version)),
                version @ 1..=40 => Some(Version::Normal(version)),
                version => return Err(usage_error!("unsupported version: {}", version)),
            },
            Err(_) => return Err(usage_error!("illegal version: {}", version)),
        },
        None => None,
    };

    let level = match matches.value_of("level") {
        Some("L") | None => EcLevel::L,
        Some("M") => EcLevel::M,
        Some("Q") => EcLevel::Q,
        Some("H") => EcLevel::H,
        Some(level) => return Err(usage_error!("illegal level: {}", level)),
    };

    let data = matches.value_of("data").unwrap();
    let code = match version {
        Some(version) => QrCode::with_version(data, version, level)?,
        None => QrCode::with_error_correction_level(data, level)?,
    };

    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{}", image);

    Ok(())
}
