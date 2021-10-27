use translate_interface::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct HelloActor {}

const TRANSLATE_ACTOR: &str = "translate/server";

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for HelloActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let message = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "message")
            .map(|(_, v)| v.to_string());

        if message.is_none() {
            return Ok(HttpResponse {
                status_code: 400,
                body: "Required param message missing"
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                ..Default::default()
            });
        };

        let lang = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "lang")
            .map(|(_, v)| v.to_string());

        if lang.is_none() {
            return Ok(HttpResponse {
                status_code: 400,
                body: "Required param lang missing"
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                ..Default::default()
            });
        };

        let target_lang = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "target_lang")
            .map(|(_, v)| v.to_string());

        if target_lang.is_none() {
            return Ok(HttpResponse {
                status_code: 400,
                body: "Required param target_lang missing"
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                ..Default::default()
            });
        };

        let input: TranslateInput = {
            TranslateInput {
                message: message.unwrap(),
                lang: lang.unwrap(),
                target_lang: target_lang.unwrap(),
            }
        };

        let translate_res = TranslateSender::to_actor(TRANSLATE_ACTOR)
            .convert(ctx, &input)
            .await?;

        if translate_res.success {
            Ok(HttpResponse {
                body: translate_res.message.unwrap().as_bytes().to_vec(),
                ..Default::default()
            })
        } else {
            Ok(HttpResponse {
                status_code: translate_res.error.unwrap() as u16,
                body: translate_res.message.unwrap().as_bytes().to_vec(),
                ..Default::default()
            })
        }
    }
}
