//@author Stanislav Polaniev <spolanyev@gmail.com>

use dictionary::frameworks_and_drivers::factory::Factory;
use dictionary::frameworks_and_drivers::front_controller::FrontController;
use dictionary::frameworks_and_drivers::interfaces::dispatcher_interface::DispatcherInterface;
use dictionary::frameworks_and_drivers::interfaces::factory_interface::FactoryInterface;
use dictionary::frameworks_and_drivers::interfaces::front_controller_interface::FrontControllerInterface;
use dictionary::frameworks_and_drivers::interfaces::http_request_interface::HttpRequestInterface;
use dictionary::frameworks_and_drivers::interfaces::http_response_interface::HttpResponseInterface;
use dictionary::frameworks_and_drivers::interfaces::router_interface::RouterInterface;
use dictionary::frameworks_and_drivers::message::dispatcher::Dispatcher;
use dictionary::frameworks_and_drivers::message::http_method::HttpMethod;
use dictionary::frameworks_and_drivers::message::http_request::HttpRequest;
use dictionary::frameworks_and_drivers::message::http_response::HttpResponse;
use dictionary::frameworks_and_drivers::message::http_status::HttpStatus;
use dictionary::frameworks_and_drivers::message::route::Route;
use dictionary::frameworks_and_drivers::message::router::Router;
use std::net::TcpListener;
use std::thread;

fn main() {
    let mut routes = vec![
        Route::new(HttpMethod::Get, "/words/*", "find_word"),
        Route::new(HttpMethod::Get, "/words", "view_all"),
    ];

    let mut router: Box<dyn RouterInterface> = Box::new(Router::new());

    while let Some(route) = routes.pop() {
        router.add_route(Box::new(route));
    }

    let factory: Box<dyn FactoryInterface> = Box::new(Factory::new());

    let dispatcher: Box<dyn DispatcherInterface> = Box::new(Dispatcher::new(&factory));

    let front_controller = FrontController::new(&dispatcher, router);

    let listener = TcpListener::bind("127.0.0.1:80").expect("Failed to bind TcpListener");
    for tcp_stream in listener.incoming() {
        let mut tcp_stream = tcp_stream.expect("Failed to unwrap tcp");
        thread::scope(|scope| {
            scope.spawn(|| {
                let http_request = HttpRequest::from_tcp_stream(&mut tcp_stream);
                if let Some(http_request) = http_request {
                    let http_request: Box<dyn HttpRequestInterface> = Box::new(http_request);
                    let http_response = front_controller.delegate(http_request);
                    http_response.respond(tcp_stream);
                } else {
                    let mut http_response = HttpResponse::new();
                    http_response.set_http_status(HttpStatus::BadRequest);
                    http_response.set_content(HttpStatus::BadRequest.get_description());
                    http_response.build();
                    http_response.respond(tcp_stream);
                }
            });
        });
    }
}
