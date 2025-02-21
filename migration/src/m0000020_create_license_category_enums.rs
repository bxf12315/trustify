use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(LicenseCategory::LicenseCategory)
                    .values([
                        LicenseCategory::SLC,
                        LicenseCategory::SLD,
                        LicenseCategory::O,
                    ])
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(
                Type::drop()
                    .name(LicenseCategory::LicenseCategory)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
pub enum LicenseCategory {
    LicenseCategory,
    SLC,
    SLD,
    O,
}
