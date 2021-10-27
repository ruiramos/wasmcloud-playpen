// translate.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.example.interfaces.translate", crate: "translation" } ]

namespace org.example.interfaces.translate

use org.wasmcloud.model#wasmbus

/// Description of Translate service
@wasmbus( actorReceive: true )
service Translate {
  version: "0.1",
  operations: [ Convert ]
}

/// Converts the input string to a result
operation Convert {
  input: TranslateInput,
  output: TranslateOutput
}

structure TranslateInput {
  @required
  message: String

  @required
  lang: String

  @required
  target_lang: String
}

structure TranslateOutput {
  @required
  success: Boolean

  message: String

  @box
  error: Integer
}
