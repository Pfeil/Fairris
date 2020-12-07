use std::collections::HashSet;

use anyhow::Error;
use serde_json as json;
use yew::{
    prelude::*,
    agent::Dispatcher,
    format::Json,
    services::fetch,
    services::fetch::FetchTask,
    services::FetchService,
    worker::Agent,
    worker::AgentLink,
    worker::Context,
    worker::HandlerId,
    Callback,
};

use crate::{
    app_state::pid_manager::PidManager, data_type_registry::Pid, pidinfo::PidInfo,
    service_communication::PidRecord,
};

pub struct PitService {
    link: AgentLink<PitService>,
    subscribers: HashSet<HandlerId>,

    task: Option<FetchTask>,
    pid_manager: Dispatcher<PidManager>,
}

#[derive(Debug)]
pub enum Request {
    Register(PidInfo),
    Update(PidInfo),
}

#[derive(Debug, Clone)]
pub enum Response {
    Registered(Pid, PidRecord),
    Updated(PidInfo),
    Error(String),
}

impl Agent for PitService {
    type Reach = Context<Self>;
    type Message = Response;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: Default::default(),

            task: None,
            pid_manager: PidManager::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        use crate::app_state::pid_manager::Incoming as PidMsg;
        match msg.clone() {
            Response::Error(e) => log::error!("PIT SERVICE ERROR: {}", e),
            Response::Registered(pid, record) => self.pid_manager.send(PidMsg::UpdateRecord(pid, record)),
            Response::Updated(info) => self.pid_manager.send(PidMsg::AddPidInfo(info)),
        }
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::Register(mut info) => self.register_pidinfo(&mut info),
            Request::Update(mut info) => self.update_pidinfo(&mut info),
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl PitService {
    pub fn update_pidinfo(&mut self, info: &mut PidInfo) {
        let pid = Pid(info.pid().clone());
        let record: PidRecord = info.as_record();
        let record = json::to_value(record).unwrap();
        self.update_json(
            &pid,
            record,
            self.link
                .clone()
                .callback(move |response: fetch::Response<Result<String, Error>>| {
                    if response.status().is_success() {
                        json::from_str(
                            response
                                .body()
                                .as_ref()
                                .expect("Get reference from body.")
                                .as_str(),
                        )
                        .and_then(|record: PidRecord| {
                            Ok(Response::Updated(PidInfo::from_registered(record)))
                        })
                        .unwrap_or_else(|e| {
                            Response::Error(format!("Error parsing record: {:?}", e))
                        })
                    } else {
                        Response::Error(format!("HTTP error: {:?}", response))
                    }
                }),
        )
    }

    pub fn register_pidinfo(&mut self, info: &mut PidInfo) {
        let old_pid = Pid(info.pid().clone());
        let record: PidRecord = info.as_record();
        let record = json::to_value(record).unwrap();
        self.register_json(
            record,
            self.link
                .clone()
                .callback(move |response: fetch::Response<Result<String, Error>>| {
                    if response.status().is_success() {
                        json::from_str(
                            response
                                .body()
                                .as_ref()
                                .expect("Get reference from body.")
                                .as_str(),
                        )
                        .and_then(|record: PidRecord| {
                            Ok(Response::Registered(
                                old_pid.clone(), // might be registered or not
                                record,
                            ))
                        })
                        .unwrap_or_else(|e| {
                            Response::Error(format!("Error parsing record: {:?}", e))
                        })
                    } else {
                        Response::Error(format!("HTTP error: {:?}", response))
                    }
                }),
        )
    }

    fn register_json(
        &mut self,
        record: serde_json::Value,
        callback: Callback<fetch::Response<Result<String, Error>>>,
    ) {
        log::debug!("register() was called.");
        let request = fetch::Request::post(PitService::get_create_uri())
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
        callback: Callback<fetch::Response<Result<String, Error>>>,
    ) {
        log::debug!("update() was called.");
        let request = fetch::Request::put(Self::get_update_uri(&pid))
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
