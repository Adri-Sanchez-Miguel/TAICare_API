# tapo-api
Código en Rust para extraer información de los enchufes inteligentes TAPO P110. 

## Instalación

Si se instala en una Raspberry Pi limpia, tras actualizar e instalar todas las librerías, hará falta tener instalado "Rust", "open-ssl" y "arp-scan" en el dispositivo. 

```bash
sudo apt update
sudo apt upgrade

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install libssl-dev
sudo apt install arp-scan
```

## Ejecución

Para su ejecución, habrá que establecer las variables de entorno "TAPO_USERNAME" (nombre de usuario con el que se registran los dispositivos en la aplicación Tapo), "TAPO_PASSWORD" (contraseña con la que se registran los dispositivos en la aplicación Tapo), "MONGODB_URI" (consultar con el autor, para añadir también la IP del dispositivo a las permitidas en la web de MongoDB Atlas), "USE_DOCKER". 

```bash
export TAPO_USERNAME=
export TAPO_PASSWORD=
export MONGODB_URI=
export USE_DOCKER='false'

cargo build
cargo run --release
```

## Docker

En caso de querer ejecutar esta API utilizando Docker para una mayor independencia del hardware, se utilizará la siguiente imagen:

```bash
docker pull adriansanchez2902/tapo-main:latest
docker run -e TAPO_USERNAME='nombre_usuario' -e TAPO_PASSWORD='password' -e MONGODB_URI='mongodb_uri' -e USE_DOCKER='true' --net=host adriansanchez2902/tapo-main:latest
```

De igual manera, para esta solución tambien habrá que consultar la URI de MongoDB y cerciorarse de que la IP desde la que se trabaja está añadida a la lista de permitidas en MongoDB Atlas.

Creada por Adrián Sánchez-Miguel para el grupo de investigación MAmI.
