use sea_orm::FromQueryResult;
use trustify_entity::qualified_purl::CanonicalPurl;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct SbomPackageLicense {
    // pub sbom_name: Option<String>,
    pub sbom_namespace: Option<String>,
    /// Package name
    pub name: String,
    /// Package version
    pub version: Option<String>,
    pub group: Option<String>,
    /// package package URL
    pub purl: Vec<Purl>,
    pub other_reference: Vec<CpeParam>,
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
pub struct CpeParam {
    pub part: Option<String>,
    pub vendor: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
    pub update: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct SbomPackageLicenseBase {
    pub sbom_name: Option<String>,
    pub sbom_namespace: Option<String>,
    pub component_group: Option<String>,
    pub component_version: Option<String>,
    pub node_id: String,
    pub sbom_id: Uuid,
    pub package_name: String,
    pub license_text: Option<String>,
}

#[derive(Debug, Clone, Default, FromQueryResult)]
pub struct SbomName {
    pub sbom_name: String,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct ExtractedLicensingInfos {
    pub license_id: String,
    pub name: String,
    pub extracted_text: String,
    pub comment: String,
}
