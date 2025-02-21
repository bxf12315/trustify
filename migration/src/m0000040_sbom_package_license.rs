use crate::UuidV4;
use crate::m0000020_create_license_category_enums::LicenseCategory;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SbomPackageLicense::Table)
                    .col(
                        ColumnDef::new(SbomPackageLicense::Id)
                            .uuid()
                            .not_null()
                            .default(Func::cust(UuidV4))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SbomPackageLicense::SbomId).uuid().not_null())
                    .col(
                        ColumnDef::new(SbomPackageLicense::LicenseId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SbomPackageLicense::NodeId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SbomPackageLicense::LicenseType).enumeration(
                        LicenseCategory::LicenseCategory,
                        [
                            LicenseCategory::SLC,
                            LicenseCategory::SLD,
                            LicenseCategory::O,
                        ],
                    ))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SbomPackageLicense::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SbomPackageLicense {
    Table,
    Id,
    SbomId,
    NodeId,
    LicenseId,
    LicenseType,
}
