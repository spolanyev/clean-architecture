//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::frameworks_and_drivers::interfaces::dispatcher_interface::DispatcherInterface;
use crate::frameworks_and_drivers::interfaces::front_controller_interface::FrontControllerInterface;
use crate::frameworks_and_drivers::interfaces::http_request::HttpRequestInterface;
use crate::frameworks_and_drivers::interfaces::route_interface::RouteInterface;
use crate::frameworks_and_drivers::interfaces::router_interface::RouterInterface;

struct FrontController {
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

    fn delegate(&self, request: Self::Request) {
        self.dispatcher.dispatch(&self.router, request);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks_and_drivers::dispatcher::Dispatcher;
    use crate::frameworks_and_drivers::http_method::HttpMethod;
    use crate::frameworks_and_drivers::http_request::HttpRequest;
    use crate::frameworks_and_drivers::route::Route;
    use crate::frameworks_and_drivers::router::Router;

    #[test]
    fn find_word() {
        let request: Box<dyn HttpRequestInterface> = Box::new(HttpRequest::new(
            HttpMethod::Get,
            "/words/ability".to_owned(),
        ));

        let route = Box::new(Route::new(
            HttpMethod::Get,
            "/words/ability".to_owned(),
            "find_word".to_owned(),
        ));

        let mut router = Box::new(Router::new());
        router.add_route(route);

        let dispatcher = Box::new(Dispatcher::new());

        let controller = FrontController::new(dispatcher, router);

        let nothing = controller.delegate(request);

        assert_eq!((), nothing);
    }
}
