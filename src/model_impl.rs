use yew::prelude::*;

use super::{Model, PidInfo};

impl Model {
    pub fn find_pidinfo_by_string(&self, pid: String) -> PidInfo {
        // TODO handle the unwrap better.
        self.known_pids.iter().find(|info| info.pid == pid).map(|info| (*info).clone()).unwrap_or_default()
    }

    pub fn view_pid_details(&self, pid: String) -> Html {
        self.known_pids.iter().find(|info| info.pid == pid).map(|info| info.view_as_details_page()).unwrap()
    }
}