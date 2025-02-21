use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExtractedLicensingInfos::Table)
                    .col(
                        ColumnDef::new(ExtractedLicensingInfos::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExtractedLicensingInfos::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExtractedLicensingInfos::LicenseId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExtractedLicensingInfos::SbomId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ExtractedLicensingInfos::ExtractedText).string())
                    .col(ColumnDef::new(ExtractedLicensingInfos::Comment).string())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(ExtractedLicensingInfos::SbomId)
                            .to(Sbom::Table, Sbom::SbomId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ExtractedLicensingInfos::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExtractedLicensingInfos {
    Table,
    Id,
    SbomId,
    Name,
    LicenseId,
    ExtractedText,
    Comment,
}

#[derive(DeriveIden)]
pub enum Sbom {
    Table,
    SbomId,
}
