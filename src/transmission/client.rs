#![allow(non_snake_case)]

use color_eyre::Result;
use http::header::HeaderMap;
use http::StatusCode;
use parking_lot::Mutex;
use reqwest::redirect::Policy;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionRequest {
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionResponse {
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub uploaded_bytes: f64,
    pub downloaded_bytes: f64,
    pub files_added: f64,
    pub session_count: f64,
    pub seconds_active: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub active_torrent_count: f64,
    pub download_speed: f64,
    pub paused_torrent_count: f64,
    pub torrent_count: f64,
    pub upload_speed: f64,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTorrentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentActionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentStartRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    pub id: Option<f64>,
    pub name: Option<String>,
    pub percent_complete: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTorrentResponse {
    pub torrents: Vec<Torrent>,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TorrentStatus {
    Stopped = 0,
    QueuedVerify = 1,
    Verifying = 2,
    QueuedDownload = 3,
    Downloading = 4,
    QueuedSeed = 5,
    Seeding = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentSummary {
    pub id: f64,
    pub name: String,
    pub percent_complete: f64,
    pub percent_done: f64,
    pub status: TorrentStatus,
    pub size_when_done: f64,
    pub piece_count: i64,
    pub pieces: String,
    pub eta: f64,
    pub peers_connected: i64,
    pub peers_getting_from_us: i64,
    pub peers_sending_to_us: i64,
    pub rate_download: i64,
    pub rate_upload: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentSummaryResponse {
    pub torrents: Vec<TorrentSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub tag: Option<u32>,
    pub arguments: Option<RequestArgs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub arguments: T,
    pub result: String,
    pub tag: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseNoArgs {
    pub result: String,
    pub tag: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestArgs {
    GetSessionArgs(GetSessionRequest),
    GetTorrentArgs(GetTorrentRequest),
    TorrentStopArgs(TorrentActionRequest),
    TorrentStartArgs(TorrentActionRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseArgs {
    GetSessionRes(GetSessionResponse),
    GetTorrentRes(GetTorrentResponse),
}

pub struct Client {
    tm_url: String,
    session_id: Mutex<Arc<String>>,
}

impl Client {
    pub async fn send(&self, request: &Request) -> color_eyre::Result<Value> {
        let mut headers = HeaderMap::new();
        {
            let session_id = self.session_id.lock();
            headers.insert("X-Transmission-Session-Id", session_id.parse().unwrap());
        }

        let mut response = HttpClient::builder()
            .redirect(Policy::default())
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()?
            .post(&self.tm_url)
            .body(serde_json::to_string(&request)?)
            .send()
            .await?;

        if response.status() == StatusCode::CONFLICT {
            let header: String = response
                .headers()
                .get("x-transmission-session-id")
                .unwrap()
                .clone()
                .to_str()
                .unwrap()
                .to_string();
            *self.session_id.lock() = Arc::new(header);

            let mut headers = HeaderMap::new();
            {
                let session_id = self.session_id.lock();
                let session_id_parsed = session_id.parse()?;
                headers.insert("X-Transmission-Session-Id", session_id_parsed);
            }
            response = HttpClient::builder()
                .redirect(Policy::default())
                .timeout(Duration::from_secs(5))
                .default_headers(headers)
                .build()?
                .post(&self.tm_url)
                .body(serde_json::to_string(&request)?)
                .send()
                .await?;
        }

        let result = response.json().await?;
        Ok(result)
    }

    pub async fn session_get(&self, fields: Vec<String>) -> Result<Response<GetSessionResponse>> {
        let request = Request {
            method: "session-get".to_string(),
            arguments: Some(RequestArgs::GetSessionArgs(GetSessionRequest { fields })),
            tag: None,
        };
        let res = self.send(&request).await?;
        let response: Response<GetSessionResponse> = serde_json::from_value(res)?;
        Ok(response)
    }

    pub async fn session_stats(&self) -> Result<Response<SessionStats>> {
        let request = Request {
            method: "session-stats".to_string(),
            arguments: None,
            tag: None,
        };
        let res = self.send(&request).await?;
        let response: Response<SessionStats> = serde_json::from_value(res)?;
        Ok(response)
    }

    pub async fn torrent_get(&self, fields: Vec<String>) -> Result<Response<GetTorrentResponse>> {
        let request = Request {
            method: "torrent-get".to_string(),
            arguments: Some(RequestArgs::GetTorrentArgs(GetTorrentRequest {
                ids: None,
                fields,
            })),
            tag: None,
        };
        let res = self.send(&request).await?;
        let response: Response<GetTorrentResponse> = serde_json::from_value(res)?;
        Ok(response)
    }

    pub async fn torrent_summary(&self) -> Result<Response<TorrentSummaryResponse>> {
        let fields = vec![
            "id",
            "name",
            "percentComplete",
            "status",
            "eta",
            "percentDone",
            "sizeWhenDone",
            "pieces",
            "pieceCount",
            "peersConnected",
            "peersGettingFromUs",
            "peersSendingToUs",
            "rateDownload",
            "rateUpload",
        ];
        let request = Request {
            method: "torrent-get".to_string(),
            arguments: Some(RequestArgs::GetTorrentArgs(GetTorrentRequest {
                ids: None,
                fields: fields.into_iter().map(|f| f.to_string()).collect(),
            })),
            tag: None,
        };
        let res = self.send(&request).await?;
        let response: Response<TorrentSummaryResponse> = serde_json::from_value(res)?;
        Ok(response)
    }

    pub async fn torrent_action(&self, action: String, id: i64) -> Result<ResponseNoArgs> {
        let request = Request {
            method: format!("torrent-{}", action),
            arguments: Some(RequestArgs::TorrentStopArgs(TorrentActionRequest {
                ids: Some(vec![id]),
            })),
            tag: None,
        };
        let res = self.send(&request).await?;
        let response: ResponseNoArgs = serde_json::from_value(res)?;
        Ok(response)
    }
}

pub struct ClientBuilder {
    tm_url: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self { tm_url: None }
    }
    pub fn transmission_url(mut self, tm_url: String) -> Self {
        self.tm_url = Some(tm_url);
        self
    }
    pub fn build(self) -> Result<Client, Box<dyn Error>> {
        let tm_url = self.tm_url.expect("Expected a URL");
        Ok(Client {
            tm_url,
            session_id: Mutex::new(Arc::new("unknown".to_string())),
        })
    }
}
