# About

It is an English-Russian Dictionary (although I am Ukrainian, my native language is Russian)

This project is done in OOP-paradigm with TDD according to Uncle Bob's interpretation of clean architecture (a real one, where outer circles' items are not referred to in the inner circles - as opposed to most implementations online)

![Clean Architecture Scheme](https://github.com/spolanyev/rust-oop-tdd-clean-architecture-web-api/blob/main/CleanArchitecture.jpg?raw=true)

Patterns used:


* Creational
  - [Abstract Factory](src/frameworks_and_drivers/factory.rs)


* Structural
  - [Adapter](src/frameworks_and_drivers/storage/file_storage_adapter.rs)
  - [Repository](src/interface_adapters/storage/word_repository.rs)
  - [Separated Interface](src/application_business_rules/interfaces/word_repository_interface.rs)


* Behavioral
  - [Identity Map](src/interface_adapters/storage/word_unit.rs)
  - [Iterator](src/enterprise_business_rules/entities/word_collection.rs)
  - [Pessimistic Offline Lock](src/interface_adapters/storage/word_unit.rs)
  - [Unit of Work](src/interface_adapters/storage/word_unit.rs)


* Architectural
  - [Application Controller](src/frameworks_and_drivers/application_controller.rs)
  - [Clean Architecture (Robert C. Martin's variant)](src)
  - [Data Mapper](src/interface_adapters/storage/word_mapper.rs)
  - [Front Controller](src/frameworks_and_drivers/front_controller.rs)
  - [MVC](src/interface_adapters/controllers/find_word_controller.rs)

The simplified flow is [FrontControllerInterface](src/frameworks_and_drivers/interfaces/front_controller_interface.rs) -> [DispatcherInterface](src/frameworks_and_drivers/interfaces/dispatcher_interface.rs) -> [ApplicationControllerInterface](src/frameworks_and_drivers/interfaces/application_controller_interface.rs) -> [WordControllerInterface](src/interface_adapters/interfaces/word_controller_interface.rs) -> [UseCaseInterface](src/application_business_rules/interfaces/use_case_interface.rs) -> [PresenterInterface](src/application_business_rules/interfaces/presenter_interface.rs) -> [ViewInterface](src/frameworks_and_drivers/interfaces/view_interface.rs) 


# Contacts

[spolanyev@gmail.com](mailto:spolanyev@gmail.com?subject=Rust%3A%20dictionary)
