//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::application_business_rules::interfaces::word_unit_interface::WordUnitInterface;
use crate::frameworks_and_drivers::interfaces::dispatcher_interface::DispatcherInterface;
use crate::frameworks_and_drivers::interfaces::front_controller_interface::FrontControllerInterface;
use crate::frameworks_and_drivers::interfaces::http_request_interface::HttpRequestInterface;
use crate::frameworks_and_drivers::interfaces::http_response_interface::HttpResponseInterface;
use crate::frameworks_and_drivers::interfaces::route_interface::RouteInterface;
use crate::frameworks_and_drivers::interfaces::router_interface::RouterInterface;

pub struct FrontController {
    dispatcher: Box<dyn DispatcherInterface>,
    router: Box<dyn RouterInterface>,
    word_unit: Box<dyn WordUnitInterface>,
}

impl FrontController {
    pub fn new(
        dispatcher: Box<dyn DispatcherInterface>,
        router: Box<dyn RouterInterface>,
        word_unit: Box<dyn WordUnitInterface>,
    ) -> Self {
        Self {
            dispatcher,
            router,
            word_unit,
        }
    }
}

impl FrontControllerInterface for FrontController {
    type UnitOfWork = Box<dyn WordUnitInterface>;
    type Request = Box<dyn HttpRequestInterface>;
    type Route = Box<dyn RouteInterface>;
    type Router = Box<dyn RouterInterface>;
    type Dispatcher = Box<dyn DispatcherInterface>;
    type Response = Box<dyn HttpResponseInterface>;

    fn delegate(&mut self, request: Self::Request) -> Self::Response {
        self.dispatcher
            .dispatch(&self.router, request, &mut self.word_unit)
    }

    fn commit_changes(&mut self) {
        self.word_unit.commit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application_business_rules::interfaces::word_unit_interface::WordUnitInterface;
    use crate::enterprise_business_rules::interfaces::word_interface::WordInterface;
    use crate::frameworks_and_drivers::factory::Factory;
    use crate::frameworks_and_drivers::interfaces::factory_interface::FactoryInterface;
    use crate::frameworks_and_drivers::message::dispatcher::Dispatcher;
    use crate::frameworks_and_drivers::message::http_method::HttpMethod;
    use crate::frameworks_and_drivers::message::http_request::HttpRequest;
    use crate::frameworks_and_drivers::message::route::Route;
    use crate::frameworks_and_drivers::message::router::Router;
    use crate::interface_adapters::storage::word_unit::WordUnit;

    #[test]
    fn find_word() {
        let http_request: Box<dyn HttpRequestInterface> = Box::new(HttpRequest::new(
            HttpMethod::Get,
            "/words/ability".to_owned(),
            None,
        ));

        let route = Box::new(Route::new(HttpMethod::Get, "/words/*", "find_word"));

        let mut router: Box<dyn RouterInterface> = Box::new(Router::new());

        router.add_route(route);

        let word_unit: Box<dyn WordUnitInterface> = {
            let words: Vec<Box<dyn WordInterface>> = vec![];
            let mut word_unit = Box::new(WordUnit::new());
            word_unit.set_identity_map(words, false);
            word_unit
        };

        let factory: Box<dyn FactoryInterface> = Box::new(Factory::new());

        let dispatcher: Box<dyn DispatcherInterface> = Box::new(Dispatcher::new(factory));

        let mut front_controller = FrontController::new(dispatcher, router, word_unit);

        let http_response = front_controller.delegate(http_request);

        assert!(http_response
            .view_response()
            .contains("Word \"ability\" is found"));

        let http_request: Box<dyn HttpRequestInterface> = Box::new(HttpRequest::new(
            HttpMethod::Get,
            "/words/qazxsw".to_owned(),
            None,
        ));

        let http_response = front_controller.delegate(http_request);

        assert!(http_response
            .view_response()
            .contains("Word \"qazxsw\" is not found"));

        front_controller.commit_changes();
    }

    #[test]
    fn view_all() {
        let http_request: Box<dyn HttpRequestInterface> =
            Box::new(HttpRequest::new(HttpMethod::Get, "/words".to_owned(), None));

        let route = Box::new(Route::new(HttpMethod::Get, "/words", "view_all"));

        let mut router: Box<dyn RouterInterface> = Box::new(Router::new());

        router.add_route(route);

        let word_unit: Box<dyn WordUnitInterface> = Box::new(WordUnit::new());

        let factory: Box<dyn FactoryInterface> = Box::new(Factory::new());

        let dispatcher: Box<dyn DispatcherInterface> = Box::new(Dispatcher::new(factory));

        let mut front_controller = FrontController::new(dispatcher, router, word_unit);

        let http_response = front_controller.delegate(http_request);

        assert!(["a", "ability", "able"]
            .iter()
            .all(|&word| http_response.view_response().contains(word)));

        front_controller.commit_changes();
    }
}
