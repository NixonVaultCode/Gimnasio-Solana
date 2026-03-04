import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

// ============================================================
// CLIENTE TYPESCRIPT - Gimnasio Solana
// Interactúa con el programa on-chain desde Solana Playground
// ============================================================

// ── Enums espejo de los definidos en Rust ───────────────────
const PlanMembresia = {
  Basico:   { basico:   {} },
  Estandar: { estandar: {} },
  Premium:  { premium:  {} },
};

const TipoEntrenamiento = {
  Fuerza:       { fuerza:       {} },
  Cardio:       { cardio:       {} },
  Flexibilidad: { flexibilidad: {} },
  HIIT:         { hiit:         {} },
  Natacion:     { natacion:     {} },
  Funcional:    { funcional:    {} },
};

const NivelDificultad = {
  Principiante: { principiante: {} },
  Intermedio:   { intermedio:   {} },
  Avanzado:     { avanzado:     {} },
};

// ── Helpers para derivar PDAs ────────────────────────────────

function obtenerPdaMiembro(
  autoridad: PublicKey,
  nombre: string,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("miembro"),
      autoridad.toBuffer(),
      Buffer.from(nombre),
    ],
    programId
  );
}

function obtenerPdaRutina(
  miembroPda: PublicKey,
  idRutina: number,
  programId: PublicKey
): [PublicKey, number] {
  const idBuffer = Buffer.alloc(4);
  idBuffer.writeUInt32LE(idRutina, 0);
  return PublicKey.findProgramAddressSync(
    [Buffer.from("rutina"), miembroPda.toBuffer(), idBuffer],
    programId
  );
}

// ── Función principal ────────────────────────────────────────

async function main() {
  const program   = pg.program;
  const autoridad = pg.wallet.publicKey;

  console.log("==========================================");
  console.log("🏋️  GIMNASIO SOLANA — Cliente TypeScript");
  console.log("==========================================");
  console.log("Wallet:", autoridad.toBase58());
  console.log("Program ID:", program.programId.toBase58());
  console.log("");

  // ==========================================================
  // CREATE — Registrar un nuevo miembro
  // ==========================================================
  const nombreMiembro = "Carlos Fitness";

  const [miembroPda] = obtenerPdaMiembro(
    autoridad,
    nombreMiembro,
    program.programId
  );

  console.log("📝 [CREATE] Registrando miembro...");
  console.log("PDA:", miembroPda.toBase58());

  const txRegistrar = await program.methods
    .registrarMiembro(
      nombreMiembro,
      28,
      "Ganar masa muscular",
      PlanMembresia.Premium
    )
    .accounts({
      miembro:       miembroPda,
      autoridad:     autoridad,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("✅ Miembro registrado! Tx:", txRegistrar);
  console.log("");

  // ==========================================================
  // READ — Leer los datos del miembro
  // ==========================================================
  console.log("👤 [READ] Leyendo datos del miembro...");

  const datosMiembro = await program.account.miembro.fetch(miembroPda);

  console.log("  Nombre   :", datosMiembro.nombre);
  console.log("  Edad     :", datosMiembro.edad);
  console.log("  Objetivo :", datosMiembro.objetivo);
  console.log("  Plan     :", JSON.stringify(datosMiembro.plan));
  console.log("  Activo   :", datosMiembro.activo);
  console.log("  Puntos   :", datosMiembro.puntosFitness.toString());
  console.log(
    "  Registro :",
    new Date(datosMiembro.fechaRegistro.toNumber() * 1000).toLocaleDateString("es-CO")
  );
  console.log("");

  // ==========================================================
  // CREATE — Agregar rutina 1 (Fuerza - Avanzado - 60 min)
  // ==========================================================
  const idRutina1 = 1;
  const [rutinaPda1] = obtenerPdaRutina(miembroPda, idRutina1, program.programId);

  console.log("💪 [CREATE] Agregando rutina 1...");

  await program.methods
    .agregarRutina(
      idRutina1,
      "Pecho y Tríceps",
      TipoEntrenamiento.Fuerza,
      60,
      NivelDificultad.Avanzado
    )
    .accounts({
      rutina:        rutinaPda1,
      miembro:       miembroPda,
      autoridad:     autoridad,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("✅ Rutina 1 agregada!");
  console.log("");

  // ==========================================================
  // UPDATE (lógica) — Completar rutina 1 → gana puntos
  // Avanzado x 60 min = 180 puntos
  // ==========================================================
  console.log("🏆 [UPDATE] Completando rutina 1...");

  await program.methods
    .completarRutina()
    .accounts({
      rutina:    rutinaPda1,
      miembro:   miembroPda,
      autoridad: autoridad,
    })
    .rpc();

  const tras_rutina1 = await program.account.miembro.fetch(miembroPda);
  console.log("✅ Rutina completada!");
  console.log("  ⭐ Puntos ganados  : 180  (60 min × 3 Avanzado)");
  console.log("  ⭐ Puntos totales  :", tras_rutina1.puntosFitness.toString());
  console.log("");

  // ==========================================================
  // CREATE — Agregar rutina 2 (HIIT - Intermedio - 30 min)
  // ==========================================================
  const idRutina2 = 2;
  const [rutinaPda2] = obtenerPdaRutina(miembroPda, idRutina2, program.programId);

  console.log("💪 [CREATE] Agregando rutina 2...");

  await program.methods
    .agregarRutina(
      idRutina2,
      "Cardio HIIT 30 min",
      TipoEntrenamiento.HIIT,
      30,
      NivelDificultad.Intermedio
    )
    .accounts({
      rutina:        rutinaPda2,
      miembro:       miembroPda,
      autoridad:     autoridad,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  await program.methods
    .completarRutina()
    .accounts({
      rutina:    rutinaPda2,
      miembro:   miembroPda,
      autoridad: autoridad,
    })
    .rpc();

  console.log("✅ Rutina 2 completada!");
  console.log("  ⭐ Puntos ganados  : 60  (30 min × 2 Intermedio)");
  console.log("");

  // ==========================================================
  // UPDATE — Actualizar objetivo y plan del miembro
  // ==========================================================
  console.log("✏️  [UPDATE] Actualizando objetivo y plan...");

  await program.methods
    .actualizarMiembro(
      "Definición muscular y resistencia",
      PlanMembresia.Estandar
    )
    .accounts({
      miembro:   miembroPda,
      autoridad: autoridad,
    })
    .rpc();

  console.log("✅ Miembro actualizado!");
  console.log("");

  // ==========================================================
  // DELETE (soft) — Dar de baja al miembro
  // ==========================================================
  console.log("❌ [DELETE] Dando de baja al miembro...");

  await program.methods
    .darBajaMiembro()
    .accounts({
      miembro:   miembroPda,
      autoridad: autoridad,
    })
    .rpc();

  console.log("✅ Baja registrada (soft-delete, historial conservado)");
  console.log("");

  // ==========================================================
  // READ FINAL — Resumen completo
  // ==========================================================
  const resumen = await program.account.miembro.fetch(miembroPda);

  console.log("==========================================");
  console.log("         📊 RESUMEN FINAL                ");
  console.log("==========================================");
  console.log("  👤 Nombre              :", resumen.nombre);
  console.log("  🎯 Objetivo            :", resumen.objetivo);
  console.log("  📋 Plan                :", JSON.stringify(resumen.plan));
  console.log("  ✅ Rutinas completadas :", resumen.rutinasCompletadas.toString());
  console.log("  ⭐ Puntos fitness      :", resumen.puntosFitness.toString(), "(esperado: 240)");
  console.log("  🔴 Activo              :", resumen.activo, "(dado de baja)");
  console.log("==========================================");
  console.log("✅ CRUD completo ejecutado exitosamente!");
}

main().catch(console.error);
```

