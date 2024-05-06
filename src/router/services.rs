use kube::{
    Client,
    api::{
        Api,
        ListParams
    }
};
use k8s_openapi::api::core::v1::{Service, ServicePort};

use axum::{
    extract::State, 
    response::Html
};
use tera::{Tera, Context};
use std::sync::Arc;
use serde::Serialize;

use crate::Result;


#[derive(Serialize)]
struct KubeService {
    name: String,
    service_ports: Vec<ServicePort>
}


async fn get_services() -> Result<Api<Service>> {
    let client = Client::try_default().await?;
    let services: Api<Service> = Api::all(client);
    
    Ok(services)
}

#[axum::debug_handler]
pub async fn services(
    State(shared_state): State<Arc<Tera>> 
) -> Html<String> {
    let api_services = get_services().await.unwrap();

    let lp = ListParams::default();

    let res = api_services
        .list(&lp)
        .await
        .unwrap();

    // let services = res.items.clone();
    let services: Vec<KubeService> = res.items
        .iter()
        .filter(|s| s.spec.to_owned().unwrap().ports.is_some_and(|p| p[0].node_port.is_some()))
        .map(|s| KubeService {
            name: s.metadata.name.to_owned().unwrap(), 
            service_ports: s.spec
                .to_owned()
                .unwrap()
                .ports
                .unwrap()
        })
        .collect();

    let mut service_context= Context::new();
    service_context.insert("services", &services);

    let services_rendered = shared_state.render("views/services.html", &service_context).unwrap();
    
    Html(services_rendered)

}
