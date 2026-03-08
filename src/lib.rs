use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod cafeteria {
    use super::*;

    //////////////////////////// Crear Cafeteria /////////////////////////////////////
    pub fn crear_cafeteria(context: Context<NuevaCafeteria>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let productos: Vec<Producto> = Vec::new();

        context.accounts.cafeteria.set_inner(Cafeteria {
            owner: owner_id,
            nombre,
            productos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Producto /////////////////////////////////////
    pub fn agregar_producto(context: Context<NuevoProducto>, nombre: String, precio: u16) -> Result<()> {

        require!(
            context.accounts.cafeteria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let producto = Producto {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.cafeteria.productos.push(producto);

        Ok(())
    }

    //////////////////////////// Eliminar Producto /////////////////////////////////////
    pub fn eliminar_producto(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.cafeteria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.cafeteria.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("Producto {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    //////////////////////////// Ver Productos /////////////////////////////////////
    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {

        require!(
            context.accounts.cafeteria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "El menú actual de la cafetería es: {:#?}",
            context.accounts.cafeteria.productos
        );

        Ok(())
    }

    //////////////////////////// Cambiar Disponibilidad /////////////////////////////////////
    pub fn alternar_disponibilidad(context: Context<NuevoProducto>, nombre: String) -> Result<()> {

        require!(
            context.accounts.cafeteria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.cafeteria.productos;

        for i in 0..productos.len() {
            let estado = productos[i].disponible;

            if productos[i].nombre == nombre {
                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                msg!(
                    "El producto: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la cafetería")]
    NoEresElOwner,
    #[msg("Error, el producto no existe")]
    ProductoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Cafeteria {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    productos: Vec<Producto>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Producto {
    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaCafeteria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Cafeteria::INIT_SPACE + 8,
        seeds = [b"cafeteria", owner.key().as_ref()],
        bump
    )]
    pub cafeteria: Account<'info, Cafeteria>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoProducto<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub cafeteria: Account<'info, Cafeteria>,
}
