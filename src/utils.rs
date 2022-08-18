use crate::class::ServiceNodeConfig;

pub fn get_service(services: &Vec<ServiceNodeConfig>, name: String) -> Option<ServiceNodeConfig> {
    for service in services.clone().into_iter() {
        if service.name == name {
            return Some(service);
        }
    }
    None
}
