use yew::{agent::Dispatcher, prelude::*};

use crate::app_state::{data::{AnnotatedImage, Data, DataID}, data_manager::DataManager};

pub struct AnnotatedImageForm {
    link: ComponentLink<Self>,
    props: Props,

    data_manager: Dispatcher<DataManager>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: DataID,
    pub image: AnnotatedImage,
}

#[derive(Debug)]
pub enum Msg {
    ImageUrlChanged(String),
    AnnotationUrlsChanged(Vec<String>),
    Error(String),
}

impl Component for AnnotatedImageForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            data_manager: DataManager::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Error(e) => log::error!("Error: {}", e),
            Msg::ImageUrlChanged(url) => {
                self.props.image.url = url;
                self.update_data();
            }
            Msg::AnnotationUrlsChanged(list) => {
                self.props.image.annotation_urls = list;
                self.update_data();
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let url = self.props.image.url.as_str();
        let on_url_changed = self.link.callback(|change: ChangeData| match change {
            ChangeData::Value(url) => Msg::ImageUrlChanged(url),
            other => Msg::Error(format!("Got unexpected value: {:?}", other))
        });
        let anno_urls = self.props.image.annotation_urls.join("\n");
        let on_anno_url_changed = self.link.callback(|change: ChangeData| match change {
            ChangeData::Value(urls) => {
                let list: Vec<String> = urls.split("\n").map(|s| String::from(s)).collect();
                Msg::AnnotationUrlsChanged(list)
            },
            other => Msg::Error(format!("Got unexpected value: {:?}", other))
        });
        html! {
            <div class="two-column-lefty">
            <label class="form-description" for=URL_FIELD_NAME>{ "URL of the image" }</label>
            <input class="form-input" id=URL_FIELD_NAME onchange=on_url_changed value=url />
            <label class="form-description" for=ANNOS_FIELD_NAME>{ "URLs of the annotations" }</label>
            <textarea class="form-input" id=ANNOS_FIELD_NAME value=anno_urls onchange=on_anno_url_changed />
            </div>
        }
    }
}

const URL_FIELD_NAME: &str = "url_field";
const ANNOS_FIELD_NAME: &str = "annos_field";

impl AnnotatedImageForm {
    fn update_data(&mut self) {
        use crate::app_state::data_manager::Incoming;

        let data = Data::AnnotatedImage(self.props.image.clone());
        let id = self.props.id;
        self.data_manager.send(Incoming::UpdateData(id, data));
    }
}