# GameVault-Hub - Solana
Este proyecto es un sistema de gestión de Avatares e Ítems para un videojuego en la red de Solana. La idea es tener un registro seguro y descentralizado de los personajes de los jugadores y su inventario, utilizando un modelo relacional basado en PDAs (Program Derived Addresses) donde cada ítem es una cuenta independiente vinculada a su avatar correspondiente.

🛠️ Tecnologías y Herramientas
Lenguaje: Rust.

Framework: Anchor.

Entorno: Solana Playground / Entorno de desarrollo local.

Archivos del Repositorio
lib.rs: Código principal del contrato inteligente en Rust con la lógica del juego.

game_crud_hub_test.ts: Script en TypeScript (Mocha/Chai) para probar que la creación de avatares y la gestión del inventario funcionan correctamente.

README.md: Documentación y guía de uso del proyecto.

Instrucciones de Uso
1. Crear un Avatar
Ejecutar la función crear_avatar para inicializar el personaje del jugador.

Semillas (Seeds): Se requiere el prefijo exacto b"avatar", el nombre del avatar y la Pubkey del dueño (jugador).

Nota: Esta cuenta almacena un vector vacío que posteriormente guardará las Pubkeys de los ítems que el jugador vaya obteniendo.

2. Agregar un Ítem al Inventario
Usar la función agregar_item para crear un arma, armadura o accesorio y vincularlo al avatar.

Datos: Se guarda el nombre del ítem, su nivel de poder y se establece su estado inicial como equipado.

Semillas (Seeds): Se requiere el prefijo exacto b"item", el nombre del ítem y la Pubkey del dueño.

Relación: La cuenta del ítem se inicializa y su dirección (PDA) se guarda automáticamente dentro del vector de ítems de la cuenta del Avatar.

3. Alternar Estado del Ítem (Equipar/Desequipar)
Usar la función alternar_estado para modificar el estado de un ítem existente.

Acción: Cambia el valor booleano equipado del ítem (si está en true pasa a false, y viceversa). Solo el dueño legítimo del avatar puede ejecutar esta acción.

4. Eliminar un Ítem
Usar la función eliminar_item para destruir un ítem del inventario.

Acción: Primero, el programa busca y elimina la Pubkey del ítem dentro del vector del Avatar. Luego, cierra la cuenta del ítem en Solana (close = owner), devolviendo los fondos de alquiler (rent) a la wallet del jugador.
