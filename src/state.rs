use std::sync::{Arc, Mutex};

pub struct UiUtilState {
    pub settings_window_open: bool,
    pub accent_color: [f32; 4],
    pub log_open: bool,
}

impl UiUtilState {
    pub fn new() -> UiUtilState {
        UiUtilState {
            settings_window_open: false,
            //our default color is purple
            accent_color: [0.4, 0.0, 1.0, 1.0],
            log_open: false,
        }
    }
}

pub struct RequestState {
    pub url: String,
    pub verb: u8,
    pub custom_verb: String, // the user may enter a custom verb/method
    pub headers: Vec<String>,
    pub body: String,
}

impl RequestState {
    //just new since we don't need to do anything special
    fn new() -> RequestState {
        RequestState {
            url: String::new(),
            verb: 0,
            custom_verb: String::new(),
            headers: Vec::new(),
            body: String::new(),
        }
    }
}

pub struct ResponseState {
    status_code: u16,
    headers: Vec<String>,
    body: String,
    time_in_ms: u32,
}

impl ResponseState {
    fn new() -> ResponseState {
        ResponseState {
            status_code: 0,
            headers: Vec::new(),
            body: String::new(),
            time_in_ms: 0,
        }
    }
}

pub struct GuiAppState {
    pub request_state: Arc<Mutex<RequestState>>,
    pub response_state: Arc<Mutex<ResponseState>>,
}

impl GuiAppState {
    pub fn new() -> GuiAppState {
        GuiAppState {
            request_state: Arc::new(Mutex::new(RequestState::new())),
            response_state: Arc::new(Mutex::new(ResponseState::new())),
        }
    }

    pub fn get_request_state(&self) -> Arc<Mutex<RequestState>> {
        self.request_state.clone()
    }

    pub fn get_response_state(&self) -> Arc<Mutex<ResponseState>> {
        self.response_state.clone()
    }
}
