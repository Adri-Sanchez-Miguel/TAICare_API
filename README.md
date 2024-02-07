# tapo-api
Código en Rust para extraer información de los enchufes inteligentes TAPO P110. Para su ejecución hará falta tener instalado Rust y open-ssl en el dispositivo. 

Para su ejecución, habrá que establecer las variables de entorno "TAPO_USERNAME", "TAPO_PASSWORD" y "MONGODB_URI". 

Después, "cargo build", "cargo run --example tapo_p110_multiple.rs".

Creada por Adrián Sánchez-Miguel para el grupo de investigación MAmI.
