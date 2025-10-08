use std::{fs::File, io::Read, io::Write, path::PathBuf, time::SystemTime};

use flate2::read::GzDecoder;
use infer::Infer;
use roblox_api::{
    api::{
        asset_delivery,
        thumbnails::{
            self,
            v1::{ThumbnailBatchRequest, ThumbnailFormat, ThumbnailRequestType, ThumbnailSize},
        },
    },
    client::Client,
};

use crate::{Config, config::DownloadPathKind};

fn download_to_file(bytes: &[u8], cfg: &Config, name: &str, extension: &str) {
    let unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let file_name = format!("{}-{unix}.{}", name, extension);
    let download_path = match cfg.download_path_type.clone().unwrap_or_default() {
        DownloadPathKind::Downloads => dirs::download_dir().unwrap_or_default(),
        DownloadPathKind::Relative => PathBuf::new(),
    };

    let local_download_path = download_path.join(&file_name);

    let mut file = File::create_new(&local_download_path).unwrap();
    file.write_all(bytes).unwrap();
}

pub(crate) async fn asset(client: &mut Client, cfg: &Config, id: u64) {
    let mut inferer = Infer::new();

    fn rbxm_matcher(bytes: &[u8]) -> bool {
        const MAGIC: &[u8] = &[
            0x3c, 0x72, 0x6f, 0x62, 0x6c, 0x6f, 0x78, 0x21, 0x89, 0xff, 0x0d, 0x0a, 0x1a, 0x0a,
        ];

        bytes.starts_with(MAGIC)
    }

    fn rbxmx_matcher(bytes: &[u8]) -> bool {
        const MAGIC: &[u8] = &[0x3c, 0x72, 0x6f, 0x62, 0x6c, 0x6f, 0x78, 0x6d, 0x6c, 0x6e];
        bytes.starts_with(MAGIC)
    }

    inferer.add("custom/rbxm", "rbxm", rbxm_matcher);
    inferer.add("custom/rbxmx", "rbxmx", rbxmx_matcher);

    let asset_bytes = asset_delivery::v1::asset(client, id).await.unwrap();
    let asset_kind = inferer.get(&asset_bytes).expect("Unknown file type");

    let (kind, bytes) = match asset_kind.mime_type() {
        "application/gzip" => {
            let mut decoder = GzDecoder::new(asset_bytes.as_slice());

            let mut bytes = Vec::new();
            decoder.read_to_end(&mut bytes).unwrap();

            let kind = inferer.get(&bytes).expect("Unknown file type");
            (kind, bytes)
        }

        _ => (asset_kind, asset_bytes),
    };

    download_to_file(&bytes, cfg, &id.to_string(), kind.extension());
}

pub(crate) async fn thumbnail(
    client: &mut Client,
    cfg: &Config,
    id: u64,
    kind: ThumbnailRequestType,
    size: ThumbnailSize,
) {
    let format = ThumbnailFormat::Png;

    let thumbnails = thumbnails::v1::batch(
        client,
        vec![ThumbnailBatchRequest {
            id,
            request_id: "",
            token: "",
            alias: "",
            kind,
            size,
            format: format.clone(),
            circular: false,
        }],
    )
    .await
    .unwrap();

    let thumbnail = thumbnails.first().unwrap();
    let bytes = reqwest::get(&thumbnail.image_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    download_to_file(&bytes, cfg, &id.to_string(), format.extension());
}
