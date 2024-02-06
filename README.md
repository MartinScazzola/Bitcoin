# Trabájo Práctico - Rusticos-AGVM

Este repositorio contiene la resolución del trabajo práctico de la materia Taller de Programación

Consiste en la implementación de un nodo de Bitcoin y una Wallet en el lenguage de programación Rust con una interfaz gráfica diseñada usando la biblioteca `gtk` y el editor Glade.

El proyecto se desarrolló en dos crates: Wallet y Node.

## Wallet
Contiene la lógica del diseño de la interfaz gráfica y de la billetera virtual.
Se encarga de la comunicación entre el Usuario y la Wallet, la cual a su vez se conecta con el Nodo.

## Node
Contiene la lógica del nodo de Bitcoin cumpliendo con su protocolo de comunicación.
Tiene implementado los mensajes y envía la información necesaria a la Wallet.
El Nodo se puede ejecutar como servidor o como cliente dependiendo del archivo de configuración.

## Documentación
EL proyecto cuenta con una documentación que sigue el estándar de Rust. 
Se puede acceder a la misma corriendo el siguiente comando en el crate correspondiente

```shell
cargo doc --open
```
## Ejecución
Para ejecutar el proyecto se deben seguir los siguientes pasos:

1. Abrir 2 terminales, una para cada crate. 

2. El crate Node necesita como parámetro la ruta al archivo de configuración.
   Entonces, sobre este crate, se ejecuta lo siguiente
   
```shell
cargo run settings/server_node.conf
```
3. En el crate Wallet no se necesitan parámetros adicionales.
   Esta terminal iniciará la aplicación la cual permite al usuario loguearse para poder usarla.
   Se debe ejecutar simplemente
```shell
cargo run 
```
   Cabe destacar que hasta que no finalice la descarga de bloques en el nodo, no se podrá realizar el logueo.
   Se mostrará una pantalla de carga mostrando el progreso de la descaga de bloques.

   ![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/143e25d4-8b53-4232-99c9-732ae29c3148)

   
4. Una vez que esté todo descargado, se pasa a la etapa de logueo

## Logueo

Al iniciar el programa, se debe ingresar: 
- Un nombre de usuario personalizado
- Una public key
- Una private key

Se asume que las claves son válidas para el correcto funcionamiento de la wallet. 
Esta solo chequea el formato. 
En caso de que un campo no cumpla con el formato, se muestra un popup indicando el error. 

![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/d03fb929-5afc-4784-b99a-139e257d0995)

## Overview
En la pestaña Overview se observa una vista general de la wallet. 
Se muestra el balance del usuario y sus transacciones recientes.
![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/48d87751-1477-4825-ac03-91cc3cb7bf1e)


## Send
La pestaña Send permite enviar transacciones a otros usuarios a partir de su address. 
Se puede cambiar el valor de la fee y de la cantidad de dinero de la transacción en múltiples outputs.
En caso de no contar con fondos suficientes para realizar la transacción, se notificará al usuario.
![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/b2dc7c1b-588e-4c81-919f-9c6a437a95d8)


## Accounts
En la pestaña Accounts se permite copiar la public key para poder compartirla. 
Además se encuentran las cuentas que el usuario tiene cargadas en la Wallet.
![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/428a69f2-9ce7-4e58-b6db-c8f0c9d32ab3)


## Transactions
Esta pestaña muestra las transacciones enviadas y recibidas de la cuenta actual.
![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/f702b80d-5ade-4d62-87a3-1e461275da5f)

## Proof
La pestaña tiene dos entradas: 
- ID de una transacción
- ID de un bloque.

Una vez llenados los campos, se le indica al usuario si la transacción pertenece al bloque o no a través de un popup.
![image](https://github.com/taller-1-fiuba-rust/23C1-Rusticos-AGVM/assets/71946855/0ef95a55-72eb-483f-af0a-979384042ff2)

## Prueba
En caso de que lo desee, puede utilizar las siguiente claves para probar la wallet
- Public Key: `02BE02F74874C31372E4779581FCA7459E8E2AFCF5AB9D3727BE43D489D030FD3F` 
- Private Key: `E0883A34FA833E81449B42EE098DF0AD5BA73A8B3BA5442F9B26D2F068E306D8` 
- Address: `mgCrQGPUouPZ8WefywVsX8anySGpD5xk1b`

