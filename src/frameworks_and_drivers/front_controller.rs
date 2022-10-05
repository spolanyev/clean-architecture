//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::frameworks_and_drivers::interfaces::dispatcher_interface::DispatcherInterface;
use crate::frameworks_and_drivers::interfaces::front_controller_interface::FrontControllerInterface;
use crate::frameworks_and_drivers::interfaces::http_request_interface::HttpRequestInterface;
use crate::frameworks_and_drivers::interfaces::http_response_interface::HttpResponseInterface;
use crate::frameworks_and_drivers::interfaces::route_interface::RouteInterface;
use crate::frameworks_and_drivers::interfaces::router_interface::RouterInterface;

pub struct FrontController {
    dispatcher: Box<dyn DispatcherInterface>,
    router: Box<dyn RouterInterface>,
}

impl FrontController {
    pub fn new(dispatcher: Box<dyn DispatcherInterface>, router: Box<dyn RouterInterface>) -> Self {
        Self { dispatcher, router }
    }
}

impl FrontControllerInterface for FrontController {
    type Request = Box<dyn HttpRequestInterface>;
    type Route = Box<dyn RouteInterface>;
    type Router = Box<dyn RouterInterface>;
    type Dispatcher = Box<dyn DispatcherInterface>;
    type Response = Box<dyn HttpResponseInterface>;

    fn delegate(&self, mut request: Self::Request) -> Self::Response {
        self.dispatcher.dispatch(&self.router, &mut request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks_and_drivers::message::dispatcher::Dispatcher;
    use crate::frameworks_and_drivers::message::http_method::HttpMethod;
    use crate::frameworks_and_drivers::message::http_request::HttpRequest;
    use crate::frameworks_and_drivers::message::route::Route;
    use crate::frameworks_and_drivers::message::router::Router;

    #[test]
    fn find_word() {
        let http_request: Box<dyn HttpRequestInterface> = Box::new(HttpRequest::new(
            HttpMethod::Get,
            "/words/ability".to_owned(),
            Some("ability".to_owned()),
        ));

        let route = Box::new(Route::new(
            HttpMethod::Get,
            "/words/*".to_owned(),
            "find_word".to_owned(),
        ));

        let mut router = Box::new(Router::new());

        router.add_route(route);

        let dispatcher = Box::new(Dispatcher::new());

        let front_controller = FrontController::new(dispatcher, router);

        let _http_response = front_controller.delegate(http_request);

        //TODO
        assert_eq!((), ());
    }
}
