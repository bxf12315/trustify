use crate::graph::sbom::common::node::NodeCreator;
use crate::graph::sbom::{Checksum, LicenseCreator, LicenseInfo, ReferenceSource};
use sea_orm::{ActiveValue::Set, ConnectionTrait, DbErr, EntityTrait};
use sea_query::OnConflict;
use tracing::instrument;
use trustify_common::db::chunk::EntityChunkedIter;
use trustify_entity::{
    cpe_license_assertion, purl_license_assertion, sbom_package, sbom_package_cpe_ref,
    sbom_package_license, sbom_package_purl_ref,
};
use uuid::Uuid;

// Creator of packages and relationships.
pub struct PackageCreator {
    sbom_id: Uuid,
    pub(crate) nodes: NodeCreator,
    pub(crate) packages: Vec<sbom_package::ActiveModel>,
    pub(crate) purl_refs: Vec<sbom_package_purl_ref::ActiveModel>,
    pub(crate) cpe_refs: Vec<sbom_package_cpe_ref::ActiveModel>,
    purl_license_assertions: Vec<purl_license_assertion::ActiveModel>,
    cpe_license_assertions: Vec<cpe_license_assertion::ActiveModel>,
    sbom_packge_licenses: Vec<sbom_package_license::ActiveModel>,
}

pub struct NodeInfoParam {
    pub node_id: String,
    pub name: String,
    pub group: Option<String>,
    pub version: Option<String>,
}

pub enum PackageReference {
    Purl {
        versioned_purl: Uuid,
        qualified_purl: Uuid,
    },
    Cpe(Uuid),
}

impl PackageCreator {
    pub fn new(sbom_id: Uuid) -> Self {
        Self {
            sbom_id,
            nodes: NodeCreator::new(sbom_id),
            packages: Vec::new(),
            purl_refs: Vec::new(),
            cpe_refs: Vec::new(),
            purl_license_assertions: Vec::new(),
            cpe_license_assertions: Vec::new(),
            sbom_packge_licenses: Vec::new(),
        }
    }

    pub fn with_capacity(sbom_id: Uuid, capacity_packages: usize) -> Self {
        Self {
            sbom_id,
            nodes: NodeCreator::with_capacity(sbom_id, capacity_packages),
            packages: Vec::with_capacity(capacity_packages),
            purl_refs: Vec::with_capacity(capacity_packages),
            cpe_refs: Vec::new(), // most packages won't have a CPE, so we start with a low number
            purl_license_assertions: Vec::new(),
            cpe_license_assertions: Vec::new(),
            sbom_packge_licenses: Vec::new(),
        }
    }

    pub fn add<I, C>(
        &mut self,
        node_info: NodeInfoParam,
        refs: impl IntoIterator<Item = PackageReference>,
        license_refs: impl IntoIterator<Item = LicenseInfo> + Clone,
        checksums: I,
        declared_licenses: Option<LicenseInfo>,
        concluded_licenses: Option<LicenseInfo>,
        cyclonedx_licenses: Option<LicenseCreator>,
    ) where
        I: IntoIterator<Item = C>,
        C: Into<Checksum>,
    {
        for r#ref in refs {
            match r#ref {
                PackageReference::Cpe(cpe) => {
                    self.cpe_refs.push(sbom_package_cpe_ref::ActiveModel {
                        sbom_id: Set(self.sbom_id),
                        node_id: Set(node_info.node_id.clone()),
                        cpe_id: Set(cpe),
                    });
                    for license in license_refs.clone() {
                        self.cpe_license_assertions
                            .push(cpe_license_assertion::ActiveModel {
                                id: Default::default(),
                                license_id: Set(license.uuid()),
                                cpe_id: Set(cpe),
                                sbom_id: Set(self.sbom_id),
                            })
                    }
                }
                PackageReference::Purl {
                    qualified_purl,
                    versioned_purl,
                } => {
                    self.purl_refs.push(sbom_package_purl_ref::ActiveModel {
                        sbom_id: Set(self.sbom_id),
                        node_id: Set(node_info.node_id.clone()),
                        qualified_purl_id: Set(qualified_purl),
                    });
                    for license in license_refs.clone() {
                        self.purl_license_assertions
                            .push(purl_license_assertion::ActiveModel {
                                id: Default::default(),
                                license_id: Set(license.uuid()),
                                versioned_purl_id: Set(versioned_purl),
                                sbom_id: Set(self.sbom_id),
                            })
                    }
                }
            }
        }

        self.nodes
            .add(node_info.node_id.clone(), node_info.name, checksums);

        self.packages.push(sbom_package::ActiveModel {
            sbom_id: Set(self.sbom_id),
            group: Set(node_info.group),
            node_id: Set(node_info.node_id.clone()),
            version: Set(node_info.version),
        });

        if let Some(declared) = declared_licenses {
            self.sbom_packge_licenses
                .push(sbom_package_license::ActiveModel {
                    id: Default::default(),
                    sbom_id: Set(self.sbom_id),
                    node_id: Set(node_info.node_id.clone()),
                    license_id: Set(declared.uuid()),
                    license_type: Set(sbom_package_license::LicenseCategory::SpdxDeclared),
                });
        }

        if let Some(concluded) = concluded_licenses {
            self.sbom_packge_licenses
                .push(sbom_package_license::ActiveModel {
                    id: Default::default(),
                    sbom_id: Set(self.sbom_id),
                    node_id: Set(node_info.node_id.clone()),
                    license_id: Set(concluded.uuid()),
                    license_type: Set(sbom_package_license::LicenseCategory::SpdxConcluded),
                });
        }

        if let Some(cyclonedx) = cyclonedx_licenses {
            for (uuid, _v) in cyclonedx.licenses {
                self.sbom_packge_licenses
                    .push(sbom_package_license::ActiveModel {
                        id: Default::default(),
                        sbom_id: Set(self.sbom_id),
                        node_id: Set(node_info.node_id.clone()),
                        license_id: Set(uuid),
                        license_type: Set(sbom_package_license::LicenseCategory::Other),
                    });
            }
        }
    }

    #[instrument(
        skip_all,
        fields(
            num_packages=self.packages.len(),
            num_purl_refs=self.purl_refs.len(),
            num_cpe_refs=self.cpe_refs.len(),
        ),
        err(level=tracing::Level::INFO)
    )]
    pub async fn create(self, db: &impl ConnectionTrait) -> Result<(), DbErr> {
        self.nodes.create(db).await?;

        for batch in &self.packages.into_iter().chunked() {
            sbom_package::Entity::insert_many(batch)
                .on_conflict(
                    OnConflict::columns([
                        sbom_package::Column::SbomId,
                        sbom_package::Column::NodeId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
        }

        for batch in &self.purl_refs.into_iter().chunked() {
            sbom_package_purl_ref::Entity::insert_many(batch)
                .on_conflict(
                    OnConflict::columns([
                        sbom_package_purl_ref::Column::SbomId,
                        sbom_package_purl_ref::Column::NodeId,
                        sbom_package_purl_ref::Column::QualifiedPurlId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
        }

        for batch in &self.cpe_refs.into_iter().chunked() {
            sbom_package_cpe_ref::Entity::insert_many(batch)
                .on_conflict(
                    OnConflict::columns([
                        sbom_package_cpe_ref::Column::SbomId,
                        sbom_package_cpe_ref::Column::NodeId,
                        sbom_package_cpe_ref::Column::CpeId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
        }

        for batch in &self.purl_license_assertions.into_iter().chunked() {
            purl_license_assertion::Entity::insert_many(batch)
                .on_conflict(
                    OnConflict::columns([
                        purl_license_assertion::Column::SbomId,
                        purl_license_assertion::Column::LicenseId,
                        purl_license_assertion::Column::VersionedPurlId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
        }

        for batch in &self.cpe_license_assertions.into_iter().chunked() {
            cpe_license_assertion::Entity::insert_many(batch)
                .on_conflict(
                    OnConflict::columns([
                        cpe_license_assertion::Column::SbomId,
                        cpe_license_assertion::Column::LicenseId,
                        cpe_license_assertion::Column::CpeId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .do_nothing()
                .exec(db)
                .await?;
        }

        for batch in &self.sbom_packge_licenses.into_iter().chunked() {
            sbom_package_license::Entity::insert_many(batch)
                // .on_conflict(
                //     OnConflict::columns([
                //         sbom_packge_license::Column::SbomId,
                //         sbom_packge_license::Column::NodeId,
                //         sbom_packge_license::Column::LicenseId,
                //         sbom_packge_license::Column::LicenseType,
                //     ])
                //     .do_nothing()
                //     .to_owned(),
                // )
                .do_nothing()
                .exec(db)
                .await?;
        }

        Ok(())
    }
}

impl<'a> ReferenceSource<'a> for PackageCreator {
    fn references(&'a self) -> impl IntoIterator<Item = &'a str> {
        self.nodes.references()
    }
}
