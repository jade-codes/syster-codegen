//! Generated AST types
//!
//! Do not edit manually.

use super::super::common::span::Span;

/// Trait for AST nodes
pub trait AstNode {
    fn span(&self) -> Span;
}

// Cross-references

#[derive(Debug, Clone, Default)]
pub struct QualifiedNameRef {
    pub path: Vec<String>,
    pub span: Span,
}

// AST nodes

/// `Identification`
#[derive(Debug, Clone)]
pub struct Identification {
    pub span: Span,
    pub declared_name: Option<String>,
    pub declared_short_name: Option<String>,
}

impl AstNode for Identification {
    fn span(&self) -> Span { self.span }
}

/// `RelationshipBody`
#[derive(Debug, Clone)]
pub struct RelationshipBody {
    pub span: Span,
    pub relationship_owned_element: Vec<RelationshipOwnedElement>,
}

impl AstNode for RelationshipBody {
    fn span(&self) -> Span { self.span }
}

/// `RelationshipOwnedElement`
#[derive(Debug, Clone)]
pub struct RelationshipOwnedElement {
    pub span: Span,
    pub owned_related_element: Vec<OwnedRelatedElement>,
    pub owned_relationship: Vec<OwnedAnnotation>,
}

impl AstNode for RelationshipOwnedElement {
    fn span(&self) -> Span { self.span }
}

/// `OwnedRelatedElement`
#[derive(Debug, Clone)]
pub enum OwnedRelatedElement {
    NonFeatureElement(Box<NonFeatureElement>),
    FeatureElement(Box<FeatureElement>),
}

/// `Dependency`
#[derive(Debug, Clone)]
pub struct Dependency {
    pub span: Span,
    pub client: Vec<QualifiedNameRef>,
    pub owned_relationship: Vec<PrefixMetadataAnnotation>,
    pub supplier: Vec<QualifiedNameRef>,
}

impl AstNode for Dependency {
    fn span(&self) -> Span { self.span }
}

/// `Annotation`
#[derive(Debug, Clone)]
pub struct Annotation {
    pub span: Span,
    pub annotated_element: QualifiedNameRef,
}

impl AstNode for Annotation {
    fn span(&self) -> Span { self.span }
}

/// `OwnedAnnotation`
#[derive(Debug, Clone)]
pub struct OwnedAnnotation {
    pub span: Span,
    pub owned_related_element: Vec<AnnotatingElement>,
}

impl AstNode for OwnedAnnotation {
    fn span(&self) -> Span { self.span }
}

/// `AnnotatingElement`
#[derive(Debug, Clone)]
pub enum AnnotatingElement {
    Comment(Box<Comment>),
    Documentation(Box<Documentation>),
    TextualRepresentation(Box<TextualRepresentation>),
    MetadataFeature(Box<MetadataFeature>),
}

/// `Comment`
#[derive(Debug, Clone)]
pub struct Comment {
    pub span: Span,
    pub body: String,
    pub locale: Option<String>,
    pub owned_relationship: Vec<Annotation>,
}

impl AstNode for Comment {
    fn span(&self) -> Span { self.span }
}

/// `Documentation`
#[derive(Debug, Clone)]
pub struct Documentation {
    pub span: Span,
    pub body: String,
    pub locale: Option<String>,
}

impl AstNode for Documentation {
    fn span(&self) -> Span { self.span }
}

/// `TextualRepresentation`
#[derive(Debug, Clone)]
pub struct TextualRepresentation {
    pub span: Span,
    pub body: String,
    pub language: String,
}

impl AstNode for TextualRepresentation {
    fn span(&self) -> Span { self.span }
}

/// `RootNamespace`
#[derive(Debug, Clone)]
pub struct RootNamespace {
    pub span: Span,
    pub namespace_body_element: Vec<NamespaceBodyElement>,
}

impl AstNode for RootNamespace {
    fn span(&self) -> Span { self.span }
}

/// `Namespace`
#[derive(Debug, Clone)]
pub struct Namespace {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for Namespace {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceDeclaration`
#[derive(Debug, Clone)]
pub struct NamespaceDeclaration {
    pub span: Span,
}

impl AstNode for NamespaceDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceBody`
#[derive(Debug, Clone)]
pub struct NamespaceBody {
    pub span: Span,
    pub namespace_body_element: Vec<NamespaceBodyElement>,
}

impl AstNode for NamespaceBody {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceBodyElement`
#[derive(Debug, Clone)]
pub enum NamespaceBodyElementOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    Import(Box<Import>),
    NamespaceMember(Box<NamespaceMember>),
}

#[derive(Debug, Clone)]
pub struct NamespaceBodyElement {
    pub span: Span,
    pub owned_relationship: Vec<NamespaceBodyElementOwnedRelationshipMember>,
}

impl AstNode for NamespaceBodyElement {
    fn span(&self) -> Span { self.span }
}

/// `MemberPrefix`
#[derive(Debug, Clone)]
pub struct MemberPrefix {
    pub span: Span,
    pub visibility: Option<Box<VisibilityIndicator>>,
}

impl AstNode for MemberPrefix {
    fn span(&self) -> Span { self.span }
}

/// `VisibilityIndicator`
#[derive(Debug, Clone)]
pub struct VisibilityIndicator {
    pub span: Span,
}

impl AstNode for VisibilityIndicator {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceMember`
#[derive(Debug, Clone)]
pub enum NamespaceMember {
    NonFeatureMember(Box<NonFeatureMember>),
    NamespaceFeatureMember(Box<NamespaceFeatureMember>),
}

/// `NonFeatureMember`
#[derive(Debug, Clone)]
pub struct NonFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<MemberElement>,
}

impl AstNode for NonFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceFeatureMember`
#[derive(Debug, Clone)]
pub struct NamespaceFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FeatureElement>,
}

impl AstNode for NamespaceFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `AliasMember`
#[derive(Debug, Clone)]
pub struct AliasMember {
    pub span: Span,
    pub member_element: QualifiedNameRef,
    pub member_name: Option<String>,
    pub member_short_name: Option<String>,
}

impl AstNode for AliasMember {
    fn span(&self) -> Span { self.span }
}

/// `QualifiedName`
#[derive(Debug, Clone)]
pub struct QualifiedName {
    pub span: Span,
}

impl AstNode for QualifiedName {
    fn span(&self) -> Span { self.span }
}

/// `Import`
#[derive(Debug, Clone)]
pub struct Import {
    pub span: Span,
    pub is_import_all: bool,
    pub visibility: Box<VisibilityIndicator>,
}

impl AstNode for Import {
    fn span(&self) -> Span { self.span }
}

/// `ImportDeclaration`
#[derive(Debug, Clone)]
pub enum ImportDeclaration {
    MembershipImport(Box<MembershipImport>),
    NamespaceImport(Box<NamespaceImport>),
}

/// `MembershipImport`
#[derive(Debug, Clone)]
pub struct MembershipImport {
    pub span: Span,
    pub imported_membership: QualifiedNameRef,
    pub is_recursive: bool,
}

impl AstNode for MembershipImport {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceImport`
#[derive(Debug, Clone)]
pub enum NamespaceImportImportedNamespaceMember {
    QualifiedNameRef(QualifiedNameRef),
    FilterPackage(Box<FilterPackage>),
}

#[derive(Debug, Clone)]
pub struct NamespaceImport {
    pub span: Span,
    pub imported_namespace: Option<NamespaceImportImportedNamespaceMember>,
    pub is_recursive: bool,
}

impl AstNode for NamespaceImport {
    fn span(&self) -> Span { self.span }
}

/// `FilterPackage`
#[derive(Debug, Clone)]
pub enum FilterPackageOwnedRelationshipMember {
    FilterPackageMember(Box<FilterPackageMember>),
    ImportDeclaration(Box<ImportDeclaration>),
}

#[derive(Debug, Clone)]
pub struct FilterPackage {
    pub span: Span,
    pub owned_relationship: Vec<FilterPackageOwnedRelationshipMember>,
}

impl AstNode for FilterPackage {
    fn span(&self) -> Span { self.span }
}

/// `FilterPackageMember`
#[derive(Debug, Clone)]
pub struct FilterPackageMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for FilterPackageMember {
    fn span(&self) -> Span { self.span }
}

/// `MemberElement`
#[derive(Debug, Clone)]
pub enum MemberElement {
    AnnotatingElement(Box<AnnotatingElement>),
    NonFeatureElement(Box<NonFeatureElement>),
}

/// `NonFeatureElement`
#[derive(Debug, Clone)]
pub enum NonFeatureElement {
    Dependency(Box<Dependency>),
    Namespace(Box<Namespace>),
    Type(Box<Type>),
    Classifier(Box<Classifier>),
    DataType(Box<DataType>),
    Class(Box<Class>),
    Structure(Box<Structure>),
    Metaclass(Box<Metaclass>),
    Association(Box<Association>),
    AssociationStructure(Box<AssociationStructure>),
    Interaction(Box<Interaction>),
    Behavior(Box<Behavior>),
    Function(Box<Function>),
    Predicate(Box<Predicate>),
    Multiplicity(Box<Multiplicity>),
    Package(Box<Package>),
    LibraryPackage(Box<LibraryPackage>),
    Specialization(Box<Specialization>),
    Conjugation(Box<Conjugation>),
    Subclassification(Box<Subclassification>),
    Disjoining(Box<Disjoining>),
    FeatureInverting(Box<FeatureInverting>),
    FeatureTyping(Box<FeatureTyping>),
    Subsetting(Box<Subsetting>),
    Redefinition(Box<Redefinition>),
    TypeFeaturing(Box<TypeFeaturing>),
}

/// `FeatureElement`
#[derive(Debug, Clone)]
pub enum FeatureElement {
    Feature(Box<Feature>),
    Step(Box<Step>),
    Expression(Box<Expression>),
    BooleanExpression(Box<BooleanExpression>),
    Invariant(Box<Invariant>),
    Connector(Box<Connector>),
    BindingConnector(Box<BindingConnector>),
    Succession(Box<Succession>),
    Flow(Box<Flow>),
    SuccessionFlow(Box<SuccessionFlow>),
}

/// `Type`
#[derive(Debug, Clone)]
pub struct Type {
    pub span: Span,
}

impl AstNode for Type {
    fn span(&self) -> Span { self.span }
}

/// `TypePrefix`
#[derive(Debug, Clone)]
pub struct TypePrefix {
    pub span: Span,
    pub is_abstract: bool,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for TypePrefix {
    fn span(&self) -> Span { self.span }
}

/// `TypeDeclaration`
#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<OwnedMultiplicity>,
    pub type_relationship_part: Vec<TypeRelationshipPart>,
}

impl AstNode for TypeDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `SpecializationPart`
#[derive(Debug, Clone)]
pub struct SpecializationPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedSpecialization>,
}

impl AstNode for SpecializationPart {
    fn span(&self) -> Span { self.span }
}

/// `ConjugationPart`
#[derive(Debug, Clone)]
pub struct ConjugationPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedConjugation>,
}

impl AstNode for ConjugationPart {
    fn span(&self) -> Span { self.span }
}

/// `TypeRelationshipPart`
#[derive(Debug, Clone)]
pub enum TypeRelationshipPart {
    DisjoiningPart(Box<DisjoiningPart>),
    UnioningPart(Box<UnioningPart>),
    IntersectingPart(Box<IntersectingPart>),
    DifferencingPart(Box<DifferencingPart>),
}

/// `DisjoiningPart`
#[derive(Debug, Clone)]
pub struct DisjoiningPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedDisjoining>,
}

impl AstNode for DisjoiningPart {
    fn span(&self) -> Span { self.span }
}

/// `UnioningPart`
#[derive(Debug, Clone)]
pub struct UnioningPart {
    pub span: Span,
    pub owned_relationship: Vec<Unioning>,
}

impl AstNode for UnioningPart {
    fn span(&self) -> Span { self.span }
}

/// `IntersectingPart`
#[derive(Debug, Clone)]
pub struct IntersectingPart {
    pub span: Span,
    pub owned_relationship: Vec<Intersecting>,
}

impl AstNode for IntersectingPart {
    fn span(&self) -> Span { self.span }
}

/// `DifferencingPart`
#[derive(Debug, Clone)]
pub struct DifferencingPart {
    pub span: Span,
    pub owned_relationship: Vec<Differencing>,
}

impl AstNode for DifferencingPart {
    fn span(&self) -> Span { self.span }
}

/// `TypeBody`
#[derive(Debug, Clone)]
pub struct TypeBody {
    pub span: Span,
    pub type_body_element: Vec<TypeBodyElement>,
}

impl AstNode for TypeBody {
    fn span(&self) -> Span { self.span }
}

/// `TypeBodyElement`
#[derive(Debug, Clone)]
pub enum TypeBodyElementOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    FeatureMember(Box<FeatureMember>),
    Import(Box<Import>),
    NonFeatureMember(Box<NonFeatureMember>),
}

#[derive(Debug, Clone)]
pub struct TypeBodyElement {
    pub span: Span,
    pub owned_relationship: Vec<TypeBodyElementOwnedRelationshipMember>,
}

impl AstNode for TypeBodyElement {
    fn span(&self) -> Span { self.span }
}

/// `Specialization`
#[derive(Debug, Clone)]
pub struct Specialization {
    pub span: Span,
}

impl AstNode for Specialization {
    fn span(&self) -> Span { self.span }
}

/// `OwnedSpecialization`
#[derive(Debug, Clone)]
pub struct OwnedSpecialization {
    pub span: Span,
}

impl AstNode for OwnedSpecialization {
    fn span(&self) -> Span { self.span }
}

/// `SpecificType`
#[derive(Debug, Clone)]
pub enum SpecificTypeSpecificMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct SpecificType {
    pub span: Span,
    pub specific: Vec<SpecificTypeSpecificMember>,
}

impl AstNode for SpecificType {
    fn span(&self) -> Span { self.span }
}

/// `GeneralType`
#[derive(Debug, Clone)]
pub enum GeneralTypeGeneralMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct GeneralType {
    pub span: Span,
    pub general: Vec<GeneralTypeGeneralMember>,
}

impl AstNode for GeneralType {
    fn span(&self) -> Span { self.span }
}

/// `Conjugation`
#[derive(Debug, Clone)]
pub enum ConjugationConjugatedTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub enum ConjugationOriginalTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub struct Conjugation {
    pub span: Span,
    pub conjugated_type: Option<ConjugationConjugatedTypeMember>,
    pub original_type: Option<ConjugationOriginalTypeMember>,
}

impl AstNode for Conjugation {
    fn span(&self) -> Span { self.span }
}

/// `OwnedConjugation`
#[derive(Debug, Clone)]
pub enum OwnedConjugationOriginalTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedConjugation {
    pub span: Span,
    pub original_type: Option<OwnedConjugationOriginalTypeMember>,
}

impl AstNode for OwnedConjugation {
    fn span(&self) -> Span { self.span }
}

/// `Disjoining`
#[derive(Debug, Clone)]
pub enum DisjoiningDisjoiningTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub enum DisjoiningTypeDisjoinedMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub struct Disjoining {
    pub span: Span,
    pub disjoining_type: Option<DisjoiningDisjoiningTypeMember>,
    pub type_disjoined: Option<DisjoiningTypeDisjoinedMember>,
}

impl AstNode for Disjoining {
    fn span(&self) -> Span { self.span }
}

/// `OwnedDisjoining`
#[derive(Debug, Clone)]
pub enum OwnedDisjoiningDisjoiningTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChain(Box<FeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedDisjoining {
    pub span: Span,
    pub disjoining_type: Option<OwnedDisjoiningDisjoiningTypeMember>,
}

impl AstNode for OwnedDisjoining {
    fn span(&self) -> Span { self.span }
}

/// `Unioning`
#[derive(Debug, Clone)]
pub struct Unioning {
    pub span: Span,
    pub owned_related_element: Vec<OwnedFeatureChain>,
    pub unioning_type: Option<QualifiedNameRef>,
}

impl AstNode for Unioning {
    fn span(&self) -> Span { self.span }
}

/// `Intersecting`
#[derive(Debug, Clone)]
pub struct Intersecting {
    pub span: Span,
    pub intersecting_type: Option<QualifiedNameRef>,
    pub owned_related_element: Vec<OwnedFeatureChain>,
}

impl AstNode for Intersecting {
    fn span(&self) -> Span { self.span }
}

/// `Differencing`
#[derive(Debug, Clone)]
pub struct Differencing {
    pub span: Span,
    pub differencing_type: Option<QualifiedNameRef>,
    pub owned_related_element: Vec<OwnedFeatureChain>,
}

impl AstNode for Differencing {
    fn span(&self) -> Span { self.span }
}

/// `FeatureMember`
#[derive(Debug, Clone)]
pub enum FeatureMember {
    TypeFeatureMember(Box<TypeFeatureMember>),
    OwnedFeatureMember(Box<OwnedFeatureMember>),
}

/// `TypeFeatureMember`
#[derive(Debug, Clone)]
pub struct TypeFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FeatureElement>,
}

impl AstNode for TypeFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureMember`
#[derive(Debug, Clone)]
pub struct OwnedFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FeatureElement>,
}

impl AstNode for OwnedFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `Classifier`
#[derive(Debug, Clone)]
pub struct Classifier {
    pub span: Span,
}

impl AstNode for Classifier {
    fn span(&self) -> Span { self.span }
}

/// `ClassifierDeclaration`
#[derive(Debug, Clone)]
pub struct ClassifierDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<OwnedMultiplicity>,
    pub type_relationship_part: Vec<TypeRelationshipPart>,
}

impl AstNode for ClassifierDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `SuperclassingPart`
#[derive(Debug, Clone)]
pub struct SuperclassingPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedSubclassification>,
}

impl AstNode for SuperclassingPart {
    fn span(&self) -> Span { self.span }
}

/// `Subclassification`
#[derive(Debug, Clone)]
pub struct Subclassification {
    pub span: Span,
    pub subclassifier: QualifiedNameRef,
    pub superclassifier: QualifiedNameRef,
}

impl AstNode for Subclassification {
    fn span(&self) -> Span { self.span }
}

/// `OwnedSubclassification`
#[derive(Debug, Clone)]
pub struct OwnedSubclassification {
    pub span: Span,
    pub superclassifier: QualifiedNameRef,
}

impl AstNode for OwnedSubclassification {
    fn span(&self) -> Span { self.span }
}

/// `Feature`
#[derive(Debug, Clone)]
pub struct Feature {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for Feature {
    fn span(&self) -> Span { self.span }
}

/// `EndFeaturePrefix`
#[derive(Debug, Clone)]
pub struct EndFeaturePrefix {
    pub span: Span,
    pub is_constant: bool,
    pub is_end: bool,
}

impl AstNode for EndFeaturePrefix {
    fn span(&self) -> Span { self.span }
}

/// `BasicFeaturePrefix`
#[derive(Debug, Clone)]
pub struct BasicFeaturePrefix {
    pub span: Span,
    pub direction: Option<Box<FeatureDirection>>,
    pub is_abstract: bool,
    pub is_composite: bool,
    pub is_constant: bool,
    pub is_derived: bool,
    pub is_portion: bool,
    pub is_variable: bool,
}

impl AstNode for BasicFeaturePrefix {
    fn span(&self) -> Span { self.span }
}

/// `FeaturePrefix`
#[derive(Debug, Clone)]
pub enum FeaturePrefixOwnedRelationshipMember {
    OwnedCrossFeatureMember(Box<OwnedCrossFeatureMember>),
    PrefixMetadataMember(Box<PrefixMetadataMember>),
}

#[derive(Debug, Clone)]
pub struct FeaturePrefix {
    pub span: Span,
    pub owned_relationship: Vec<FeaturePrefixOwnedRelationshipMember>,
}

impl AstNode for FeaturePrefix {
    fn span(&self) -> Span { self.span }
}

/// `OwnedCrossFeatureMember`
#[derive(Debug, Clone)]
pub struct OwnedCrossFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedCrossFeature>,
}

impl AstNode for OwnedCrossFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `OwnedCrossFeature`
#[derive(Debug, Clone)]
pub struct OwnedCrossFeature {
    pub span: Span,
}

impl AstNode for OwnedCrossFeature {
    fn span(&self) -> Span { self.span }
}

/// `FeatureDirection`
#[derive(Debug, Clone)]
pub struct FeatureDirection {
    pub span: Span,
}

impl AstNode for FeatureDirection {
    fn span(&self) -> Span { self.span }
}

/// `FeatureDeclaration`
#[derive(Debug, Clone)]
pub struct FeatureDeclaration {
    pub span: Span,
    pub feature_relationship_part: Vec<FeatureRelationshipPart>,
    pub is_sufficient: bool,
}

impl AstNode for FeatureDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `FeatureIdentification`
#[derive(Debug, Clone)]
pub struct FeatureIdentification {
    pub span: Span,
    pub declared_name: Option<String>,
    pub declared_short_name: Option<String>,
}

impl AstNode for FeatureIdentification {
    fn span(&self) -> Span { self.span }
}

/// `FeatureRelationshipPart`
#[derive(Debug, Clone)]
pub enum FeatureRelationshipPart {
    TypeRelationshipPart(Box<TypeRelationshipPart>),
    ChainingPart(Box<ChainingPart>),
    InvertingPart(Box<InvertingPart>),
    TypeFeaturingPart(Box<TypeFeaturingPart>),
}

/// `ChainingPart`
#[derive(Debug, Clone)]
pub struct ChainingPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureChaining>,
}

impl AstNode for ChainingPart {
    fn span(&self) -> Span { self.span }
}

/// `InvertingPart`
#[derive(Debug, Clone)]
pub struct InvertingPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureInverting>,
}

impl AstNode for InvertingPart {
    fn span(&self) -> Span { self.span }
}

/// `TypeFeaturingPart`
#[derive(Debug, Clone)]
pub struct TypeFeaturingPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedTypeFeaturing>,
    pub owned_type_featuring: Vec<OwnedTypeFeaturing>,
}

impl AstNode for TypeFeaturingPart {
    fn span(&self) -> Span { self.span }
}

/// `FeatureSpecializationPart`
#[derive(Debug, Clone)]
pub struct FeatureSpecializationPart {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
}

impl AstNode for FeatureSpecializationPart {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicityPart`
#[derive(Debug, Clone)]
pub struct MultiplicityPart {
    pub span: Span,
    pub is_ordered: bool,
    pub owned_relationship: Vec<OwnedMultiplicity>,
}

impl AstNode for MultiplicityPart {
    fn span(&self) -> Span { self.span }
}

/// `FeatureSpecialization`
#[derive(Debug, Clone)]
pub enum FeatureSpecialization {
    Typings(Box<Typings>),
    Subsettings(Box<Subsettings>),
    References(Box<References>),
    Crosses(Box<Crosses>),
    Redefinitions(Box<Redefinitions>),
}

/// `Typings`
#[derive(Debug, Clone)]
pub struct Typings {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for Typings {
    fn span(&self) -> Span { self.span }
}

/// `TypedBy`
#[derive(Debug, Clone)]
pub struct TypedBy {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for TypedBy {
    fn span(&self) -> Span { self.span }
}

/// `Subsettings`
#[derive(Debug, Clone)]
pub struct Subsettings {
    pub span: Span,
    pub owned_relationship: Vec<OwnedSubsetting>,
}

impl AstNode for Subsettings {
    fn span(&self) -> Span { self.span }
}

/// `Subsets`
#[derive(Debug, Clone)]
pub struct Subsets {
    pub span: Span,
    pub owned_relationship: Vec<OwnedSubsetting>,
}

impl AstNode for Subsets {
    fn span(&self) -> Span { self.span }
}

/// `References`
#[derive(Debug, Clone)]
pub struct References {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for References {
    fn span(&self) -> Span { self.span }
}

/// `Crosses`
#[derive(Debug, Clone)]
pub struct Crosses {
    pub span: Span,
    pub owned_relationship: Vec<OwnedCrossSubsetting>,
}

impl AstNode for Crosses {
    fn span(&self) -> Span { self.span }
}

/// `Redefinitions`
#[derive(Debug, Clone)]
pub struct Redefinitions {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for Redefinitions {
    fn span(&self) -> Span { self.span }
}

/// `Redefines`
#[derive(Debug, Clone)]
pub struct Redefines {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for Redefines {
    fn span(&self) -> Span { self.span }
}

/// `FeatureTyping`
#[derive(Debug, Clone)]
pub struct FeatureTyping {
    pub span: Span,
    pub typed_feature: QualifiedNameRef,
}

impl AstNode for FeatureTyping {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureTyping`
#[derive(Debug, Clone)]
pub struct OwnedFeatureTyping {
    pub span: Span,
}

impl AstNode for OwnedFeatureTyping {
    fn span(&self) -> Span { self.span }
}

/// `Subsetting`
#[derive(Debug, Clone)]
pub struct Subsetting {
    pub span: Span,
}

impl AstNode for Subsetting {
    fn span(&self) -> Span { self.span }
}

/// `OwnedSubsetting`
#[derive(Debug, Clone)]
pub struct OwnedSubsetting {
    pub span: Span,
}

impl AstNode for OwnedSubsetting {
    fn span(&self) -> Span { self.span }
}

/// `OwnedReferenceSubsetting`
#[derive(Debug, Clone)]
pub struct OwnedReferenceSubsetting {
    pub span: Span,
}

impl AstNode for OwnedReferenceSubsetting {
    fn span(&self) -> Span { self.span }
}

/// `OwnedCrossSubsetting`
#[derive(Debug, Clone)]
pub struct OwnedCrossSubsetting {
    pub span: Span,
}

impl AstNode for OwnedCrossSubsetting {
    fn span(&self) -> Span { self.span }
}

/// `Redefinition`
#[derive(Debug, Clone)]
pub struct Redefinition {
    pub span: Span,
}

impl AstNode for Redefinition {
    fn span(&self) -> Span { self.span }
}

/// `OwnedRedefinition`
#[derive(Debug, Clone)]
pub struct OwnedRedefinition {
    pub span: Span,
}

impl AstNode for OwnedRedefinition {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureChain`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChain {
    pub span: Span,
}

impl AstNode for OwnedFeatureChain {
    fn span(&self) -> Span { self.span }
}

/// `FeatureChain`
#[derive(Debug, Clone)]
pub struct FeatureChain {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureChaining>,
}

impl AstNode for FeatureChain {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureChaining`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChaining {
    pub span: Span,
    pub chaining_feature: QualifiedNameRef,
}

impl AstNode for OwnedFeatureChaining {
    fn span(&self) -> Span { self.span }
}

/// `FeatureInverting`
#[derive(Debug, Clone)]
pub enum FeatureInvertingFeatureInvertedMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct FeatureInverting {
    pub span: Span,
    pub feature_inverted: Option<FeatureInvertingFeatureInvertedMember>,
    pub inverting_feature: Option<QualifiedNameRef>,
    pub owned_related_element: Vec<OwnedFeatureChain>,
}

impl AstNode for FeatureInverting {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureInverting`
#[derive(Debug, Clone)]
pub enum OwnedFeatureInvertingInvertingFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedFeatureInverting {
    pub span: Span,
    pub inverting_feature: Option<OwnedFeatureInvertingInvertingFeatureMember>,
}

impl AstNode for OwnedFeatureInverting {
    fn span(&self) -> Span { self.span }
}

/// `TypeFeaturing`
#[derive(Debug, Clone)]
pub struct TypeFeaturing {
    pub span: Span,
    pub feature_of_type: QualifiedNameRef,
    pub featuring_type: QualifiedNameRef,
}

impl AstNode for TypeFeaturing {
    fn span(&self) -> Span { self.span }
}

/// `OwnedTypeFeaturing`
#[derive(Debug, Clone)]
pub struct OwnedTypeFeaturing {
    pub span: Span,
    pub featuring_type: QualifiedNameRef,
}

impl AstNode for OwnedTypeFeaturing {
    fn span(&self) -> Span { self.span }
}

/// `DataType`
#[derive(Debug, Clone)]
pub struct DataType {
    pub span: Span,
}

impl AstNode for DataType {
    fn span(&self) -> Span { self.span }
}

/// `Class`
#[derive(Debug, Clone)]
pub struct Class {
    pub span: Span,
}

impl AstNode for Class {
    fn span(&self) -> Span { self.span }
}

/// `Structure`
#[derive(Debug, Clone)]
pub struct Structure {
    pub span: Span,
}

impl AstNode for Structure {
    fn span(&self) -> Span { self.span }
}

/// `Association`
#[derive(Debug, Clone)]
pub struct Association {
    pub span: Span,
}

impl AstNode for Association {
    fn span(&self) -> Span { self.span }
}

/// `AssociationStructure`
#[derive(Debug, Clone)]
pub struct AssociationStructure {
    pub span: Span,
}

impl AstNode for AssociationStructure {
    fn span(&self) -> Span { self.span }
}

/// `Connector`
#[derive(Debug, Clone)]
pub struct Connector {
    pub span: Span,
}

impl AstNode for Connector {
    fn span(&self) -> Span { self.span }
}

/// `ConnectorDeclaration`
#[derive(Debug, Clone)]
pub enum ConnectorDeclaration {
    BinaryConnectorDeclaration(Box<BinaryConnectorDeclaration>),
    NaryConnectorDeclaration(Box<NaryConnectorDeclaration>),
}

/// `BinaryConnectorDeclaration`
#[derive(Debug, Clone)]
pub struct BinaryConnectorDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for BinaryConnectorDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `NaryConnectorDeclaration`
#[derive(Debug, Clone)]
pub struct NaryConnectorDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for NaryConnectorDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `ConnectorEndMember`
#[derive(Debug, Clone)]
pub struct ConnectorEndMember {
    pub span: Span,
    pub owned_related_element: Vec<ConnectorEnd>,
}

impl AstNode for ConnectorEndMember {
    fn span(&self) -> Span { self.span }
}

/// `ConnectorEnd`
#[derive(Debug, Clone)]
pub enum ConnectorEndOwnedRelationshipMember {
    OwnedCrossMultiplicityMember(Box<OwnedCrossMultiplicityMember>),
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
}

#[derive(Debug, Clone)]
pub struct ConnectorEnd {
    pub span: Span,
    pub declared_name: Option<String>,
    pub owned_relationship: Vec<ConnectorEndOwnedRelationshipMember>,
}

impl AstNode for ConnectorEnd {
    fn span(&self) -> Span { self.span }
}

/// `OwnedCrossMultiplicityMember`
#[derive(Debug, Clone)]
pub struct OwnedCrossMultiplicityMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedCrossMultiplicity>,
}

impl AstNode for OwnedCrossMultiplicityMember {
    fn span(&self) -> Span { self.span }
}

/// `OwnedCrossMultiplicity`
#[derive(Debug, Clone)]
pub struct OwnedCrossMultiplicity {
    pub span: Span,
    pub owned_relationship: Vec<OwnedMultiplicity>,
}

impl AstNode for OwnedCrossMultiplicity {
    fn span(&self) -> Span { self.span }
}

/// `BindingConnector`
#[derive(Debug, Clone)]
pub struct BindingConnector {
    pub span: Span,
}

impl AstNode for BindingConnector {
    fn span(&self) -> Span { self.span }
}

/// `BindingConnectorDeclaration`
#[derive(Debug, Clone)]
pub struct BindingConnectorDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for BindingConnectorDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `Succession`
#[derive(Debug, Clone)]
pub struct Succession {
    pub span: Span,
}

impl AstNode for Succession {
    fn span(&self) -> Span { self.span }
}

/// `SuccessionDeclaration`
#[derive(Debug, Clone)]
pub struct SuccessionDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for SuccessionDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `Behavior`
#[derive(Debug, Clone)]
pub struct Behavior {
    pub span: Span,
}

impl AstNode for Behavior {
    fn span(&self) -> Span { self.span }
}

/// `Step`
#[derive(Debug, Clone)]
pub struct Step {
    pub span: Span,
}

impl AstNode for Step {
    fn span(&self) -> Span { self.span }
}

/// `Function`
#[derive(Debug, Clone)]
pub struct Function {
    pub span: Span,
}

impl AstNode for Function {
    fn span(&self) -> Span { self.span }
}

/// `FunctionBody`
#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub span: Span,
}

impl AstNode for FunctionBody {
    fn span(&self) -> Span { self.span }
}

/// `FunctionBodyPart`
#[derive(Debug, Clone)]
pub enum FunctionBodyPartOwnedRelationshipMember {
    ResultExpressionMember(Box<ResultExpressionMember>),
    ReturnFeatureMember(Box<ReturnFeatureMember>),
}

#[derive(Debug, Clone)]
pub struct FunctionBodyPart {
    pub span: Span,
    pub owned_relationship: Vec<FunctionBodyPartOwnedRelationshipMember>,
}

impl AstNode for FunctionBodyPart {
    fn span(&self) -> Span { self.span }
}

/// `ReturnFeatureMember`
#[derive(Debug, Clone)]
pub struct ReturnFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FeatureElement>,
}

impl AstNode for ReturnFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `ResultExpressionMember`
#[derive(Debug, Clone)]
pub struct ResultExpressionMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for ResultExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `Expression`
#[derive(Debug, Clone)]
pub struct Expression {
    pub span: Span,
}

impl AstNode for Expression {
    fn span(&self) -> Span { self.span }
}

/// `Predicate`
#[derive(Debug, Clone)]
pub struct Predicate {
    pub span: Span,
}

impl AstNode for Predicate {
    fn span(&self) -> Span { self.span }
}

/// `BooleanExpression`
#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub span: Span,
}

impl AstNode for BooleanExpression {
    fn span(&self) -> Span { self.span }
}

/// `Invariant`
#[derive(Debug, Clone)]
pub struct Invariant {
    pub span: Span,
    pub is_negated: bool,
}

impl AstNode for Invariant {
    fn span(&self) -> Span { self.span }
}

/// `OwnedExpressionReferenceMember`
#[derive(Debug, Clone)]
pub struct OwnedExpressionReferenceMember {
    pub span: Span,
    pub owned_relationship: Vec<OwnedExpressionReference>,
}

impl AstNode for OwnedExpressionReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `OwnedExpressionReference`
#[derive(Debug, Clone)]
pub struct OwnedExpressionReference {
    pub span: Span,
    pub owned_relationship: Vec<OwnedExpressionMember>,
}

impl AstNode for OwnedExpressionReference {
    fn span(&self) -> Span { self.span }
}

/// `OwnedExpressionMember`
#[derive(Debug, Clone)]
pub struct OwnedExpressionMember {
    pub span: Span,
    pub owned_feature_member: Box<OwnedExpression>,
}

impl AstNode for OwnedExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `OwnedExpression`
#[derive(Debug, Clone)]
pub enum OwnedExpression {
    ConditionalExpression(Box<ConditionalExpression>),
    ConditionalBinaryOperatorExpression(Box<ConditionalBinaryOperatorExpression>),
    BinaryOperatorExpression(Box<BinaryOperatorExpression>),
    UnaryOperatorExpression(Box<UnaryOperatorExpression>),
    ClassificationExpression(Box<ClassificationExpression>),
    MetaclassificationExpression(Box<MetaclassificationExpression>),
    ExtentExpression(Box<ExtentExpression>),
    PrimaryExpression(Box<PrimaryExpression>),
}

/// `ConditionalExpression`
#[derive(Debug, Clone)]
pub enum ConditionalExpressionOwnedRelationshipMember {
    ArgumentExpressionMember(Box<ArgumentExpressionMember>),
    ArgumentMember(Box<ArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
}

#[derive(Debug, Clone)]
pub struct ConditionalExpression {
    pub span: Span,
    pub operator: bool,
    pub owned_relationship: Vec<ConditionalExpressionOwnedRelationshipMember>,
}

impl AstNode for ConditionalExpression {
    fn span(&self) -> Span { self.span }
}

/// `ConditionalBinaryOperatorExpression`
#[derive(Debug, Clone)]
pub enum ConditionalBinaryOperatorExpressionOwnedRelationshipMember {
    ArgumentExpressionMember(Box<ArgumentExpressionMember>),
    ArgumentMember(Box<ArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
}

#[derive(Debug, Clone)]
pub struct ConditionalBinaryOperatorExpression {
    pub span: Span,
    pub operator: Box<ConditionalBinaryOperator>,
    pub owned_relationship: Vec<ConditionalBinaryOperatorExpressionOwnedRelationshipMember>,
}

impl AstNode for ConditionalBinaryOperatorExpression {
    fn span(&self) -> Span { self.span }
}

/// `ConditionalBinaryOperator`
#[derive(Debug, Clone)]
pub struct ConditionalBinaryOperator {
    pub span: Span,
}

impl AstNode for ConditionalBinaryOperator {
    fn span(&self) -> Span { self.span }
}

/// `BinaryOperatorExpression`
#[derive(Debug, Clone)]
pub enum BinaryOperatorExpressionOwnedRelationshipMember {
    ArgumentMember(Box<ArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorExpression {
    pub span: Span,
    pub operator: Box<BinaryOperator>,
    pub owned_relationship: Vec<BinaryOperatorExpressionOwnedRelationshipMember>,
}

impl AstNode for BinaryOperatorExpression {
    fn span(&self) -> Span { self.span }
}

/// `BinaryOperator`
#[derive(Debug, Clone)]
pub struct BinaryOperator {
    pub span: Span,
}

impl AstNode for BinaryOperator {
    fn span(&self) -> Span { self.span }
}

/// `UnaryOperatorExpression`
#[derive(Debug, Clone)]
pub enum UnaryOperatorExpressionOwnedRelationshipMember {
    ArgumentMember(Box<ArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
}

#[derive(Debug, Clone)]
pub struct UnaryOperatorExpression {
    pub span: Span,
    pub operator: Box<UnaryOperator>,
    pub owned_relationship: Vec<UnaryOperatorExpressionOwnedRelationshipMember>,
}

impl AstNode for UnaryOperatorExpression {
    fn span(&self) -> Span { self.span }
}

/// `UnaryOperator`
#[derive(Debug, Clone)]
pub struct UnaryOperator {
    pub span: Span,
}

impl AstNode for UnaryOperator {
    fn span(&self) -> Span { self.span }
}

/// `ClassificationExpression`
#[derive(Debug, Clone)]
pub enum ClassificationExpressionOperatorMember {
    CastOperator(Box<CastOperator>),
    ClassificationTestOperator(Box<ClassificationTestOperator>),
}

#[derive(Debug, Clone)]
pub enum ClassificationExpressionOwnedRelationshipMember {
    ArgumentMember(Box<ArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
    TypeReferenceMember(Box<TypeReferenceMember>),
    TypeResultMember(Box<TypeResultMember>),
}

#[derive(Debug, Clone)]
pub struct ClassificationExpression {
    pub span: Span,
    pub operator: Option<ClassificationExpressionOperatorMember>,
    pub owned_relationship: Vec<ClassificationExpressionOwnedRelationshipMember>,
}

impl AstNode for ClassificationExpression {
    fn span(&self) -> Span { self.span }
}

/// `ClassificationTestOperator`
#[derive(Debug, Clone)]
pub struct ClassificationTestOperator {
    pub span: Span,
}

impl AstNode for ClassificationTestOperator {
    fn span(&self) -> Span { self.span }
}

/// `CastOperator`
#[derive(Debug, Clone)]
pub struct CastOperator {
    pub span: Span,
}

impl AstNode for CastOperator {
    fn span(&self) -> Span { self.span }
}

/// `MetaclassificationExpression`
#[derive(Debug, Clone)]
pub enum MetaclassificationExpressionOperatorMember {
    ClassificationTestOperator(Box<ClassificationTestOperator>),
    MetaCastOperator(Box<MetaCastOperator>),
}

#[derive(Debug, Clone)]
pub enum MetaclassificationExpressionOwnedRelationshipMember {
    EmptyResultMember(Box<EmptyResultMember>),
    MetadataArgumentMember(Box<MetadataArgumentMember>),
    TypeReferenceMember(Box<TypeReferenceMember>),
    TypeResultMember(Box<TypeResultMember>),
}

#[derive(Debug, Clone)]
pub struct MetaclassificationExpression {
    pub span: Span,
    pub operator: Option<MetaclassificationExpressionOperatorMember>,
    pub owned_relationship: Vec<MetaclassificationExpressionOwnedRelationshipMember>,
}

impl AstNode for MetaclassificationExpression {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentMember`
#[derive(Debug, Clone)]
pub struct ArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<Argument>,
}

impl AstNode for ArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `Argument`
#[derive(Debug, Clone)]
pub struct Argument {
    pub span: Span,
    pub owned_relationship: Vec<ArgumentValue>,
}

impl AstNode for Argument {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentValue`
#[derive(Debug, Clone)]
pub struct ArgumentValue {
    pub span: Span,
    pub value: Box<OwnedExpression>,
}

impl AstNode for ArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentExpressionMember`
#[derive(Debug, Clone)]
pub struct ArgumentExpressionMember {
    pub span: Span,
    pub owned_related_element: Vec<ArgumentExpression>,
}

impl AstNode for ArgumentExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentExpression`
#[derive(Debug, Clone)]
pub struct ArgumentExpression {
    pub span: Span,
    pub owned_relationship: Vec<ArgumentExpressionValue>,
}

impl AstNode for ArgumentExpression {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentExpressionValue`
#[derive(Debug, Clone)]
pub struct ArgumentExpressionValue {
    pub span: Span,
    pub value: Box<OwnedExpressionReference>,
}

impl AstNode for ArgumentExpressionValue {
    fn span(&self) -> Span { self.span }
}

/// `MetadataArgumentMember`
#[derive(Debug, Clone)]
pub struct MetadataArgumentMember {
    pub span: Span,
    pub owned_related_element: Vec<MetadataArgument>,
}

impl AstNode for MetadataArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `MetadataArgument`
#[derive(Debug, Clone)]
pub struct MetadataArgument {
    pub span: Span,
    pub owned_relationship: Vec<MetadataValue>,
}

impl AstNode for MetadataArgument {
    fn span(&self) -> Span { self.span }
}

/// `MetadataValue`
#[derive(Debug, Clone)]
pub struct MetadataValue {
    pub span: Span,
    pub value: Box<MetadataReference>,
}

impl AstNode for MetadataValue {
    fn span(&self) -> Span { self.span }
}

/// `MetadataReference`
#[derive(Debug, Clone)]
pub struct MetadataReference {
    pub span: Span,
    pub owned_relationship: Vec<ElementReferenceMember>,
}

impl AstNode for MetadataReference {
    fn span(&self) -> Span { self.span }
}

/// `MetaclassificationTestOperator`
#[derive(Debug, Clone)]
pub struct MetaclassificationTestOperator {
    pub span: Span,
}

impl AstNode for MetaclassificationTestOperator {
    fn span(&self) -> Span { self.span }
}

/// `MetaCastOperator`
#[derive(Debug, Clone)]
pub struct MetaCastOperator {
    pub span: Span,
}

impl AstNode for MetaCastOperator {
    fn span(&self) -> Span { self.span }
}

/// `ExtentExpression`
#[derive(Debug, Clone)]
pub struct ExtentExpression {
    pub span: Span,
    pub operator: bool,
    pub owned_relationship: Vec<TypeReferenceMember>,
}

impl AstNode for ExtentExpression {
    fn span(&self) -> Span { self.span }
}

/// `TypeReferenceMember`
#[derive(Debug, Clone)]
pub struct TypeReferenceMember {
    pub span: Span,
    pub owned_member_feature: Box<TypeReference>,
}

impl AstNode for TypeReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `TypeResultMember`
#[derive(Debug, Clone)]
pub struct TypeResultMember {
    pub span: Span,
    pub owned_member_feature: Box<TypeReference>,
}

impl AstNode for TypeResultMember {
    fn span(&self) -> Span { self.span }
}

/// `TypeReference`
#[derive(Debug, Clone)]
pub struct TypeReference {
    pub span: Span,
    pub owned_relationship: Vec<ReferenceTyping>,
}

impl AstNode for TypeReference {
    fn span(&self) -> Span { self.span }
}

/// `ReferenceTyping`
#[derive(Debug, Clone)]
pub struct ReferenceTyping {
    pub span: Span,
    pub type_: QualifiedNameRef,
}

impl AstNode for ReferenceTyping {
    fn span(&self) -> Span { self.span }
}

/// `EmptyResultMember`
#[derive(Debug, Clone)]
pub struct EmptyResultMember {
    pub span: Span,
    pub owned_related_element: Vec<EmptyFeature>,
}

impl AstNode for EmptyResultMember {
    fn span(&self) -> Span { self.span }
}

/// `EmptyFeature`
#[derive(Debug, Clone)]
pub struct EmptyFeature {
    pub span: Span,
}

impl AstNode for EmptyFeature {
    fn span(&self) -> Span { self.span }
}

/// `PrimaryExpression`
#[derive(Debug, Clone)]
pub enum PrimaryExpression {
    FeatureChainExpression(Box<FeatureChainExpression>),
    NonFeatureChainPrimaryExpression(Box<NonFeatureChainPrimaryExpression>),
}

/// `PrimaryArgumentValue`
#[derive(Debug, Clone)]
pub struct PrimaryArgumentValue {
    pub span: Span,
    pub value: Box<PrimaryExpression>,
}

impl AstNode for PrimaryArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `PrimaryArgument`
#[derive(Debug, Clone)]
pub struct PrimaryArgument {
    pub span: Span,
    pub owned_relationship: Vec<PrimaryArgumentValue>,
}

impl AstNode for PrimaryArgument {
    fn span(&self) -> Span { self.span }
}

/// `PrimaryArgumentMember`
#[derive(Debug, Clone)]
pub struct PrimaryArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<PrimaryArgument>,
}

impl AstNode for PrimaryArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `NonFeatureChainPrimaryExpression`
#[derive(Debug, Clone)]
pub enum NonFeatureChainPrimaryExpression {
    BracketExpression(Box<BracketExpression>),
    IndexExpression(Box<IndexExpression>),
    SequenceExpression(Box<SequenceExpression>),
    SelectExpression(Box<SelectExpression>),
    CollectExpression(Box<CollectExpression>),
    FunctionOperationExpression(Box<FunctionOperationExpression>),
    BaseExpression(Box<BaseExpression>),
}

/// `NonFeatureChainPrimaryArgumentValue`
#[derive(Debug, Clone)]
pub struct NonFeatureChainPrimaryArgumentValue {
    pub span: Span,
    pub value: Box<NonFeatureChainPrimaryExpression>,
}

impl AstNode for NonFeatureChainPrimaryArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `NonFeatureChainPrimaryArgument`
#[derive(Debug, Clone)]
pub struct NonFeatureChainPrimaryArgument {
    pub span: Span,
    pub owned_relationship: Vec<NonFeatureChainPrimaryArgumentValue>,
}

impl AstNode for NonFeatureChainPrimaryArgument {
    fn span(&self) -> Span { self.span }
}

/// `NonFeatureChainPrimaryArgumentMember`
#[derive(Debug, Clone)]
pub struct NonFeatureChainPrimaryArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<PrimaryArgument>,
}

impl AstNode for NonFeatureChainPrimaryArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `BracketExpression`
#[derive(Debug, Clone)]
pub enum BracketExpressionOwnedRelationshipMember {
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
    SequenceExpressionListMember(Box<SequenceExpressionListMember>),
}

#[derive(Debug, Clone)]
pub struct BracketExpression {
    pub span: Span,
    pub operator: bool,
    pub owned_relationship: Vec<BracketExpressionOwnedRelationshipMember>,
}

impl AstNode for BracketExpression {
    fn span(&self) -> Span { self.span }
}

/// `IndexExpression`
#[derive(Debug, Clone)]
pub enum IndexExpressionOwnedRelationshipMember {
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
    SequenceExpressionListMember(Box<SequenceExpressionListMember>),
}

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub span: Span,
    pub owned_relationship: Vec<IndexExpressionOwnedRelationshipMember>,
}

impl AstNode for IndexExpression {
    fn span(&self) -> Span { self.span }
}

/// `SequenceExpression`
#[derive(Debug, Clone)]
pub struct SequenceExpression {
    pub span: Span,
}

impl AstNode for SequenceExpression {
    fn span(&self) -> Span { self.span }
}

/// `SequenceExpressionList`
#[derive(Debug, Clone)]
pub struct SequenceExpressionList {
    pub span: Span,
}

impl AstNode for SequenceExpressionList {
    fn span(&self) -> Span { self.span }
}

/// `SequenceOperatorExpression`
#[derive(Debug, Clone)]
pub enum SequenceOperatorExpressionOwnedRelationshipMember {
    OwnedExpressionMember(Box<OwnedExpressionMember>),
    SequenceExpressionListMember(Box<SequenceExpressionListMember>),
}

#[derive(Debug, Clone)]
pub struct SequenceOperatorExpression {
    pub span: Span,
    pub operator: bool,
    pub owned_relationship: Vec<SequenceOperatorExpressionOwnedRelationshipMember>,
}

impl AstNode for SequenceOperatorExpression {
    fn span(&self) -> Span { self.span }
}

/// `SequenceExpressionListMember`
#[derive(Debug, Clone)]
pub struct SequenceExpressionListMember {
    pub span: Span,
    pub owned_member_feature: Box<SequenceExpressionList>,
}

impl AstNode for SequenceExpressionListMember {
    fn span(&self) -> Span { self.span }
}

/// `FeatureChainExpression`
#[derive(Debug, Clone)]
pub enum FeatureChainExpressionOwnedRelationshipMember {
    FeatureChainMember(Box<FeatureChainMember>),
    NonFeatureChainPrimaryArgumentMember(Box<NonFeatureChainPrimaryArgumentMember>),
}

#[derive(Debug, Clone)]
pub struct FeatureChainExpression {
    pub span: Span,
    pub owned_relationship: Vec<FeatureChainExpressionOwnedRelationshipMember>,
}

impl AstNode for FeatureChainExpression {
    fn span(&self) -> Span { self.span }
}

/// `CollectExpression`
#[derive(Debug, Clone)]
pub enum CollectExpressionOwnedRelationshipMember {
    BodyArgumentMember(Box<BodyArgumentMember>),
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
}

#[derive(Debug, Clone)]
pub struct CollectExpression {
    pub span: Span,
    pub owned_relationship: Vec<CollectExpressionOwnedRelationshipMember>,
}

impl AstNode for CollectExpression {
    fn span(&self) -> Span { self.span }
}

/// `SelectExpression`
#[derive(Debug, Clone)]
pub enum SelectExpressionOwnedRelationshipMember {
    BodyArgumentMember(Box<BodyArgumentMember>),
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
}

#[derive(Debug, Clone)]
pub struct SelectExpression {
    pub span: Span,
    pub owned_relationship: Vec<SelectExpressionOwnedRelationshipMember>,
}

impl AstNode for SelectExpression {
    fn span(&self) -> Span { self.span }
}

/// `FunctionOperationExpression`
#[derive(Debug, Clone)]
pub enum FunctionOperationExpressionOwnedRelationshipMember {
    BodyArgumentMember(Box<BodyArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
    FunctionReferenceArgumentMember(Box<FunctionReferenceArgumentMember>),
    InstantiatedTypeMember(Box<InstantiatedTypeMember>),
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
}

#[derive(Debug, Clone)]
pub struct FunctionOperationExpression {
    pub span: Span,
    pub owned_relationship: Vec<FunctionOperationExpressionOwnedRelationshipMember>,
}

impl AstNode for FunctionOperationExpression {
    fn span(&self) -> Span { self.span }
}

/// `BodyArgumentMember`
#[derive(Debug, Clone)]
pub struct BodyArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<BodyArgument>,
}

impl AstNode for BodyArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `BodyArgument`
#[derive(Debug, Clone)]
pub struct BodyArgument {
    pub span: Span,
    pub owned_relationship: Vec<BodyArgumentValue>,
}

impl AstNode for BodyArgument {
    fn span(&self) -> Span { self.span }
}

/// `BodyArgumentValue`
#[derive(Debug, Clone)]
pub struct BodyArgumentValue {
    pub span: Span,
    pub value: Box<BodyExpression>,
}

impl AstNode for BodyArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReferenceArgumentMember`
#[derive(Debug, Clone)]
pub struct FunctionReferenceArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<FunctionReferenceArgument>,
}

impl AstNode for FunctionReferenceArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReferenceArgument`
#[derive(Debug, Clone)]
pub struct FunctionReferenceArgument {
    pub span: Span,
    pub owned_relationship: Vec<FunctionReferenceArgumentValue>,
}

impl AstNode for FunctionReferenceArgument {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReferenceArgumentValue`
#[derive(Debug, Clone)]
pub struct FunctionReferenceArgumentValue {
    pub span: Span,
    pub value: Box<FunctionReferenceExpression>,
}

impl AstNode for FunctionReferenceArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReferenceExpression`
#[derive(Debug, Clone)]
pub struct FunctionReferenceExpression {
    pub span: Span,
    pub owned_relationship: Vec<FunctionReferenceMember>,
}

impl AstNode for FunctionReferenceExpression {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReferenceMember`
#[derive(Debug, Clone)]
pub struct FunctionReferenceMember {
    pub span: Span,
    pub owned_member_feature: Box<FunctionReference>,
}

impl AstNode for FunctionReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `FunctionReference`
#[derive(Debug, Clone)]
pub struct FunctionReference {
    pub span: Span,
    pub owned_relationship: Vec<ReferenceTyping>,
}

impl AstNode for FunctionReference {
    fn span(&self) -> Span { self.span }
}

/// `FeatureChainMember`
#[derive(Debug, Clone)]
pub enum FeatureChainMember {
    FeatureReferenceMember(Box<FeatureReferenceMember>),
    OwnedFeatureChainMember(Box<OwnedFeatureChainMember>),
}

/// `OwnedFeatureChainMember`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChainMember {
    pub span: Span,
    pub owned_member_element: Box<FeatureChain>,
}

impl AstNode for OwnedFeatureChainMember {
    fn span(&self) -> Span { self.span }
}

/// `BaseExpression`
#[derive(Debug, Clone)]
pub enum BaseExpression {
    NullExpression(Box<NullExpression>),
    LiteralExpression(Box<LiteralExpression>),
    FeatureReferenceExpression(Box<FeatureReferenceExpression>),
    MetadataAccessExpression(Box<MetadataAccessExpression>),
    InvocationExpression(Box<InvocationExpression>),
    ConstructorExpression(Box<ConstructorExpression>),
    BodyExpression(Box<BodyExpression>),
}

/// `NullExpression`
#[derive(Debug, Clone)]
pub struct NullExpression {
    pub span: Span,
}

impl AstNode for NullExpression {
    fn span(&self) -> Span { self.span }
}

/// `FeatureReferenceExpression`
#[derive(Debug, Clone)]
pub enum FeatureReferenceExpressionOwnedRelationshipMember {
    EmptyResultMember(Box<EmptyResultMember>),
    FeatureReferenceMember(Box<FeatureReferenceMember>),
}

#[derive(Debug, Clone)]
pub struct FeatureReferenceExpression {
    pub span: Span,
    pub owned_relationship: Vec<FeatureReferenceExpressionOwnedRelationshipMember>,
}

impl AstNode for FeatureReferenceExpression {
    fn span(&self) -> Span { self.span }
}

/// `FeatureReferenceMember`
#[derive(Debug, Clone)]
pub struct FeatureReferenceMember {
    pub span: Span,
    pub member_element: Box<FeatureReference>,
}

impl AstNode for FeatureReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `FeatureReference`
#[derive(Debug, Clone)]
pub struct FeatureReference {
    pub span: Span,
}

impl AstNode for FeatureReference {
    fn span(&self) -> Span { self.span }
}

/// `MetadataAccessExpression`
#[derive(Debug, Clone)]
pub struct MetadataAccessExpression {
    pub span: Span,
    pub owned_relationship: Vec<ElementReferenceMember>,
}

impl AstNode for MetadataAccessExpression {
    fn span(&self) -> Span { self.span }
}

/// `ElementReferenceMember`
#[derive(Debug, Clone)]
pub struct ElementReferenceMember {
    pub span: Span,
    pub member_element: QualifiedNameRef,
}

impl AstNode for ElementReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `InvocationExpression`
#[derive(Debug, Clone)]
pub enum InvocationExpressionOwnedRelationshipMember {
    EmptyResultMember(Box<EmptyResultMember>),
    InstantiatedTypeMember(Box<InstantiatedTypeMember>),
}

#[derive(Debug, Clone)]
pub struct InvocationExpression {
    pub span: Span,
    pub owned_relationship: Vec<InvocationExpressionOwnedRelationshipMember>,
}

impl AstNode for InvocationExpression {
    fn span(&self) -> Span { self.span }
}

/// `ConstructorExpression`
#[derive(Debug, Clone)]
pub enum ConstructorExpressionOwnedRelationshipMember {
    ConstructorResultMember(Box<ConstructorResultMember>),
    InstantiatedTypeMember(Box<InstantiatedTypeMember>),
}

#[derive(Debug, Clone)]
pub struct ConstructorExpression {
    pub span: Span,
    pub owned_relationship: Vec<ConstructorExpressionOwnedRelationshipMember>,
}

impl AstNode for ConstructorExpression {
    fn span(&self) -> Span { self.span }
}

/// `ConstructorResultMember`
#[derive(Debug, Clone)]
pub struct ConstructorResultMember {
    pub span: Span,
    pub owned_related_element: Vec<ConstructorResult>,
}

impl AstNode for ConstructorResultMember {
    fn span(&self) -> Span { self.span }
}

/// `ConstructorResult`
#[derive(Debug, Clone)]
pub struct ConstructorResult {
    pub span: Span,
}

impl AstNode for ConstructorResult {
    fn span(&self) -> Span { self.span }
}

/// `InstantiatedTypeMember`
#[derive(Debug, Clone)]
pub struct InstantiatedTypeMember {
    pub span: Span,
    pub member_element: Option<Box<InstantiatedTypeReference>>,
}

impl AstNode for InstantiatedTypeMember {
    fn span(&self) -> Span { self.span }
}

/// `InstantiatedTypeReference`
#[derive(Debug, Clone)]
pub struct InstantiatedTypeReference {
    pub span: Span,
}

impl AstNode for InstantiatedTypeReference {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentList`
#[derive(Debug, Clone)]
pub struct ArgumentList {
    pub span: Span,
}

impl AstNode for ArgumentList {
    fn span(&self) -> Span { self.span }
}

/// `PositionalArgumentList`
#[derive(Debug, Clone)]
pub struct PositionalArgumentList {
    pub span: Span,
    pub owned_relationship: Vec<ArgumentMember>,
}

impl AstNode for PositionalArgumentList {
    fn span(&self) -> Span { self.span }
}

/// `NamedArgumentList`
#[derive(Debug, Clone)]
pub struct NamedArgumentList {
    pub span: Span,
    pub owned_relationship: Vec<NamedArgumentMember>,
}

impl AstNode for NamedArgumentList {
    fn span(&self) -> Span { self.span }
}

/// `NamedArgumentMember`
#[derive(Debug, Clone)]
pub struct NamedArgumentMember {
    pub span: Span,
    pub owned_member_feature: Box<NamedArgument>,
}

impl AstNode for NamedArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `NamedArgument`
#[derive(Debug, Clone)]
pub enum NamedArgumentOwnedRelationshipMember {
    ArgumentValue(Box<ArgumentValue>),
    ParameterRedefinition(Box<ParameterRedefinition>),
}

#[derive(Debug, Clone)]
pub struct NamedArgument {
    pub span: Span,
    pub owned_relationship: Vec<NamedArgumentOwnedRelationshipMember>,
}

impl AstNode for NamedArgument {
    fn span(&self) -> Span { self.span }
}

/// `ParameterRedefinition`
#[derive(Debug, Clone)]
pub struct ParameterRedefinition {
    pub span: Span,
    pub redefined_feature: QualifiedNameRef,
}

impl AstNode for ParameterRedefinition {
    fn span(&self) -> Span { self.span }
}

/// `BodyExpression`
#[derive(Debug, Clone)]
pub struct BodyExpression {
    pub span: Span,
    pub owned_relationship: Vec<ExpressionBodyMember>,
}

impl AstNode for BodyExpression {
    fn span(&self) -> Span { self.span }
}

/// `ExpressionBodyMember`
#[derive(Debug, Clone)]
pub struct ExpressionBodyMember {
    pub span: Span,
    pub owned_member_feature: Box<ExpressionBody>,
}

impl AstNode for ExpressionBodyMember {
    fn span(&self) -> Span { self.span }
}

/// `ExpressionBody`
#[derive(Debug, Clone)]
pub struct ExpressionBody {
    pub span: Span,
}

impl AstNode for ExpressionBody {
    fn span(&self) -> Span { self.span }
}

/// `LiteralExpression`
#[derive(Debug, Clone)]
pub enum LiteralExpression {
    LiteralBoolean(Box<LiteralBoolean>),
    LiteralString(Box<LiteralString>),
    LiteralInteger(Box<LiteralInteger>),
    LiteralReal(Box<LiteralReal>),
    LiteralInfinity(Box<LiteralInfinity>),
}

/// `LiteralBoolean`
#[derive(Debug, Clone)]
pub struct LiteralBoolean {
    pub span: Span,
    pub value: Box<BooleanValue>,
}

impl AstNode for LiteralBoolean {
    fn span(&self) -> Span { self.span }
}

/// `BooleanValue`
#[derive(Debug, Clone)]
pub struct BooleanValue {
    pub span: Span,
}

impl AstNode for BooleanValue {
    fn span(&self) -> Span { self.span }
}

/// `LiteralString`
#[derive(Debug, Clone)]
pub struct LiteralString {
    pub span: Span,
    pub value: String,
}

impl AstNode for LiteralString {
    fn span(&self) -> Span { self.span }
}

/// `LiteralInteger`
#[derive(Debug, Clone)]
pub struct LiteralInteger {
    pub span: Span,
    pub value: String,
}

impl AstNode for LiteralInteger {
    fn span(&self) -> Span { self.span }
}

/// `LiteralReal`
#[derive(Debug, Clone)]
pub struct LiteralReal {
    pub span: Span,
    pub value: Box<RealValue>,
}

impl AstNode for LiteralReal {
    fn span(&self) -> Span { self.span }
}

/// `RealValue`
#[derive(Debug, Clone)]
pub struct RealValue {
    pub span: Span,
}

impl AstNode for RealValue {
    fn span(&self) -> Span { self.span }
}

/// `LiteralInfinity`
#[derive(Debug, Clone)]
pub struct LiteralInfinity {
    pub span: Span,
}

impl AstNode for LiteralInfinity {
    fn span(&self) -> Span { self.span }
}

/// `Interaction`
#[derive(Debug, Clone)]
pub struct Interaction {
    pub span: Span,
}

impl AstNode for Interaction {
    fn span(&self) -> Span { self.span }
}

/// `Flow`
#[derive(Debug, Clone)]
pub struct Flow {
    pub span: Span,
}

impl AstNode for Flow {
    fn span(&self) -> Span { self.span }
}

/// `SuccessionFlow`
#[derive(Debug, Clone)]
pub struct SuccessionFlow {
    pub span: Span,
}

impl AstNode for SuccessionFlow {
    fn span(&self) -> Span { self.span }
}

/// `FlowDeclaration`
#[derive(Debug, Clone)]
pub enum FlowDeclarationOwnedRelationshipMember {
    FlowEndMember(Box<FlowEndMember>),
    PayloadFeatureMember(Box<PayloadFeatureMember>),
}

#[derive(Debug, Clone)]
pub struct FlowDeclaration {
    pub span: Span,
    pub is_sufficient: bool,
    pub owned_relationship: Vec<FlowDeclarationOwnedRelationshipMember>,
}

impl AstNode for FlowDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `PayloadFeatureMember`
#[derive(Debug, Clone)]
pub struct PayloadFeatureMember {
    pub span: Span,
    pub owned_related_element: Box<PayloadFeature>,
}

impl AstNode for PayloadFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `PayloadFeature`
#[derive(Debug, Clone)]
pub enum PayloadFeatureOwnedRelationshipMember {
    OwnedFeatureTyping(Box<OwnedFeatureTyping>),
    OwnedMultiplicity(Box<OwnedMultiplicity>),
}

#[derive(Debug, Clone)]
pub struct PayloadFeature {
    pub span: Span,
    pub owned_relationship: Vec<PayloadFeatureOwnedRelationshipMember>,
}

impl AstNode for PayloadFeature {
    fn span(&self) -> Span { self.span }
}

/// `PayloadFeatureSpecializationPart`
#[derive(Debug, Clone)]
pub struct PayloadFeatureSpecializationPart {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
}

impl AstNode for PayloadFeatureSpecializationPart {
    fn span(&self) -> Span { self.span }
}

/// `FlowEndMember`
#[derive(Debug, Clone)]
pub struct FlowEndMember {
    pub span: Span,
    pub owned_related_element: Vec<FlowEnd>,
}

impl AstNode for FlowEndMember {
    fn span(&self) -> Span { self.span }
}

/// `FlowEnd`
#[derive(Debug, Clone)]
pub enum FlowEndOwnedRelationshipMember {
    FlowFeatureMember(Box<FlowFeatureMember>),
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
}

#[derive(Debug, Clone)]
pub struct FlowEnd {
    pub span: Span,
    pub owned_relationship: Vec<FlowEndOwnedRelationshipMember>,
}

impl AstNode for FlowEnd {
    fn span(&self) -> Span { self.span }
}

/// `FlowFeatureMember`
#[derive(Debug, Clone)]
pub struct FlowFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FlowFeature>,
}

impl AstNode for FlowFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `FlowFeature`
#[derive(Debug, Clone)]
pub struct FlowFeature {
    pub span: Span,
    pub owned_relationship: Vec<FlowFeatureRedefinition>,
}

impl AstNode for FlowFeature {
    fn span(&self) -> Span { self.span }
}

/// `FlowFeatureRedefinition`
#[derive(Debug, Clone)]
pub struct FlowFeatureRedefinition {
    pub span: Span,
    pub redefined_feature: QualifiedNameRef,
}

impl AstNode for FlowFeatureRedefinition {
    fn span(&self) -> Span { self.span }
}

/// `ValuePart`
#[derive(Debug, Clone)]
pub struct ValuePart {
    pub span: Span,
    pub owned_relationship: Vec<FeatureValue>,
}

impl AstNode for ValuePart {
    fn span(&self) -> Span { self.span }
}

/// `FeatureValue`
#[derive(Debug, Clone)]
pub struct FeatureValue {
    pub span: Span,
    pub is_default: bool,
    pub is_initial: bool,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for FeatureValue {
    fn span(&self) -> Span { self.span }
}

/// `Multiplicity`
#[derive(Debug, Clone)]
pub enum Multiplicity {
    MultiplicitySubset(Box<MultiplicitySubset>),
    MultiplicityRange(Box<MultiplicityRange>),
}

/// `MultiplicitySubset`
#[derive(Debug, Clone)]
pub struct MultiplicitySubset {
    pub span: Span,
}

impl AstNode for MultiplicitySubset {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicityRange`
#[derive(Debug, Clone)]
pub struct MultiplicityRange {
    pub span: Span,
}

impl AstNode for MultiplicityRange {
    fn span(&self) -> Span { self.span }
}

/// `OwnedMultiplicity`
#[derive(Debug, Clone)]
pub struct OwnedMultiplicity {
    pub span: Span,
    pub owned_related_element: Vec<OwnedMultiplicityRange>,
}

impl AstNode for OwnedMultiplicity {
    fn span(&self) -> Span { self.span }
}

/// `OwnedMultiplicityRange`
#[derive(Debug, Clone)]
pub struct OwnedMultiplicityRange {
    pub span: Span,
}

impl AstNode for OwnedMultiplicityRange {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicityBounds`
#[derive(Debug, Clone)]
pub struct MultiplicityBounds {
    pub span: Span,
    pub owned_relationship: Vec<MultiplicityExpressionMember>,
}

impl AstNode for MultiplicityBounds {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicityExpressionMember`
#[derive(Debug, Clone)]
pub enum MultiplicityExpressionMemberOwnedRelatedElementMember {
    FeatureReferenceExpression(Box<FeatureReferenceExpression>),
    LiteralExpression(Box<LiteralExpression>),
}

#[derive(Debug, Clone)]
pub struct MultiplicityExpressionMember {
    pub span: Span,
    pub owned_related_element: Vec<MultiplicityExpressionMemberOwnedRelatedElementMember>,
}

impl AstNode for MultiplicityExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `Metaclass`
#[derive(Debug, Clone)]
pub struct Metaclass {
    pub span: Span,
}

impl AstNode for Metaclass {
    fn span(&self) -> Span { self.span }
}

/// `PrefixMetadataAnnotation`
#[derive(Debug, Clone)]
pub struct PrefixMetadataAnnotation {
    pub span: Span,
    pub owned_related_element: Vec<PrefixMetadataFeature>,
}

impl AstNode for PrefixMetadataAnnotation {
    fn span(&self) -> Span { self.span }
}

/// `PrefixMetadataMember`
#[derive(Debug, Clone)]
pub struct PrefixMetadataMember {
    pub span: Span,
    pub owned_related_element: Vec<PrefixMetadataFeature>,
}

impl AstNode for PrefixMetadataMember {
    fn span(&self) -> Span { self.span }
}

/// `PrefixMetadataFeature`
#[derive(Debug, Clone)]
pub struct PrefixMetadataFeature {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for PrefixMetadataFeature {
    fn span(&self) -> Span { self.span }
}

/// `MetadataFeature`
#[derive(Debug, Clone)]
pub enum MetadataFeatureOwnedRelationshipMember {
    Annotation(Box<Annotation>),
    PrefixMetadataMember(Box<PrefixMetadataMember>),
}

#[derive(Debug, Clone)]
pub struct MetadataFeature {
    pub span: Span,
    pub owned_relationship: Vec<MetadataFeatureOwnedRelationshipMember>,
}

impl AstNode for MetadataFeature {
    fn span(&self) -> Span { self.span }
}

/// `MetadataFeatureDeclaration`
#[derive(Debug, Clone)]
pub struct MetadataFeatureDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for MetadataFeatureDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBody`
#[derive(Debug, Clone)]
pub struct MetadataBody {
    pub span: Span,
    pub owned_relationship: Vec<MetadataBodyElement>,
}

impl AstNode for MetadataBody {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBodyElement`
#[derive(Debug, Clone)]
pub enum MetadataBodyElement {
    NonFeatureMember(Box<NonFeatureMember>),
    MetadataBodyFeatureMember(Box<MetadataBodyFeatureMember>),
    AliasMember(Box<AliasMember>),
    Import(Box<Import>),
}

/// `MetadataBodyFeatureMember`
#[derive(Debug, Clone)]
pub struct MetadataBodyFeatureMember {
    pub span: Span,
    pub owned_member_feature: Box<MetadataBodyFeature>,
}

impl AstNode for MetadataBodyFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBodyFeature`
#[derive(Debug, Clone)]
pub struct MetadataBodyFeature {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for MetadataBodyFeature {
    fn span(&self) -> Span { self.span }
}

/// `Package`
#[derive(Debug, Clone)]
pub struct Package {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for Package {
    fn span(&self) -> Span { self.span }
}

/// `LibraryPackage`
#[derive(Debug, Clone)]
pub struct LibraryPackage {
    pub span: Span,
    pub is_standard: bool,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for LibraryPackage {
    fn span(&self) -> Span { self.span }
}

/// `PackageDeclaration`
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub span: Span,
}

impl AstNode for PackageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `PackageBody`
#[derive(Debug, Clone)]
pub struct PackageBody {
    pub span: Span,
    pub owned_relationship: Vec<ElementFilterMember>,
}

impl AstNode for PackageBody {
    fn span(&self) -> Span { self.span }
}

/// `ElementFilterMember`
#[derive(Debug, Clone)]
pub struct ElementFilterMember {
    pub span: Span,
    pub condition: Box<OwnedExpression>,
}

impl AstNode for ElementFilterMember {
    fn span(&self) -> Span { self.span }
}

// Top-level node wrapper

/// Wraps every AST node type so a single parse dispatch can
/// return the concrete result without erasing it.
#[derive(Debug, Clone)]
pub enum AstNodeKind {
    Identification(Box<Identification>),
    RelationshipBody(Box<RelationshipBody>),
    RelationshipOwnedElement(Box<RelationshipOwnedElement>),
    OwnedRelatedElement(OwnedRelatedElement),
    Dependency(Box<Dependency>),
    Annotation(Box<Annotation>),
    OwnedAnnotation(Box<OwnedAnnotation>),
    AnnotatingElement(AnnotatingElement),
    Comment(Box<Comment>),
    Documentation(Box<Documentation>),
    TextualRepresentation(Box<TextualRepresentation>),
    RootNamespace(Box<RootNamespace>),
    Namespace(Box<Namespace>),
    NamespaceDeclaration(Box<NamespaceDeclaration>),
    NamespaceBody(Box<NamespaceBody>),
    NamespaceBodyElement(Box<NamespaceBodyElement>),
    MemberPrefix(Box<MemberPrefix>),
    VisibilityIndicator(Box<VisibilityIndicator>),
    NamespaceMember(NamespaceMember),
    NonFeatureMember(Box<NonFeatureMember>),
    NamespaceFeatureMember(Box<NamespaceFeatureMember>),
    AliasMember(Box<AliasMember>),
    QualifiedName(Box<QualifiedName>),
    Import(Box<Import>),
    ImportDeclaration(ImportDeclaration),
    MembershipImport(Box<MembershipImport>),
    NamespaceImport(Box<NamespaceImport>),
    FilterPackage(Box<FilterPackage>),
    FilterPackageMember(Box<FilterPackageMember>),
    MemberElement(MemberElement),
    NonFeatureElement(NonFeatureElement),
    FeatureElement(FeatureElement),
    Type(Box<Type>),
    TypePrefix(Box<TypePrefix>),
    TypeDeclaration(Box<TypeDeclaration>),
    SpecializationPart(Box<SpecializationPart>),
    ConjugationPart(Box<ConjugationPart>),
    TypeRelationshipPart(TypeRelationshipPart),
    DisjoiningPart(Box<DisjoiningPart>),
    UnioningPart(Box<UnioningPart>),
    IntersectingPart(Box<IntersectingPart>),
    DifferencingPart(Box<DifferencingPart>),
    TypeBody(Box<TypeBody>),
    TypeBodyElement(Box<TypeBodyElement>),
    Specialization(Box<Specialization>),
    OwnedSpecialization(Box<OwnedSpecialization>),
    SpecificType(Box<SpecificType>),
    GeneralType(Box<GeneralType>),
    Conjugation(Box<Conjugation>),
    OwnedConjugation(Box<OwnedConjugation>),
    Disjoining(Box<Disjoining>),
    OwnedDisjoining(Box<OwnedDisjoining>),
    Unioning(Box<Unioning>),
    Intersecting(Box<Intersecting>),
    Differencing(Box<Differencing>),
    FeatureMember(FeatureMember),
    TypeFeatureMember(Box<TypeFeatureMember>),
    OwnedFeatureMember(Box<OwnedFeatureMember>),
    Classifier(Box<Classifier>),
    ClassifierDeclaration(Box<ClassifierDeclaration>),
    SuperclassingPart(Box<SuperclassingPart>),
    Subclassification(Box<Subclassification>),
    OwnedSubclassification(Box<OwnedSubclassification>),
    Feature(Box<Feature>),
    EndFeaturePrefix(Box<EndFeaturePrefix>),
    BasicFeaturePrefix(Box<BasicFeaturePrefix>),
    FeaturePrefix(Box<FeaturePrefix>),
    OwnedCrossFeatureMember(Box<OwnedCrossFeatureMember>),
    OwnedCrossFeature(Box<OwnedCrossFeature>),
    FeatureDirection(Box<FeatureDirection>),
    FeatureDeclaration(Box<FeatureDeclaration>),
    FeatureIdentification(Box<FeatureIdentification>),
    FeatureRelationshipPart(FeatureRelationshipPart),
    ChainingPart(Box<ChainingPart>),
    InvertingPart(Box<InvertingPart>),
    TypeFeaturingPart(Box<TypeFeaturingPart>),
    FeatureSpecializationPart(Box<FeatureSpecializationPart>),
    MultiplicityPart(Box<MultiplicityPart>),
    FeatureSpecialization(FeatureSpecialization),
    Typings(Box<Typings>),
    TypedBy(Box<TypedBy>),
    Subsettings(Box<Subsettings>),
    Subsets(Box<Subsets>),
    References(Box<References>),
    Crosses(Box<Crosses>),
    Redefinitions(Box<Redefinitions>),
    Redefines(Box<Redefines>),
    FeatureTyping(Box<FeatureTyping>),
    OwnedFeatureTyping(Box<OwnedFeatureTyping>),
    Subsetting(Box<Subsetting>),
    OwnedSubsetting(Box<OwnedSubsetting>),
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
    OwnedCrossSubsetting(Box<OwnedCrossSubsetting>),
    Redefinition(Box<Redefinition>),
    OwnedRedefinition(Box<OwnedRedefinition>),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
    FeatureChain(Box<FeatureChain>),
    OwnedFeatureChaining(Box<OwnedFeatureChaining>),
    FeatureInverting(Box<FeatureInverting>),
    OwnedFeatureInverting(Box<OwnedFeatureInverting>),
    TypeFeaturing(Box<TypeFeaturing>),
    OwnedTypeFeaturing(Box<OwnedTypeFeaturing>),
    DataType(Box<DataType>),
    Class(Box<Class>),
    Structure(Box<Structure>),
    Association(Box<Association>),
    AssociationStructure(Box<AssociationStructure>),
    Connector(Box<Connector>),
    ConnectorDeclaration(ConnectorDeclaration),
    BinaryConnectorDeclaration(Box<BinaryConnectorDeclaration>),
    NaryConnectorDeclaration(Box<NaryConnectorDeclaration>),
    ConnectorEndMember(Box<ConnectorEndMember>),
    ConnectorEnd(Box<ConnectorEnd>),
    OwnedCrossMultiplicityMember(Box<OwnedCrossMultiplicityMember>),
    OwnedCrossMultiplicity(Box<OwnedCrossMultiplicity>),
    BindingConnector(Box<BindingConnector>),
    BindingConnectorDeclaration(Box<BindingConnectorDeclaration>),
    Succession(Box<Succession>),
    SuccessionDeclaration(Box<SuccessionDeclaration>),
    Behavior(Box<Behavior>),
    Step(Box<Step>),
    Function(Box<Function>),
    FunctionBody(Box<FunctionBody>),
    FunctionBodyPart(Box<FunctionBodyPart>),
    ReturnFeatureMember(Box<ReturnFeatureMember>),
    ResultExpressionMember(Box<ResultExpressionMember>),
    Expression(Box<Expression>),
    Predicate(Box<Predicate>),
    BooleanExpression(Box<BooleanExpression>),
    Invariant(Box<Invariant>),
    OwnedExpressionReferenceMember(Box<OwnedExpressionReferenceMember>),
    OwnedExpressionReference(Box<OwnedExpressionReference>),
    OwnedExpressionMember(Box<OwnedExpressionMember>),
    OwnedExpression(OwnedExpression),
    ConditionalExpression(Box<ConditionalExpression>),
    ConditionalBinaryOperatorExpression(Box<ConditionalBinaryOperatorExpression>),
    ConditionalBinaryOperator(Box<ConditionalBinaryOperator>),
    BinaryOperatorExpression(Box<BinaryOperatorExpression>),
    BinaryOperator(Box<BinaryOperator>),
    UnaryOperatorExpression(Box<UnaryOperatorExpression>),
    UnaryOperator(Box<UnaryOperator>),
    ClassificationExpression(Box<ClassificationExpression>),
    ClassificationTestOperator(Box<ClassificationTestOperator>),
    CastOperator(Box<CastOperator>),
    MetaclassificationExpression(Box<MetaclassificationExpression>),
    ArgumentMember(Box<ArgumentMember>),
    Argument(Box<Argument>),
    ArgumentValue(Box<ArgumentValue>),
    ArgumentExpressionMember(Box<ArgumentExpressionMember>),
    ArgumentExpression(Box<ArgumentExpression>),
    ArgumentExpressionValue(Box<ArgumentExpressionValue>),
    MetadataArgumentMember(Box<MetadataArgumentMember>),
    MetadataArgument(Box<MetadataArgument>),
    MetadataValue(Box<MetadataValue>),
    MetadataReference(Box<MetadataReference>),
    MetaclassificationTestOperator(Box<MetaclassificationTestOperator>),
    MetaCastOperator(Box<MetaCastOperator>),
    ExtentExpression(Box<ExtentExpression>),
    TypeReferenceMember(Box<TypeReferenceMember>),
    TypeResultMember(Box<TypeResultMember>),
    TypeReference(Box<TypeReference>),
    ReferenceTyping(Box<ReferenceTyping>),
    EmptyResultMember(Box<EmptyResultMember>),
    EmptyFeature(Box<EmptyFeature>),
    PrimaryExpression(PrimaryExpression),
    PrimaryArgumentValue(Box<PrimaryArgumentValue>),
    PrimaryArgument(Box<PrimaryArgument>),
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
    NonFeatureChainPrimaryExpression(NonFeatureChainPrimaryExpression),
    NonFeatureChainPrimaryArgumentValue(Box<NonFeatureChainPrimaryArgumentValue>),
    NonFeatureChainPrimaryArgument(Box<NonFeatureChainPrimaryArgument>),
    NonFeatureChainPrimaryArgumentMember(Box<NonFeatureChainPrimaryArgumentMember>),
    BracketExpression(Box<BracketExpression>),
    IndexExpression(Box<IndexExpression>),
    SequenceExpression(Box<SequenceExpression>),
    SequenceExpressionList(Box<SequenceExpressionList>),
    SequenceOperatorExpression(Box<SequenceOperatorExpression>),
    SequenceExpressionListMember(Box<SequenceExpressionListMember>),
    FeatureChainExpression(Box<FeatureChainExpression>),
    CollectExpression(Box<CollectExpression>),
    SelectExpression(Box<SelectExpression>),
    FunctionOperationExpression(Box<FunctionOperationExpression>),
    BodyArgumentMember(Box<BodyArgumentMember>),
    BodyArgument(Box<BodyArgument>),
    BodyArgumentValue(Box<BodyArgumentValue>),
    FunctionReferenceArgumentMember(Box<FunctionReferenceArgumentMember>),
    FunctionReferenceArgument(Box<FunctionReferenceArgument>),
    FunctionReferenceArgumentValue(Box<FunctionReferenceArgumentValue>),
    FunctionReferenceExpression(Box<FunctionReferenceExpression>),
    FunctionReferenceMember(Box<FunctionReferenceMember>),
    FunctionReference(Box<FunctionReference>),
    FeatureChainMember(FeatureChainMember),
    OwnedFeatureChainMember(Box<OwnedFeatureChainMember>),
    BaseExpression(BaseExpression),
    NullExpression(Box<NullExpression>),
    FeatureReferenceExpression(Box<FeatureReferenceExpression>),
    FeatureReferenceMember(Box<FeatureReferenceMember>),
    FeatureReference(Box<FeatureReference>),
    MetadataAccessExpression(Box<MetadataAccessExpression>),
    ElementReferenceMember(Box<ElementReferenceMember>),
    InvocationExpression(Box<InvocationExpression>),
    ConstructorExpression(Box<ConstructorExpression>),
    ConstructorResultMember(Box<ConstructorResultMember>),
    ConstructorResult(Box<ConstructorResult>),
    InstantiatedTypeMember(Box<InstantiatedTypeMember>),
    InstantiatedTypeReference(Box<InstantiatedTypeReference>),
    ArgumentList(Box<ArgumentList>),
    PositionalArgumentList(Box<PositionalArgumentList>),
    NamedArgumentList(Box<NamedArgumentList>),
    NamedArgumentMember(Box<NamedArgumentMember>),
    NamedArgument(Box<NamedArgument>),
    ParameterRedefinition(Box<ParameterRedefinition>),
    BodyExpression(Box<BodyExpression>),
    ExpressionBodyMember(Box<ExpressionBodyMember>),
    ExpressionBody(Box<ExpressionBody>),
    LiteralExpression(LiteralExpression),
    LiteralBoolean(Box<LiteralBoolean>),
    BooleanValue(Box<BooleanValue>),
    LiteralString(Box<LiteralString>),
    LiteralInteger(Box<LiteralInteger>),
    LiteralReal(Box<LiteralReal>),
    RealValue(Box<RealValue>),
    LiteralInfinity(Box<LiteralInfinity>),
    Interaction(Box<Interaction>),
    Flow(Box<Flow>),
    SuccessionFlow(Box<SuccessionFlow>),
    FlowDeclaration(Box<FlowDeclaration>),
    PayloadFeatureMember(Box<PayloadFeatureMember>),
    PayloadFeature(Box<PayloadFeature>),
    PayloadFeatureSpecializationPart(Box<PayloadFeatureSpecializationPart>),
    FlowEndMember(Box<FlowEndMember>),
    FlowEnd(Box<FlowEnd>),
    FlowFeatureMember(Box<FlowFeatureMember>),
    FlowFeature(Box<FlowFeature>),
    FlowFeatureRedefinition(Box<FlowFeatureRedefinition>),
    ValuePart(Box<ValuePart>),
    FeatureValue(Box<FeatureValue>),
    Multiplicity(Multiplicity),
    MultiplicitySubset(Box<MultiplicitySubset>),
    MultiplicityRange(Box<MultiplicityRange>),
    OwnedMultiplicity(Box<OwnedMultiplicity>),
    OwnedMultiplicityRange(Box<OwnedMultiplicityRange>),
    MultiplicityBounds(Box<MultiplicityBounds>),
    MultiplicityExpressionMember(Box<MultiplicityExpressionMember>),
    Metaclass(Box<Metaclass>),
    PrefixMetadataAnnotation(Box<PrefixMetadataAnnotation>),
    PrefixMetadataMember(Box<PrefixMetadataMember>),
    PrefixMetadataFeature(Box<PrefixMetadataFeature>),
    MetadataFeature(Box<MetadataFeature>),
    MetadataFeatureDeclaration(Box<MetadataFeatureDeclaration>),
    MetadataBody(Box<MetadataBody>),
    MetadataBodyElement(MetadataBodyElement),
    MetadataBodyFeatureMember(Box<MetadataBodyFeatureMember>),
    MetadataBodyFeature(Box<MetadataBodyFeature>),
    Package(Box<Package>),
    LibraryPackage(Box<LibraryPackage>),
    PackageDeclaration(Box<PackageDeclaration>),
    PackageBody(Box<PackageBody>),
    ElementFilterMember(Box<ElementFilterMember>),
}

impl AstNodeKind {
    /// Return the span of the contained node.
    pub fn span(&self) -> Span {
        match self {
            AstNodeKind::Identification(v) => v.span,
            AstNodeKind::RelationshipBody(v) => v.span,
            AstNodeKind::RelationshipOwnedElement(v) => v.span,
            AstNodeKind::OwnedRelatedElement(_v) => { Span::default() },
            AstNodeKind::Dependency(v) => v.span,
            AstNodeKind::Annotation(v) => v.span,
            AstNodeKind::OwnedAnnotation(v) => v.span,
            AstNodeKind::AnnotatingElement(_v) => { Span::default() },
            AstNodeKind::Comment(v) => v.span,
            AstNodeKind::Documentation(v) => v.span,
            AstNodeKind::TextualRepresentation(v) => v.span,
            AstNodeKind::RootNamespace(v) => v.span,
            AstNodeKind::Namespace(v) => v.span,
            AstNodeKind::NamespaceDeclaration(v) => v.span,
            AstNodeKind::NamespaceBody(v) => v.span,
            AstNodeKind::NamespaceBodyElement(v) => v.span,
            AstNodeKind::MemberPrefix(v) => v.span,
            AstNodeKind::VisibilityIndicator(v) => v.span,
            AstNodeKind::NamespaceMember(_v) => { Span::default() },
            AstNodeKind::NonFeatureMember(v) => v.span,
            AstNodeKind::NamespaceFeatureMember(v) => v.span,
            AstNodeKind::AliasMember(v) => v.span,
            AstNodeKind::QualifiedName(v) => v.span,
            AstNodeKind::Import(v) => v.span,
            AstNodeKind::ImportDeclaration(_v) => { Span::default() },
            AstNodeKind::MembershipImport(v) => v.span,
            AstNodeKind::NamespaceImport(v) => v.span,
            AstNodeKind::FilterPackage(v) => v.span,
            AstNodeKind::FilterPackageMember(v) => v.span,
            AstNodeKind::MemberElement(_v) => { Span::default() },
            AstNodeKind::NonFeatureElement(_v) => { Span::default() },
            AstNodeKind::FeatureElement(_v) => { Span::default() },
            AstNodeKind::Type(v) => v.span,
            AstNodeKind::TypePrefix(v) => v.span,
            AstNodeKind::TypeDeclaration(v) => v.span,
            AstNodeKind::SpecializationPart(v) => v.span,
            AstNodeKind::ConjugationPart(v) => v.span,
            AstNodeKind::TypeRelationshipPart(_v) => { Span::default() },
            AstNodeKind::DisjoiningPart(v) => v.span,
            AstNodeKind::UnioningPart(v) => v.span,
            AstNodeKind::IntersectingPart(v) => v.span,
            AstNodeKind::DifferencingPart(v) => v.span,
            AstNodeKind::TypeBody(v) => v.span,
            AstNodeKind::TypeBodyElement(v) => v.span,
            AstNodeKind::Specialization(v) => v.span,
            AstNodeKind::OwnedSpecialization(v) => v.span,
            AstNodeKind::SpecificType(v) => v.span,
            AstNodeKind::GeneralType(v) => v.span,
            AstNodeKind::Conjugation(v) => v.span,
            AstNodeKind::OwnedConjugation(v) => v.span,
            AstNodeKind::Disjoining(v) => v.span,
            AstNodeKind::OwnedDisjoining(v) => v.span,
            AstNodeKind::Unioning(v) => v.span,
            AstNodeKind::Intersecting(v) => v.span,
            AstNodeKind::Differencing(v) => v.span,
            AstNodeKind::FeatureMember(_v) => { Span::default() },
            AstNodeKind::TypeFeatureMember(v) => v.span,
            AstNodeKind::OwnedFeatureMember(v) => v.span,
            AstNodeKind::Classifier(v) => v.span,
            AstNodeKind::ClassifierDeclaration(v) => v.span,
            AstNodeKind::SuperclassingPart(v) => v.span,
            AstNodeKind::Subclassification(v) => v.span,
            AstNodeKind::OwnedSubclassification(v) => v.span,
            AstNodeKind::Feature(v) => v.span,
            AstNodeKind::EndFeaturePrefix(v) => v.span,
            AstNodeKind::BasicFeaturePrefix(v) => v.span,
            AstNodeKind::FeaturePrefix(v) => v.span,
            AstNodeKind::OwnedCrossFeatureMember(v) => v.span,
            AstNodeKind::OwnedCrossFeature(v) => v.span,
            AstNodeKind::FeatureDirection(v) => v.span,
            AstNodeKind::FeatureDeclaration(v) => v.span,
            AstNodeKind::FeatureIdentification(v) => v.span,
            AstNodeKind::FeatureRelationshipPart(_v) => { Span::default() },
            AstNodeKind::ChainingPart(v) => v.span,
            AstNodeKind::InvertingPart(v) => v.span,
            AstNodeKind::TypeFeaturingPart(v) => v.span,
            AstNodeKind::FeatureSpecializationPart(v) => v.span,
            AstNodeKind::MultiplicityPart(v) => v.span,
            AstNodeKind::FeatureSpecialization(_v) => { Span::default() },
            AstNodeKind::Typings(v) => v.span,
            AstNodeKind::TypedBy(v) => v.span,
            AstNodeKind::Subsettings(v) => v.span,
            AstNodeKind::Subsets(v) => v.span,
            AstNodeKind::References(v) => v.span,
            AstNodeKind::Crosses(v) => v.span,
            AstNodeKind::Redefinitions(v) => v.span,
            AstNodeKind::Redefines(v) => v.span,
            AstNodeKind::FeatureTyping(v) => v.span,
            AstNodeKind::OwnedFeatureTyping(v) => v.span,
            AstNodeKind::Subsetting(v) => v.span,
            AstNodeKind::OwnedSubsetting(v) => v.span,
            AstNodeKind::OwnedReferenceSubsetting(v) => v.span,
            AstNodeKind::OwnedCrossSubsetting(v) => v.span,
            AstNodeKind::Redefinition(v) => v.span,
            AstNodeKind::OwnedRedefinition(v) => v.span,
            AstNodeKind::OwnedFeatureChain(v) => v.span,
            AstNodeKind::FeatureChain(v) => v.span,
            AstNodeKind::OwnedFeatureChaining(v) => v.span,
            AstNodeKind::FeatureInverting(v) => v.span,
            AstNodeKind::OwnedFeatureInverting(v) => v.span,
            AstNodeKind::TypeFeaturing(v) => v.span,
            AstNodeKind::OwnedTypeFeaturing(v) => v.span,
            AstNodeKind::DataType(v) => v.span,
            AstNodeKind::Class(v) => v.span,
            AstNodeKind::Structure(v) => v.span,
            AstNodeKind::Association(v) => v.span,
            AstNodeKind::AssociationStructure(v) => v.span,
            AstNodeKind::Connector(v) => v.span,
            AstNodeKind::ConnectorDeclaration(_v) => { Span::default() },
            AstNodeKind::BinaryConnectorDeclaration(v) => v.span,
            AstNodeKind::NaryConnectorDeclaration(v) => v.span,
            AstNodeKind::ConnectorEndMember(v) => v.span,
            AstNodeKind::ConnectorEnd(v) => v.span,
            AstNodeKind::OwnedCrossMultiplicityMember(v) => v.span,
            AstNodeKind::OwnedCrossMultiplicity(v) => v.span,
            AstNodeKind::BindingConnector(v) => v.span,
            AstNodeKind::BindingConnectorDeclaration(v) => v.span,
            AstNodeKind::Succession(v) => v.span,
            AstNodeKind::SuccessionDeclaration(v) => v.span,
            AstNodeKind::Behavior(v) => v.span,
            AstNodeKind::Step(v) => v.span,
            AstNodeKind::Function(v) => v.span,
            AstNodeKind::FunctionBody(v) => v.span,
            AstNodeKind::FunctionBodyPart(v) => v.span,
            AstNodeKind::ReturnFeatureMember(v) => v.span,
            AstNodeKind::ResultExpressionMember(v) => v.span,
            AstNodeKind::Expression(v) => v.span,
            AstNodeKind::Predicate(v) => v.span,
            AstNodeKind::BooleanExpression(v) => v.span,
            AstNodeKind::Invariant(v) => v.span,
            AstNodeKind::OwnedExpressionReferenceMember(v) => v.span,
            AstNodeKind::OwnedExpressionReference(v) => v.span,
            AstNodeKind::OwnedExpressionMember(v) => v.span,
            AstNodeKind::OwnedExpression(_v) => { Span::default() },
            AstNodeKind::ConditionalExpression(v) => v.span,
            AstNodeKind::ConditionalBinaryOperatorExpression(v) => v.span,
            AstNodeKind::ConditionalBinaryOperator(v) => v.span,
            AstNodeKind::BinaryOperatorExpression(v) => v.span,
            AstNodeKind::BinaryOperator(v) => v.span,
            AstNodeKind::UnaryOperatorExpression(v) => v.span,
            AstNodeKind::UnaryOperator(v) => v.span,
            AstNodeKind::ClassificationExpression(v) => v.span,
            AstNodeKind::ClassificationTestOperator(v) => v.span,
            AstNodeKind::CastOperator(v) => v.span,
            AstNodeKind::MetaclassificationExpression(v) => v.span,
            AstNodeKind::ArgumentMember(v) => v.span,
            AstNodeKind::Argument(v) => v.span,
            AstNodeKind::ArgumentValue(v) => v.span,
            AstNodeKind::ArgumentExpressionMember(v) => v.span,
            AstNodeKind::ArgumentExpression(v) => v.span,
            AstNodeKind::ArgumentExpressionValue(v) => v.span,
            AstNodeKind::MetadataArgumentMember(v) => v.span,
            AstNodeKind::MetadataArgument(v) => v.span,
            AstNodeKind::MetadataValue(v) => v.span,
            AstNodeKind::MetadataReference(v) => v.span,
            AstNodeKind::MetaclassificationTestOperator(v) => v.span,
            AstNodeKind::MetaCastOperator(v) => v.span,
            AstNodeKind::ExtentExpression(v) => v.span,
            AstNodeKind::TypeReferenceMember(v) => v.span,
            AstNodeKind::TypeResultMember(v) => v.span,
            AstNodeKind::TypeReference(v) => v.span,
            AstNodeKind::ReferenceTyping(v) => v.span,
            AstNodeKind::EmptyResultMember(v) => v.span,
            AstNodeKind::EmptyFeature(v) => v.span,
            AstNodeKind::PrimaryExpression(_v) => { Span::default() },
            AstNodeKind::PrimaryArgumentValue(v) => v.span,
            AstNodeKind::PrimaryArgument(v) => v.span,
            AstNodeKind::PrimaryArgumentMember(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryExpression(_v) => { Span::default() },
            AstNodeKind::NonFeatureChainPrimaryArgumentValue(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryArgument(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryArgumentMember(v) => v.span,
            AstNodeKind::BracketExpression(v) => v.span,
            AstNodeKind::IndexExpression(v) => v.span,
            AstNodeKind::SequenceExpression(v) => v.span,
            AstNodeKind::SequenceExpressionList(v) => v.span,
            AstNodeKind::SequenceOperatorExpression(v) => v.span,
            AstNodeKind::SequenceExpressionListMember(v) => v.span,
            AstNodeKind::FeatureChainExpression(v) => v.span,
            AstNodeKind::CollectExpression(v) => v.span,
            AstNodeKind::SelectExpression(v) => v.span,
            AstNodeKind::FunctionOperationExpression(v) => v.span,
            AstNodeKind::BodyArgumentMember(v) => v.span,
            AstNodeKind::BodyArgument(v) => v.span,
            AstNodeKind::BodyArgumentValue(v) => v.span,
            AstNodeKind::FunctionReferenceArgumentMember(v) => v.span,
            AstNodeKind::FunctionReferenceArgument(v) => v.span,
            AstNodeKind::FunctionReferenceArgumentValue(v) => v.span,
            AstNodeKind::FunctionReferenceExpression(v) => v.span,
            AstNodeKind::FunctionReferenceMember(v) => v.span,
            AstNodeKind::FunctionReference(v) => v.span,
            AstNodeKind::FeatureChainMember(_v) => { Span::default() },
            AstNodeKind::OwnedFeatureChainMember(v) => v.span,
            AstNodeKind::BaseExpression(_v) => { Span::default() },
            AstNodeKind::NullExpression(v) => v.span,
            AstNodeKind::FeatureReferenceExpression(v) => v.span,
            AstNodeKind::FeatureReferenceMember(v) => v.span,
            AstNodeKind::FeatureReference(v) => v.span,
            AstNodeKind::MetadataAccessExpression(v) => v.span,
            AstNodeKind::ElementReferenceMember(v) => v.span,
            AstNodeKind::InvocationExpression(v) => v.span,
            AstNodeKind::ConstructorExpression(v) => v.span,
            AstNodeKind::ConstructorResultMember(v) => v.span,
            AstNodeKind::ConstructorResult(v) => v.span,
            AstNodeKind::InstantiatedTypeMember(v) => v.span,
            AstNodeKind::InstantiatedTypeReference(v) => v.span,
            AstNodeKind::ArgumentList(v) => v.span,
            AstNodeKind::PositionalArgumentList(v) => v.span,
            AstNodeKind::NamedArgumentList(v) => v.span,
            AstNodeKind::NamedArgumentMember(v) => v.span,
            AstNodeKind::NamedArgument(v) => v.span,
            AstNodeKind::ParameterRedefinition(v) => v.span,
            AstNodeKind::BodyExpression(v) => v.span,
            AstNodeKind::ExpressionBodyMember(v) => v.span,
            AstNodeKind::ExpressionBody(v) => v.span,
            AstNodeKind::LiteralExpression(_v) => { Span::default() },
            AstNodeKind::LiteralBoolean(v) => v.span,
            AstNodeKind::BooleanValue(v) => v.span,
            AstNodeKind::LiteralString(v) => v.span,
            AstNodeKind::LiteralInteger(v) => v.span,
            AstNodeKind::LiteralReal(v) => v.span,
            AstNodeKind::RealValue(v) => v.span,
            AstNodeKind::LiteralInfinity(v) => v.span,
            AstNodeKind::Interaction(v) => v.span,
            AstNodeKind::Flow(v) => v.span,
            AstNodeKind::SuccessionFlow(v) => v.span,
            AstNodeKind::FlowDeclaration(v) => v.span,
            AstNodeKind::PayloadFeatureMember(v) => v.span,
            AstNodeKind::PayloadFeature(v) => v.span,
            AstNodeKind::PayloadFeatureSpecializationPart(v) => v.span,
            AstNodeKind::FlowEndMember(v) => v.span,
            AstNodeKind::FlowEnd(v) => v.span,
            AstNodeKind::FlowFeatureMember(v) => v.span,
            AstNodeKind::FlowFeature(v) => v.span,
            AstNodeKind::FlowFeatureRedefinition(v) => v.span,
            AstNodeKind::ValuePart(v) => v.span,
            AstNodeKind::FeatureValue(v) => v.span,
            AstNodeKind::Multiplicity(_v) => { Span::default() },
            AstNodeKind::MultiplicitySubset(v) => v.span,
            AstNodeKind::MultiplicityRange(v) => v.span,
            AstNodeKind::OwnedMultiplicity(v) => v.span,
            AstNodeKind::OwnedMultiplicityRange(v) => v.span,
            AstNodeKind::MultiplicityBounds(v) => v.span,
            AstNodeKind::MultiplicityExpressionMember(v) => v.span,
            AstNodeKind::Metaclass(v) => v.span,
            AstNodeKind::PrefixMetadataAnnotation(v) => v.span,
            AstNodeKind::PrefixMetadataMember(v) => v.span,
            AstNodeKind::PrefixMetadataFeature(v) => v.span,
            AstNodeKind::MetadataFeature(v) => v.span,
            AstNodeKind::MetadataFeatureDeclaration(v) => v.span,
            AstNodeKind::MetadataBody(v) => v.span,
            AstNodeKind::MetadataBodyElement(_v) => { Span::default() },
            AstNodeKind::MetadataBodyFeatureMember(v) => v.span,
            AstNodeKind::MetadataBodyFeature(v) => v.span,
            AstNodeKind::Package(v) => v.span,
            AstNodeKind::LibraryPackage(v) => v.span,
            AstNodeKind::PackageDeclaration(v) => v.span,
            AstNodeKind::PackageBody(v) => v.span,
            AstNodeKind::ElementFilterMember(v) => v.span,
        }
    }

    /// Return the variant name as a string.
    pub fn kind_name(&self) -> &'static str {
        match self {
            AstNodeKind::Identification(_) => "Identification",
            AstNodeKind::RelationshipBody(_) => "RelationshipBody",
            AstNodeKind::RelationshipOwnedElement(_) => "RelationshipOwnedElement",
            AstNodeKind::OwnedRelatedElement(_) => "OwnedRelatedElement",
            AstNodeKind::Dependency(_) => "Dependency",
            AstNodeKind::Annotation(_) => "Annotation",
            AstNodeKind::OwnedAnnotation(_) => "OwnedAnnotation",
            AstNodeKind::AnnotatingElement(_) => "AnnotatingElement",
            AstNodeKind::Comment(_) => "Comment",
            AstNodeKind::Documentation(_) => "Documentation",
            AstNodeKind::TextualRepresentation(_) => "TextualRepresentation",
            AstNodeKind::RootNamespace(_) => "RootNamespace",
            AstNodeKind::Namespace(_) => "Namespace",
            AstNodeKind::NamespaceDeclaration(_) => "NamespaceDeclaration",
            AstNodeKind::NamespaceBody(_) => "NamespaceBody",
            AstNodeKind::NamespaceBodyElement(_) => "NamespaceBodyElement",
            AstNodeKind::MemberPrefix(_) => "MemberPrefix",
            AstNodeKind::VisibilityIndicator(_) => "VisibilityIndicator",
            AstNodeKind::NamespaceMember(_) => "NamespaceMember",
            AstNodeKind::NonFeatureMember(_) => "NonFeatureMember",
            AstNodeKind::NamespaceFeatureMember(_) => "NamespaceFeatureMember",
            AstNodeKind::AliasMember(_) => "AliasMember",
            AstNodeKind::QualifiedName(_) => "QualifiedName",
            AstNodeKind::Import(_) => "Import",
            AstNodeKind::ImportDeclaration(_) => "ImportDeclaration",
            AstNodeKind::MembershipImport(_) => "MembershipImport",
            AstNodeKind::NamespaceImport(_) => "NamespaceImport",
            AstNodeKind::FilterPackage(_) => "FilterPackage",
            AstNodeKind::FilterPackageMember(_) => "FilterPackageMember",
            AstNodeKind::MemberElement(_) => "MemberElement",
            AstNodeKind::NonFeatureElement(_) => "NonFeatureElement",
            AstNodeKind::FeatureElement(_) => "FeatureElement",
            AstNodeKind::Type(_) => "Type",
            AstNodeKind::TypePrefix(_) => "TypePrefix",
            AstNodeKind::TypeDeclaration(_) => "TypeDeclaration",
            AstNodeKind::SpecializationPart(_) => "SpecializationPart",
            AstNodeKind::ConjugationPart(_) => "ConjugationPart",
            AstNodeKind::TypeRelationshipPart(_) => "TypeRelationshipPart",
            AstNodeKind::DisjoiningPart(_) => "DisjoiningPart",
            AstNodeKind::UnioningPart(_) => "UnioningPart",
            AstNodeKind::IntersectingPart(_) => "IntersectingPart",
            AstNodeKind::DifferencingPart(_) => "DifferencingPart",
            AstNodeKind::TypeBody(_) => "TypeBody",
            AstNodeKind::TypeBodyElement(_) => "TypeBodyElement",
            AstNodeKind::Specialization(_) => "Specialization",
            AstNodeKind::OwnedSpecialization(_) => "OwnedSpecialization",
            AstNodeKind::SpecificType(_) => "SpecificType",
            AstNodeKind::GeneralType(_) => "GeneralType",
            AstNodeKind::Conjugation(_) => "Conjugation",
            AstNodeKind::OwnedConjugation(_) => "OwnedConjugation",
            AstNodeKind::Disjoining(_) => "Disjoining",
            AstNodeKind::OwnedDisjoining(_) => "OwnedDisjoining",
            AstNodeKind::Unioning(_) => "Unioning",
            AstNodeKind::Intersecting(_) => "Intersecting",
            AstNodeKind::Differencing(_) => "Differencing",
            AstNodeKind::FeatureMember(_) => "FeatureMember",
            AstNodeKind::TypeFeatureMember(_) => "TypeFeatureMember",
            AstNodeKind::OwnedFeatureMember(_) => "OwnedFeatureMember",
            AstNodeKind::Classifier(_) => "Classifier",
            AstNodeKind::ClassifierDeclaration(_) => "ClassifierDeclaration",
            AstNodeKind::SuperclassingPart(_) => "SuperclassingPart",
            AstNodeKind::Subclassification(_) => "Subclassification",
            AstNodeKind::OwnedSubclassification(_) => "OwnedSubclassification",
            AstNodeKind::Feature(_) => "Feature",
            AstNodeKind::EndFeaturePrefix(_) => "EndFeaturePrefix",
            AstNodeKind::BasicFeaturePrefix(_) => "BasicFeaturePrefix",
            AstNodeKind::FeaturePrefix(_) => "FeaturePrefix",
            AstNodeKind::OwnedCrossFeatureMember(_) => "OwnedCrossFeatureMember",
            AstNodeKind::OwnedCrossFeature(_) => "OwnedCrossFeature",
            AstNodeKind::FeatureDirection(_) => "FeatureDirection",
            AstNodeKind::FeatureDeclaration(_) => "FeatureDeclaration",
            AstNodeKind::FeatureIdentification(_) => "FeatureIdentification",
            AstNodeKind::FeatureRelationshipPart(_) => "FeatureRelationshipPart",
            AstNodeKind::ChainingPart(_) => "ChainingPart",
            AstNodeKind::InvertingPart(_) => "InvertingPart",
            AstNodeKind::TypeFeaturingPart(_) => "TypeFeaturingPart",
            AstNodeKind::FeatureSpecializationPart(_) => "FeatureSpecializationPart",
            AstNodeKind::MultiplicityPart(_) => "MultiplicityPart",
            AstNodeKind::FeatureSpecialization(_) => "FeatureSpecialization",
            AstNodeKind::Typings(_) => "Typings",
            AstNodeKind::TypedBy(_) => "TypedBy",
            AstNodeKind::Subsettings(_) => "Subsettings",
            AstNodeKind::Subsets(_) => "Subsets",
            AstNodeKind::References(_) => "References",
            AstNodeKind::Crosses(_) => "Crosses",
            AstNodeKind::Redefinitions(_) => "Redefinitions",
            AstNodeKind::Redefines(_) => "Redefines",
            AstNodeKind::FeatureTyping(_) => "FeatureTyping",
            AstNodeKind::OwnedFeatureTyping(_) => "OwnedFeatureTyping",
            AstNodeKind::Subsetting(_) => "Subsetting",
            AstNodeKind::OwnedSubsetting(_) => "OwnedSubsetting",
            AstNodeKind::OwnedReferenceSubsetting(_) => "OwnedReferenceSubsetting",
            AstNodeKind::OwnedCrossSubsetting(_) => "OwnedCrossSubsetting",
            AstNodeKind::Redefinition(_) => "Redefinition",
            AstNodeKind::OwnedRedefinition(_) => "OwnedRedefinition",
            AstNodeKind::OwnedFeatureChain(_) => "OwnedFeatureChain",
            AstNodeKind::FeatureChain(_) => "FeatureChain",
            AstNodeKind::OwnedFeatureChaining(_) => "OwnedFeatureChaining",
            AstNodeKind::FeatureInverting(_) => "FeatureInverting",
            AstNodeKind::OwnedFeatureInverting(_) => "OwnedFeatureInverting",
            AstNodeKind::TypeFeaturing(_) => "TypeFeaturing",
            AstNodeKind::OwnedTypeFeaturing(_) => "OwnedTypeFeaturing",
            AstNodeKind::DataType(_) => "DataType",
            AstNodeKind::Class(_) => "Class",
            AstNodeKind::Structure(_) => "Structure",
            AstNodeKind::Association(_) => "Association",
            AstNodeKind::AssociationStructure(_) => "AssociationStructure",
            AstNodeKind::Connector(_) => "Connector",
            AstNodeKind::ConnectorDeclaration(_) => "ConnectorDeclaration",
            AstNodeKind::BinaryConnectorDeclaration(_) => "BinaryConnectorDeclaration",
            AstNodeKind::NaryConnectorDeclaration(_) => "NaryConnectorDeclaration",
            AstNodeKind::ConnectorEndMember(_) => "ConnectorEndMember",
            AstNodeKind::ConnectorEnd(_) => "ConnectorEnd",
            AstNodeKind::OwnedCrossMultiplicityMember(_) => "OwnedCrossMultiplicityMember",
            AstNodeKind::OwnedCrossMultiplicity(_) => "OwnedCrossMultiplicity",
            AstNodeKind::BindingConnector(_) => "BindingConnector",
            AstNodeKind::BindingConnectorDeclaration(_) => "BindingConnectorDeclaration",
            AstNodeKind::Succession(_) => "Succession",
            AstNodeKind::SuccessionDeclaration(_) => "SuccessionDeclaration",
            AstNodeKind::Behavior(_) => "Behavior",
            AstNodeKind::Step(_) => "Step",
            AstNodeKind::Function(_) => "Function",
            AstNodeKind::FunctionBody(_) => "FunctionBody",
            AstNodeKind::FunctionBodyPart(_) => "FunctionBodyPart",
            AstNodeKind::ReturnFeatureMember(_) => "ReturnFeatureMember",
            AstNodeKind::ResultExpressionMember(_) => "ResultExpressionMember",
            AstNodeKind::Expression(_) => "Expression",
            AstNodeKind::Predicate(_) => "Predicate",
            AstNodeKind::BooleanExpression(_) => "BooleanExpression",
            AstNodeKind::Invariant(_) => "Invariant",
            AstNodeKind::OwnedExpressionReferenceMember(_) => "OwnedExpressionReferenceMember",
            AstNodeKind::OwnedExpressionReference(_) => "OwnedExpressionReference",
            AstNodeKind::OwnedExpressionMember(_) => "OwnedExpressionMember",
            AstNodeKind::OwnedExpression(_) => "OwnedExpression",
            AstNodeKind::ConditionalExpression(_) => "ConditionalExpression",
            AstNodeKind::ConditionalBinaryOperatorExpression(_) => "ConditionalBinaryOperatorExpression",
            AstNodeKind::ConditionalBinaryOperator(_) => "ConditionalBinaryOperator",
            AstNodeKind::BinaryOperatorExpression(_) => "BinaryOperatorExpression",
            AstNodeKind::BinaryOperator(_) => "BinaryOperator",
            AstNodeKind::UnaryOperatorExpression(_) => "UnaryOperatorExpression",
            AstNodeKind::UnaryOperator(_) => "UnaryOperator",
            AstNodeKind::ClassificationExpression(_) => "ClassificationExpression",
            AstNodeKind::ClassificationTestOperator(_) => "ClassificationTestOperator",
            AstNodeKind::CastOperator(_) => "CastOperator",
            AstNodeKind::MetaclassificationExpression(_) => "MetaclassificationExpression",
            AstNodeKind::ArgumentMember(_) => "ArgumentMember",
            AstNodeKind::Argument(_) => "Argument",
            AstNodeKind::ArgumentValue(_) => "ArgumentValue",
            AstNodeKind::ArgumentExpressionMember(_) => "ArgumentExpressionMember",
            AstNodeKind::ArgumentExpression(_) => "ArgumentExpression",
            AstNodeKind::ArgumentExpressionValue(_) => "ArgumentExpressionValue",
            AstNodeKind::MetadataArgumentMember(_) => "MetadataArgumentMember",
            AstNodeKind::MetadataArgument(_) => "MetadataArgument",
            AstNodeKind::MetadataValue(_) => "MetadataValue",
            AstNodeKind::MetadataReference(_) => "MetadataReference",
            AstNodeKind::MetaclassificationTestOperator(_) => "MetaclassificationTestOperator",
            AstNodeKind::MetaCastOperator(_) => "MetaCastOperator",
            AstNodeKind::ExtentExpression(_) => "ExtentExpression",
            AstNodeKind::TypeReferenceMember(_) => "TypeReferenceMember",
            AstNodeKind::TypeResultMember(_) => "TypeResultMember",
            AstNodeKind::TypeReference(_) => "TypeReference",
            AstNodeKind::ReferenceTyping(_) => "ReferenceTyping",
            AstNodeKind::EmptyResultMember(_) => "EmptyResultMember",
            AstNodeKind::EmptyFeature(_) => "EmptyFeature",
            AstNodeKind::PrimaryExpression(_) => "PrimaryExpression",
            AstNodeKind::PrimaryArgumentValue(_) => "PrimaryArgumentValue",
            AstNodeKind::PrimaryArgument(_) => "PrimaryArgument",
            AstNodeKind::PrimaryArgumentMember(_) => "PrimaryArgumentMember",
            AstNodeKind::NonFeatureChainPrimaryExpression(_) => "NonFeatureChainPrimaryExpression",
            AstNodeKind::NonFeatureChainPrimaryArgumentValue(_) => "NonFeatureChainPrimaryArgumentValue",
            AstNodeKind::NonFeatureChainPrimaryArgument(_) => "NonFeatureChainPrimaryArgument",
            AstNodeKind::NonFeatureChainPrimaryArgumentMember(_) => "NonFeatureChainPrimaryArgumentMember",
            AstNodeKind::BracketExpression(_) => "BracketExpression",
            AstNodeKind::IndexExpression(_) => "IndexExpression",
            AstNodeKind::SequenceExpression(_) => "SequenceExpression",
            AstNodeKind::SequenceExpressionList(_) => "SequenceExpressionList",
            AstNodeKind::SequenceOperatorExpression(_) => "SequenceOperatorExpression",
            AstNodeKind::SequenceExpressionListMember(_) => "SequenceExpressionListMember",
            AstNodeKind::FeatureChainExpression(_) => "FeatureChainExpression",
            AstNodeKind::CollectExpression(_) => "CollectExpression",
            AstNodeKind::SelectExpression(_) => "SelectExpression",
            AstNodeKind::FunctionOperationExpression(_) => "FunctionOperationExpression",
            AstNodeKind::BodyArgumentMember(_) => "BodyArgumentMember",
            AstNodeKind::BodyArgument(_) => "BodyArgument",
            AstNodeKind::BodyArgumentValue(_) => "BodyArgumentValue",
            AstNodeKind::FunctionReferenceArgumentMember(_) => "FunctionReferenceArgumentMember",
            AstNodeKind::FunctionReferenceArgument(_) => "FunctionReferenceArgument",
            AstNodeKind::FunctionReferenceArgumentValue(_) => "FunctionReferenceArgumentValue",
            AstNodeKind::FunctionReferenceExpression(_) => "FunctionReferenceExpression",
            AstNodeKind::FunctionReferenceMember(_) => "FunctionReferenceMember",
            AstNodeKind::FunctionReference(_) => "FunctionReference",
            AstNodeKind::FeatureChainMember(_) => "FeatureChainMember",
            AstNodeKind::OwnedFeatureChainMember(_) => "OwnedFeatureChainMember",
            AstNodeKind::BaseExpression(_) => "BaseExpression",
            AstNodeKind::NullExpression(_) => "NullExpression",
            AstNodeKind::FeatureReferenceExpression(_) => "FeatureReferenceExpression",
            AstNodeKind::FeatureReferenceMember(_) => "FeatureReferenceMember",
            AstNodeKind::FeatureReference(_) => "FeatureReference",
            AstNodeKind::MetadataAccessExpression(_) => "MetadataAccessExpression",
            AstNodeKind::ElementReferenceMember(_) => "ElementReferenceMember",
            AstNodeKind::InvocationExpression(_) => "InvocationExpression",
            AstNodeKind::ConstructorExpression(_) => "ConstructorExpression",
            AstNodeKind::ConstructorResultMember(_) => "ConstructorResultMember",
            AstNodeKind::ConstructorResult(_) => "ConstructorResult",
            AstNodeKind::InstantiatedTypeMember(_) => "InstantiatedTypeMember",
            AstNodeKind::InstantiatedTypeReference(_) => "InstantiatedTypeReference",
            AstNodeKind::ArgumentList(_) => "ArgumentList",
            AstNodeKind::PositionalArgumentList(_) => "PositionalArgumentList",
            AstNodeKind::NamedArgumentList(_) => "NamedArgumentList",
            AstNodeKind::NamedArgumentMember(_) => "NamedArgumentMember",
            AstNodeKind::NamedArgument(_) => "NamedArgument",
            AstNodeKind::ParameterRedefinition(_) => "ParameterRedefinition",
            AstNodeKind::BodyExpression(_) => "BodyExpression",
            AstNodeKind::ExpressionBodyMember(_) => "ExpressionBodyMember",
            AstNodeKind::ExpressionBody(_) => "ExpressionBody",
            AstNodeKind::LiteralExpression(_) => "LiteralExpression",
            AstNodeKind::LiteralBoolean(_) => "LiteralBoolean",
            AstNodeKind::BooleanValue(_) => "BooleanValue",
            AstNodeKind::LiteralString(_) => "LiteralString",
            AstNodeKind::LiteralInteger(_) => "LiteralInteger",
            AstNodeKind::LiteralReal(_) => "LiteralReal",
            AstNodeKind::RealValue(_) => "RealValue",
            AstNodeKind::LiteralInfinity(_) => "LiteralInfinity",
            AstNodeKind::Interaction(_) => "Interaction",
            AstNodeKind::Flow(_) => "Flow",
            AstNodeKind::SuccessionFlow(_) => "SuccessionFlow",
            AstNodeKind::FlowDeclaration(_) => "FlowDeclaration",
            AstNodeKind::PayloadFeatureMember(_) => "PayloadFeatureMember",
            AstNodeKind::PayloadFeature(_) => "PayloadFeature",
            AstNodeKind::PayloadFeatureSpecializationPart(_) => "PayloadFeatureSpecializationPart",
            AstNodeKind::FlowEndMember(_) => "FlowEndMember",
            AstNodeKind::FlowEnd(_) => "FlowEnd",
            AstNodeKind::FlowFeatureMember(_) => "FlowFeatureMember",
            AstNodeKind::FlowFeature(_) => "FlowFeature",
            AstNodeKind::FlowFeatureRedefinition(_) => "FlowFeatureRedefinition",
            AstNodeKind::ValuePart(_) => "ValuePart",
            AstNodeKind::FeatureValue(_) => "FeatureValue",
            AstNodeKind::Multiplicity(_) => "Multiplicity",
            AstNodeKind::MultiplicitySubset(_) => "MultiplicitySubset",
            AstNodeKind::MultiplicityRange(_) => "MultiplicityRange",
            AstNodeKind::OwnedMultiplicity(_) => "OwnedMultiplicity",
            AstNodeKind::OwnedMultiplicityRange(_) => "OwnedMultiplicityRange",
            AstNodeKind::MultiplicityBounds(_) => "MultiplicityBounds",
            AstNodeKind::MultiplicityExpressionMember(_) => "MultiplicityExpressionMember",
            AstNodeKind::Metaclass(_) => "Metaclass",
            AstNodeKind::PrefixMetadataAnnotation(_) => "PrefixMetadataAnnotation",
            AstNodeKind::PrefixMetadataMember(_) => "PrefixMetadataMember",
            AstNodeKind::PrefixMetadataFeature(_) => "PrefixMetadataFeature",
            AstNodeKind::MetadataFeature(_) => "MetadataFeature",
            AstNodeKind::MetadataFeatureDeclaration(_) => "MetadataFeatureDeclaration",
            AstNodeKind::MetadataBody(_) => "MetadataBody",
            AstNodeKind::MetadataBodyElement(_) => "MetadataBodyElement",
            AstNodeKind::MetadataBodyFeatureMember(_) => "MetadataBodyFeatureMember",
            AstNodeKind::MetadataBodyFeature(_) => "MetadataBodyFeature",
            AstNodeKind::Package(_) => "Package",
            AstNodeKind::LibraryPackage(_) => "LibraryPackage",
            AstNodeKind::PackageDeclaration(_) => "PackageDeclaration",
            AstNodeKind::PackageBody(_) => "PackageBody",
            AstNodeKind::ElementFilterMember(_) => "ElementFilterMember",
        }
    }
}

