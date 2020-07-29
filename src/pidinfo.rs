use super::AppRoute;
use crate::service_communication::pit_record::PidRecord;
use serde_json as json;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PidInfo {
    record: PidRecord,
    state: State,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Unregistered,
    Modified,
    Clean,
}

impl Default for PidInfo {
    fn default() -> Self {
        PidInfo {
            record: PidRecord::default(),
            state: State::Unregistered,
        }
    }
}

impl PidInfo {
    pub fn from_registered(record: PidRecord) -> Self {
        Self::from(record, State::Clean)
    }

    pub fn from_unregistered(record: PidRecord) -> Self {
        Self::from(record, State::Unregistered)
    }

    pub fn from_modified(record: PidRecord) -> Self {
        Self::from(record, State::Modified)
    }

    fn from(record: PidRecord, state: State) -> Self {
        Self { record, state }
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn pid(&self) -> &String {
        &self.record.pid
    }

    pub fn view_as_list_item(&self) -> Html {
        let pid = self.record.pid.clone();
        html! {
                // TODO make sure that in the details page, the actual data is shown.
                <RouterButton<AppRoute> route=AppRoute::Details{path: pid} classes="piditem">
                    <p>{ self.record.pid.as_str() }</p>
                    <p>{ self.record.describe() }</p>
                    <p>{ format!("{:?}", self.state) }</p>
                </RouterButton<AppRoute>>
        }
    }

    pub fn view_as_details_page(&self) -> Html {
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <div class="two-column-lefty">
                    <div class="image-placeholder"><p>{ "IMAGE" }</p></div>
                    <div class="two-column-lefty">
                        <p class="align-right">{ "PID:" }</p>
                        <p>{ self.record.pid.as_str() }</p>
                        <p class="align-right">{ "Description:" }</p>
                        <p>{ self.record.describe() }</p>
                        <p class="align-right">{ "Status:" }</p>
                        <p>{ format!("{:?}", self.state) }</p>
                        {
                            self.view_record()
                        }
                    </div>
                </div>
                <div class="fdo-actions"><p>{ "Placeholder for Action Buttons here." }</p></div>
                <div class="action-placeholder"><p>{ "Placeholder for action visualization. This could be i.e. viewing raw metadata, visualizations, or the possibility to update your FDO." }</p></div>
            </div>
        }
    }

    fn view_record(&self) -> Html {
        self.record
            .entries
            .keys()
            .map(|property_pid| self.record.entries.get(property_pid).unwrap())
            .flatten()
            .map(|entry| {
                html! {
                    <>
                        <p class="align-right">{ &entry.name } <br/> { &entry.key }</p>
                        <p>{ json::to_string_pretty(&entry.value).unwrap() }</p>
                    </>
                }
            })
            .collect()
    }
}
