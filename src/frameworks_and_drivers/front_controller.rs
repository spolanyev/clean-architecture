//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::frameworks_and_drivers::interfaces::dispatcher_interface::DispatcherInterface;
use crate::frameworks_and_drivers::interfaces::front_controller_interface::FrontControllerInterface;
use crate::frameworks_and_drivers::interfaces::http_request_interface::HttpRequestInterface;
use crate::frameworks_and_drivers::interfaces::http_response_interface::HttpResponseInterface;
use crate::frameworks_and_drivers::interfaces::route_interface::RouteInterface;
use crate::frameworks_and_drivers::interfaces::router_interface::RouterInterface;

pub struct FrontController<'a> {
    dispatcher: &'a Box<dyn DispatcherInterface + 'a>,
    router: Box<dyn RouterInterface>,
}

impl<'a> FrontController<'a> {
    pub fn new(
        dispatcher: &'a Box<dyn DispatcherInterface + 'a>,
        router: Box<dyn RouterInterface>,
    ) -> Self {
        Self { dispatcher, router }
    }
}

impl<'a> FrontControllerInterface for FrontController<'a> {
    type Request = Box<dyn HttpRequestInterface>;
    type Route = Box<dyn RouteInterface>;
    type Router = Box<dyn RouterInterface>;
    type Dispatcher = Box<dyn DispatcherInterface>;
    type Response = Box<dyn HttpResponseInterface>;

    fn delegate(&self, request: Self::Request) -> Self::Response {
        self.dispatcher.dispatch(&self.router, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks_and_drivers::factory::Factory;
    use crate::frameworks_and_drivers::interfaces::factory_interface::FactoryInterface;
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

        let route = Box::new(Route::new(HttpMethod::Get, "/words/*", "find_word"));

        let mut router: Box<dyn RouterInterface> = Box::new(Router::new());

        router.add_route(route);

        let factory: Box<dyn FactoryInterface> = Box::new(Factory::new());

        let dispatcher: Box<dyn DispatcherInterface> = Box::new(Dispatcher::new(&factory));

        let front_controller = FrontController::new(&dispatcher, router);

        let http_response = front_controller.delegate(http_request);

        assert!(http_response
            .view_response()
            .contains("Word \"ability\" is found"));

        let http_request: Box<dyn HttpRequestInterface> = Box::new(HttpRequest::new(
            HttpMethod::Get,
            "/words/qazxsw".to_owned(),
            Some("ability".to_owned()),
        ));

        let http_response = front_controller.delegate(http_request);

        assert!(http_response
            .view_response()
            .contains("Word \"qazxsw\" is not found"));
    }
}
