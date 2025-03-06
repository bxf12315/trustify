use crate::license::model::sbom_license::SbomName;
use crate::license::service::{LicenseService, license_export::LicenseExporter};
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};
use std::fs::File;
use std::io::Write;
use test_context::test_context;
use test_log::test;
use trustify_entity::sbom;
use trustify_test_context::TrustifyContext;
use uuid::Uuid;

#[derive(Debug, Clone, FromQueryResult, Default)]
pub struct Sbom {
    pub sbom_id: Uuid,
    pub sbom_namespace: String,
}

#[test_context(TrustifyContext)]
#[test(tokio::test)]
async fn test_spdx(ctx: &TrustifyContext) -> Result<(), anyhow::Error> {
    let _result = ctx
        .ingest_document("spdx/SATELLITE-6.15-RHEL-8.json")
        .await?;

    let result_sbom: Option<Sbom> = sbom::Entity::find()
        .column_as(sbom::Column::SbomId, "sbom_id")
        .column_as(sbom::Column::DocumentId, "sbom_namespace")
        .into_model::<Sbom>()
        .one(&ctx.db)
        .await?;

    assert_eq!(
        "https://access.redhat.com/security/data/sbom/spdx/SATELLITE-6.15-RHEL-8",
        result_sbom.clone().unwrap_or_default().sbom_namespace
    );
    if let Some(id) = result_sbom {
        let license_service = LicenseService::new(ctx.db.clone());
        let (sbom_license_list, sbom_license_info_list, sbom_name) = license_service
            .license_export(trustify_common::id::Id::Uuid(id.sbom_id), &ctx.db)
            .await?;

        let exporter = LicenseExporter::new(
            sbom_name.unwrap_or_else(SbomName::default).sbom_name,
            sbom_license_list.clone(),
            sbom_license_info_list.clone(),
        );
        let mut file = File::create("/tmp/quarkus-bom-2.13.8.Final-redhat-00004.tar.gz")
            .unwrap_or_else(|_| panic!("create file failed"));
        file.write_all(
            &exporter
                .generate()
                .unwrap_or_else(|_| panic!("generate failed")),
        )
        .unwrap_or_else(|_| panic!("write file failed"));

        assert_eq!(49, sbom_license_info_list.len());
        assert_eq!(2294, sbom_license_list.len());
    }
    Ok(())
}

#[test_context(TrustifyContext)]
#[test(tokio::test)]
async fn test_cyclonedx(ctx: &TrustifyContext) -> Result<(), anyhow::Error> {
    let _result = ctx
        .ingest_document("cyclonedx/openssl-3.0.7-18.el9_2.cdx_1.6.sbom.json")
        .await?;

    let result_sbom: Option<Sbom> = sbom::Entity::find()
        .column_as(sbom::Column::SbomId, "sbom_id")
        .column_as(sbom::Column::DocumentId, "sbom_namespace")
        .into_model::<Sbom>()
        .one(&ctx.db)
        .await?;

    assert_eq!(
        "urn:uuid:a4f16b62-fea9-42c1-8365-d72d3cef37d1/1",
        result_sbom.clone().unwrap_or_default().sbom_namespace
    );
    if let Some(id) = result_sbom {
        let license_service = LicenseService::new(ctx.db.clone());
        let (sbom_license_list, sbom_license_info_list, sbom_name) = license_service
            .license_export(trustify_common::id::Id::Uuid(id.sbom_id), &ctx.db)
            .await?;

        let exporter = LicenseExporter::new(
            sbom_name.unwrap_or_else(SbomName::default).sbom_name,
            sbom_license_list.clone(),
            sbom_license_info_list.clone(),
        );
        let mut file = File::create("/tmp/openssl-3.0.7-18.el9_2.cdx_1.6.sbom.json")
            .unwrap_or_else(|_| panic!("create file failed"));
        file.write_all(
            &exporter
                .generate()
                .unwrap_or_else(|_| panic!("generate failed")),
        )
        .unwrap_or_else(|_| panic!("write file failed"));

        assert_eq!(0, sbom_license_info_list.len());
        assert_eq!(39, sbom_license_list.len());
    }
    Ok(())
}
