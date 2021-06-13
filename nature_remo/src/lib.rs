use remo::cloud;
pub use remo::cloud::{APIError, Device, Model, RequestBody, Signal, UpdateAirconSettingsResponse};
use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
pub struct Client {
    token: String,
    base_url: Url,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(token: String) -> Self {
        Client {
            token: token,
            base_url: Url::parse("https://api.nature.global").unwrap(),
            client: reqwest::blocking::Client::new(),
        }
    }
    fn get<T>(&self, path: &str) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        self.make_request::<T>(path, None)
    }

    fn post<T>(&self, path: &str, body: &RequestBody) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        self.make_request::<T>(path, Some(body))
    }

    fn make_request<T>(&self, path: &str, body: Option<&RequestBody>) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        let request_url = self.base_url.join(path).unwrap();
        let client = match body {
            Some(body) => self.client.post(request_url.as_str()).form(body),
            None => self.client.get(request_url.as_str()),
        };
        let resp = client
            .header("Authorization", format!("Bearer {}", &self.token))
            .send()
            .unwrap();

        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.json().unwrap()),
            reqwest::StatusCode::UNAUTHORIZED => Err(APIError::AuthError),
            code => Err(APIError::Error {
                error: format!("Returned with non-200 code: {}", code),
            }),
        }
    }

    pub fn get_appliances(&self) -> Result<Vec<Appliance>, APIError> {
        self.get::<Vec<Appliance>>("/1/appliances")
    }

    pub fn update_aircon_settings(
        &self,
        aircon_id: &str,
        body: &RequestBody,
    ) -> Result<UpdateAirconSettingsResponse, APIError> {
        self.post::<UpdateAirconSettingsResponse>(
            &format!("/1/appliances/{}/aircon_settings", aircon_id),
            body,
        )
    }

    pub fn update_light_state(
        &self,
        light_id: &str,
        button_name: &str,
    ) -> Result<UpdateLightStateResponse, APIError> {
        let mut params = cloud::RequestBody::new();
        params.insert("button", button_name);
        self.post::<UpdateLightStateResponse>(&format!("/1/appliances/{}/light", light_id), &params)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appliance {
    pub id: String,
    pub device: Device,
    pub model: Option<Model>,
    pub nickname: String,
    pub image: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub settings: Option<AirconSettings>,
    pub aircon: Option<Aircon>,
    pub tv: Option<Tv>,
    pub light: Option<Light>,
    pub signals: Vec<Signal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AirconModeValue {
    pub temp: Vec<String>,
    pub dir: Vec<String>,
    pub vol: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconModes {
    pub auto: AirconModeValue,
    pub cool: AirconModeValue,
    pub warm: AirconModeValue,
    pub dry: AirconModeValue,
    pub blow: AirconModeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconRange {
    pub modes: AirconModes,
    #[serde(rename = "fixedButtons")]
    pub fixed_buttons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aircon {
    pub range: AirconRange,
    #[serde(rename = "tempUnit")]
    pub temp_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconSettings {
    pub mode: String,
    pub temp: String,
    pub temp_unit: String,
    pub vol: String,
    pub dir: String,
    pub button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tv {
    pub state: TvState,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvState {
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub state: LightState,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightState {
    pub brightness: String,
    pub power: String,
    pub last_button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pub name: String,
    pub image: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLightStateResponse {
    pub brightness: String,
    pub power: String,
    pub last_button: String,
}
