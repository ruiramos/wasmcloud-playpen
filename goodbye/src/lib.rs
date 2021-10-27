#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use translate_interface::*;
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Translate)]
struct GoodbyeActor {}

#[async_trait]
impl Translate for GoodbyeActor {
    async fn convert(&self, _ctx: &Context, input: &TranslateInput) -> RpcResult<TranslateOutput> {
        let dict = DICTIONARY.get(&format!("{}_{}", &input.lang, &input.target_lang) as &str);

        if let Some(dict) = dict {
            let response = match dict.get(&input.message as &str) {
                Some(t) => TranslateOutput {
                    message: Some(t.to_string()),
                    success: true,
                    error: None,
                },
                None => TranslateOutput {
                    success: false,
                    error: Some(404),
                    message: Some("Not found".to_string()),
                },
            };
            Ok(response)
        } else {
            return Ok({
                TranslateOutput {
                    message: Some(format!(
                        "Dictionary not fount for {} to {}",
                        &input.lang, &input.target_lang
                    )),
                    success: false,
                    error: Some(500),
                }
            });
        }
    }
}

lazy_static! {
    static ref EN_PT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("hello", "ola");
        m.insert("goodbye", "adeus");
        m.insert("thanks", "obrigado");
        m.insert("please", "por favor");
        m
    };
    static ref PT_EN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ola", "hello");
        m.insert("adeus", "goodbye");
        m.insert("obrigado", "thanks");
        m.insert("por favor", "please");
        m
    };
    static ref DICTIONARY: HashMap<&'static str, &'static HashMap<&'static str, &'static str>> = {
        let mut d = HashMap::new();
        d.insert("pt_en", &*PT_EN);
        d.insert("en_pt", &*EN_PT);
        d
    };
}
