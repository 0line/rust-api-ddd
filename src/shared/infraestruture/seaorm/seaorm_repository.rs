use std::sync::Arc;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, ActiveModelTrait, DbErr};

pub struct SeaOrmRepository<T>
where
    T: EntityTrait + Clone + 'static,
    T::Model: IntoActiveModel<T::ActiveModel>, // Restricción adicional
    T::ActiveModel: Send,
{
    client: Arc<DatabaseConnection>,
    entity: T,
}

impl<T> SeaOrmRepository<T>
where
    T: EntityTrait + Clone + 'static,
    T::Model: IntoActiveModel<T::ActiveModel>, // Restricción adicional
    T::ActiveModel: Send,
{
    pub fn new(client: Arc<DatabaseConnection>, entity: T) -> Self {
        Self { client, entity }
    }

    // Función para persistir un agregado
    pub async fn persist(&self, aggregate_root: T::Model) -> Result<(), DbErr> where <T as EntityTrait>::ActiveModel: Send {
        let active_model = aggregate_root.into_active_model();
        active_model.insert(self.client()).await?;
        Ok(())
    }

    // Función para obtener el cliente
    pub fn client(&self) -> &DatabaseConnection {
        &self.client
    }

    // Función para obtener la entidad asociada
    pub fn entity(&self) -> &T {
        &self.entity
    }
}
