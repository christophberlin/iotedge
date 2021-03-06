// Copyright (c) Microsoft. All rights reserved.

use std::marker::PhantomData;
use std::time::Duration;

use edgelet_core::*;
use failure::Fail;
use futures::future::{self, FutureResult};
use futures::prelude::*;
use futures::IntoFuture;
use hyper::Body;

#[derive(Clone, Debug)]
pub struct NullRegistry<E: Fail> {
    phantom: PhantomData<E>,
}

impl<E: Fail> NullRegistry<E> {
    pub fn new() -> Self {
        NullRegistry {
            phantom: PhantomData,
        }
    }
}

impl<E: Fail> Default for NullRegistry<E> {
    fn default() -> Self {
        NullRegistry::new()
    }
}

impl<E: Fail> ModuleRegistry for NullRegistry<E> {
    type Error = E;
    type PullFuture = FutureResult<(), Self::Error>;
    type RemoveFuture = FutureResult<(), Self::Error>;
    type Config = TestConfig;

    fn pull(&self, _config: &Self::Config) -> Self::PullFuture {
        future::ok(())
    }

    fn remove(&self, _name: &str) -> Self::RemoveFuture {
        future::ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestConfig {
    image: String,
}

impl TestConfig {
    pub fn new(image: String) -> Self {
        TestConfig { image }
    }

    pub fn image(&self) -> &str {
        &self.image
    }
}

#[derive(Clone, Debug)]
pub struct TestModule<E: Fail> {
    name: String,
    config: TestConfig,
    state: Result<ModuleRuntimeState, E>,
}

impl<E: Fail> TestModule<E> {
    pub fn new(name: String, config: TestConfig, state: Result<ModuleRuntimeState, E>) -> Self {
        TestModule {
            name,
            config,
            state,
        }
    }
}

impl<E: Clone + Fail> Module for TestModule<E> {
    type Config = TestConfig;
    type Error = E;
    type RuntimeStateFuture = FutureResult<ModuleRuntimeState, Self::Error>;

    fn name(&self) -> &str {
        &self.name
    }

    fn type_(&self) -> &str {
        "test"
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn runtime_state(&self) -> Self::RuntimeStateFuture {
        self.state.clone().into_future()
    }
}

#[derive(Clone)]
pub struct TestRuntime<E: Fail> {
    module: Result<TestModule<E>, E>,
    registry: NullRegistry<E>,
}

impl<E: Fail> TestRuntime<E> {
    pub fn new(module: Result<TestModule<E>, E>) -> Self {
        TestRuntime {
            module,
            registry: NullRegistry::new(),
        }
    }
}

pub struct EmptyBody<E> {
    phantom: PhantomData<E>,
}

impl<E> EmptyBody<E> {
    pub fn new() -> Self {
        EmptyBody {
            phantom: PhantomData,
        }
    }
}

impl<E> Default for EmptyBody<E> {
    fn default() -> Self {
        EmptyBody::new()
    }
}

impl<E> Stream for EmptyBody<E> {
    type Item = String;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}

impl<E> From<EmptyBody<E>> for Body {
    fn from(_: EmptyBody<E>) -> Body {
        Body::empty()
    }
}

impl<E: Clone + Fail> ModuleRuntime for TestRuntime<E> {
    type Error = E;
    type Config = TestConfig;
    type Module = TestModule<E>;
    type ModuleRegistry = NullRegistry<E>;
    type Chunk = String;
    type Logs = EmptyBody<Self::Error>;

    type CreateFuture = FutureResult<(), Self::Error>;
    type InitFuture = FutureResult<(), Self::Error>;
    type ListFuture = FutureResult<Vec<Self::Module>, Self::Error>;
    type LogsFuture = FutureResult<Self::Logs, Self::Error>;
    type RemoveFuture = FutureResult<(), Self::Error>;
    type RestartFuture = FutureResult<(), Self::Error>;
    type StartFuture = FutureResult<(), Self::Error>;
    type StopFuture = FutureResult<(), Self::Error>;
    type SystemInfoFuture = FutureResult<SystemInfo, Self::Error>;
    type RemoveAllFuture = FutureResult<(), Self::Error>;

    fn system_info(&self) -> Self::SystemInfoFuture {
        match self.module {
            Ok(_) => future::ok(SystemInfo::new(
                "os_type_sample".to_string(),
                "architecture_sample".to_string(),
            )),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn init(&self) -> Self::InitFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn create(&self, _module: ModuleSpec<Self::Config>) -> Self::CreateFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn start(&self, _id: &str) -> Self::StartFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn stop(&self, _id: &str, _wait_before_kill: Option<Duration>) -> Self::StopFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn restart(&self, _id: &str) -> Self::RestartFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn remove(&self, _id: &str) -> Self::RemoveFuture {
        match self.module {
            Ok(_) => future::ok(()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn list(&self) -> Self::ListFuture {
        match self.module {
            Ok(ref m) => future::ok(vec![m.clone()]),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn logs(&self, _id: &str, _options: &LogOptions) -> Self::LogsFuture {
        match self.module {
            Ok(ref _m) => future::ok(EmptyBody::new()),
            Err(ref e) => future::err(e.clone()),
        }
    }

    fn registry(&self) -> &Self::ModuleRegistry {
        &self.registry
    }

    fn remove_all(&self) -> Self::RemoveAllFuture {
        future::ok(())
    }
}
