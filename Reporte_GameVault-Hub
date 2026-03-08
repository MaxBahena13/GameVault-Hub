## 🛠️ Reporte Técnico: Game CRUD Hub

> **Almacenamiento y PDAs**
> 
> Este contrato utiliza **Program Derived Addresses (PDAs)** para asegurar que cada avatar y cada ítem estén vinculados de forma criptográfica y única a la *wallet* de su dueño. Cero colisiones, máxima seguridad en la red. 🔒

---

### 🧬 Estructura de la PDA

La dirección de las cuentas en la blockchain se deriva de manera determinista utilizando la siguiente arquitectura de semillas (*seeds*):

* **Semilla estática:** `b"avatar"` o `b"item"` (dependiendo del tipo de cuenta).
* **Identificador dinámico:** El `nombre` asignado al personaje o al objeto.
* **Firma de autoridad:** La llave pública (`Pubkey`) del *owner*.

---

### 💾 Gestión de Espacio y Optimización de Renta

Se ha calculado rigurosamente el espacio en bytes (`InitSpace`) necesario para soportar la data del juego, pagando solo lo justo de renta en Solana:

* 📝 **Nombres y Metadatos:** Cadenas de texto (`String`) restringidas con límites de caracteres estrictos (`#[max_len]`) para evitar desbordamientos de memoria.
* ⚔️ **Estadísticas Ligeras:** Valores numéricos supereficientes (`u16` para el poder, `bool` para el estado del equipamiento).
* 🎒 **Inventario Escalable:** Un vector (`Vec<Pubkey>`) configurado para almacenar las referencias exactas de los ítems adquiridos, optimizando al máximo el uso de memoria en los validadores de la red.
