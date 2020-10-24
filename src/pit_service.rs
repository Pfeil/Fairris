use anyhow::Error;
use serde_json as json;
use yew::{
    format::Json,
    services::fetch::{FetchTask, Request, Response},
    services::FetchService,
    Callback, ComponentLink,
};

use crate::{data_type_registry::Pid, pidinfo::PidInfo, service_communication::PidRecord, Model};

pub struct PitService {
    task: Option<FetchTask>,
    model_link: ComponentLink<Model>,
}

impl PitService {
    pub fn new(link: ComponentLink<Model>) -> Self {
        Self {
            task: None,
            model_link: link,
        }
    }

    pub fn update_pidinfo(&mut self, info: &mut PidInfo) {
        let pid = Pid(info.pid().clone());
        let record: PidRecord = info.as_record();
        let record = json::to_value(record).unwrap();
        let model_link = self.model_link.clone();
        self.update_json(
            &pid,
            record,
            self.model_link.clone().callback(move |response: Response<Result<String, Error>>| {
                if response.status().is_success() {
                    json::from_str(
                        response
                            .body()
                            .as_ref()
                            .expect("Get reference from body.")
                            .as_str(),
                    )
                    .and_then(|record: PidRecord| {
                        Ok(super::Msg::AddPidItem(
                            PidInfo::from_registered(record, model_link.clone()),
                        ))
                    })
                    .unwrap_or_else(|e| super::Msg::Error(format!("Error parsing record: {:?}", e)))
                } else {
                    super::Msg::Error(format!("HTTP error: {:?}", response))
                }
            }),
        )
    }

    pub fn register_pidinfo(&mut self, info: &mut PidInfo) {
        let old_pid = Pid(info.pid().clone());
        let record: PidRecord = info.as_record();
        let record = json::to_value(record).unwrap();
        let model_link = self.model_link.clone();
        self.register_json(
            record,
            self.model_link.clone().callback(move |response: Response<Result<String, Error>>| {
                if response.status().is_success() {
                    json::from_str(
                        response
                            .body()
                            .as_ref()
                            .expect("Get reference from body.")
                            .as_str(),
                    )
                    .and_then(|record: PidRecord| {
                        Ok(super::Msg::ReplaceItemWithPid(
                            old_pid.clone(), // might be registered or not
                            PidInfo::from_registered(record, model_link.clone()),
                        ))
                    })
                    .unwrap_or_else(|e| super::Msg::Error(format!("Error parsing record: {:?}", e)))
                } else {
                    super::Msg::Error(format!("HTTP error: {:?}", response))
                }
            }),
        )
    }

    fn register_json(
        &mut self,
        record: serde_json::Value,
        callback: Callback<Response<Result<String, Error>>>,
    ) {
        log::debug!("register() was called.");
        let request = Request::post(PitService::get_create_uri())
            .header("Content-Type", "application/json")
            .body(Json(&record))
            .expect("Failed to build this request.");
        let task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to register metadata: {}", e));
        self.task = task.ok();
        log::debug!("register() has finished.");
    }

    fn update_json(
        &mut self,
        pid: &Pid,
        record: serde_json::Value,
        callback: Callback<Response<Result<String, Error>>>,
    ) {
        log::debug!("update() was called.");
        let request = Request::put(Self::get_update_uri(&pid))
            .header("Content-Type", "application/json")
            .body(Json(&record))
            .expect("Failed to build this request.");
        let task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to update metadata: {}", e));
        self.task = task.ok();
        log::debug!("update() has finished.");
    }

    // TODO make this configurable at compile/run time
    fn get_base_uri() -> &'static str {
        "http://localhost:8090"
    }

    fn get_create_uri() -> String {
        format!("{}/{}/", Self::get_base_uri(), "api/v1/pit/pid")
    }

    fn get_update_uri(pid: &Pid) -> String {
        format!("{}/{}/{}", Self::get_base_uri(), "api/v1/pit/pid", *pid)
    }
}
