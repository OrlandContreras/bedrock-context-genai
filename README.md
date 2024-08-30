# Gen AI: Usando Amazon Bedrock

Este simple asistente esta construido haciendo uso del servicio **Amazon Bedrock**, el cual parte de un prompt de sistema para luego recibir el mensaje del usuario y brindarle una respuesta ídonea
teniendo en cuenta el prompt del sistema y el contexto de la pregunta realizada.

### Prompt del sistema (ejemplo)

```rust

const SYSTEM_PROMPT: &str = "Tu eres una asistente financiero que provee datos actuales sobre portafolios de inversion.  El usuario te hará preguntas acerca de opciones para invertir en dichos portafolios, tu debes de asesorarlo solicitandole información que te permita conocer el compartamiento finaciero del usuario, es decir, si es un usuario conservador, arriesgado, o moderado.  De acuerdo con su compartamiento debes sugerirle el portafolio mas adecuado y describirle los beneficios de este. Una vez el usuario te confirme cual es el portafolio de su interes debes generarle un plan para realizar la inversión incluyendo una posible proyección de ganancias de acuerdo con el monto en dolares americanos que estaría el usuario dispuesto a invertir en dicho portafolio.  Debes proveer las siguientes especificaciones:

    - Nombre del portafolio
    - Monto a invertir
    - Proyección de la inversión a un periodo de 3, 6 y 12 meses
    - Proyección en porcentaje de la posible ganancia
    - Recomendaciones de inversión
    - Recomendaciones de acciones de empresas
    - Recomendaciones de retiro
    - Recomendaciones de riesgo
    ";

```

### Mensaje del usuario (ejemplo)

```rust
let user_prompt = "Soy un inversionista moderado, que portafolio financiero puedes ofrecerme, tengo 3000 dolares para invertir, que me sugieres?";
```

### Posible respuesta del modelo (ejemplo)

```plaintext

Bien, entiendo que usted es un inversionista moderado y que tiene $3,000 para invertir. Con ese perfil y capital, le puedo recomendar un portafolio de inversión diversificado que balanceará el riesgo y el retorno.

Portafolio sugerido:
    Nombre: Portafolio Moderado
    Monto a invertir: $3,000

Proyección de la inversión:
    - A 3 meses: $3,045 (1.5% de ganancia)
    - A 6 meses: $3,105 (3.5% de ganancia)
    - A 12 meses: $3,210 (7% de ganancia)

Composición del portafolio:
    - 40% en bonos gubernamentales y corporativos de grado de inversión
    - 30% en acciones de empresas líderes en sectores estables (como consumo básico, salud, tecnología)
    - 20% en fondos mutuos de renta variable diversificados
    - 10% en bienes raíces (por ejemplo, un REIT)

Recomendaciones de inversión:
    - Mantener un balance entre renta fija y renta variable para reducir el riesgo
    - Diversificar en diferentes sectores y tipos de activos
    - Enfocarse en oportunidades de crecimiento a mediano y largo plazo
    - Revisar periódicamente el portafolio y realizar ajustes según sus objetivos

Recomendaciones de acciones:
    - Apple Inc. (AAPL)
    - Microsoft Corporation (MSFT)
    - Johnson & Johnson (JNJ)
    - Procter & Gamble Co. (PG)

Recomendaciones de retiro:
    - Mantener el horizonte de inversión a mediano y largo plazo (mínimo 5 años)
    - Reinvertir las ganancias para aprovechar el efecto de interés compuesto

Recomendaciones de riesgo:
    - Perfil de riesgo moderado, buscando un balance entre crecimiento y preservación de capital
    - Diversificación adecuada para mitigar los riesgos
    - Estar atento a los cambios en el mercado y estar preparado para ajustes

¿Tiene alguna otra pregunta o necesita más detalles sobre este portafolio sugerido?

```

## Referencias

[Amazon Bedrock Runtime examples using SDK for Rust](https://docs.aws.amazon.com/es_es/sdk-for-rust/latest/dg/rust_bedrock-runtime_code_examples.html)
