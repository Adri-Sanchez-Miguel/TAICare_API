# tapo-api
Código en Rust para extraer información de los enchufes inteligentes TAPO P110. 

## Instalación

Si se instala en una Raspberry Pi limpia, tras "sudo apt update" y "sudo apt upgrade", hará falta tener instalado Rust ("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh") y open-ssl ("sudo apt install libssl-dev") en el dispositivo. 

Para su ejecución, habrá que establecer las variables de entorno "TAPO_USERNAME", "TAPO_PASSWORD" y "MONGODB_URI" (consultar con el autor, para añadir también la IP del dispositivo a las permitidas). 

## Ejecución

Después, "cargo build", "cargo run --example tapo_p110_multiple".

Creada por Adrián Sánchez-Miguel para el grupo de investigación MAmI.
