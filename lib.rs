use anchor_lang::prelude::*;

declare_id!("37am8Yq32pP3odzsgYSBop6Z2Zuv4iQi3dS6K8n3fe6M");

#[program]
pub mod game_crud_hub {
    use super::*;

    // Inicializa un nuevo avatar y lo vincula a la wallet del jugador
    pub fn crear_avatar(context: Context<NuevoAvatar>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        let items = Vec::<Pubkey>::new();

        context.accounts.avatar.set_inner(Avatar {
            owner: owner_id,
            nombre: nombre.clone(),
            items,
        });

        msg!("Avatar {} creado exitosamente!. Owner id: {}", nombre, owner_id);

        Ok(())
    }

    // Crea un item, guarda sus datos y añade su Pubkey al inventario del avatar
    pub fn agregar_item(context: Context<NuevoItem>, nombre: String, poder: u16) -> Result<()> {
        require!(
            context.accounts.avatar.owner == context.accounts.owner.key(),
            ErrorJuego::NoAutorizado
        );

        let item = Item {
            avatar: context.accounts.avatar.nombre.clone(),
            nombre: nombre.clone(),
            poder,
            equipado: true, // Se equipa por defecto al crearse
        };

        context.accounts.item.set_inner(item);
        context.accounts.avatar.items.push(context.accounts.item.key());

        msg!(
            "Item {}, agregado exitosamente al avatar {}!. Owner id: {}",
            nombre,
            context.accounts.avatar.nombre,
            context.accounts.owner.key()
        );

        Ok(())
    }

    // Borra la referencia del item en el avatar (la cuenta en sí se cierra en el Contexto)
    pub fn eliminar_item(context: Context<EliminarItem>, nombre: String) -> Result<()> {
        require!(
            context.accounts.avatar.owner == context.accounts.owner.key(),
            ErrorJuego::NoAutorizado
        );

        let avatar = &mut context.accounts.avatar;

        require!(
            context.accounts.item.avatar == avatar.nombre,
            ErrorJuego::ItemNoPertenece
        );

        let pos = avatar
            .items
            .iter()
            .position(|&x| x == context.accounts.item.key())
            .ok_or(ErrorJuego::ItemNoExiste)?;

        avatar.items.remove(pos); // Saca el item del vector

        msg!(
            "Item '{}' eliminado exitosamente del avatar {}!. Owner id: {}",
            nombre,
            avatar.nombre,
            context.accounts.owner.key()
        );

        Ok(())
    }

    // Cambia el booleano del item de true a false o viceversa
    pub fn alternar_estado(context: Context<ModificarItem>, nombre: String) -> Result<()> {
        require!(
            context.accounts.avatar.owner == context.accounts.owner.key(),
            ErrorJuego::NoAutorizado
        );

        let item = &mut context.accounts.item;
        item.equipado = !item.equipado; // Inversión rápida del estado

        msg!(
            "El item: {} ahora tiene estado de equipado: {}",
            nombre,
            item.equipado
        );

        Ok(())
    }
}

// --- Manejo de Errores ---

#[error_code]
pub enum ErrorJuego {
    #[msg("Error, no autorizado.")]
    NoAutorizado,
    #[msg("Error, el item no existe en el inventario de este avatar.")]
    ItemNoExiste,
    #[msg("Error, el item no pertenece a este avatar.")]
    ItemNoPertenece,
}

// --- Estructuras de Datos (Cuentas) ---

#[account]
#[derive(InitSpace)]
pub struct Avatar {
    pub owner: Pubkey,
    #[max_len(30)]
    pub nombre: String,
    #[max_len(10)]
    pub items: Vec<Pubkey>, // Limitado a 10 items por diseño de InitSpace
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Item {
    #[max_len(30)]
    pub avatar: String, // Referencia string al avatar dueño
    #[max_len(40)]
    pub nombre: String,
    pub poder: u16,
    pub equipado: bool,
}

// --- Contextos (Validación de Cuentas) ---

#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevoAvatar<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    // Genera el PDA usando el prefijo, nombre y llave del dueño
    #[account(
        init,
        payer = owner,
        space = 8 + Avatar::INIT_SPACE,
        seeds = [b"avatar", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub avatar: Account<'info, Avatar>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevoItem<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Item::INIT_SPACE,
        seeds = [b"item", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub item: Account<'info, Item>,
    
    #[account(mut)]
    pub avatar: Account<'info, Avatar>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarItem<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub item: Account<'info, Item>,
    #[account(mut)]
    pub avatar: Account<'info, Avatar>,
}

#[derive(Accounts)]
pub struct EliminarItem<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    // 'close = owner' destruye la cuenta y devuelve los fondos (rent) al dueño
    #[account(
        mut,
        close = owner, 
        constraint = item.avatar == avatar.nombre @ ErrorJuego::ItemNoPertenece
    )]
    pub item: Account<'info, Item>,
    #[account(mut)]
    pub avatar: Account<'info, Avatar>,
}
