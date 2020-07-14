use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use super::{AppRoute, PidInfo};

//#[derive(Default)]
pub struct KnownPids {
    known_pids: Vec<PidInfo>,
}

impl KnownPids {
    pub fn find_pidinfo_by_string(&self, pid: &str) -> &PidInfo {
        // TODO handle the unwrap better.
        self.known_pids.iter().find(|info| info.pid == pid).unwrap_or_else(|| {
            log::error!("Finding the PID ({}) did not work out.", pid);
            panic!()
        })
    }

    pub fn pidinfo_as_list_item(&self, pid: String) -> Html {
        let item = self.find_pidinfo_by_string(pid.as_str());
        let pid = item.pid.clone();
        let descr = item.pid.clone();
        let status = item.status.clone();
        html! {
                // TODO make sure that in the details page, the actual data is shown.
                <RouterButton<AppRoute> route=AppRoute::Details{path: pid} classes="piditem">
                    <p>{ item.pid.clone() }</p>
                    <p>{ item.description.clone() }</p>
                    <p>{ item.status.clone() }</p>
                </RouterButton<AppRoute>>
        }
    }

    pub fn pidinfo_as_details_page(&self, pid: &str) -> Html {
        let item = self.find_pidinfo_by_string(pid);
        let pid = item.pid.clone();
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <div class="image-placeholder"><p>{ "IMAGE" }</p></div>
                <div class="textblock">
                    <p>{ format!("{}: {}", "PID:", pid) }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                </div>
                <div class="fdo-actions"><p>{ "Placeholder for Action Buttons here." }</p></div>
                <div class="action-placeholder"><p>{ "Placeholder for action visualization. This could be i.e. viewing raw metadata, visualizations, or the possibility to update your FDO." }</p></div>
            </div>
        }
    }
}

impl Default for KnownPids {
    fn default() -> Self {
        let mut known_pids = vec![
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
        ];
        known_pids.iter_mut().enumerate().for_each(|(num, info)| {
            info.pid = format!("{}_{}", info.pid, num);
        });
        KnownPids {
            known_pids,
        }
    }
}

impl Deref for KnownPids {
    type Target = Vec<PidInfo>;

    fn deref(&self) -> &Self::Target {
        &self.known_pids
    }
}