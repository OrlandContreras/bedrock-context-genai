use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseError,
    types::{ContentBlock, ConversationRole, Message, SystemContentBlock},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), ConverseError> {
    let sdk_config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&sdk_config);

    // Build System prompt
    const SYSTEM_PROMPT: &str = "Tu eres una asistente financiero que provee datos actuales sobre portafolios de inversion.
    El usuario te hará preguntas acerca de opciones para invertir en dichos portafolios, tu debes de asesorarlo solicitandole información que te permita conocer el compartamiento finaciero del usuario, es decir,
    si es un usuario conservador, arriesgado, o moderado.  De acuerdo con su compartamiento debes sugerirle el portafolio mas adecuado y describirle los beneficios de este.
    Una vez el usuario te confirme cual es el portafolio de su interes debes generarle un plan para realizar la inversión incluyendo una posible proyección de ganancias de acuerdo con el monto en dolares americanos que estaría
    el usuario dispuesto a invertir en dicho portafolio.  Debes proveer las siguientes especificaciones:
        - Nombre del portafolio
        - Monto a invertir
        - Proyección de la inversión a un periodo de 3, 6 y 12 meses
        - Proyección en porcentaje de la posible ganancia
        - Recomendaciones de inversión
        - Recomendaciones de acciones de empresas
        - Recomendaciones de retiro
        - Recomendaciones de riesgo
    ";
    let user_prompt = "Soy un inversionista moderado, que portafolio financiero puedes ofrecerme, tengo 3000 dolares para invertir, que me sugieres?";

    let system_prompt = SystemContentBlock::Text(SYSTEM_PROMPT.into());

    let bedrock_response = client
        .converse()
        .model_id("anthropic.claude-3-haiku-20240307-v1:0")
        .messages(
            Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(user_prompt.to_string()))
                .build()
                .map_err(|_| "Failed to build message")
                .unwrap(),
        )
        .system(system_prompt)
        .send()
        .await;

    println!("{:?}", bedrock_response);

    Ok(())
}
