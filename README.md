# 🏋️ Gimnasio Solana

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blue?style=for-the-badge)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)

Sistema de gestión de un gimnasio desarrollado como **Solana Program** usando **Rust + Anchor**. Permite registrar miembros, gestionar rutinas de entrenamiento y acumular puntos de fitness, todo almacenado **on-chain** en la red Solana.

---

## 📁 Estructura del proyecto
```
Gimnasio-Solana/
├── src/
│   └── lib.rs         # Programa Rust + Anchor (lógica on-chain)
├── client/
│   └── gimnasio.ts    # Cliente TypeScript para Solana Playground
└── README.md
```

---

## ✅ Operaciones CRUD

| Operación | Instrucción | Descripción |
|-----------|-------------|-------------|
| CREATE | `registrar_miembro` | Crea cuenta PDA del miembro |
| CREATE | `agregar_rutina` | Agrega rutina de entrenamiento |
| READ | `account.miembro.fetch()` | Lee datos on-chain |
| UPDATE | `actualizar_miembro` | Modifica objetivo y plan |
| UPDATE | `completar_rutina` | Marca rutina y otorga puntos |
| DELETE | `dar_baja_miembro` | Soft-delete del miembro |

---

## 🏆 Sistema de puntos fitness

Los puntos se calculan automáticamente al completar una rutina:

| Nivel | Multiplicador | Ejemplo (60 min) |
|-------|--------------|------------------|
| Principiante | × 1 | 60 puntos |
| Intermedio | × 2 | 120 puntos |
| Avanzado | × 3 | 180 puntos |

---

## 📦 Cuentas on-chain

### Miembro
| Campo | Tipo | Descripción |
|-------|------|-------------|
| `autoridad` | `Pubkey` | Wallet propietaria |
| `nombre` | `String` | Max 50 caracteres |
| `edad` | `u8` | Rango 16–99 |
| `objetivo` | `String` | Meta fitness, max 100 chars |
| `plan` | `PlanMembresia` | Basico / Estandar / Premium |
| `rutinas_completadas` | `u32` | Contador total |
| `puntos_fitness` | `u32` | Sistema de gamificación |
| `activo` | `bool` | Estado de membresía |

### Rutina
| Campo | Tipo | Descripción |
|-------|------|-------------|
| `miembro` | `Pubkey` | Referencia al miembro |
| `id_rutina` | `u32` | ID único por miembro |
| `nombre` | `String` | Max 60 caracteres |
| `tipo_entrenamiento` | `TipoEntrenamiento` | Fuerza / Cardio / HIIT / etc |
| `duracion_minutos` | `u16` | Rango 15–300 min |
| `nivel_dificultad` | `NivelDificultad` | Principiante / Intermedio / Avanzado |
| `completada` | `bool` | Estado de la rutina |

---

## 🔐 Derivación de PDAs
```
Miembro → ["miembro", autoridad, nombre]
Rutina  → ["rutina",  miembro_pda, id_rutina_bytes]
```

---

## 🚀 Cómo ejecutar el proyecto

### 1. Hacer Fork
Haz clic en **Fork** para clonar este repositorio a tu cuenta.

### 2. Importar en Solana Playground
Copia la URL de tu repo forkeado y ábrela así:
```
https://beta.solpg.io/github.com/TU_USUARIO/Gimnasio-Solana
```

### 3. Conectar wallet en devnet
Clic en **Not Connected** → **Continue** → se crea tu wallet automáticamente.

### 4. Pedir SOL de prueba
Clic en tu saldo → **Get 2 SOL** (gratis en devnet).

### 5. Compilar y desplegar
- Pestaña **Build & Deploy** → clic en **Build**
- Luego clic en **Deploy**
- Copia el **Program ID** generado

### 6. Reemplazar Program ID
En `src/lib.rs` reemplaza:
```rust
declare_id!("GimSoLXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
```
con tu Program ID real. Vuelve a hacer **Build → Deploy**.

### 7. Ejecutar el cliente
Abre `client/gimnasio.ts` → clic en **Run** → observa los logs del CRUD completo.

---

## ❌ Errores personalizados

| Error | Descripción |
|-------|-------------|
| `NombreInvalido` | Vacío o mayor a 50 chars |
| `EdadFueraDeRango` | Fuera del rango 16–99 |
| `ObjetivoInvalido` | Vacío o mayor a 100 chars |
| `DuracionInvalida` | Fuera del rango 15–300 min |
| `NoAutorizado` | Firmante no es el propietario |
| `MiembroInactivo` | Miembro dado de baja |
| `RutinaYaCompletada` | No se puede completar dos veces |
| `Overflow` | Desbordamiento en contadores |

---

## 🛠️ Tecnologías

- **Rust** — Lenguaje principal del programa on-chain
- **Anchor Framework** — Framework para desarrollo en Solana
- **Solana Web3.js** — Librería para interacción con la red
- **TypeScript** — Cliente para pruebas y demostración
- **Solana Devnet** — Red de pruebas sin dinero real

---

## 📚 Recursos útiles

- [Solana Playground](https://beta.solpg.io)
- [Documentación de Anchor](https://www.anchor-lang.com)
- [Documentación de Solana](https://docs.solana.com)
- [Solana Devnet Explorer](https://explorer.solana.com/?cluster=devnet)
```
