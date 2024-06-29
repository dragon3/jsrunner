use tonic::{transport::Server, Request, Response, Status};

use jsrunner::js_runner_service_server::{JsRunnerService, JsRunnerServiceServer};
use jsrunner::{RunRequest, RunResponse};

pub mod jsrunner {
    tonic::include_proto!("jsrunner");
}

#[derive(Default)]
pub struct Svc {}

#[tonic::async_trait]
impl JsRunnerService for Svc {
    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        let params = v8::CreateParams::default().heap_limits(0, 1 * 1024 * 1024); // 1 MB
        let isolate = &mut v8::Isolate::new(params);

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, request.into_inner().code.as_str()).unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("result: {}", result.to_rust_string_lossy(scope));

        let reply = jsrunner::RunResponse {
            result: result.to_rust_string_lossy(scope),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let svc = Svc::default();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    println!("RunnerService listening on {}", addr);

    Server::builder()
        .add_service(JsRunnerServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
