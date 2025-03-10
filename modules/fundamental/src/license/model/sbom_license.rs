use sea_orm::FromQueryResult;
use trustify_entity::qualified_purl::CanonicalPurl;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct SbomPackageLicense {
    /// package package URL
    pub purl: Vec<Purl>,
    pub other_reference: Vec<trustify_entity::cpe::Model>,
    /// List of all package license
    pub license_text: Option<String>,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct Sbom {
    pub sbom_id: Uuid,
    pub node_id: String,
    pub sbom_namespace: String,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct Purl {
    pub purl: CanonicalPurl,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct SbomPackageLicenseBase {
    pub node_id: Option<String>,
    pub sbom_id: Uuid,
    pub license_text: Option<String>,
}

#[derive(Debug, Clone, Default, FromQueryResult)]
pub struct SbomNameGroupVersion {
    pub sbom_namespace: String,
    pub sbom_name: String,
    pub sbom_group: Option<String>,
    pub sbom_version: String,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct ExtractedLicensingInfos {
    pub license_id: String,
    pub name: String,
    pub extracted_text: String,
    pub comment: String,
}
