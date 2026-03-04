use anchor_lang::prelude::*;

declare_id!("GimSoLXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod gimnasio_solana {
    use super::*;

    pub fn registrar_miembro(
        ctx: Context<RegistrarMiembro>,
        nombre: String,
        edad: u8,
        objetivo: String,
        plan: PlanMembresia,
    ) -> Result<()> {
        require!(nombre.len() > 0 && nombre.len() <= 50, GimnasioError::NombreInvalido);
        require!(edad >= 16 && edad <= 99, GimnasioError::EdadFueraDeRango);
        require!(objetivo.len() > 0 && objetivo.len() <= 100, GimnasioError::ObjetivoInvalido);

        let miembro = &mut ctx.accounts.miembro;
        let clock = Clock::get()?;

        miembro.autoridad           = ctx.accounts.autoridad.key();
        miembro.nombre              = nombre;
        miembro.edad                = edad;
        miembro.objetivo            = objetivo;
        miembro.plan                = plan;
        miembro.rutinas_completadas = 0;
        miembro.puntos_fitness      = 0;
        miembro.activo              = true;
        miembro.fecha_registro      = clock.unix_timestamp;
        miembro.ultima_visita       = clock.unix_timestamp;
        miembro.bump                = ctx.bumps.miembro;

        msg!("🏋️ Miembro registrado: {}", miembro.nombre);
        msg!("📋 Plan: {:?}", miembro.plan);
        Ok(())
    }

    pub fn actualizar_miembro(
        ctx: Context<ActualizarMiembro>,
        nuevo_objetivo: String,
        nuevo_plan: PlanMembresia,
    ) -> Result<()> {
        require!(nuevo_objetivo.len() > 0 && nuevo_objetivo.len() <= 100, GimnasioError::ObjetivoInvalido);

        let miembro = &mut ctx.accounts.miembro;
        require!(miembro.autoridad == ctx.accounts.autoridad.key(), GimnasioError::NoAutorizado);

        miembro.objetivo = nuevo_objetivo;
        miembro.plan     = nuevo_plan;

        msg!("✏️ Miembro actualizado: {}", miembro.nombre);
        msg!("🎯 Nuevo objetivo: {}", miembro.objetivo);
        Ok(())
    }

    pub fn agregar_rutina(
        ctx: Context<AgregarRutina>,
        id_rutina: u32,
        nombre: String,
        tipo_entrenamiento: TipoEntrenamiento,
        duracion_minutos: u16,
        nivel_dificultad: NivelDificultad,
    ) -> Result<()> {
        require!(nombre.len() > 0 && nombre.len() <= 60, GimnasioError::NombreInvalido);
        require!(duracion_minutos >= 15 && duracion_minutos <= 300, GimnasioError::DuracionInvalida);

        let miembro = &mut ctx.accounts.miembro;
        require!(miembro.activo, GimnasioError::MiembroInactivo);

        let rutina = &mut ctx.accounts.rutina;
        let clock  = Clock::get()?;

        rutina.miembro            = miembro.key();
        rutina.id_rutina          = id_rutina;
        rutina.nombre             = nombre;
        rutina.tipo_entrenamiento = tipo_entrenamiento;
        rutina.duracion_minutos   = duracion_minutos;
        rutina.nivel_dificultad   = nivel_dificultad;
        rutina.completada         = false;
        rutina.fecha_creacion     = clock.unix_timestamp;
        rutina.bump               = ctx.bumps.rutina;

        msg!("💪 Rutina agregada: {}", rutina.nombre);
        msg!("⏱️ Duración: {} minutos", rutina.duracion_minutos);
        Ok(())
    }

    pub fn completar_rutina(ctx: Context<CompletarRutina>) -> Result<()> {
        let rutina  = &mut ctx.accounts.rutina;
        let miembro = &mut ctx.accounts.miembro;
        let clock   = Clock::get()?;

        require!(!rutina.completada, GimnasioError::RutinaYaCompletada);
        require!(miembro.activo, GimnasioError::MiembroInactivo);

        let puntos: u32 = match rutina.nivel_dificultad {
            NivelDificultad::Principiante => (rutina.duracion_minutos as u32) * 1,
            NivelDificultad::Intermedio   => (rutina.duracion_minutos as u32) * 2,
            NivelDificultad::Avanzado     => (rutina.duracion_minutos as u32) * 3,
        };

        rutina.completada = true;

        miembro.rutinas_completadas = miembro.rutinas_completadas
            .checked_add(1).ok_or(GimnasioError::Overflow)?;
        miembro.puntos_fitness = miembro.puntos_fitness
            .checked_add(puntos).ok_or(GimnasioError::Overflow)?;
        miembro.ultima_visita = clock.unix_timestamp;

        msg!("✅ Rutina completada: {}", rutina.nombre);
        msg!("🏆 Puntos ganados: {}", puntos);
        msg!("⭐ Total puntos fitness: {}", miembro.puntos_fitness);
        Ok(())
    }

    pub fn dar_baja_miembro(ctx: Context<DarBajaMiembro>) -> Result<()> {
        let miembro = &mut ctx.accounts.miembro;
        require!(miembro.autoridad == ctx.accounts.autoridad.key(), GimnasioError::NoAutorizado);
        require!(miembro.activo, GimnasioError::MiembroInactivo);

        miembro.activo = false;

        msg!("❌ Baja registrada: {}", miembro.nombre);
        msg!("📊 Rutinas completadas: {}", miembro.rutinas_completadas);
        msg!("⭐ Puntos acumulados: {}", miembro.puntos_fitness);
        Ok(())
    }
}

#[account]
pub struct Miembro {
    pub autoridad:           Pubkey,
    pub nombre:              String,
    pub edad:                u8,
    pub objetivo:            String,
    pub plan:                PlanMembresia,
    pub rutinas_completadas: u32,
    pub puntos_fitness:      u32,
    pub activo:              bool,
    pub fecha_registro:      i64,
    pub ultima_visita:       i64,
    pub bump:                u8,
}

impl Miembro {
    pub const LEN: usize = 8 + 32 + (4+50) + 1 + (4+100) + 1 + 4 + 4 + 1 + 8 + 8 + 1;
}

#[account]
pub struct Rutina {
    pub miembro:            Pubkey,
    pub id_rutina:          u32,
    pub nombre:             String,
    pub tipo_entrenamiento: TipoEntrenamiento,
    pub duracion_minutos:   u16,
    pub nivel_dificultad:   NivelDificultad,
    pub completada:         bool,
    pub fecha_creacion:     i64,
    pub bump:               u8,
}

impl Rutina {
    pub const LEN: usize = 8 + 32 + 4 + (4+60) + 1 + 2 + 1 + 1 + 8 + 1;
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarMiembro<'info> {
    #[account(
        init,
        payer = autoridad,
        space = Miembro::LEN,
        seeds = [b"miembro", autoridad.key().as_ref(), nombre.as_bytes()],
        bump
    )]
    pub miembro:        Account<'info, Miembro>,
    #[account(mut)]
    pub autoridad:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarMiembro<'info> {
    #[account(
        mut,
        seeds = [b"miembro", autoridad.key().as_ref(), miembro.nombre.as_bytes()],
        bump = miembro.bump
    )]
    pub miembro:   Account<'info, Miembro>,
    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(id_rutina: u32)]
pub struct AgregarRutina<'info> {
    #[account(
        init,
        payer = autoridad,
        space = Rutina::LEN,
        seeds = [b"rutina", miembro.key().as_ref(), &id_rutina.to_le_bytes()],
        bump
    )]
    pub rutina: Account<'info, Rutina>,
    #[account(
        mut,
        seeds = [b"miembro", autoridad.key().as_ref(), miembro.nombre.as_bytes()],
        bump = miembro.bump
    )]
    pub miembro:        Account<'info, Miembro>,
    #[account(mut)]
    pub autoridad:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompletarRutina<'info> {
    #[account(
        mut,
        seeds = [b"rutina", miembro.key().as_ref(), &rutina.id_rutina.to_le_bytes()],
        bump = rutina.bump
    )]
    pub rutina: Account<'info, Rutina>,
    #[account(
        mut,
        seeds = [b"miembro", autoridad.key().as_ref(), miembro.nombre.as_bytes()],
        bump = miembro.bump
    )]
    pub miembro:   Account<'info, Miembro>,
    pub autoridad: Signer<'info>,
}

#[derive(Accounts)]
pub struct DarBajaMiembro<'info> {
    #[account(
        mut,
        seeds = [b"miembro", autoridad.key().as_ref(), miembro.nombre.as_bytes()],
        bump = miembro.bump
    )]
    pub miembro:   Account<'info, Miembro>,
    pub autoridad: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PlanMembresia {
    Basico,
    Estandar,
    Premium,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TipoEntrenamiento {
    Fuerza,
    Cardio,
    Flexibilidad,
    HIIT,
    Natacion,
    Funcional,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum NivelDificultad {
    Principiante,
    Intermedio,
    Avanzado,
}

#[error_code]
pub enum GimnasioError {
    #[msg("El nombre no puede estar vacío ni superar 50 caracteres")]
    NombreInvalido,
    #[msg("La edad debe estar entre 16 y 99 años")]
    EdadFueraDeRango,
    #[msg("El objetivo no puede estar vacío ni superar 100 caracteres")]
    ObjetivoInvalido,
    #[msg("La duración debe estar entre 15 y 300 minutos")]
    DuracionInvalida,
    #[msg("No tienes autorización para realizar esta acción")]
    NoAutorizado,
    #[msg("El miembro no está activo en el gimnasio")]
    MiembroInactivo,
    #[msg("Esta rutina ya fue completada anteriormente")]
    RutinaYaCompletada,
    #[msg("Desbordamiento aritmético en los contadores")]
    Overflow,
}
