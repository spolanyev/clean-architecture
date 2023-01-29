//@author Stanislav Polaniev <spolanyev@gmail.com>

use dictionary::application_business_rules::interfaces::word_unit_interface::WordUnitInterface;
use dictionary::frameworks_and_drivers::environment::Environment;
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
use dictionary::interface_adapters::storage::word_unit::WordUnit;
use std::net::TcpListener;
use std::sync::{Arc, Condvar, Mutex};
use std::{env, thread};

fn main() {
    if env::var("RUST_ENV").is_err() {
        env::set_var("RUST_ENV", Environment::get_rust_env(".env"));
    }

    let factory: Box<dyn FactoryInterface> = Box::new(Factory::new());
    let dispatcher: Box<dyn DispatcherInterface> = Box::new(Dispatcher::new(factory));
    let word_unit: Box<dyn WordUnitInterface> = Box::new(WordUnit::new());
    let router: Box<dyn RouterInterface> = {
        let mut routes = vec![
            Route::new(HttpMethod::Get, "/words/*", "find_word"),
            Route::new(HttpMethod::Get, "/words", "view_all"),
            Route::new(HttpMethod::Post, "/words", "add_word"),
            Route::new(HttpMethod::Put, "/words", "update_word"),
            Route::new(HttpMethod::Delete, "/words/*", "delete_word"),
        ];

        let mut router = Box::new(Router::new());
        while let Some(route) = routes.pop() {
            router.add_route(Box::new(route));
        }
        router
    };
    let mut front_controller = FrontController::new(dispatcher, router, word_unit);

    let listener = TcpListener::bind("127.0.0.1:80").expect("Could not start server");
    let active_connections = Arc::new((Mutex::new(0), Condvar::new()));
    let connection_limit = 100;
    for stream in listener.incoming() {
        let active_connections = Arc::clone(&active_connections);
        match stream {
            Ok(mut tcp_stream) => {
                thread::scope(|scope| {
                    let (connections, semaphore) = &*active_connections;
                    let mut connections = connections.lock().expect("Lock failed");

                    while *connections >= connection_limit {
                        connections = semaphore.wait(connections).expect("Wait failed");
                    }
                    *connections += 1;

                    let handle = scope.spawn(|| {
                        let http_request = HttpRequest::from_tcp_stream(&mut tcp_stream);
                        if let Some(http_request) = http_request {
                            let http_request: Box<dyn HttpRequestInterface> =
                                Box::new(http_request);
                            let http_response = front_controller.delegate(http_request);
                            front_controller.commit_changes();
                            http_response.respond(tcp_stream);
                        } else {
                            let mut http_response = HttpResponse::new();
                            http_response.set_http_status(HttpStatus::BadRequest);
                            http_response.set_content(HttpStatus::BadRequest.get_description());
                            http_response.build();
                            http_response.respond(tcp_stream);
                        }
                    });

                    if let Err(error) = handle.join() {
                        println!("Thread panicked, error is `{error:#?}`");
                    }

                    *connections -= 1;
                    semaphore.notify_one();
                });
            }
            Err(error) => println!("Connection failed, error is `{error:#?}`"),
        }
    }
}
