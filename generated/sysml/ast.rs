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

/// `AssignmentTargetMember`
#[derive(Debug, Clone)]
pub struct AssignmentTargetMember {
    pub span: Span,
    pub owned_related_element: Vec<AssignmentTargetParameter>,
}

impl AstNode for AssignmentTargetMember {
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

/// `InterfaceDefinition`
#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub span: Span,
}

impl AstNode for InterfaceDefinition {
    fn span(&self) -> Span { self.span }
}

/// `FlowPayloadFeature`
#[derive(Debug, Clone)]
pub struct FlowPayloadFeature {
    pub span: Span,
}

impl AstNode for FlowPayloadFeature {
    fn span(&self) -> Span { self.span }
}

/// `ActionBody`
#[derive(Debug, Clone)]
pub struct ActionBody {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for ActionBody {
    fn span(&self) -> Span { self.span }
}

/// `StateDefinition`
#[derive(Debug, Clone)]
pub struct StateDefinition {
    pub span: Span,
}

impl AstNode for StateDefinition {
    fn span(&self) -> Span { self.span }
}

/// `SubjectUsage`
#[derive(Debug, Clone)]
pub struct SubjectUsage {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for SubjectUsage {
    fn span(&self) -> Span { self.span }
}

/// `Expose`
#[derive(Debug, Clone)]
pub struct Expose {
    pub span: Span,
}

impl AstNode for Expose {
    fn span(&self) -> Span { self.span }
}

/// `EmptyMultiplicityMember`
#[derive(Debug, Clone)]
pub struct EmptyMultiplicityMember {
    pub span: Span,
    pub owned_related_element: Vec<EmptyMultiplicity>,
}

impl AstNode for EmptyMultiplicityMember {
    fn span(&self) -> Span { self.span }
}

/// `Definition`
#[derive(Debug, Clone)]
pub struct Definition {
    pub span: Span,
}

impl AstNode for Definition {
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

/// `FeatureSpecializationPart`
#[derive(Debug, Clone)]
pub struct FeatureSpecializationPart {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
}

impl AstNode for FeatureSpecializationPart {
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

/// `InterfaceBodyItem`
#[derive(Debug, Clone)]
pub enum InterfaceBodyItemOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    DefinitionMember(Box<DefinitionMember>),
    Import(Box<Import>),
    InterfaceNonOccurrenceUsageMember(Box<InterfaceNonOccurrenceUsageMember>),
    InterfaceOccurrenceUsageMember(Box<InterfaceOccurrenceUsageMember>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
    VariantUsageMember(Box<VariantUsageMember>),
}

#[derive(Debug, Clone)]
pub struct InterfaceBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<InterfaceBodyItemOwnedRelationshipMember>,
}

impl AstNode for InterfaceBodyItem {
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

/// `EndUsagePrefix`
#[derive(Debug, Clone)]
pub struct EndUsagePrefix {
    pub span: Span,
    pub is_end: bool,
    pub owned_relationship: Vec<OwnedCrossFeatureMember>,
}

impl AstNode for EndUsagePrefix {
    fn span(&self) -> Span { self.span }
}

/// `DefaultInterfaceEnd`
#[derive(Debug, Clone)]
pub struct DefaultInterfaceEnd {
    pub span: Span,
    pub is_end: bool,
}

impl AstNode for DefaultInterfaceEnd {
    fn span(&self) -> Span { self.span }
}

/// `ObjectiveMember`
#[derive(Debug, Clone)]
pub struct ObjectiveMember {
    pub span: Span,
    pub owned_related_element: Vec<ObjectiveRequirementUsage>,
}

impl AstNode for ObjectiveMember {
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

/// `ActionTargetSuccession`
#[derive(Debug, Clone)]
pub struct ActionTargetSuccession {
    pub span: Span,
}

impl AstNode for ActionTargetSuccession {
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

/// `VerificationCaseDefinition`
#[derive(Debug, Clone)]
pub struct VerificationCaseDefinition {
    pub span: Span,
}

impl AstNode for VerificationCaseDefinition {
    fn span(&self) -> Span { self.span }
}

/// `AcceptParameterPart`
#[derive(Debug, Clone)]
pub enum AcceptParameterPartOwnedRelationshipMember {
    NodeParameterMember(Box<NodeParameterMember>),
    PayloadParameterMember(Box<PayloadParameterMember>),
}

#[derive(Debug, Clone)]
pub struct AcceptParameterPart {
    pub span: Span,
    pub owned_relationship: Vec<AcceptParameterPartOwnedRelationshipMember>,
}

impl AstNode for AcceptParameterPart {
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

/// `BodyArgument`
#[derive(Debug, Clone)]
pub struct BodyArgument {
    pub span: Span,
    pub owned_relationship: Vec<BodyArgumentValue>,
}

impl AstNode for BodyArgument {
    fn span(&self) -> Span { self.span }
}

/// `BinaryConnectorPart`
#[derive(Debug, Clone)]
pub struct BinaryConnectorPart {
    pub span: Span,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for BinaryConnectorPart {
    fn span(&self) -> Span { self.span }
}

/// `ControlNode`
#[derive(Debug, Clone)]
pub enum ControlNode {
    MergeNode(Box<MergeNode>),
    DecisionNode(Box<DecisionNode>),
    JoinNode(Box<JoinNode>),
    ForkNode(Box<ForkNode>),
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

/// `FlowUsage`
#[derive(Debug, Clone)]
pub struct FlowUsage {
    pub span: Span,
}

impl AstNode for FlowUsage {
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

/// `FlowFeatureMember`
#[derive(Debug, Clone)]
pub struct FlowFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FlowFeature>,
}

impl AstNode for FlowFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `Message`
#[derive(Debug, Clone)]
pub struct Message {
    pub span: Span,
}

impl AstNode for Message {
    fn span(&self) -> Span { self.span }
}

/// `ConstraintUsageDeclaration`
#[derive(Debug, Clone)]
pub struct ConstraintUsageDeclaration {
    pub span: Span,
}

impl AstNode for ConstraintUsageDeclaration {
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

/// `SourceEndMember`
#[derive(Debug, Clone)]
pub struct SourceEndMember {
    pub span: Span,
    pub owned_related_element: Vec<SourceEnd>,
}

impl AstNode for SourceEndMember {
    fn span(&self) -> Span { self.span }
}

/// `DefaultTargetSuccession`
#[derive(Debug, Clone)]
pub struct DefaultTargetSuccession {
    pub span: Span,
    pub owned_relationship: Vec<TransitionSuccessionMember>,
}

impl AstNode for DefaultTargetSuccession {
    fn span(&self) -> Span { self.span }
}

/// `AnalysisCaseDefinition`
#[derive(Debug, Clone)]
pub struct AnalysisCaseDefinition {
    pub span: Span,
}

impl AstNode for AnalysisCaseDefinition {
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

/// `FunctionReference`
#[derive(Debug, Clone)]
pub struct FunctionReference {
    pub span: Span,
    pub owned_relationship: Vec<ReferenceTyping>,
}

impl AstNode for FunctionReference {
    fn span(&self) -> Span { self.span }
}

/// `MemberElement`
#[derive(Debug, Clone)]
pub enum MemberElement {
    AnnotatingElement(Box<AnnotatingElement>),
    NonFeatureElement(Box<NonFeatureElement>),
}

/// `AllocationUsageDeclaration`
#[derive(Debug, Clone)]
pub struct AllocationUsageDeclaration {
    pub span: Span,
}

impl AstNode for AllocationUsageDeclaration {
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

/// `OwnedExpressionReference`
#[derive(Debug, Clone)]
pub struct OwnedExpressionReference {
    pub span: Span,
    pub owned_relationship: Vec<OwnedExpressionMember>,
}

impl AstNode for OwnedExpressionReference {
    fn span(&self) -> Span { self.span }
}

/// `RootNamespace`
#[derive(Debug, Clone)]
pub struct RootNamespace {
    pub span: Span,
    pub package_body_element: Vec<PackageBodyElement>,
}

impl AstNode for RootNamespace {
    fn span(&self) -> Span { self.span }
}

/// `SatisfactionFeatureValue`
#[derive(Debug, Clone)]
pub struct SatisfactionFeatureValue {
    pub span: Span,
    pub owned_related_element: Vec<SatisfactionReferenceExpression>,
}

impl AstNode for SatisfactionFeatureValue {
    fn span(&self) -> Span { self.span }
}

/// `Typings`
#[derive(Debug, Clone)]
pub struct Typings {
    pub span: Span,
    pub owned_relationship: Vec<FeatureTyping>,
}

impl AstNode for Typings {
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

/// `IntersectingPart`
#[derive(Debug, Clone)]
pub struct IntersectingPart {
    pub span: Span,
    pub owned_relationship: Vec<Intersecting>,
}

impl AstNode for IntersectingPart {
    fn span(&self) -> Span { self.span }
}

/// `DefinitionBodyItem`
#[derive(Debug, Clone)]
pub enum DefinitionBodyItemOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    DefinitionMember(Box<DefinitionMember>),
    Import(Box<Import>),
    NonOccurrenceUsageMember(Box<NonOccurrenceUsageMember>),
    OccurrenceUsageMember(Box<OccurrenceUsageMember>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
    VariantUsageMember(Box<VariantUsageMember>),
}

#[derive(Debug, Clone)]
pub struct DefinitionBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<DefinitionBodyItemOwnedRelationshipMember>,
}

impl AstNode for DefinitionBodyItem {
    fn span(&self) -> Span { self.span }
}

/// `SubclassificationPart`
#[derive(Debug, Clone)]
pub struct SubclassificationPart {
    pub span: Span,
    pub owned_relationship: Vec<OwnedSubclassification>,
}

impl AstNode for SubclassificationPart {
    fn span(&self) -> Span { self.span }
}

/// `ExpressionParameterMember`
#[derive(Debug, Clone)]
pub struct ExpressionParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for ExpressionParameterMember {
    fn span(&self) -> Span { self.span }
}

/// `SatisfactionParameter`
#[derive(Debug, Clone)]
pub struct SatisfactionParameter {
    pub span: Span,
    pub owned_relationship: Vec<SatisfactionFeatureValue>,
}

impl AstNode for SatisfactionParameter {
    fn span(&self) -> Span { self.span }
}

/// `EffectBehaviorMember`
#[derive(Debug, Clone)]
pub struct EffectBehaviorMember {
    pub span: Span,
    pub owned_related_element: Vec<EffectBehaviorUsage>,
}

impl AstNode for EffectBehaviorMember {
    fn span(&self) -> Span { self.span }
}

/// `Usage`
#[derive(Debug, Clone)]
pub struct Usage {
    pub span: Span,
}

impl AstNode for Usage {
    fn span(&self) -> Span { self.span }
}

/// `DefinitionMember`
#[derive(Debug, Clone)]
pub struct DefinitionMember {
    pub span: Span,
    pub owned_related_element: Vec<DefinitionElement>,
}

impl AstNode for DefinitionMember {
    fn span(&self) -> Span { self.span }
}

/// `ItemUsage`
#[derive(Debug, Clone)]
pub struct ItemUsage {
    pub span: Span,
}

impl AstNode for ItemUsage {
    fn span(&self) -> Span { self.span }
}

/// `BasicUsagePrefix`
#[derive(Debug, Clone)]
pub struct BasicUsagePrefix {
    pub span: Span,
    pub is_reference: bool,
}

impl AstNode for BasicUsagePrefix {
    fn span(&self) -> Span { self.span }
}

/// `OwnedSubsetting`
#[derive(Debug, Clone)]
pub enum OwnedSubsettingSubsettedFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedSubsetting {
    pub span: Span,
    pub subsetted_feature: Option<OwnedSubsettingSubsettedFeatureMember>,
}

impl AstNode for OwnedSubsetting {
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

/// `FeatureChainMember`
#[derive(Debug, Clone)]
pub struct FeatureChainMember {
    pub span: Span,
    pub member_element: Option<QualifiedNameRef>,
}

impl AstNode for FeatureChainMember {
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

/// `Redefines`
#[derive(Debug, Clone)]
pub struct Redefines {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for Redefines {
    fn span(&self) -> Span { self.span }
}

/// `PortUsage`
#[derive(Debug, Clone)]
pub struct PortUsage {
    pub span: Span,
}

impl AstNode for PortUsage {
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

/// `IfNodeParameterMember`
#[derive(Debug, Clone)]
pub struct IfNodeParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<IfNode>,
}

impl AstNode for IfNodeParameterMember {
    fn span(&self) -> Span { self.span }
}

/// `TransitionUsageMember`
#[derive(Debug, Clone)]
pub struct TransitionUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<TransitionUsage>,
}

impl AstNode for TransitionUsageMember {
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

/// `ObjectiveRequirementUsage`
#[derive(Debug, Clone)]
pub struct ObjectiveRequirementUsage {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for ObjectiveRequirementUsage {
    fn span(&self) -> Span { self.span }
}

/// `VariantReference`
#[derive(Debug, Clone)]
pub struct VariantReference {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for VariantReference {
    fn span(&self) -> Span { self.span }
}

/// `TriggerFeatureValue`
#[derive(Debug, Clone)]
pub struct TriggerFeatureValue {
    pub span: Span,
    pub owned_related_element: Vec<TriggerExpression>,
}

impl AstNode for TriggerFeatureValue {
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

/// `MetadataUsageDeclaration`
#[derive(Debug, Clone)]
pub struct MetadataUsageDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for MetadataUsageDeclaration {
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

/// `TypeRelationshipPart`
#[derive(Debug, Clone)]
pub enum TypeRelationshipPart {
    DisjoiningPart(Box<DisjoiningPart>),
    UnioningPart(Box<UnioningPart>),
    IntersectingPart(Box<IntersectingPart>),
    DifferencingPart(Box<DifferencingPart>),
}

/// `TypedBy`
#[derive(Debug, Clone)]
pub struct TypedBy {
    pub span: Span,
    pub owned_relationship: Vec<FeatureTyping>,
}

impl AstNode for TypedBy {
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

/// `TransitionUsage`
#[derive(Debug, Clone)]
pub enum TransitionUsageOwnedRelationshipMember {
    EffectBehaviorMember(Box<EffectBehaviorMember>),
    EmptyParameterMember(Box<EmptyParameterMember>),
    FeatureChainMember(Box<FeatureChainMember>),
    GuardExpressionMember(Box<GuardExpressionMember>),
    TransitionSuccessionMember(Box<TransitionSuccessionMember>),
    TriggerActionMember(Box<TriggerActionMember>),
}

#[derive(Debug, Clone)]
pub struct TransitionUsage {
    pub span: Span,
    pub owned_relationship: Vec<TransitionUsageOwnedRelationshipMember>,
}

impl AstNode for TransitionUsage {
    fn span(&self) -> Span { self.span }
}

/// `StakeholderMember`
#[derive(Debug, Clone)]
pub struct StakeholderMember {
    pub span: Span,
    pub owned_related_element: Vec<StakeholderUsage>,
}

impl AstNode for StakeholderMember {
    fn span(&self) -> Span { self.span }
}

/// `FilterPackage`
#[derive(Debug, Clone)]
pub enum FilterPackageOwnedRelationshipMember {
    FilterPackageImport(Box<FilterPackageImport>),
    FilterPackageMember(Box<FilterPackageMember>),
}

#[derive(Debug, Clone)]
pub struct FilterPackage {
    pub span: Span,
    pub owned_relationship: Vec<FilterPackageOwnedRelationshipMember>,
}

impl AstNode for FilterPackage {
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

/// `BindingConnector`
#[derive(Debug, Clone)]
pub struct BindingConnector {
    pub span: Span,
}

impl AstNode for BindingConnector {
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

/// `DependencyDeclaration`
#[derive(Debug, Clone)]
pub struct DependencyDeclaration {
    pub span: Span,
    pub client: Vec<QualifiedNameRef>,
    pub supplier: Vec<QualifiedNameRef>,
}

impl AstNode for DependencyDeclaration {
    fn span(&self) -> Span { self.span }
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

/// `ViewpointUsage`
#[derive(Debug, Clone)]
pub struct ViewpointUsage {
    pub span: Span,
}

impl AstNode for ViewpointUsage {
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

/// `TargetSuccession`
#[derive(Debug, Clone)]
pub enum TargetSuccessionOwnedRelationshipMember {
    ConnectorEndMember(Box<ConnectorEndMember>),
    SourceEndMember(Box<SourceEndMember>),
}

#[derive(Debug, Clone)]
pub struct TargetSuccession {
    pub span: Span,
    pub owned_relationship: Vec<TargetSuccessionOwnedRelationshipMember>,
}

impl AstNode for TargetSuccession {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBodyUsageMember`
#[derive(Debug, Clone)]
pub struct MetadataBodyUsageMember {
    pub span: Span,
    pub owned_member_feature: Box<MetadataBodyUsage>,
}

impl AstNode for MetadataBodyUsageMember {
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

/// `CalculationBody`
#[derive(Debug, Clone)]
pub struct CalculationBody {
    pub span: Span,
}

impl AstNode for CalculationBody {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceMember`
#[derive(Debug, Clone)]
pub enum NamespaceMember {
    NonFeatureMember(Box<NonFeatureMember>),
    NamespaceFeatureMember(Box<NamespaceFeatureMember>),
}

/// `Dependency`
#[derive(Debug, Clone)]
pub struct Dependency {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataAnnotation>,
}

impl AstNode for Dependency {
    fn span(&self) -> Span { self.span }
}

/// `OwnedFeatureTyping`
#[derive(Debug, Clone)]
pub enum OwnedFeatureTypingTypeMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedFeatureTyping {
    pub span: Span,
    pub type_: Option<OwnedFeatureTypingTypeMember>,
}

impl AstNode for OwnedFeatureTyping {
    fn span(&self) -> Span { self.span }
}

/// `ActionUsageDeclaration`
#[derive(Debug, Clone)]
pub struct ActionUsageDeclaration {
    pub span: Span,
}

impl AstNode for ActionUsageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `DecisionNode`
#[derive(Debug, Clone)]
pub struct DecisionNode {
    pub span: Span,
    pub is_composite: bool,
}

impl AstNode for DecisionNode {
    fn span(&self) -> Span { self.span }
}

/// `EffectBehaviorUsage`
#[derive(Debug, Clone)]
pub enum EffectBehaviorUsage {
    EmptyActionUsage(Box<EmptyActionUsage>),
    TransitionPerformActionUsage(Box<TransitionPerformActionUsage>),
    TransitionAcceptActionUsage(Box<TransitionAcceptActionUsage>),
    TransitionSendActionUsage(Box<TransitionSendActionUsage>),
    TransitionAssignmentActionUsage(Box<TransitionAssignmentActionUsage>),
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

/// `ViewDefinitionBodyItem`
#[derive(Debug, Clone)]
pub enum ViewDefinitionBodyItemOwnedRelationshipMember {
    ElementFilterMember(Box<ElementFilterMember>),
    ViewRenderingMember(Box<ViewRenderingMember>),
}

#[derive(Debug, Clone)]
pub struct ViewDefinitionBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<ViewDefinitionBodyItemOwnedRelationshipMember>,
}

impl AstNode for ViewDefinitionBodyItem {
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

/// `SequenceExpression`
#[derive(Debug, Clone)]
pub struct SequenceExpression {
    pub span: Span,
}

impl AstNode for SequenceExpression {
    fn span(&self) -> Span { self.span }
}

/// `ConstraintDefinition`
#[derive(Debug, Clone)]
pub struct ConstraintDefinition {
    pub span: Span,
}

impl AstNode for ConstraintDefinition {
    fn span(&self) -> Span { self.span }
}

/// `TriggerActionMember`
#[derive(Debug, Clone)]
pub struct TriggerActionMember {
    pub span: Span,
    pub owned_related_element: Vec<TriggerAction>,
}

impl AstNode for TriggerActionMember {
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

/// `OwnedRedefinition`
#[derive(Debug, Clone)]
pub enum OwnedRedefinitionRedefinedFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedRedefinition {
    pub span: Span,
    pub redefined_feature: Option<OwnedRedefinitionRedefinedFeatureMember>,
}

impl AstNode for OwnedRedefinition {
    fn span(&self) -> Span { self.span }
}

/// `EmptyMultiplicity`
#[derive(Debug, Clone)]
pub struct EmptyMultiplicity {
    pub span: Span,
}

impl AstNode for EmptyMultiplicity {
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

/// `InterfaceNonOccurrenceUsageElement`
#[derive(Debug, Clone)]
pub enum InterfaceNonOccurrenceUsageElement {
    ReferenceUsage(Box<ReferenceUsage>),
    AttributeUsage(Box<AttributeUsage>),
    EnumerationUsage(Box<EnumerationUsage>),
    BindingConnectorAsUsage(Box<BindingConnectorAsUsage>),
    SuccessionAsUsage(Box<SuccessionAsUsage>),
}

/// `Metaclass`
#[derive(Debug, Clone)]
pub struct Metaclass {
    pub span: Span,
}

impl AstNode for Metaclass {
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

/// `OwnedSpecialization`
#[derive(Debug, Clone)]
pub struct OwnedSpecialization {
    pub span: Span,
}

impl AstNode for OwnedSpecialization {
    fn span(&self) -> Span { self.span }
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

/// `TypeResultMember`
#[derive(Debug, Clone)]
pub struct TypeResultMember {
    pub span: Span,
    pub owned_member_feature: Box<TypeReference>,
}

impl AstNode for TypeResultMember {
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

/// `AssertConstraintUsage`
#[derive(Debug, Clone)]
pub struct AssertConstraintUsage {
    pub span: Span,
    pub is_negated: bool,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for AssertConstraintUsage {
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

/// `SourceSuccession`
#[derive(Debug, Clone)]
pub struct SourceSuccession {
    pub span: Span,
    pub owned_relationship: Vec<SourceEndMember>,
}

impl AstNode for SourceSuccession {
    fn span(&self) -> Span { self.span }
}

/// `ActorMember`
#[derive(Debug, Clone)]
pub struct ActorMember {
    pub span: Span,
    pub owned_related_element: Vec<ActorUsage>,
}

impl AstNode for ActorMember {
    fn span(&self) -> Span { self.span }
}

/// `ViewRenderingUsage`
#[derive(Debug, Clone)]
pub struct ViewRenderingUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for ViewRenderingUsage {
    fn span(&self) -> Span { self.span }
}

/// `UsagePrefix`
#[derive(Debug, Clone)]
pub struct UsagePrefix {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for UsagePrefix {
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

/// `ExtendedDefinition`
#[derive(Debug, Clone)]
pub struct ExtendedDefinition {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
}

impl AstNode for ExtendedDefinition {
    fn span(&self) -> Span { self.span }
}

/// `OccurrenceUsageElement`
#[derive(Debug, Clone)]
pub enum OccurrenceUsageElement {
    StructureUsageElement(Box<StructureUsageElement>),
    BehaviorUsageElement(Box<BehaviorUsageElement>),
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

/// `Behavior`
#[derive(Debug, Clone)]
pub struct Behavior {
    pub span: Span,
}

impl AstNode for Behavior {
    fn span(&self) -> Span { self.span }
}

/// `InitialNodeMember`
#[derive(Debug, Clone)]
pub struct InitialNodeMember {
    pub span: Span,
    pub member_feature: QualifiedNameRef,
}

impl AstNode for InitialNodeMember {
    fn span(&self) -> Span { self.span }
}

/// `PortionUsage`
#[derive(Debug, Clone)]
pub struct PortionUsage {
    pub span: Span,
    pub is_individual: bool,
    pub portion_kind: Box<PortionKind>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for PortionUsage {
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

/// `PrimaryArgumentMember`
#[derive(Debug, Clone)]
pub struct PrimaryArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<PrimaryArgument>,
}

impl AstNode for PrimaryArgumentMember {
    fn span(&self) -> Span { self.span }
}

/// `NonOccurrenceUsageElement`
#[derive(Debug, Clone)]
pub enum NonOccurrenceUsageElement {
    DefaultReferenceUsage(Box<DefaultReferenceUsage>),
    ReferenceUsage(Box<ReferenceUsage>),
    AttributeUsage(Box<AttributeUsage>),
    EnumerationUsage(Box<EnumerationUsage>),
    BindingConnectorAsUsage(Box<BindingConnectorAsUsage>),
    SuccessionAsUsage(Box<SuccessionAsUsage>),
    ExtendedUsage(Box<ExtendedUsage>),
}

/// `TransitionSuccessionMember`
#[derive(Debug, Clone)]
pub struct TransitionSuccessionMember {
    pub span: Span,
    pub owned_related_element: Vec<TransitionSuccession>,
}

impl AstNode for TransitionSuccessionMember {
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

/// `AllocationUsage`
#[derive(Debug, Clone)]
pub struct AllocationUsage {
    pub span: Span,
}

impl AstNode for AllocationUsage {
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

/// `EnumerationDefinition`
#[derive(Debug, Clone)]
pub struct EnumerationDefinition {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
}

impl AstNode for EnumerationDefinition {
    fn span(&self) -> Span { self.span }
}

/// `Type`
#[derive(Debug, Clone)]
pub struct Type {
    pub span: Span,
}

impl AstNode for Type {
    fn span(&self) -> Span { self.span }
}

/// `StatePerformActionUsage`
#[derive(Debug, Clone)]
pub struct StatePerformActionUsage {
    pub span: Span,
}

impl AstNode for StatePerformActionUsage {
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

/// `MessageEvent`
#[derive(Debug, Clone)]
pub struct MessageEvent {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for MessageEvent {
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

/// `CalculationBodyPart`
#[derive(Debug, Clone)]
pub struct CalculationBodyPart {
    pub span: Span,
    pub calculation_body_item: Vec<CalculationBodyItem>,
    pub owned_relationship: Vec<ResultExpressionMember>,
}

impl AstNode for CalculationBodyPart {
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

/// `ResultExpressionMember`
#[derive(Debug, Clone)]
pub struct ResultExpressionMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for ResultExpressionMember {
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

/// `ActionBodyParameterMember`
#[derive(Debug, Clone)]
pub struct ActionBodyParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<ActionBodyParameter>,
}

impl AstNode for ActionBodyParameterMember {
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

/// `NonFeatureChainPrimaryArgumentMember`
#[derive(Debug, Clone)]
pub struct NonFeatureChainPrimaryArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<PrimaryArgument>,
}

impl AstNode for NonFeatureChainPrimaryArgumentMember {
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

/// `VariantUsageMember`
#[derive(Debug, Clone)]
pub struct VariantUsageMember {
    pub span: Span,
    pub owned_variant_usage: Box<VariantUsageElement>,
}

impl AstNode for VariantUsageMember {
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

/// `FunctionOperationExpression`
#[derive(Debug, Clone)]
pub enum FunctionOperationExpressionOwnedRelationshipMember {
    BodyArgumentMember(Box<BodyArgumentMember>),
    EmptyResultMember(Box<EmptyResultMember>),
    FunctionReferenceArgumentMember(Box<FunctionReferenceArgumentMember>),
    InvocationTypeMember(Box<InvocationTypeMember>),
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

/// `BooleanValue`
#[derive(Debug, Clone)]
pub struct BooleanValue {
    pub span: Span,
}

impl AstNode for BooleanValue {
    fn span(&self) -> Span { self.span }
}

/// `RequirementVerificationMember`
#[derive(Debug, Clone)]
pub struct RequirementVerificationMember {
    pub span: Span,
    pub owned_related_element: Vec<RequirementVerificationUsage>,
}

impl AstNode for RequirementVerificationMember {
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

/// `ActionNodePrefix`
#[derive(Debug, Clone)]
pub struct ActionNodePrefix {
    pub span: Span,
}

impl AstNode for ActionNodePrefix {
    fn span(&self) -> Span { self.span }
}

/// `SenderReceiverPart`
#[derive(Debug, Clone)]
pub enum SenderReceiverPartOwnedRelationshipMember {
    EmptyParameterMember(Box<EmptyParameterMember>),
    NodeParameterMember(Box<NodeParameterMember>),
}

#[derive(Debug, Clone)]
pub struct SenderReceiverPart {
    pub span: Span,
    pub owned_relationship: Vec<SenderReceiverPartOwnedRelationshipMember>,
}

impl AstNode for SenderReceiverPart {
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

/// `RealValue`
#[derive(Debug, Clone)]
pub struct RealValue {
    pub span: Span,
}

impl AstNode for RealValue {
    fn span(&self) -> Span { self.span }
}

/// `DefinitionElement`
#[derive(Debug, Clone)]
pub enum DefinitionElement {
    Package(Box<Package>),
    LibraryPackage(Box<LibraryPackage>),
    AnnotatingElement(Box<AnnotatingElement>),
    Dependency(Box<Dependency>),
    AttributeDefinition(Box<AttributeDefinition>),
    EnumerationDefinition(Box<EnumerationDefinition>),
    OccurrenceDefinition(Box<OccurrenceDefinition>),
    IndividualDefinition(Box<IndividualDefinition>),
    ItemDefinition(Box<ItemDefinition>),
    PartDefinition(Box<PartDefinition>),
    ConnectionDefinition(Box<ConnectionDefinition>),
    FlowDefinition(Box<FlowDefinition>),
    InterfaceDefinition(Box<InterfaceDefinition>),
    PortDefinition(Box<PortDefinition>),
    ActionDefinition(Box<ActionDefinition>),
    CalculationDefinition(Box<CalculationDefinition>),
    StateDefinition(Box<StateDefinition>),
    ConstraintDefinition(Box<ConstraintDefinition>),
    RequirementDefinition(Box<RequirementDefinition>),
    ConcernDefinition(Box<ConcernDefinition>),
    CaseDefinition(Box<CaseDefinition>),
    AnalysisCaseDefinition(Box<AnalysisCaseDefinition>),
    VerificationCaseDefinition(Box<VerificationCaseDefinition>),
    UseCaseDefinition(Box<UseCaseDefinition>),
    ViewDefinition(Box<ViewDefinition>),
    ViewpointDefinition(Box<ViewpointDefinition>),
    RenderingDefinition(Box<RenderingDefinition>),
    MetadataDefinition(Box<MetadataDefinition>),
    ExtendedDefinition(Box<ExtendedDefinition>),
}

/// `Connector`
#[derive(Debug, Clone)]
pub struct Connector {
    pub span: Span,
}

impl AstNode for Connector {
    fn span(&self) -> Span { self.span }
}

/// `VerificationCaseUsage`
#[derive(Debug, Clone)]
pub struct VerificationCaseUsage {
    pub span: Span,
}

impl AstNode for VerificationCaseUsage {
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

/// `Crosses`
#[derive(Debug, Clone)]
pub struct Crosses {
    pub span: Span,
    pub owned_relationship: Vec<OwnedCrossSubsetting>,
}

impl AstNode for Crosses {
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

/// `NonFeatureChainPrimaryArgumentValue`
#[derive(Debug, Clone)]
pub struct NonFeatureChainPrimaryArgumentValue {
    pub span: Span,
    pub value: Box<NonFeatureChainPrimaryExpression>,
}

impl AstNode for NonFeatureChainPrimaryArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `ExhibitStateUsage`
#[derive(Debug, Clone)]
pub struct ExhibitStateUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for ExhibitStateUsage {
    fn span(&self) -> Span { self.span }
}

/// `NonOccurrenceUsageMember`
#[derive(Debug, Clone)]
pub struct NonOccurrenceUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<NonOccurrenceUsageElement>,
}

impl AstNode for NonOccurrenceUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `ExtendedUsage`
#[derive(Debug, Clone)]
pub struct ExtendedUsage {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for ExtendedUsage {
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

/// `InterfaceNonOccurrenceUsageMember`
#[derive(Debug, Clone)]
pub struct InterfaceNonOccurrenceUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<InterfaceNonOccurrenceUsageElement>,
}

impl AstNode for InterfaceNonOccurrenceUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `DoActionMember`
#[derive(Debug, Clone)]
pub struct DoActionMember {
    pub span: Span,
    pub kind: bool,
    pub owned_related_element: Vec<StateActionUsage>,
}

impl AstNode for DoActionMember {
    fn span(&self) -> Span { self.span }
}

/// `ConjugatedPortDefinition`
#[derive(Debug, Clone)]
pub struct ConjugatedPortDefinition {
    pub span: Span,
    pub owned_relationship: Vec<PortConjugation>,
}

impl AstNode for ConjugatedPortDefinition {
    fn span(&self) -> Span { self.span }
}

/// `DefaultReferenceUsage`
#[derive(Debug, Clone)]
pub struct DefaultReferenceUsage {
    pub span: Span,
}

impl AstNode for DefaultReferenceUsage {
    fn span(&self) -> Span { self.span }
}

/// `NonBehaviorBodyItem`
#[derive(Debug, Clone)]
pub enum NonBehaviorBodyItemOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    DefinitionMember(Box<DefinitionMember>),
    Import(Box<Import>),
    NonOccurrenceUsageMember(Box<NonOccurrenceUsageMember>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
    StructureUsageMember(Box<StructureUsageMember>),
    VariantUsageMember(Box<VariantUsageMember>),
}

#[derive(Debug, Clone)]
pub struct NonBehaviorBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<NonBehaviorBodyItemOwnedRelationshipMember>,
}

impl AstNode for NonBehaviorBodyItem {
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

/// `JoinNode`
#[derive(Debug, Clone)]
pub struct JoinNode {
    pub span: Span,
    pub is_composite: bool,
}

impl AstNode for JoinNode {
    fn span(&self) -> Span { self.span }
}

/// `ActionNodeMember`
#[derive(Debug, Clone)]
pub struct ActionNodeMember {
    pub span: Span,
    pub owned_related_element: Vec<ActionNode>,
}

impl AstNode for ActionNodeMember {
    fn span(&self) -> Span { self.span }
}

/// `TransitionSuccession`
#[derive(Debug, Clone)]
pub enum TransitionSuccessionOwnedRelationshipMember {
    ConnectorEndMember(Box<ConnectorEndMember>),
    EmptyEndMember(Box<EmptyEndMember>),
}

#[derive(Debug, Clone)]
pub struct TransitionSuccession {
    pub span: Span,
    pub owned_relationship: Vec<TransitionSuccessionOwnedRelationshipMember>,
}

impl AstNode for TransitionSuccession {
    fn span(&self) -> Span { self.span }
}

/// `SuccessionFlowUsage`
#[derive(Debug, Clone)]
pub struct SuccessionFlowUsage {
    pub span: Span,
}

impl AstNode for SuccessionFlowUsage {
    fn span(&self) -> Span { self.span }
}

/// `ViewRenderingMember`
#[derive(Debug, Clone)]
pub struct ViewRenderingMember {
    pub span: Span,
    pub owned_related_element: Vec<ViewRenderingUsage>,
}

impl AstNode for ViewRenderingMember {
    fn span(&self) -> Span { self.span }
}

/// `EmptyParameterMember`
#[derive(Debug, Clone)]
pub struct EmptyParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<EmptyUsage>,
}

impl AstNode for EmptyParameterMember {
    fn span(&self) -> Span { self.span }
}

/// `Multiplicity`
#[derive(Debug, Clone)]
pub enum Multiplicity {
    MultiplicitySubset(Box<MultiplicitySubset>),
    MultiplicityRange(Box<MultiplicityRange>),
}

/// `EnumeratedValue`
#[derive(Debug, Clone)]
pub struct EnumeratedValue {
    pub span: Span,
}

impl AstNode for EnumeratedValue {
    fn span(&self) -> Span { self.span }
}

/// `StakeholderUsage`
#[derive(Debug, Clone)]
pub struct StakeholderUsage {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for StakeholderUsage {
    fn span(&self) -> Span { self.span }
}

/// `TransitionPerformActionUsage`
#[derive(Debug, Clone)]
pub struct TransitionPerformActionUsage {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for TransitionPerformActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `BasicDefinitionPrefix`
#[derive(Debug, Clone)]
pub struct BasicDefinitionPrefix {
    pub span: Span,
    pub is_abstract: bool,
    pub is_variation: bool,
}

impl AstNode for BasicDefinitionPrefix {
    fn span(&self) -> Span { self.span }
}

/// `UsageCompletion`
#[derive(Debug, Clone)]
pub struct UsageCompletion {
    pub span: Span,
}

impl AstNode for UsageCompletion {
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

/// `PartDefinition`
#[derive(Debug, Clone)]
pub struct PartDefinition {
    pub span: Span,
}

impl AstNode for PartDefinition {
    fn span(&self) -> Span { self.span }
}

/// `RequirementBodyItem`
#[derive(Debug, Clone)]
pub enum RequirementBodyItemOwnedRelationshipMember {
    ActorMember(Box<ActorMember>),
    FramedConcernMember(Box<FramedConcernMember>),
    RequirementConstraintMember(Box<RequirementConstraintMember>),
    RequirementVerificationMember(Box<RequirementVerificationMember>),
    StakeholderMember(Box<StakeholderMember>),
    SubjectMember(Box<SubjectMember>),
}

#[derive(Debug, Clone)]
pub struct RequirementBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<RequirementBodyItemOwnedRelationshipMember>,
}

impl AstNode for RequirementBodyItem {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBodyUsage`
#[derive(Debug, Clone)]
pub struct MetadataBodyUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for MetadataBodyUsage {
    fn span(&self) -> Span { self.span }
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

/// `InterfacePart`
#[derive(Debug, Clone)]
pub enum InterfacePart {
    BinaryInterfacePart(Box<BinaryInterfacePart>),
    NaryInterfacePart(Box<NaryInterfacePart>),
}

/// `TransitionAcceptActionUsage`
#[derive(Debug, Clone)]
pub struct TransitionAcceptActionUsage {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for TransitionAcceptActionUsage {
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

/// `BindingConnectorAsUsage`
#[derive(Debug, Clone)]
pub struct BindingConnectorAsUsage {
    pub span: Span,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for BindingConnectorAsUsage {
    fn span(&self) -> Span { self.span }
}

/// `UsageDeclaration`
#[derive(Debug, Clone)]
pub struct UsageDeclaration {
    pub span: Span,
}

impl AstNode for UsageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `ControlNodePrefix`
#[derive(Debug, Clone)]
pub struct ControlNodePrefix {
    pub span: Span,
    pub is_individual: bool,
    pub portion_kind: Option<Box<PortionKind>>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for ControlNodePrefix {
    fn span(&self) -> Span { self.span }
}

/// `CaseBodyItem`
#[derive(Debug, Clone)]
pub enum CaseBodyItemOwnedRelationshipMember {
    ActorMember(Box<ActorMember>),
    ObjectiveMember(Box<ObjectiveMember>),
    SubjectMember(Box<SubjectMember>),
}

#[derive(Debug, Clone)]
pub struct CaseBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<CaseBodyItemOwnedRelationshipMember>,
}

impl AstNode for CaseBodyItem {
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

/// `UseCaseUsage`
#[derive(Debug, Clone)]
pub struct UseCaseUsage {
    pub span: Span,
}

impl AstNode for UseCaseUsage {
    fn span(&self) -> Span { self.span }
}

/// `ItemDefinition`
#[derive(Debug, Clone)]
pub struct ItemDefinition {
    pub span: Span,
}

impl AstNode for ItemDefinition {
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

/// `StructureUsageMember`
#[derive(Debug, Clone)]
pub struct StructureUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<StructureUsageElement>,
}

impl AstNode for StructureUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `ElementFilterMember`
#[derive(Debug, Clone)]
pub struct ElementFilterMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for ElementFilterMember {
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

/// `FramedConcernUsage`
#[derive(Debug, Clone)]
pub struct FramedConcernUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for FramedConcernUsage {
    fn span(&self) -> Span { self.span }
}

/// `InterfaceUsageDeclaration`
#[derive(Debug, Clone)]
pub struct InterfaceUsageDeclaration {
    pub span: Span,
}

impl AstNode for InterfaceUsageDeclaration {
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

/// `Step`
#[derive(Debug, Clone)]
pub struct Step {
    pub span: Span,
}

impl AstNode for Step {
    fn span(&self) -> Span { self.span }
}

/// `PerformActionUsage`
#[derive(Debug, Clone)]
pub struct PerformActionUsage {
    pub span: Span,
}

impl AstNode for PerformActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `MetadataUsage`
#[derive(Debug, Clone)]
pub struct MetadataUsage {
    pub span: Span,
    pub owned_relationship: Vec<Annotation>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for MetadataUsage {
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

/// `FlowEnd`
#[derive(Debug, Clone)]
pub enum FlowEndOwnedRelationshipMember {
    FlowEndSubsetting(Box<FlowEndSubsetting>),
    FlowFeatureMember(Box<FlowFeatureMember>),
}

#[derive(Debug, Clone)]
pub struct FlowEnd {
    pub span: Span,
    pub owned_relationship: Vec<FlowEndOwnedRelationshipMember>,
}

impl AstNode for FlowEnd {
    fn span(&self) -> Span { self.span }
}

/// `RenderingUsage`
#[derive(Debug, Clone)]
pub struct RenderingUsage {
    pub span: Span,
}

impl AstNode for RenderingUsage {
    fn span(&self) -> Span { self.span }
}

/// `OwnedReferenceSubsetting`
#[derive(Debug, Clone)]
pub enum OwnedReferenceSubsettingReferencedFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedReferenceSubsetting {
    pub span: Span,
    pub referenced_feature: Option<OwnedReferenceSubsettingReferencedFeatureMember>,
}

impl AstNode for OwnedReferenceSubsetting {
    fn span(&self) -> Span { self.span }
}

/// `StateAcceptActionUsage`
#[derive(Debug, Clone)]
pub struct StateAcceptActionUsage {
    pub span: Span,
}

impl AstNode for StateAcceptActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `ViewDefinition`
#[derive(Debug, Clone)]
pub struct ViewDefinition {
    pub span: Span,
}

impl AstNode for ViewDefinition {
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

/// `BodyArgumentValue`
#[derive(Debug, Clone)]
pub struct BodyArgumentValue {
    pub span: Span,
    pub value: Box<BodyExpression>,
}

impl AstNode for BodyArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `BehaviorUsageMember`
#[derive(Debug, Clone)]
pub struct BehaviorUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<BehaviorUsageElement>,
}

impl AstNode for BehaviorUsageMember {
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

/// `EntryTransitionMember`
#[derive(Debug, Clone)]
pub enum EntryTransitionMemberOwnedRelatedElementMember {
    GuardedTargetSuccession(Box<GuardedTargetSuccession>),
    TargetSuccession(Box<TargetSuccession>),
}

#[derive(Debug, Clone)]
pub struct EntryTransitionMember {
    pub span: Span,
    pub owned_related_element: Vec<EntryTransitionMemberOwnedRelatedElementMember>,
}

impl AstNode for EntryTransitionMember {
    fn span(&self) -> Span { self.span }
}

/// `AcceptNodeDeclaration`
#[derive(Debug, Clone)]
pub struct AcceptNodeDeclaration {
    pub span: Span,
}

impl AstNode for AcceptNodeDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `FeatureBinding`
#[derive(Debug, Clone)]
pub struct FeatureBinding {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for FeatureBinding {
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

/// `BinaryOperator`
#[derive(Debug, Clone)]
pub struct BinaryOperator {
    pub span: Span,
}

impl AstNode for BinaryOperator {
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

/// `EmptyFeature`
#[derive(Debug, Clone)]
pub struct EmptyFeature {
    pub span: Span,
}

impl AstNode for EmptyFeature {
    fn span(&self) -> Span { self.span }
}

/// `RenderingDefinition`
#[derive(Debug, Clone)]
pub struct RenderingDefinition {
    pub span: Span,
}

impl AstNode for RenderingDefinition {
    fn span(&self) -> Span { self.span }
}

/// `CalculationDefinition`
#[derive(Debug, Clone)]
pub struct CalculationDefinition {
    pub span: Span,
}

impl AstNode for CalculationDefinition {
    fn span(&self) -> Span { self.span }
}

/// `TerminateNode`
#[derive(Debug, Clone)]
pub struct TerminateNode {
    pub span: Span,
    pub owned_relationship: Vec<NodeParameterMember>,
}

impl AstNode for TerminateNode {
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

/// `PayloadParameterMember`
#[derive(Debug, Clone)]
pub struct PayloadParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<PayloadParameter>,
}

impl AstNode for PayloadParameterMember {
    fn span(&self) -> Span { self.span }
}

/// `OccurrenceUsageMember`
#[derive(Debug, Clone)]
pub struct OccurrenceUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<OccurrenceUsageElement>,
}

impl AstNode for OccurrenceUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `UsageExtensionKeyword`
#[derive(Debug, Clone)]
pub struct UsageExtensionKeyword {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for UsageExtensionKeyword {
    fn span(&self) -> Span { self.span }
}

/// `ConjugatedPortDefinitionMember`
#[derive(Debug, Clone)]
pub struct ConjugatedPortDefinitionMember {
    pub span: Span,
    pub owned_related_element: Vec<ConjugatedPortDefinition>,
}

impl AstNode for ConjugatedPortDefinitionMember {
    fn span(&self) -> Span { self.span }
}

/// `TriggerAction`
#[derive(Debug, Clone)]
pub struct TriggerAction {
    pub span: Span,
}

impl AstNode for TriggerAction {
    fn span(&self) -> Span { self.span }
}

/// `PrefixMetadataMember`
#[derive(Debug, Clone)]
pub struct PrefixMetadataMember {
    pub span: Span,
    pub owned_related_element: Box<PrefixMetadataUsage>,
}

impl AstNode for PrefixMetadataMember {
    fn span(&self) -> Span { self.span }
}

/// `ViewBody`
#[derive(Debug, Clone)]
pub struct ViewBody {
    pub span: Span,
    pub view_body_item: Vec<ViewBodyItem>,
}

impl AstNode for ViewBody {
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

/// `DefinitionDeclaration`
#[derive(Debug, Clone)]
pub struct DefinitionDeclaration {
    pub span: Span,
}

impl AstNode for DefinitionDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `SourceSuccessionMember`
#[derive(Debug, Clone)]
pub struct SourceSuccessionMember {
    pub span: Span,
    pub owned_related_element: Vec<SourceSuccession>,
}

impl AstNode for SourceSuccessionMember {
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

/// `OccurrenceUsage`
#[derive(Debug, Clone)]
pub struct OccurrenceUsage {
    pub span: Span,
}

impl AstNode for OccurrenceUsage {
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

/// `AssignmentNodeDeclaration`
#[derive(Debug, Clone)]
pub enum AssignmentNodeDeclarationOwnedRelationshipMember {
    AssignmentTargetMember(Box<AssignmentTargetMember>),
    FeatureChainMember(Box<FeatureChainMember>),
    NodeParameterMember(Box<NodeParameterMember>),
}

#[derive(Debug, Clone)]
pub struct AssignmentNodeDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<AssignmentNodeDeclarationOwnedRelationshipMember>,
}

impl AstNode for AssignmentNodeDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `ConcernUsage`
#[derive(Debug, Clone)]
pub struct ConcernUsage {
    pub span: Span,
}

impl AstNode for ConcernUsage {
    fn span(&self) -> Span { self.span }
}

/// `StateAssignmentActionUsage`
#[derive(Debug, Clone)]
pub struct StateAssignmentActionUsage {
    pub span: Span,
}

impl AstNode for StateAssignmentActionUsage {
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

/// `MetadataDefinition`
#[derive(Debug, Clone)]
pub struct MetadataDefinition {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
    pub is_abstract: bool,
}

impl AstNode for MetadataDefinition {
    fn span(&self) -> Span { self.span }
}

/// `ActionDefinition`
#[derive(Debug, Clone)]
pub struct ActionDefinition {
    pub span: Span,
}

impl AstNode for ActionDefinition {
    fn span(&self) -> Span { self.span }
}

/// `IfNode`
#[derive(Debug, Clone)]
pub enum IfNodeOwnedRelationshipMember {
    ActionBodyParameterMember(Box<ActionBodyParameterMember>),
    ExpressionParameterMember(Box<ExpressionParameterMember>),
}

#[derive(Debug, Clone)]
pub struct IfNode {
    pub span: Span,
    pub owned_relationship: Vec<IfNodeOwnedRelationshipMember>,
}

impl AstNode for IfNode {
    fn span(&self) -> Span { self.span }
}

/// `NaryConnectorPart`
#[derive(Debug, Clone)]
pub struct NaryConnectorPart {
    pub span: Span,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for NaryConnectorPart {
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

/// `BooleanExpression`
#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub span: Span,
}

impl AstNode for BooleanExpression {
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

/// `RequirementDefinition`
#[derive(Debug, Clone)]
pub struct RequirementDefinition {
    pub span: Span,
}

impl AstNode for RequirementDefinition {
    fn span(&self) -> Span { self.span }
}

/// `GuardedTargetSuccession`
#[derive(Debug, Clone)]
pub enum GuardedTargetSuccessionOwnedRelationshipMember {
    GuardExpressionMember(Box<GuardExpressionMember>),
    TransitionSuccessionMember(Box<TransitionSuccessionMember>),
}

#[derive(Debug, Clone)]
pub struct GuardedTargetSuccession {
    pub span: Span,
    pub owned_relationship: Vec<GuardedTargetSuccessionOwnedRelationshipMember>,
}

impl AstNode for GuardedTargetSuccession {
    fn span(&self) -> Span { self.span }
}

/// `PackageBody`
#[derive(Debug, Clone)]
pub struct PackageBody {
    pub span: Span,
    pub package_body_element: Vec<PackageBodyElement>,
}

impl AstNode for PackageBody {
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

/// `DefinitionPrefix`
#[derive(Debug, Clone)]
pub struct DefinitionPrefix {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
}

impl AstNode for DefinitionPrefix {
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

/// `BinaryInterfacePart`
#[derive(Debug, Clone)]
pub struct BinaryInterfacePart {
    pub span: Span,
    pub owned_relationship: Vec<InterfaceEndMember>,
}

impl AstNode for BinaryInterfacePart {
    fn span(&self) -> Span { self.span }
}

/// `ActionTargetSuccessionMember`
#[derive(Debug, Clone)]
pub struct ActionTargetSuccessionMember {
    pub span: Span,
    pub owned_related_element: Vec<ActionTargetSuccession>,
}

impl AstNode for ActionTargetSuccessionMember {
    fn span(&self) -> Span { self.span }
}

/// `SatisfactionSubjectMember`
#[derive(Debug, Clone)]
pub struct SatisfactionSubjectMember {
    pub span: Span,
    pub owned_related_element: Vec<SatisfactionParameter>,
}

impl AstNode for SatisfactionSubjectMember {
    fn span(&self) -> Span { self.span }
}

/// `MessageEventMember`
#[derive(Debug, Clone)]
pub struct MessageEventMember {
    pub span: Span,
    pub owned_related_element: Vec<MessageEvent>,
}

impl AstNode for MessageEventMember {
    fn span(&self) -> Span { self.span }
}

/// `ActionNodeUsageDeclaration`
#[derive(Debug, Clone)]
pub struct ActionNodeUsageDeclaration {
    pub span: Span,
}

impl AstNode for ActionNodeUsageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `InterfaceEnd`
#[derive(Debug, Clone)]
pub enum InterfaceEndOwnedRelationshipMember {
    OwnedCrossMultiplicityMember(Box<OwnedCrossMultiplicityMember>),
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
}

#[derive(Debug, Clone)]
pub struct InterfaceEnd {
    pub span: Span,
    pub declared_name: Option<String>,
    pub owned_relationship: Vec<InterfaceEndOwnedRelationshipMember>,
}

impl AstNode for InterfaceEnd {
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

/// `PayloadFeatureMember`
#[derive(Debug, Clone)]
pub struct PayloadFeatureMember {
    pub span: Span,
    pub owned_related_element: Box<PayloadFeature>,
}

impl AstNode for PayloadFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `FlowDeclaration`
#[derive(Debug, Clone)]
pub enum FlowDeclarationOwnedRelationshipMember {
    FlowEndMember(Box<FlowEndMember>),
    FlowPayloadFeatureMember(Box<FlowPayloadFeatureMember>),
}

#[derive(Debug, Clone)]
pub struct FlowDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<FlowDeclarationOwnedRelationshipMember>,
}

impl AstNode for FlowDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `StateUsage`
#[derive(Debug, Clone)]
pub struct StateUsage {
    pub span: Span,
}

impl AstNode for StateUsage {
    fn span(&self) -> Span { self.span }
}

/// `OwnedSubclassification`
#[derive(Debug, Clone)]
pub struct OwnedSubclassification {
    pub span: Span,
    pub super_classifier: QualifiedNameRef,
}

impl AstNode for OwnedSubclassification {
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

/// `EmptyActionUsage`
#[derive(Debug, Clone)]
pub struct EmptyActionUsage {
    pub span: Span,
}

impl AstNode for EmptyActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `RequirementConstraintMember`
#[derive(Debug, Clone)]
pub struct RequirementConstraintMember {
    pub span: Span,
    pub owned_related_element: Vec<RequirementConstraintUsage>,
}

impl AstNode for RequirementConstraintMember {
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

/// `Argument`
#[derive(Debug, Clone)]
pub struct Argument {
    pub span: Span,
    pub owned_relationship: Vec<ArgumentValue>,
}

impl AstNode for Argument {
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

/// `UsageBody`
#[derive(Debug, Clone)]
pub struct UsageBody {
    pub span: Span,
}

impl AstNode for UsageBody {
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

/// `OccurrenceDefinitionPrefix`
#[derive(Debug, Clone)]
pub struct OccurrenceDefinitionPrefix {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
    pub is_individual: bool,
    pub owned_relationship: Vec<EmptyMultiplicityMember>,
}

impl AstNode for OccurrenceDefinitionPrefix {
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

/// `PayloadFeatureSpecializationPart`
#[derive(Debug, Clone)]
pub struct PayloadFeatureSpecializationPart {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
}

impl AstNode for PayloadFeatureSpecializationPart {
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

/// `OwnedCrossSubsetting`
#[derive(Debug, Clone)]
pub enum OwnedCrossSubsettingCrossedFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
}

#[derive(Debug, Clone)]
pub struct OwnedCrossSubsetting {
    pub span: Span,
    pub crossed_feature: Option<OwnedCrossSubsettingCrossedFeatureMember>,
}

impl AstNode for OwnedCrossSubsetting {
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

/// `PackageBodyElement`
#[derive(Debug, Clone)]
pub enum PackageBodyElementOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    ElementFilterMember(Box<ElementFilterMember>),
    Import(Box<Import>),
    PackageMember(Box<PackageMember>),
}

#[derive(Debug, Clone)]
pub struct PackageBodyElement {
    pub span: Span,
    pub owned_relationship: Vec<PackageBodyElementOwnedRelationshipMember>,
}

impl AstNode for PackageBodyElement {
    fn span(&self) -> Span { self.span }
}

/// `ViewpointDefinition`
#[derive(Debug, Clone)]
pub struct ViewpointDefinition {
    pub span: Span,
}

impl AstNode for ViewpointDefinition {
    fn span(&self) -> Span { self.span }
}

/// `TransitionAssignmentActionUsage`
#[derive(Debug, Clone)]
pub struct TransitionAssignmentActionUsage {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for TransitionAssignmentActionUsage {
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

/// `InterfaceOccurrenceUsageElement`
#[derive(Debug, Clone)]
pub enum InterfaceOccurrenceUsageElement {
    DefaultInterfaceEnd(Box<DefaultInterfaceEnd>),
    StructureUsageElement(Box<StructureUsageElement>),
    BehaviorUsageElement(Box<BehaviorUsageElement>),
}

/// `ViewBodyItem`
#[derive(Debug, Clone)]
pub enum ViewBodyItemOwnedRelationshipMember {
    ElementFilterMember(Box<ElementFilterMember>),
    Expose(Box<Expose>),
    ViewRenderingMember(Box<ViewRenderingMember>),
}

#[derive(Debug, Clone)]
pub struct ViewBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<ViewBodyItemOwnedRelationshipMember>,
}

impl AstNode for ViewBodyItem {
    fn span(&self) -> Span { self.span }
}

/// `RequirementBody`
#[derive(Debug, Clone)]
pub struct RequirementBody {
    pub span: Span,
    pub requirement_body_item: Vec<RequirementBodyItem>,
}

impl AstNode for RequirementBody {
    fn span(&self) -> Span { self.span }
}

/// `EventOccurrenceUsage`
#[derive(Debug, Clone)]
pub struct EventOccurrenceUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for EventOccurrenceUsage {
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

/// `UnextendedUsagePrefix`
#[derive(Debug, Clone)]
pub enum UnextendedUsagePrefix {
    EndUsagePrefix(Box<EndUsagePrefix>),
    BasicUsagePrefix(Box<BasicUsagePrefix>),
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

/// `AnnotatingMember`
#[derive(Debug, Clone)]
pub struct AnnotatingMember {
    pub span: Span,
    pub owned_related_element: Vec<AnnotatingElement>,
}

impl AstNode for AnnotatingMember {
    fn span(&self) -> Span { self.span }
}

/// `TransitionSendActionUsage`
#[derive(Debug, Clone)]
pub struct TransitionSendActionUsage {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for TransitionSendActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `StateBodyItem`
#[derive(Debug, Clone)]
pub enum StateBodyItemOwnedRelationshipMember {
    BehaviorUsageMember(Box<BehaviorUsageMember>),
    DoActionMember(Box<DoActionMember>),
    EntryActionMember(Box<EntryActionMember>),
    EntryTransitionMember(Box<EntryTransitionMember>),
    ExitActionMember(Box<ExitActionMember>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
    TargetTransitionUsageMember(Box<TargetTransitionUsageMember>),
    TransitionUsageMember(Box<TransitionUsageMember>),
}

#[derive(Debug, Clone)]
pub struct StateBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<StateBodyItemOwnedRelationshipMember>,
}

impl AstNode for StateBodyItem {
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

/// `EmptyEndMember`
#[derive(Debug, Clone)]
pub struct EmptyEndMember {
    pub span: Span,
    pub owned_related_element: Vec<EmptyFeature>,
}

impl AstNode for EmptyEndMember {
    fn span(&self) -> Span { self.span }
}

/// `PortionKind`
#[derive(Debug, Clone)]
pub struct PortionKind {
    pub span: Span,
}

impl AstNode for PortionKind {
    fn span(&self) -> Span { self.span }
}

/// `RequirementKind`
#[derive(Debug, Clone)]
pub struct RequirementKind {
    pub span: Span,
}

impl AstNode for RequirementKind {
    fn span(&self) -> Span { self.span }
}

/// `TriggerExpression`
#[derive(Debug, Clone)]
pub enum TriggerExpressionOwnedRelationshipMember {
    ArgumentExpressionMember(Box<ArgumentExpressionMember>),
    ArgumentMember(Box<ArgumentMember>),
}

#[derive(Debug, Clone)]
pub struct TriggerExpression {
    pub span: Span,
    pub kind: bool,
    pub owned_relationship: Vec<TriggerExpressionOwnedRelationshipMember>,
}

impl AstNode for TriggerExpression {
    fn span(&self) -> Span { self.span }
}

/// `ForLoopNode`
#[derive(Debug, Clone)]
pub enum ForLoopNodeOwnedRelationshipMember {
    ActionBodyParameterMember(Box<ActionBodyParameterMember>),
    ForVariableDeclarationMember(Box<ForVariableDeclarationMember>),
    NodeParameterMember(Box<NodeParameterMember>),
}

#[derive(Debug, Clone)]
pub struct ForLoopNode {
    pub span: Span,
    pub owned_relationship: Vec<ForLoopNodeOwnedRelationshipMember>,
}

impl AstNode for ForLoopNode {
    fn span(&self) -> Span { self.span }
}

/// `RefPrefix`
#[derive(Debug, Clone)]
pub struct RefPrefix {
    pub span: Span,
    pub direction: Option<Box<FeatureDirection>>,
    pub is_abstract: bool,
    pub is_constant: bool,
    pub is_derived: bool,
    pub is_variation: bool,
}

impl AstNode for RefPrefix {
    fn span(&self) -> Span { self.span }
}

/// `ActionNode`
#[derive(Debug, Clone)]
pub enum ActionNode {
    ControlNode(Box<ControlNode>),
    SendNode(Box<SendNode>),
    AcceptNode(Box<AcceptNode>),
    AssignmentNode(Box<AssignmentNode>),
    TerminateNode(Box<TerminateNode>),
    IfNode(Box<IfNode>),
    WhileLoopNode(Box<WhileLoopNode>),
    ForLoopNode(Box<ForLoopNode>),
}

/// `AttributeUsage`
#[derive(Debug, Clone)]
pub struct AttributeUsage {
    pub span: Span,
}

impl AstNode for AttributeUsage {
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

/// `FlowDefinition`
#[derive(Debug, Clone)]
pub struct FlowDefinition {
    pub span: Span,
}

impl AstNode for FlowDefinition {
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

/// `FeatureTyping`
#[derive(Debug, Clone)]
pub enum FeatureTyping {
    OwnedFeatureTyping(Box<OwnedFeatureTyping>),
    ConjugatedPortTyping(Box<ConjugatedPortTyping>),
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

/// `MetadataBodyFeature`
#[derive(Debug, Clone)]
pub struct MetadataBodyFeature {
    pub span: Span,
    pub owned_relationship: Vec<OwnedRedefinition>,
}

impl AstNode for MetadataBodyFeature {
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

/// `AssignmentTargetParameter`
#[derive(Debug, Clone)]
pub struct AssignmentTargetParameter {
    pub span: Span,
    pub owned_relationship: Vec<AssignmentTargetBinding>,
}

impl AstNode for AssignmentTargetParameter {
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

/// `SuccessionFlow`
#[derive(Debug, Clone)]
pub struct SuccessionFlow {
    pub span: Span,
}

impl AstNode for SuccessionFlow {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicitySubset`
#[derive(Debug, Clone)]
pub struct MultiplicitySubset {
    pub span: Span,
}

impl AstNode for MultiplicitySubset {
    fn span(&self) -> Span { self.span }
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

/// `PortConjugation`
#[derive(Debug, Clone)]
pub struct PortConjugation {
    pub span: Span,
}

impl AstNode for PortConjugation {
    fn span(&self) -> Span { self.span }
}

/// `ViewUsage`
#[derive(Debug, Clone)]
pub struct ViewUsage {
    pub span: Span,
}

impl AstNode for ViewUsage {
    fn span(&self) -> Span { self.span }
}

/// `SatisfyRequirementUsage`
#[derive(Debug, Clone)]
pub enum SatisfyRequirementUsageOwnedRelationshipMember {
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
    SatisfactionSubjectMember(Box<SatisfactionSubjectMember>),
}

#[derive(Debug, Clone)]
pub struct SatisfyRequirementUsage {
    pub span: Span,
    pub is_negated: bool,
    pub owned_relationship: Vec<SatisfyRequirementUsageOwnedRelationshipMember>,
}

impl AstNode for SatisfyRequirementUsage {
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

/// `FunctionReferenceArgument`
#[derive(Debug, Clone)]
pub struct FunctionReferenceArgument {
    pub span: Span,
    pub owned_relationship: Vec<FunctionReferenceArgumentValue>,
}

impl AstNode for FunctionReferenceArgument {
    fn span(&self) -> Span { self.span }
}

/// `EnumerationBody`
#[derive(Debug, Clone)]
pub enum EnumerationBodyOwnedRelationshipMember {
    AnnotatingMember(Box<AnnotatingMember>),
    EnumerationUsageMember(Box<EnumerationUsageMember>),
}

#[derive(Debug, Clone)]
pub struct EnumerationBody {
    pub span: Span,
    pub owned_relationship: Vec<EnumerationBodyOwnedRelationshipMember>,
}

impl AstNode for EnumerationBody {
    fn span(&self) -> Span { self.span }
}

/// `MetadataBody`
#[derive(Debug, Clone)]
pub enum MetadataBodyOwnedRelationshipMember {
    AliasMember(Box<AliasMember>),
    DefinitionMember(Box<DefinitionMember>),
    Import(Box<Import>),
    MetadataBodyUsageMember(Box<MetadataBodyUsageMember>),
}

#[derive(Debug, Clone)]
pub struct MetadataBody {
    pub span: Span,
    pub owned_relationship: Vec<MetadataBodyOwnedRelationshipMember>,
}

impl AstNode for MetadataBody {
    fn span(&self) -> Span { self.span }
}

/// `ReturnParameterMember`
#[derive(Debug, Clone)]
pub struct ReturnParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<UsageElement>,
}

impl AstNode for ReturnParameterMember {
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

/// `NamespaceBody`
#[derive(Debug, Clone)]
pub struct NamespaceBody {
    pub span: Span,
    pub namespace_body_element: Vec<NamespaceBodyElement>,
}

impl AstNode for NamespaceBody {
    fn span(&self) -> Span { self.span }
}

/// `CaseBody`
#[derive(Debug, Clone)]
pub struct CaseBody {
    pub span: Span,
    pub case_body_item: Vec<CaseBodyItem>,
    pub owned_relationship: Vec<ResultExpressionMember>,
}

impl AstNode for CaseBody {
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

/// `PackageMember`
#[derive(Debug, Clone)]
pub enum PackageMemberOwnedRelatedElementMember {
    DefinitionElement(Box<DefinitionElement>),
    UsageElement(Box<UsageElement>),
}

#[derive(Debug, Clone)]
pub struct PackageMember {
    pub span: Span,
    pub owned_related_element: Vec<PackageMemberOwnedRelatedElementMember>,
}

impl AstNode for PackageMember {
    fn span(&self) -> Span { self.span }
}

/// `InterfaceOccurrenceUsageMember`
#[derive(Debug, Clone)]
pub struct InterfaceOccurrenceUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<InterfaceOccurrenceUsageElement>,
}

impl AstNode for InterfaceOccurrenceUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `ActionBodyParameter`
#[derive(Debug, Clone)]
pub struct ActionBodyParameter {
    pub span: Span,
    pub action_body_item: Vec<ActionBodyItem>,
}

impl AstNode for ActionBodyParameter {
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

/// `SendNode`
#[derive(Debug, Clone)]
pub enum SendNodeOwnedRelationshipMember {
    EmptyParameterMember(Box<EmptyParameterMember>),
    NodeParameterMember(Box<NodeParameterMember>),
}

#[derive(Debug, Clone)]
pub struct SendNode {
    pub span: Span,
    pub owned_relationship: Vec<SendNodeOwnedRelationshipMember>,
}

impl AstNode for SendNode {
    fn span(&self) -> Span { self.span }
}

/// `ForVariableDeclaration`
#[derive(Debug, Clone)]
pub struct ForVariableDeclaration {
    pub span: Span,
}

impl AstNode for ForVariableDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `AnalysisCaseUsage`
#[derive(Debug, Clone)]
pub struct AnalysisCaseUsage {
    pub span: Span,
}

impl AstNode for AnalysisCaseUsage {
    fn span(&self) -> Span { self.span }
}

/// `DefinitionExtensionKeyword`
#[derive(Debug, Clone)]
pub struct DefinitionExtensionKeyword {
    pub span: Span,
    pub owned_relationship: Vec<PrefixMetadataMember>,
}

impl AstNode for DefinitionExtensionKeyword {
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

/// `OwnedFeatureChain`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChain {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureChaining>,
}

impl AstNode for OwnedFeatureChain {
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

/// `NullExpression`
#[derive(Debug, Clone)]
pub struct NullExpression {
    pub span: Span,
}

impl AstNode for NullExpression {
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

/// `FeatureReferenceMember`
#[derive(Debug, Clone)]
pub struct FeatureReferenceMember {
    pub span: Span,
    pub member_element: Box<FeatureReference>,
}

impl AstNode for FeatureReferenceMember {
    fn span(&self) -> Span { self.span }
}

/// `MultiplicityRange`
#[derive(Debug, Clone)]
pub struct MultiplicityRange {
    pub span: Span,
    pub owned_relationship: Vec<MultiplicityExpressionMember>,
}

impl AstNode for MultiplicityRange {
    fn span(&self) -> Span { self.span }
}

/// `EnumerationUsage`
#[derive(Debug, Clone)]
pub struct EnumerationUsage {
    pub span: Span,
}

impl AstNode for EnumerationUsage {
    fn span(&self) -> Span { self.span }
}

/// `PayloadParameter`
#[derive(Debug, Clone)]
pub struct PayloadParameter {
    pub span: Span,
}

impl AstNode for PayloadParameter {
    fn span(&self) -> Span { self.span }
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

/// `ParameterRedefinition`
#[derive(Debug, Clone)]
pub struct ParameterRedefinition {
    pub span: Span,
    pub redefined_feature: QualifiedNameRef,
}

impl AstNode for ParameterRedefinition {
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

/// `MergeNode`
#[derive(Debug, Clone)]
pub struct MergeNode {
    pub span: Span,
    pub is_composite: bool,
}

impl AstNode for MergeNode {
    fn span(&self) -> Span { self.span }
}

/// `MembershipExpose`
#[derive(Debug, Clone)]
pub struct MembershipExpose {
    pub span: Span,
}

impl AstNode for MembershipExpose {
    fn span(&self) -> Span { self.span }
}

/// `StateActionUsage`
#[derive(Debug, Clone)]
pub struct StateActionUsage {
    pub span: Span,
}

impl AstNode for StateActionUsage {
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

/// `AssignmentNode`
#[derive(Debug, Clone)]
pub struct AssignmentNode {
    pub span: Span,
}

impl AstNode for AssignmentNode {
    fn span(&self) -> Span { self.span }
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

/// `NodeParameter`
#[derive(Debug, Clone)]
pub struct NodeParameter {
    pub span: Span,
    pub owned_relationship: Vec<FeatureBinding>,
}

impl AstNode for NodeParameter {
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

/// `FlowPayloadFeatureMember`
#[derive(Debug, Clone)]
pub struct FlowPayloadFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FlowPayloadFeature>,
}

impl AstNode for FlowPayloadFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `CalculationUsage`
#[derive(Debug, Clone)]
pub struct CalculationUsage {
    pub span: Span,
}

impl AstNode for CalculationUsage {
    fn span(&self) -> Span { self.span }
}

/// `SubjectMember`
#[derive(Debug, Clone)]
pub struct SubjectMember {
    pub span: Span,
    pub owned_related_element: Vec<SubjectUsage>,
}

impl AstNode for SubjectMember {
    fn span(&self) -> Span { self.span }
}

/// `ArgumentExpressionValue`
#[derive(Debug, Clone)]
pub struct ArgumentExpressionValue {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpressionReference>,
}

impl AstNode for ArgumentExpressionValue {
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

/// `PrefixMetadataAnnotation`
#[derive(Debug, Clone)]
pub struct PrefixMetadataAnnotation {
    pub span: Span,
    pub annotating_element: Box<PrefixMetadataUsage>,
}

impl AstNode for PrefixMetadataAnnotation {
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

/// `SuccessionAsUsage`
#[derive(Debug, Clone)]
pub struct SuccessionAsUsage {
    pub span: Span,
    pub owned_relationship: Vec<ConnectorEndMember>,
}

impl AstNode for SuccessionAsUsage {
    fn span(&self) -> Span { self.span }
}

/// `PrefixMetadataUsage`
#[derive(Debug, Clone)]
pub struct PrefixMetadataUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureTyping>,
}

impl AstNode for PrefixMetadataUsage {
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

/// `IndividualDefinition`
#[derive(Debug, Clone)]
pub struct IndividualDefinition {
    pub span: Span,
    pub definition_extension_keyword: Vec<DefinitionExtensionKeyword>,
    pub is_individual: bool,
    pub owned_relationship: Vec<EmptyMultiplicityMember>,
}

impl AstNode for IndividualDefinition {
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

/// `InterfaceBody`
#[derive(Debug, Clone)]
pub struct InterfaceBody {
    pub span: Span,
    pub interface_body_item: Vec<InterfaceBodyItem>,
}

impl AstNode for InterfaceBody {
    fn span(&self) -> Span { self.span }
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

/// `FeatureChainPrefix`
#[derive(Debug, Clone)]
pub struct FeatureChainPrefix {
    pub span: Span,
    pub owned_relationship: Vec<OwnedFeatureChaining>,
}

impl AstNode for FeatureChainPrefix {
    fn span(&self) -> Span { self.span }
}

/// `RequirementUsage`
#[derive(Debug, Clone)]
pub struct RequirementUsage {
    pub span: Span,
}

impl AstNode for RequirementUsage {
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

/// `ReferenceUsage`
#[derive(Debug, Clone)]
pub struct ReferenceUsage {
    pub span: Span,
}

impl AstNode for ReferenceUsage {
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

/// `CaseDefinition`
#[derive(Debug, Clone)]
pub struct CaseDefinition {
    pub span: Span,
}

impl AstNode for CaseDefinition {
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

/// `Succession`
#[derive(Debug, Clone)]
pub struct Succession {
    pub span: Span,
}

impl AstNode for Succession {
    fn span(&self) -> Span { self.span }
}

/// `UsageElement`
#[derive(Debug, Clone)]
pub enum UsageElement {
    NonOccurrenceUsageElement(Box<NonOccurrenceUsageElement>),
    OccurrenceUsageElement(Box<OccurrenceUsageElement>),
}

/// `IncludeUseCaseUsage`
#[derive(Debug, Clone)]
pub struct IncludeUseCaseUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for IncludeUseCaseUsage {
    fn span(&self) -> Span { self.span }
}

/// `ConnectorPart`
#[derive(Debug, Clone)]
pub enum ConnectorPart {
    BinaryConnectorPart(Box<BinaryConnectorPart>),
    NaryConnectorPart(Box<NaryConnectorPart>),
}

/// `TargetTransitionUsageMember`
#[derive(Debug, Clone)]
pub struct TargetTransitionUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<TargetTransitionUsage>,
}

impl AstNode for TargetTransitionUsageMember {
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

/// `LiteralInteger`
#[derive(Debug, Clone)]
pub struct LiteralInteger {
    pub span: Span,
    pub value: String,
}

impl AstNode for LiteralInteger {
    fn span(&self) -> Span { self.span }
}

/// `InterfaceUsage`
#[derive(Debug, Clone)]
pub struct InterfaceUsage {
    pub span: Span,
}

impl AstNode for InterfaceUsage {
    fn span(&self) -> Span { self.span }
}

/// `RequirementConstraintUsage`
#[derive(Debug, Clone)]
pub struct RequirementConstraintUsage {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for RequirementConstraintUsage {
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

/// `TriggerValuePart`
#[derive(Debug, Clone)]
pub struct TriggerValuePart {
    pub span: Span,
    pub owned_relationship: Vec<TriggerFeatureValue>,
}

impl AstNode for TriggerValuePart {
    fn span(&self) -> Span { self.span }
}

/// `AcceptNode`
#[derive(Debug, Clone)]
pub struct AcceptNode {
    pub span: Span,
}

impl AstNode for AcceptNode {
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

/// `SourceEnd`
#[derive(Debug, Clone)]
pub struct SourceEnd {
    pub span: Span,
    pub owned_relationship: Vec<OwnedMultiplicity>,
}

impl AstNode for SourceEnd {
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

/// `WhileLoopNode`
#[derive(Debug, Clone)]
pub enum WhileLoopNodeOwnedRelationshipMember {
    ActionBodyParameterMember(Box<ActionBodyParameterMember>),
    EmptyParameterMember(Box<EmptyParameterMember>),
    ExpressionParameterMember(Box<ExpressionParameterMember>),
}

#[derive(Debug, Clone)]
pub struct WhileLoopNode {
    pub span: Span,
    pub owned_relationship: Vec<WhileLoopNodeOwnedRelationshipMember>,
}

impl AstNode for WhileLoopNode {
    fn span(&self) -> Span { self.span }
}

/// `DefinitionBody`
#[derive(Debug, Clone)]
pub struct DefinitionBody {
    pub span: Span,
    pub definition_body_item: Vec<DefinitionBodyItem>,
}

impl AstNode for DefinitionBody {
    fn span(&self) -> Span { self.span }
}

/// `ImportDeclaration`
#[derive(Debug, Clone)]
pub enum ImportDeclaration {
    MembershipImport(Box<MembershipImport>),
    NamespaceImport(Box<NamespaceImport>),
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

/// `OwnedFeatureChaining`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChaining {
    pub span: Span,
    pub chaining_feature: QualifiedNameRef,
}

impl AstNode for OwnedFeatureChaining {
    fn span(&self) -> Span { self.span }
}

/// `NaryInterfacePart`
#[derive(Debug, Clone)]
pub struct NaryInterfacePart {
    pub span: Span,
    pub owned_relationship: Vec<InterfaceEndMember>,
}

impl AstNode for NaryInterfacePart {
    fn span(&self) -> Span { self.span }
}

/// `ConnectorDeclaration`
#[derive(Debug, Clone)]
pub enum ConnectorDeclaration {
    BinaryConnectorDeclaration(Box<BinaryConnectorDeclaration>),
    NaryConnectorDeclaration(Box<NaryConnectorDeclaration>),
}

/// `GuardExpressionMember`
#[derive(Debug, Clone)]
pub struct GuardExpressionMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedExpression>,
}

impl AstNode for GuardExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `OccurrenceDefinition`
#[derive(Debug, Clone)]
pub struct OccurrenceDefinition {
    pub span: Span,
}

impl AstNode for OccurrenceDefinition {
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

/// `FramedConcernMember`
#[derive(Debug, Clone)]
pub struct FramedConcernMember {
    pub span: Span,
    pub owned_related_element: Vec<FramedConcernUsage>,
}

impl AstNode for FramedConcernMember {
    fn span(&self) -> Span { self.span }
}

/// `ExitActionMember`
#[derive(Debug, Clone)]
pub struct ExitActionMember {
    pub span: Span,
    pub kind: bool,
    pub owned_related_element: Vec<StateActionUsage>,
}

impl AstNode for ExitActionMember {
    fn span(&self) -> Span { self.span }
}

/// `NamespaceExpose`
#[derive(Debug, Clone)]
pub struct NamespaceExpose {
    pub span: Span,
}

impl AstNode for NamespaceExpose {
    fn span(&self) -> Span { self.span }
}

/// `AssignmentTargetBinding`
#[derive(Debug, Clone)]
pub struct AssignmentTargetBinding {
    pub span: Span,
    pub owned_related_element: Vec<NonFeatureChainPrimaryExpression>,
}

impl AstNode for AssignmentTargetBinding {
    fn span(&self) -> Span { self.span }
}

/// `PartUsage`
#[derive(Debug, Clone)]
pub struct PartUsage {
    pub span: Span,
}

impl AstNode for PartUsage {
    fn span(&self) -> Span { self.span }
}

/// `ForVariableDeclarationMember`
#[derive(Debug, Clone)]
pub struct ForVariableDeclarationMember {
    pub span: Span,
    pub owned_related_element: Vec<UsageDeclaration>,
}

impl AstNode for ForVariableDeclarationMember {
    fn span(&self) -> Span { self.span }
}

/// `StateSendActionUsage`
#[derive(Debug, Clone)]
pub struct StateSendActionUsage {
    pub span: Span,
}

impl AstNode for StateSendActionUsage {
    fn span(&self) -> Span { self.span }
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

/// `ConnectionUsage`
#[derive(Debug, Clone)]
pub struct ConnectionUsage {
    pub span: Span,
}

impl AstNode for ConnectionUsage {
    fn span(&self) -> Span { self.span }
}

/// `NodeParameterMember`
#[derive(Debug, Clone)]
pub struct NodeParameterMember {
    pub span: Span,
    pub owned_related_element: Vec<NodeParameter>,
}

impl AstNode for NodeParameterMember {
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

/// `VariantUsageElement`
#[derive(Debug, Clone)]
pub enum VariantUsageElement {
    VariantReference(Box<VariantReference>),
    ReferenceUsage(Box<ReferenceUsage>),
    AttributeUsage(Box<AttributeUsage>),
    BindingConnectorAsUsage(Box<BindingConnectorAsUsage>),
    SuccessionAsUsage(Box<SuccessionAsUsage>),
    OccurrenceUsage(Box<OccurrenceUsage>),
    IndividualUsage(Box<IndividualUsage>),
    PortionUsage(Box<PortionUsage>),
    EventOccurrenceUsage(Box<EventOccurrenceUsage>),
    ItemUsage(Box<ItemUsage>),
    PartUsage(Box<PartUsage>),
    ViewUsage(Box<ViewUsage>),
    RenderingUsage(Box<RenderingUsage>),
    PortUsage(Box<PortUsage>),
    ConnectionUsage(Box<ConnectionUsage>),
    InterfaceUsage(Box<InterfaceUsage>),
    AllocationUsage(Box<AllocationUsage>),
    Message(Box<Message>),
    FlowUsage(Box<FlowUsage>),
    SuccessionFlowUsage(Box<SuccessionFlowUsage>),
    BehaviorUsageElement(Box<BehaviorUsageElement>),
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

/// `OwnedFeatureChainMember`
#[derive(Debug, Clone)]
pub struct OwnedFeatureChainMember {
    pub span: Span,
    pub owned_related_element: Vec<OwnedFeatureChain>,
}

impl AstNode for OwnedFeatureChainMember {
    fn span(&self) -> Span { self.span }
}

/// `FlowEndSubsetting`
#[derive(Debug, Clone)]
pub enum FlowEndSubsettingReferencedFeatureMember {
    QualifiedNameRef(QualifiedNameRef),
    FeatureChainPrefix(Box<FeatureChainPrefix>),
}

#[derive(Debug, Clone)]
pub struct FlowEndSubsetting {
    pub span: Span,
    pub referenced_feature: Option<FlowEndSubsettingReferencedFeatureMember>,
}

impl AstNode for FlowEndSubsetting {
    fn span(&self) -> Span { self.span }
}

/// `ConjugatedPortTyping`
#[derive(Debug, Clone)]
pub struct ConjugatedPortTyping {
    pub span: Span,
    pub original_port_definition: QualifiedNameRef,
}

impl AstNode for ConjugatedPortTyping {
    fn span(&self) -> Span { self.span }
}

/// `TargetTransitionUsage`
#[derive(Debug, Clone)]
pub enum TargetTransitionUsageOwnedRelationshipMember {
    EffectBehaviorMember(Box<EffectBehaviorMember>),
    EmptyParameterMember(Box<EmptyParameterMember>),
    GuardExpressionMember(Box<GuardExpressionMember>),
    TransitionSuccessionMember(Box<TransitionSuccessionMember>),
    TriggerActionMember(Box<TriggerActionMember>),
}

#[derive(Debug, Clone)]
pub struct TargetTransitionUsage {
    pub span: Span,
    pub owned_relationship: Vec<TargetTransitionUsageOwnedRelationshipMember>,
}

impl AstNode for TargetTransitionUsage {
    fn span(&self) -> Span { self.span }
}

/// `RequirementVerificationUsage`
#[derive(Debug, Clone)]
pub struct RequirementVerificationUsage {
    pub span: Span,
    pub feature_specialization: Vec<FeatureSpecialization>,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for RequirementVerificationUsage {
    fn span(&self) -> Span { self.span }
}

/// `ActionUsage`
#[derive(Debug, Clone)]
pub struct ActionUsage {
    pub span: Span,
}

impl AstNode for ActionUsage {
    fn span(&self) -> Span { self.span }
}

/// `CaseUsage`
#[derive(Debug, Clone)]
pub struct CaseUsage {
    pub span: Span,
}

impl AstNode for CaseUsage {
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

/// `ConcernDefinition`
#[derive(Debug, Clone)]
pub struct ConcernDefinition {
    pub span: Span,
}

impl AstNode for ConcernDefinition {
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

/// `PrimaryArgumentValue`
#[derive(Debug, Clone)]
pub struct PrimaryArgumentValue {
    pub span: Span,
    pub value: Box<PrimaryExpression>,
}

impl AstNode for PrimaryArgumentValue {
    fn span(&self) -> Span { self.span }
}

/// `OccurrenceUsagePrefix`
#[derive(Debug, Clone)]
pub struct OccurrenceUsagePrefix {
    pub span: Span,
    pub is_individual: bool,
    pub portion_kind: Option<Box<PortionKind>>,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for OccurrenceUsagePrefix {
    fn span(&self) -> Span { self.span }
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

/// `AllocationDefinition`
#[derive(Debug, Clone)]
pub struct AllocationDefinition {
    pub span: Span,
}

impl AstNode for AllocationDefinition {
    fn span(&self) -> Span { self.span }
}

/// `ActorUsage`
#[derive(Debug, Clone)]
pub struct ActorUsage {
    pub span: Span,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for ActorUsage {
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

/// `EmptyUsage`
#[derive(Debug, Clone)]
pub struct EmptyUsage {
    pub span: Span,
}

impl AstNode for EmptyUsage {
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

/// `RelationshipBody`
#[derive(Debug, Clone)]
pub struct RelationshipBody {
    pub span: Span,
    pub owned_relationship: Vec<OwnedAnnotation>,
}

impl AstNode for RelationshipBody {
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

/// `ConditionalBinaryOperator`
#[derive(Debug, Clone)]
pub struct ConditionalBinaryOperator {
    pub span: Span,
}

impl AstNode for ConditionalBinaryOperator {
    fn span(&self) -> Span { self.span }
}

/// `BehaviorUsageElement`
#[derive(Debug, Clone)]
pub enum BehaviorUsageElement {
    ActionUsage(Box<ActionUsage>),
    CalculationUsage(Box<CalculationUsage>),
    StateUsage(Box<StateUsage>),
    ConstraintUsage(Box<ConstraintUsage>),
    RequirementUsage(Box<RequirementUsage>),
    ConcernUsage(Box<ConcernUsage>),
    CaseUsage(Box<CaseUsage>),
    AnalysisCaseUsage(Box<AnalysisCaseUsage>),
    VerificationCaseUsage(Box<VerificationCaseUsage>),
    UseCaseUsage(Box<UseCaseUsage>),
    ViewpointUsage(Box<ViewpointUsage>),
    PerformActionUsage(Box<PerformActionUsage>),
    ExhibitStateUsage(Box<ExhibitStateUsage>),
    IncludeUseCaseUsage(Box<IncludeUseCaseUsage>),
    AssertConstraintUsage(Box<AssertConstraintUsage>),
    SatisfyRequirementUsage(Box<SatisfyRequirementUsage>),
}

/// `ActionBehaviorMember`
#[derive(Debug, Clone)]
pub enum ActionBehaviorMember {
    BehaviorUsageMember(Box<BehaviorUsageMember>),
    ActionNodeMember(Box<ActionNodeMember>),
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

/// `StateDefBody`
#[derive(Debug, Clone)]
pub struct StateDefBody {
    pub span: Span,
    pub is_parallel: bool,
    pub state_body_item: Vec<StateBodyItem>,
}

impl AstNode for StateDefBody {
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

/// `ActionBodyItem`
#[derive(Debug, Clone)]
pub enum ActionBodyItemOwnedRelationshipMember {
    ActionBehaviorMember(Box<ActionBehaviorMember>),
    ActionTargetSuccessionMember(Box<ActionTargetSuccessionMember>),
    GuardedSuccessionMember(Box<GuardedSuccessionMember>),
    InitialNodeMember(Box<InitialNodeMember>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
}

#[derive(Debug, Clone)]
pub struct ActionBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<ActionBodyItemOwnedRelationshipMember>,
}

impl AstNode for ActionBodyItem {
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

/// `SatisfactionReferenceExpression`
#[derive(Debug, Clone)]
pub struct SatisfactionReferenceExpression {
    pub span: Span,
    pub owned_relationship: Vec<FeatureChainMember>,
}

impl AstNode for SatisfactionReferenceExpression {
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

/// `ArgumentMember`
#[derive(Debug, Clone)]
pub struct ArgumentMember {
    pub span: Span,
    pub owned_member_parameter: Box<Argument>,
}

impl AstNode for ArgumentMember {
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

/// `ArgumentList`
#[derive(Debug, Clone)]
pub struct ArgumentList {
    pub span: Span,
}

impl AstNode for ArgumentList {
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

/// `IndividualUsage`
#[derive(Debug, Clone)]
pub struct IndividualUsage {
    pub span: Span,
    pub is_individual: bool,
    pub usage_extension_keyword: Vec<UsageExtensionKeyword>,
}

impl AstNode for IndividualUsage {
    fn span(&self) -> Span { self.span }
}

/// `GuardedSuccession`
#[derive(Debug, Clone)]
pub enum GuardedSuccessionOwnedRelationshipMember {
    FeatureChainMember(Box<FeatureChainMember>),
    GuardExpressionMember(Box<GuardExpressionMember>),
    TransitionSuccessionMember(Box<TransitionSuccessionMember>),
}

#[derive(Debug, Clone)]
pub struct GuardedSuccession {
    pub span: Span,
    pub owned_relationship: Vec<GuardedSuccessionOwnedRelationshipMember>,
}

impl AstNode for GuardedSuccession {
    fn span(&self) -> Span { self.span }
}

/// `EntryActionMember`
#[derive(Debug, Clone)]
pub struct EntryActionMember {
    pub span: Span,
    pub kind: bool,
    pub owned_related_element: Vec<StateActionUsage>,
}

impl AstNode for EntryActionMember {
    fn span(&self) -> Span { self.span }
}

/// `PrimaryExpression`
#[derive(Debug, Clone)]
pub enum PrimaryExpression {
    FeatureChainExpression(Box<FeatureChainExpression>),
    NonFeatureChainPrimaryExpression(Box<NonFeatureChainPrimaryExpression>),
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

/// `OwnedExpressionMember`
#[derive(Debug, Clone)]
pub struct OwnedExpressionMember {
    pub span: Span,
    pub owned_feature_member: Box<OwnedExpression>,
}

impl AstNode for OwnedExpressionMember {
    fn span(&self) -> Span { self.span }
}

/// `InterfaceEndMember`
#[derive(Debug, Clone)]
pub struct InterfaceEndMember {
    pub span: Span,
    pub owned_related_element: Vec<InterfaceEnd>,
}

impl AstNode for InterfaceEndMember {
    fn span(&self) -> Span { self.span }
}

/// `SendNodeDeclaration`
#[derive(Debug, Clone)]
pub struct SendNodeDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<NodeParameterMember>,
}

impl AstNode for SendNodeDeclaration {
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

/// `InstantiatedTypeReference`
#[derive(Debug, Clone)]
pub struct InstantiatedTypeReference {
    pub span: Span,
}

impl AstNode for InstantiatedTypeReference {
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

/// `OwnedMultiplicity`
#[derive(Debug, Clone)]
pub struct OwnedMultiplicity {
    pub span: Span,
    pub owned_related_element: Vec<MultiplicityRange>,
}

impl AstNode for OwnedMultiplicity {
    fn span(&self) -> Span { self.span }
}

/// `AttributeDefinition`
#[derive(Debug, Clone)]
pub struct AttributeDefinition {
    pub span: Span,
}

impl AstNode for AttributeDefinition {
    fn span(&self) -> Span { self.span }
}

/// `PortDefinition`
#[derive(Debug, Clone)]
pub struct PortDefinition {
    pub span: Span,
    pub owned_relationship: Vec<ConjugatedPortDefinitionMember>,
}

impl AstNode for PortDefinition {
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

/// `UseCaseDefinition`
#[derive(Debug, Clone)]
pub struct UseCaseDefinition {
    pub span: Span,
}

impl AstNode for UseCaseDefinition {
    fn span(&self) -> Span { self.span }
}

/// `GuardedSuccessionMember`
#[derive(Debug, Clone)]
pub struct GuardedSuccessionMember {
    pub span: Span,
    pub owned_related_element: Vec<GuardedSuccession>,
}

impl AstNode for GuardedSuccessionMember {
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

/// `ExpressionBody`
#[derive(Debug, Clone)]
pub struct ExpressionBody {
    pub span: Span,
}

impl AstNode for ExpressionBody {
    fn span(&self) -> Span { self.span }
}

/// `ConstraintUsage`
#[derive(Debug, Clone)]
pub struct ConstraintUsage {
    pub span: Span,
}

impl AstNode for ConstraintUsage {
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

/// `ReturnFeatureMember`
#[derive(Debug, Clone)]
pub struct ReturnFeatureMember {
    pub span: Span,
    pub owned_related_element: Vec<FeatureElement>,
}

impl AstNode for ReturnFeatureMember {
    fn span(&self) -> Span { self.span }
}

/// `FeatureMember`
#[derive(Debug, Clone)]
pub enum FeatureMember {
    TypeFeatureMember(Box<TypeFeatureMember>),
    OwnedFeatureMember(Box<OwnedFeatureMember>),
}

/// `ForkNode`
#[derive(Debug, Clone)]
pub struct ForkNode {
    pub span: Span,
    pub is_composite: bool,
}

impl AstNode for ForkNode {
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

/// `StructureUsageElement`
#[derive(Debug, Clone)]
pub enum StructureUsageElement {
    OccurrenceUsage(Box<OccurrenceUsage>),
    IndividualUsage(Box<IndividualUsage>),
    PortionUsage(Box<PortionUsage>),
    EventOccurrenceUsage(Box<EventOccurrenceUsage>),
    ItemUsage(Box<ItemUsage>),
    PartUsage(Box<PartUsage>),
    ViewUsage(Box<ViewUsage>),
    RenderingUsage(Box<RenderingUsage>),
    PortUsage(Box<PortUsage>),
    ConnectionUsage(Box<ConnectionUsage>),
    InterfaceUsage(Box<InterfaceUsage>),
    AllocationUsage(Box<AllocationUsage>),
    Message(Box<Message>),
    FlowUsage(Box<FlowUsage>),
    SuccessionFlowUsage(Box<SuccessionFlowUsage>),
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

/// `MetaclassificationTestOperator`
#[derive(Debug, Clone)]
pub struct MetaclassificationTestOperator {
    pub span: Span,
}

impl AstNode for MetaclassificationTestOperator {
    fn span(&self) -> Span { self.span }
}

/// `ViewDefinitionBody`
#[derive(Debug, Clone)]
pub struct ViewDefinitionBody {
    pub span: Span,
    pub view_definition_body_item: Vec<ViewDefinitionBodyItem>,
}

impl AstNode for ViewDefinitionBody {
    fn span(&self) -> Span { self.span }
}

/// `ConnectionDefinition`
#[derive(Debug, Clone)]
pub struct ConnectionDefinition {
    pub span: Span,
}

impl AstNode for ConnectionDefinition {
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

/// `PerformActionUsageDeclaration`
#[derive(Debug, Clone)]
pub struct PerformActionUsageDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<OwnedReferenceSubsetting>,
}

impl AstNode for PerformActionUsageDeclaration {
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

/// `SequenceExpressionListMember`
#[derive(Debug, Clone)]
pub struct SequenceExpressionListMember {
    pub span: Span,
    pub owned_member_feature: Box<SequenceExpressionList>,
}

impl AstNode for SequenceExpressionListMember {
    fn span(&self) -> Span { self.span }
}

/// `StateUsageBody`
#[derive(Debug, Clone)]
pub struct StateUsageBody {
    pub span: Span,
    pub is_parallel: bool,
    pub state_body_item: Vec<StateBodyItem>,
}

impl AstNode for StateUsageBody {
    fn span(&self) -> Span { self.span }
}

/// `CalculationBodyItem`
#[derive(Debug, Clone)]
pub struct CalculationBodyItem {
    pub span: Span,
    pub owned_relationship: Vec<ReturnParameterMember>,
}

impl AstNode for CalculationBodyItem {
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

/// `OwnedRelatedElement`
#[derive(Debug, Clone)]
pub enum OwnedRelatedElement {
    NonFeatureElement(Box<NonFeatureElement>),
    FeatureElement(Box<FeatureElement>),
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

/// `LiteralString`
#[derive(Debug, Clone)]
pub struct LiteralString {
    pub span: Span,
    pub value: String,
}

impl AstNode for LiteralString {
    fn span(&self) -> Span { self.span }
}

/// `EnumerationUsageMember`
#[derive(Debug, Clone)]
pub struct EnumerationUsageMember {
    pub span: Span,
    pub owned_related_element: Vec<EnumeratedValue>,
}

impl AstNode for EnumerationUsageMember {
    fn span(&self) -> Span { self.span }
}

/// `MessageDeclaration`
#[derive(Debug, Clone)]
pub enum MessageDeclarationOwnedRelationshipMember {
    FlowPayloadFeatureMember(Box<FlowPayloadFeatureMember>),
    MessageEventMember(Box<MessageEventMember>),
}

#[derive(Debug, Clone)]
pub struct MessageDeclaration {
    pub span: Span,
    pub owned_relationship: Vec<MessageDeclarationOwnedRelationshipMember>,
}

impl AstNode for MessageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `CalculationUsageDeclaration`
#[derive(Debug, Clone)]
pub struct CalculationUsageDeclaration {
    pub span: Span,
}

impl AstNode for CalculationUsageDeclaration {
    fn span(&self) -> Span { self.span }
}

/// `FilterPackageImport`
#[derive(Debug, Clone)]
pub struct FilterPackageImport {
    pub span: Span,
}

impl AstNode for FilterPackageImport {
    fn span(&self) -> Span { self.span }
}

/// `InvocationTypeMember`
#[derive(Debug, Clone)]
pub struct InvocationTypeMember {
    pub span: Span,
}

impl AstNode for InvocationTypeMember {
    fn span(&self) -> Span { self.span }
}

// Top-level node wrapper

/// Wraps every AST node type so a single parse dispatch can
/// return the concrete result without erasing it.
#[derive(Debug, Clone)]
pub enum AstNodeKind {
    AssignmentTargetMember(Box<AssignmentTargetMember>),
    ExpressionBodyMember(Box<ExpressionBodyMember>),
    InterfaceDefinition(Box<InterfaceDefinition>),
    FlowPayloadFeature(Box<FlowPayloadFeature>),
    ActionBody(Box<ActionBody>),
    StateDefinition(Box<StateDefinition>),
    SubjectUsage(Box<SubjectUsage>),
    Expose(Box<Expose>),
    EmptyMultiplicityMember(Box<EmptyMultiplicityMember>),
    Definition(Box<Definition>),
    SuperclassingPart(Box<SuperclassingPart>),
    Documentation(Box<Documentation>),
    FeatureSpecializationPart(Box<FeatureSpecializationPart>),
    Interaction(Box<Interaction>),
    InterfaceBodyItem(Box<InterfaceBodyItem>),
    OwnedTypeFeaturing(Box<OwnedTypeFeaturing>),
    EndUsagePrefix(Box<EndUsagePrefix>),
    DefaultInterfaceEnd(Box<DefaultInterfaceEnd>),
    ObjectiveMember(Box<ObjectiveMember>),
    ArgumentExpressionMember(Box<ArgumentExpressionMember>),
    NamespaceImport(Box<NamespaceImport>),
    ActionTargetSuccession(Box<ActionTargetSuccession>),
    TypeReferenceMember(Box<TypeReferenceMember>),
    VerificationCaseDefinition(Box<VerificationCaseDefinition>),
    AcceptParameterPart(Box<AcceptParameterPart>),
    TypeFeaturing(Box<TypeFeaturing>),
    BodyArgument(Box<BodyArgument>),
    BinaryConnectorPart(Box<BinaryConnectorPart>),
    ControlNode(ControlNode),
    MemberPrefix(Box<MemberPrefix>),
    FlowUsage(Box<FlowUsage>),
    InvocationExpression(Box<InvocationExpression>),
    FlowFeatureMember(Box<FlowFeatureMember>),
    Message(Box<Message>),
    ConstraintUsageDeclaration(Box<ConstraintUsageDeclaration>),
    BaseExpression(BaseExpression),
    SourceEndMember(Box<SourceEndMember>),
    DefaultTargetSuccession(Box<DefaultTargetSuccession>),
    AnalysisCaseDefinition(Box<AnalysisCaseDefinition>),
    OwnedExpression(OwnedExpression),
    FunctionReference(Box<FunctionReference>),
    MemberElement(MemberElement),
    AllocationUsageDeclaration(Box<AllocationUsageDeclaration>),
    ArgumentExpression(Box<ArgumentExpression>),
    OwnedExpressionReference(Box<OwnedExpressionReference>),
    RootNamespace(Box<RootNamespace>),
    SatisfactionFeatureValue(Box<SatisfactionFeatureValue>),
    Typings(Box<Typings>),
    IndexExpression(Box<IndexExpression>),
    IntersectingPart(Box<IntersectingPart>),
    DefinitionBodyItem(Box<DefinitionBodyItem>),
    SubclassificationPart(Box<SubclassificationPart>),
    ExpressionParameterMember(Box<ExpressionParameterMember>),
    SatisfactionParameter(Box<SatisfactionParameter>),
    EffectBehaviorMember(Box<EffectBehaviorMember>),
    Usage(Box<Usage>),
    DefinitionMember(Box<DefinitionMember>),
    ItemUsage(Box<ItemUsage>),
    BasicUsagePrefix(Box<BasicUsagePrefix>),
    OwnedSubsetting(Box<OwnedSubsetting>),
    InstantiatedTypeMember(Box<InstantiatedTypeMember>),
    FeatureChainMember(Box<FeatureChainMember>),
    NamespaceDeclaration(Box<NamespaceDeclaration>),
    Unioning(Box<Unioning>),
    OwnedConjugation(Box<OwnedConjugation>),
    Redefines(Box<Redefines>),
    PortUsage(Box<PortUsage>),
    TypeDeclaration(Box<TypeDeclaration>),
    IfNodeParameterMember(Box<IfNodeParameterMember>),
    TransitionUsageMember(Box<TransitionUsageMember>),
    Flow(Box<Flow>),
    ObjectiveRequirementUsage(Box<ObjectiveRequirementUsage>),
    VariantReference(Box<VariantReference>),
    TriggerFeatureValue(Box<TriggerFeatureValue>),
    UnioningPart(Box<UnioningPart>),
    SpecificType(Box<SpecificType>),
    MetadataUsageDeclaration(Box<MetadataUsageDeclaration>),
    TypeFeaturingPart(Box<TypeFeaturingPart>),
    TypeRelationshipPart(TypeRelationshipPart),
    TypedBy(Box<TypedBy>),
    FunctionReferenceExpression(Box<FunctionReferenceExpression>),
    TransitionUsage(Box<TransitionUsage>),
    StakeholderMember(Box<StakeholderMember>),
    FilterPackage(Box<FilterPackage>),
    BodyArgumentMember(Box<BodyArgumentMember>),
    BindingConnector(Box<BindingConnector>),
    OwnedFeatureMember(Box<OwnedFeatureMember>),
    SelectExpression(Box<SelectExpression>),
    DependencyDeclaration(Box<DependencyDeclaration>),
    FeatureElement(FeatureElement),
    ViewpointUsage(Box<ViewpointUsage>),
    SequenceOperatorExpression(Box<SequenceOperatorExpression>),
    TargetSuccession(Box<TargetSuccession>),
    MetadataBodyUsageMember(Box<MetadataBodyUsageMember>),
    SuccessionDeclaration(Box<SuccessionDeclaration>),
    CalculationBody(Box<CalculationBody>),
    NamespaceMember(NamespaceMember),
    Dependency(Box<Dependency>),
    OwnedFeatureTyping(Box<OwnedFeatureTyping>),
    ActionUsageDeclaration(Box<ActionUsageDeclaration>),
    DecisionNode(Box<DecisionNode>),
    EffectBehaviorUsage(EffectBehaviorUsage),
    FunctionReferenceArgumentValue(Box<FunctionReferenceArgumentValue>),
    ViewDefinitionBodyItem(Box<ViewDefinitionBodyItem>),
    MetadataBodyElement(MetadataBodyElement),
    SequenceExpression(Box<SequenceExpression>),
    ConstraintDefinition(Box<ConstraintDefinition>),
    TriggerActionMember(Box<TriggerActionMember>),
    MetadataArgumentMember(Box<MetadataArgumentMember>),
    Identification(Box<Identification>),
    OwnedRedefinition(Box<OwnedRedefinition>),
    EmptyMultiplicity(Box<EmptyMultiplicity>),
    OwnedAnnotation(Box<OwnedAnnotation>),
    InterfaceNonOccurrenceUsageElement(InterfaceNonOccurrenceUsageElement),
    Metaclass(Box<Metaclass>),
    OwnedExpressionReferenceMember(Box<OwnedExpressionReferenceMember>),
    OwnedSpecialization(Box<OwnedSpecialization>),
    NonFeatureMember(Box<NonFeatureMember>),
    ConnectorEnd(Box<ConnectorEnd>),
    TypeResultMember(Box<TypeResultMember>),
    MultiplicityExpressionMember(Box<MultiplicityExpressionMember>),
    AssertConstraintUsage(Box<AssertConstraintUsage>),
    Invariant(Box<Invariant>),
    Conjugation(Box<Conjugation>),
    SourceSuccession(Box<SourceSuccession>),
    ActorMember(Box<ActorMember>),
    ViewRenderingUsage(Box<ViewRenderingUsage>),
    UsagePrefix(Box<UsagePrefix>),
    BracketExpression(Box<BracketExpression>),
    ExtendedDefinition(Box<ExtendedDefinition>),
    OccurrenceUsageElement(OccurrenceUsageElement),
    MultiplicityBounds(Box<MultiplicityBounds>),
    Behavior(Box<Behavior>),
    InitialNodeMember(Box<InitialNodeMember>),
    PortionUsage(Box<PortionUsage>),
    FunctionBody(Box<FunctionBody>),
    PrimaryArgumentMember(Box<PrimaryArgumentMember>),
    NonOccurrenceUsageElement(NonOccurrenceUsageElement),
    TransitionSuccessionMember(Box<TransitionSuccessionMember>),
    NaryConnectorDeclaration(Box<NaryConnectorDeclaration>),
    AllocationUsage(Box<AllocationUsage>),
    FeatureReferenceExpression(Box<FeatureReferenceExpression>),
    EnumerationDefinition(Box<EnumerationDefinition>),
    Type(Box<Type>),
    StatePerformActionUsage(Box<StatePerformActionUsage>),
    ArgumentValue(Box<ArgumentValue>),
    MessageEvent(Box<MessageEvent>),
    AnnotatingElement(AnnotatingElement),
    CalculationBodyPart(Box<CalculationBodyPart>),
    OwnedCrossMultiplicityMember(Box<OwnedCrossMultiplicityMember>),
    BinaryOperatorExpression(Box<BinaryOperatorExpression>),
    ResultExpressionMember(Box<ResultExpressionMember>),
    LiteralReal(Box<LiteralReal>),
    ConstructorExpression(Box<ConstructorExpression>),
    ActionBodyParameterMember(Box<ActionBodyParameterMember>),
    NonFeatureChainPrimaryExpression(NonFeatureChainPrimaryExpression),
    NonFeatureChainPrimaryArgumentMember(Box<NonFeatureChainPrimaryArgumentMember>),
    BodyExpression(Box<BodyExpression>),
    VariantUsageMember(Box<VariantUsageMember>),
    TypeBody(Box<TypeBody>),
    FunctionOperationExpression(Box<FunctionOperationExpression>),
    BooleanValue(Box<BooleanValue>),
    RequirementVerificationMember(Box<RequirementVerificationMember>),
    NamedArgumentList(Box<NamedArgumentList>),
    ActionNodePrefix(Box<ActionNodePrefix>),
    SenderReceiverPart(Box<SenderReceiverPart>),
    UnaryOperatorExpression(Box<UnaryOperatorExpression>),
    RealValue(Box<RealValue>),
    DefinitionElement(DefinitionElement),
    Connector(Box<Connector>),
    VerificationCaseUsage(Box<VerificationCaseUsage>),
    NonFeatureChainPrimaryArgument(Box<NonFeatureChainPrimaryArgument>),
    Crosses(Box<Crosses>),
    BasicFeaturePrefix(Box<BasicFeaturePrefix>),
    NonFeatureChainPrimaryArgumentValue(Box<NonFeatureChainPrimaryArgumentValue>),
    ExhibitStateUsage(Box<ExhibitStateUsage>),
    NonOccurrenceUsageMember(Box<NonOccurrenceUsageMember>),
    ExtendedUsage(Box<ExtendedUsage>),
    References(Box<References>),
    PayloadFeature(Box<PayloadFeature>),
    InterfaceNonOccurrenceUsageMember(Box<InterfaceNonOccurrenceUsageMember>),
    DoActionMember(Box<DoActionMember>),
    ConjugatedPortDefinition(Box<ConjugatedPortDefinition>),
    DefaultReferenceUsage(Box<DefaultReferenceUsage>),
    NonBehaviorBodyItem(Box<NonBehaviorBodyItem>),
    ConstructorResultMember(Box<ConstructorResultMember>),
    JoinNode(Box<JoinNode>),
    ActionNodeMember(Box<ActionNodeMember>),
    TransitionSuccession(Box<TransitionSuccession>),
    SuccessionFlowUsage(Box<SuccessionFlowUsage>),
    ViewRenderingMember(Box<ViewRenderingMember>),
    EmptyParameterMember(Box<EmptyParameterMember>),
    Multiplicity(Multiplicity),
    EnumeratedValue(Box<EnumeratedValue>),
    StakeholderUsage(Box<StakeholderUsage>),
    TransitionPerformActionUsage(Box<TransitionPerformActionUsage>),
    BasicDefinitionPrefix(Box<BasicDefinitionPrefix>),
    UsageCompletion(Box<UsageCompletion>),
    SpecializationPart(Box<SpecializationPart>),
    PartDefinition(Box<PartDefinition>),
    RequirementBodyItem(Box<RequirementBodyItem>),
    MetadataBodyUsage(Box<MetadataBodyUsage>),
    ChainingPart(Box<ChainingPart>),
    Feature(Box<Feature>),
    EndFeaturePrefix(Box<EndFeaturePrefix>),
    InterfacePart(InterfacePart),
    TransitionAcceptActionUsage(Box<TransitionAcceptActionUsage>),
    FeatureInverting(Box<FeatureInverting>),
    BindingConnectorAsUsage(Box<BindingConnectorAsUsage>),
    UsageDeclaration(Box<UsageDeclaration>),
    ControlNodePrefix(Box<ControlNodePrefix>),
    CaseBodyItem(Box<CaseBodyItem>),
    OwnedCrossFeature(Box<OwnedCrossFeature>),
    GeneralType(Box<GeneralType>),
    UseCaseUsage(Box<UseCaseUsage>),
    ItemDefinition(Box<ItemDefinition>),
    Disjoining(Box<Disjoining>),
    StructureUsageMember(Box<StructureUsageMember>),
    ElementFilterMember(Box<ElementFilterMember>),
    Intersecting(Box<Intersecting>),
    FramedConcernUsage(Box<FramedConcernUsage>),
    InterfaceUsageDeclaration(Box<InterfaceUsageDeclaration>),
    LiteralExpression(LiteralExpression),
    Step(Box<Step>),
    PerformActionUsage(Box<PerformActionUsage>),
    MetadataUsage(Box<MetadataUsage>),
    Namespace(Box<Namespace>),
    FlowEnd(Box<FlowEnd>),
    RenderingUsage(Box<RenderingUsage>),
    OwnedReferenceSubsetting(Box<OwnedReferenceSubsetting>),
    StateAcceptActionUsage(Box<StateAcceptActionUsage>),
    ViewDefinition(Box<ViewDefinition>),
    FlowFeatureRedefinition(Box<FlowFeatureRedefinition>),
    BodyArgumentValue(Box<BodyArgumentValue>),
    BehaviorUsageMember(Box<BehaviorUsageMember>),
    Structure(Box<Structure>),
    EntryTransitionMember(Box<EntryTransitionMember>),
    AcceptNodeDeclaration(Box<AcceptNodeDeclaration>),
    FeatureBinding(Box<FeatureBinding>),
    DataType(Box<DataType>),
    BinaryOperator(Box<BinaryOperator>),
    Predicate(Box<Predicate>),
    EmptyFeature(Box<EmptyFeature>),
    RenderingDefinition(Box<RenderingDefinition>),
    CalculationDefinition(Box<CalculationDefinition>),
    TerminateNode(Box<TerminateNode>),
    LiteralInfinity(Box<LiteralInfinity>),
    PayloadParameterMember(Box<PayloadParameterMember>),
    OccurrenceUsageMember(Box<OccurrenceUsageMember>),
    UsageExtensionKeyword(Box<UsageExtensionKeyword>),
    ConjugatedPortDefinitionMember(Box<ConjugatedPortDefinitionMember>),
    TriggerAction(Box<TriggerAction>),
    PrefixMetadataMember(Box<PrefixMetadataMember>),
    ViewBody(Box<ViewBody>),
    MetaCastOperator(Box<MetaCastOperator>),
    DefinitionDeclaration(Box<DefinitionDeclaration>),
    SourceSuccessionMember(Box<SourceSuccessionMember>),
    MetadataFeatureDeclaration(Box<MetadataFeatureDeclaration>),
    OccurrenceUsage(Box<OccurrenceUsage>),
    Specialization(Box<Specialization>),
    AssignmentNodeDeclaration(Box<AssignmentNodeDeclaration>),
    ConcernUsage(Box<ConcernUsage>),
    StateAssignmentActionUsage(Box<StateAssignmentActionUsage>),
    MetadataArgument(Box<MetadataArgument>),
    MetadataDefinition(Box<MetadataDefinition>),
    ActionDefinition(Box<ActionDefinition>),
    IfNode(Box<IfNode>),
    NaryConnectorPart(Box<NaryConnectorPart>),
    DifferencingPart(Box<DifferencingPart>),
    BooleanExpression(Box<BooleanExpression>),
    FlowFeature(Box<FlowFeature>),
    RequirementDefinition(Box<RequirementDefinition>),
    GuardedTargetSuccession(Box<GuardedTargetSuccession>),
    PackageBody(Box<PackageBody>),
    FunctionReferenceMember(Box<FunctionReferenceMember>),
    DefinitionPrefix(Box<DefinitionPrefix>),
    ConditionalBinaryOperatorExpression(Box<ConditionalBinaryOperatorExpression>),
    BinaryInterfacePart(Box<BinaryInterfacePart>),
    ActionTargetSuccessionMember(Box<ActionTargetSuccessionMember>),
    SatisfactionSubjectMember(Box<SatisfactionSubjectMember>),
    MessageEventMember(Box<MessageEventMember>),
    ActionNodeUsageDeclaration(Box<ActionNodeUsageDeclaration>),
    InterfaceEnd(Box<InterfaceEnd>),
    PackageDeclaration(Box<PackageDeclaration>),
    PayloadFeatureMember(Box<PayloadFeatureMember>),
    FlowDeclaration(Box<FlowDeclaration>),
    StateUsage(Box<StateUsage>),
    OwnedSubclassification(Box<OwnedSubclassification>),
    PrefixMetadataFeature(Box<PrefixMetadataFeature>),
    EmptyActionUsage(Box<EmptyActionUsage>),
    RequirementConstraintMember(Box<RequirementConstraintMember>),
    FeaturePrefix(Box<FeaturePrefix>),
    Argument(Box<Argument>),
    MultiplicityPart(Box<MultiplicityPart>),
    UsageBody(Box<UsageBody>),
    Annotation(Box<Annotation>),
    OccurrenceDefinitionPrefix(Box<OccurrenceDefinitionPrefix>),
    Expression(Box<Expression>),
    OwnedFeatureInverting(Box<OwnedFeatureInverting>),
    PayloadFeatureSpecializationPart(Box<PayloadFeatureSpecializationPart>),
    TypeBodyElement(Box<TypeBodyElement>),
    OwnedCrossSubsetting(Box<OwnedCrossSubsetting>),
    Class(Box<Class>),
    LibraryPackage(Box<LibraryPackage>),
    PackageBodyElement(Box<PackageBodyElement>),
    ViewpointDefinition(Box<ViewpointDefinition>),
    TransitionAssignmentActionUsage(Box<TransitionAssignmentActionUsage>),
    MetadataValue(Box<MetadataValue>),
    InterfaceOccurrenceUsageElement(InterfaceOccurrenceUsageElement),
    ViewBodyItem(Box<ViewBodyItem>),
    RequirementBody(Box<RequirementBody>),
    EventOccurrenceUsage(Box<EventOccurrenceUsage>),
    FlowEndMember(Box<FlowEndMember>),
    MetadataFeature(Box<MetadataFeature>),
    UnextendedUsagePrefix(UnextendedUsagePrefix),
    BinaryConnectorDeclaration(Box<BinaryConnectorDeclaration>),
    AnnotatingMember(Box<AnnotatingMember>),
    TransitionSendActionUsage(Box<TransitionSendActionUsage>),
    StateBodyItem(Box<StateBodyItem>),
    SequenceExpressionList(Box<SequenceExpressionList>),
    EmptyEndMember(Box<EmptyEndMember>),
    PortionKind(Box<PortionKind>),
    RequirementKind(Box<RequirementKind>),
    TriggerExpression(Box<TriggerExpression>),
    ForLoopNode(Box<ForLoopNode>),
    RefPrefix(Box<RefPrefix>),
    ActionNode(ActionNode),
    AttributeUsage(Box<AttributeUsage>),
    NamedArgumentMember(Box<NamedArgumentMember>),
    FlowDefinition(Box<FlowDefinition>),
    FunctionReferenceArgumentMember(Box<FunctionReferenceArgumentMember>),
    FeatureTyping(FeatureTyping),
    AliasMember(Box<AliasMember>),
    Subclassification(Box<Subclassification>),
    MetadataBodyFeature(Box<MetadataBodyFeature>),
    ElementReferenceMember(Box<ElementReferenceMember>),
    AssignmentTargetParameter(Box<AssignmentTargetParameter>),
    VisibilityIndicator(Box<VisibilityIndicator>),
    SuccessionFlow(Box<SuccessionFlow>),
    MultiplicitySubset(Box<MultiplicitySubset>),
    MembershipImport(Box<MembershipImport>),
    NonFeatureElement(NonFeatureElement),
    PortConjugation(Box<PortConjugation>),
    ViewUsage(Box<ViewUsage>),
    SatisfyRequirementUsage(Box<SatisfyRequirementUsage>),
    InvertingPart(Box<InvertingPart>),
    FunctionReferenceArgument(Box<FunctionReferenceArgument>),
    EnumerationBody(Box<EnumerationBody>),
    MetadataBody(Box<MetadataBody>),
    ReturnParameterMember(Box<ReturnParameterMember>),
    Classifier(Box<Classifier>),
    NamespaceBody(Box<NamespaceBody>),
    CaseBody(Box<CaseBody>),
    QualifiedName(Box<QualifiedName>),
    PackageMember(Box<PackageMember>),
    InterfaceOccurrenceUsageMember(Box<InterfaceOccurrenceUsageMember>),
    ActionBodyParameter(Box<ActionBodyParameter>),
    ConjugationPart(Box<ConjugationPart>),
    SendNode(Box<SendNode>),
    ForVariableDeclaration(Box<ForVariableDeclaration>),
    AnalysisCaseUsage(Box<AnalysisCaseUsage>),
    DefinitionExtensionKeyword(Box<DefinitionExtensionKeyword>),
    FeatureReference(Box<FeatureReference>),
    OwnedFeatureChain(Box<OwnedFeatureChain>),
    PositionalArgumentList(Box<PositionalArgumentList>),
    NullExpression(Box<NullExpression>),
    TypeReference(Box<TypeReference>),
    FeatureReferenceMember(Box<FeatureReferenceMember>),
    MultiplicityRange(Box<MultiplicityRange>),
    EnumerationUsage(Box<EnumerationUsage>),
    PayloadParameter(Box<PayloadParameter>),
    LiteralBoolean(Box<LiteralBoolean>),
    ParameterRedefinition(Box<ParameterRedefinition>),
    NamedArgument(Box<NamedArgument>),
    MergeNode(Box<MergeNode>),
    MembershipExpose(Box<MembershipExpose>),
    StateActionUsage(Box<StateActionUsage>),
    FeatureChainExpression(Box<FeatureChainExpression>),
    AssignmentNode(Box<AssignmentNode>),
    TypeFeatureMember(Box<TypeFeatureMember>),
    NodeParameter(Box<NodeParameter>),
    OwnedCrossMultiplicity(Box<OwnedCrossMultiplicity>),
    FlowPayloadFeatureMember(Box<FlowPayloadFeatureMember>),
    CalculationUsage(Box<CalculationUsage>),
    SubjectMember(Box<SubjectMember>),
    ArgumentExpressionValue(Box<ArgumentExpressionValue>),
    NamespaceBodyElement(Box<NamespaceBodyElement>),
    PrefixMetadataAnnotation(Box<PrefixMetadataAnnotation>),
    TypePrefix(Box<TypePrefix>),
    SuccessionAsUsage(Box<SuccessionAsUsage>),
    PrefixMetadataUsage(Box<PrefixMetadataUsage>),
    FeatureDirection(Box<FeatureDirection>),
    MetaclassificationExpression(Box<MetaclassificationExpression>),
    FunctionBodyPart(Box<FunctionBodyPart>),
    IndividualDefinition(Box<IndividualDefinition>),
    UnaryOperator(Box<UnaryOperator>),
    ClassificationTestOperator(Box<ClassificationTestOperator>),
    CastOperator(Box<CastOperator>),
    InterfaceBody(Box<InterfaceBody>),
    DisjoiningPart(Box<DisjoiningPart>),
    FeatureChainPrefix(Box<FeatureChainPrefix>),
    RequirementUsage(Box<RequirementUsage>),
    EmptyResultMember(Box<EmptyResultMember>),
    ReferenceUsage(Box<ReferenceUsage>),
    MetadataAccessExpression(Box<MetadataAccessExpression>),
    CaseDefinition(Box<CaseDefinition>),
    FeatureChain(Box<FeatureChain>),
    Succession(Box<Succession>),
    UsageElement(UsageElement),
    IncludeUseCaseUsage(Box<IncludeUseCaseUsage>),
    ConnectorPart(ConnectorPart),
    TargetTransitionUsageMember(Box<TargetTransitionUsageMember>),
    FeatureSpecialization(FeatureSpecialization),
    LiteralInteger(Box<LiteralInteger>),
    InterfaceUsage(Box<InterfaceUsage>),
    RequirementConstraintUsage(Box<RequirementConstraintUsage>),
    FeatureValue(Box<FeatureValue>),
    TriggerValuePart(Box<TriggerValuePart>),
    AcceptNode(Box<AcceptNode>),
    FilterPackageMember(Box<FilterPackageMember>),
    SourceEnd(Box<SourceEnd>),
    Association(Box<Association>),
    ExtentExpression(Box<ExtentExpression>),
    WhileLoopNode(Box<WhileLoopNode>),
    DefinitionBody(Box<DefinitionBody>),
    ImportDeclaration(ImportDeclaration),
    ClassificationExpression(Box<ClassificationExpression>),
    OwnedFeatureChaining(Box<OwnedFeatureChaining>),
    NaryInterfacePart(Box<NaryInterfacePart>),
    ConnectorDeclaration(ConnectorDeclaration),
    GuardExpressionMember(Box<GuardExpressionMember>),
    OccurrenceDefinition(Box<OccurrenceDefinition>),
    Differencing(Box<Differencing>),
    FramedConcernMember(Box<FramedConcernMember>),
    ExitActionMember(Box<ExitActionMember>),
    NamespaceExpose(Box<NamespaceExpose>),
    AssignmentTargetBinding(Box<AssignmentTargetBinding>),
    PartUsage(Box<PartUsage>),
    ForVariableDeclarationMember(Box<ForVariableDeclarationMember>),
    StateSendActionUsage(Box<StateSendActionUsage>),
    MetadataBodyFeatureMember(Box<MetadataBodyFeatureMember>),
    ConnectionUsage(Box<ConnectionUsage>),
    NodeParameterMember(Box<NodeParameterMember>),
    Function(Box<Function>),
    VariantUsageElement(VariantUsageElement),
    FeatureIdentification(Box<FeatureIdentification>),
    OwnedFeatureChainMember(Box<OwnedFeatureChainMember>),
    FlowEndSubsetting(Box<FlowEndSubsetting>),
    ConjugatedPortTyping(Box<ConjugatedPortTyping>),
    TargetTransitionUsage(Box<TargetTransitionUsage>),
    RequirementVerificationUsage(Box<RequirementVerificationUsage>),
    ActionUsage(Box<ActionUsage>),
    CaseUsage(Box<CaseUsage>),
    TextualRepresentation(Box<TextualRepresentation>),
    ConcernDefinition(Box<ConcernDefinition>),
    OwnedCrossFeatureMember(Box<OwnedCrossFeatureMember>),
    PrimaryArgumentValue(Box<PrimaryArgumentValue>),
    OccurrenceUsagePrefix(Box<OccurrenceUsagePrefix>),
    Comment(Box<Comment>),
    AllocationDefinition(Box<AllocationDefinition>),
    ActorUsage(Box<ActorUsage>),
    MetadataReference(Box<MetadataReference>),
    EmptyUsage(Box<EmptyUsage>),
    FeatureDeclaration(Box<FeatureDeclaration>),
    RelationshipBody(Box<RelationshipBody>),
    FeatureRelationshipPart(FeatureRelationshipPart),
    ConditionalBinaryOperator(Box<ConditionalBinaryOperator>),
    BehaviorUsageElement(BehaviorUsageElement),
    ActionBehaviorMember(ActionBehaviorMember),
    ValuePart(Box<ValuePart>),
    StateDefBody(Box<StateDefBody>),
    Redefinition(Box<Redefinition>),
    ActionBodyItem(Box<ActionBodyItem>),
    ClassifierDeclaration(Box<ClassifierDeclaration>),
    SatisfactionReferenceExpression(Box<SatisfactionReferenceExpression>),
    Subsetting(Box<Subsetting>),
    RelationshipOwnedElement(Box<RelationshipOwnedElement>),
    ArgumentMember(Box<ArgumentMember>),
    Subsets(Box<Subsets>),
    ArgumentList(Box<ArgumentList>),
    Redefinitions(Box<Redefinitions>),
    IndividualUsage(Box<IndividualUsage>),
    GuardedSuccession(Box<GuardedSuccession>),
    EntryActionMember(Box<EntryActionMember>),
    PrimaryExpression(PrimaryExpression),
    BindingConnectorDeclaration(Box<BindingConnectorDeclaration>),
    OwnedExpressionMember(Box<OwnedExpressionMember>),
    InterfaceEndMember(Box<InterfaceEndMember>),
    SendNodeDeclaration(Box<SendNodeDeclaration>),
    ReferenceTyping(Box<ReferenceTyping>),
    InstantiatedTypeReference(Box<InstantiatedTypeReference>),
    Subsettings(Box<Subsettings>),
    OwnedMultiplicity(Box<OwnedMultiplicity>),
    AttributeDefinition(Box<AttributeDefinition>),
    PortDefinition(Box<PortDefinition>),
    OwnedMultiplicityRange(Box<OwnedMultiplicityRange>),
    UseCaseDefinition(Box<UseCaseDefinition>),
    GuardedSuccessionMember(Box<GuardedSuccessionMember>),
    AssociationStructure(Box<AssociationStructure>),
    ExpressionBody(Box<ExpressionBody>),
    ConstraintUsage(Box<ConstraintUsage>),
    PrimaryArgument(Box<PrimaryArgument>),
    Import(Box<Import>),
    ReturnFeatureMember(Box<ReturnFeatureMember>),
    FeatureMember(FeatureMember),
    ForkNode(Box<ForkNode>),
    ConstructorResult(Box<ConstructorResult>),
    StructureUsageElement(StructureUsageElement),
    NamespaceFeatureMember(Box<NamespaceFeatureMember>),
    MetaclassificationTestOperator(Box<MetaclassificationTestOperator>),
    ViewDefinitionBody(Box<ViewDefinitionBody>),
    ConnectionDefinition(Box<ConnectionDefinition>),
    OwnedDisjoining(Box<OwnedDisjoining>),
    ConditionalExpression(Box<ConditionalExpression>),
    PerformActionUsageDeclaration(Box<PerformActionUsageDeclaration>),
    Package(Box<Package>),
    SequenceExpressionListMember(Box<SequenceExpressionListMember>),
    StateUsageBody(Box<StateUsageBody>),
    CalculationBodyItem(Box<CalculationBodyItem>),
    CollectExpression(Box<CollectExpression>),
    OwnedRelatedElement(OwnedRelatedElement),
    ConnectorEndMember(Box<ConnectorEndMember>),
    LiteralString(Box<LiteralString>),
    EnumerationUsageMember(Box<EnumerationUsageMember>),
    MessageDeclaration(Box<MessageDeclaration>),
    CalculationUsageDeclaration(Box<CalculationUsageDeclaration>),
    FilterPackageImport(Box<FilterPackageImport>),
    InvocationTypeMember(Box<InvocationTypeMember>),
}

impl AstNodeKind {
    /// Return the span of the contained node.
    pub fn span(&self) -> Span {
        match self {
            AstNodeKind::AssignmentTargetMember(v) => v.span,
            AstNodeKind::ExpressionBodyMember(v) => v.span,
            AstNodeKind::InterfaceDefinition(v) => v.span,
            AstNodeKind::FlowPayloadFeature(v) => v.span,
            AstNodeKind::ActionBody(v) => v.span,
            AstNodeKind::StateDefinition(v) => v.span,
            AstNodeKind::SubjectUsage(v) => v.span,
            AstNodeKind::Expose(v) => v.span,
            AstNodeKind::EmptyMultiplicityMember(v) => v.span,
            AstNodeKind::Definition(v) => v.span,
            AstNodeKind::SuperclassingPart(v) => v.span,
            AstNodeKind::Documentation(v) => v.span,
            AstNodeKind::FeatureSpecializationPart(v) => v.span,
            AstNodeKind::Interaction(v) => v.span,
            AstNodeKind::InterfaceBodyItem(v) => v.span,
            AstNodeKind::OwnedTypeFeaturing(v) => v.span,
            AstNodeKind::EndUsagePrefix(v) => v.span,
            AstNodeKind::DefaultInterfaceEnd(v) => v.span,
            AstNodeKind::ObjectiveMember(v) => v.span,
            AstNodeKind::ArgumentExpressionMember(v) => v.span,
            AstNodeKind::NamespaceImport(v) => v.span,
            AstNodeKind::ActionTargetSuccession(v) => v.span,
            AstNodeKind::TypeReferenceMember(v) => v.span,
            AstNodeKind::VerificationCaseDefinition(v) => v.span,
            AstNodeKind::AcceptParameterPart(v) => v.span,
            AstNodeKind::TypeFeaturing(v) => v.span,
            AstNodeKind::BodyArgument(v) => v.span,
            AstNodeKind::BinaryConnectorPart(v) => v.span,
            AstNodeKind::ControlNode(_v) => { Span::default() },
            AstNodeKind::MemberPrefix(v) => v.span,
            AstNodeKind::FlowUsage(v) => v.span,
            AstNodeKind::InvocationExpression(v) => v.span,
            AstNodeKind::FlowFeatureMember(v) => v.span,
            AstNodeKind::Message(v) => v.span,
            AstNodeKind::ConstraintUsageDeclaration(v) => v.span,
            AstNodeKind::BaseExpression(_v) => { Span::default() },
            AstNodeKind::SourceEndMember(v) => v.span,
            AstNodeKind::DefaultTargetSuccession(v) => v.span,
            AstNodeKind::AnalysisCaseDefinition(v) => v.span,
            AstNodeKind::OwnedExpression(_v) => { Span::default() },
            AstNodeKind::FunctionReference(v) => v.span,
            AstNodeKind::MemberElement(_v) => { Span::default() },
            AstNodeKind::AllocationUsageDeclaration(v) => v.span,
            AstNodeKind::ArgumentExpression(v) => v.span,
            AstNodeKind::OwnedExpressionReference(v) => v.span,
            AstNodeKind::RootNamespace(v) => v.span,
            AstNodeKind::SatisfactionFeatureValue(v) => v.span,
            AstNodeKind::Typings(v) => v.span,
            AstNodeKind::IndexExpression(v) => v.span,
            AstNodeKind::IntersectingPart(v) => v.span,
            AstNodeKind::DefinitionBodyItem(v) => v.span,
            AstNodeKind::SubclassificationPart(v) => v.span,
            AstNodeKind::ExpressionParameterMember(v) => v.span,
            AstNodeKind::SatisfactionParameter(v) => v.span,
            AstNodeKind::EffectBehaviorMember(v) => v.span,
            AstNodeKind::Usage(v) => v.span,
            AstNodeKind::DefinitionMember(v) => v.span,
            AstNodeKind::ItemUsage(v) => v.span,
            AstNodeKind::BasicUsagePrefix(v) => v.span,
            AstNodeKind::OwnedSubsetting(v) => v.span,
            AstNodeKind::InstantiatedTypeMember(v) => v.span,
            AstNodeKind::FeatureChainMember(v) => v.span,
            AstNodeKind::NamespaceDeclaration(v) => v.span,
            AstNodeKind::Unioning(v) => v.span,
            AstNodeKind::OwnedConjugation(v) => v.span,
            AstNodeKind::Redefines(v) => v.span,
            AstNodeKind::PortUsage(v) => v.span,
            AstNodeKind::TypeDeclaration(v) => v.span,
            AstNodeKind::IfNodeParameterMember(v) => v.span,
            AstNodeKind::TransitionUsageMember(v) => v.span,
            AstNodeKind::Flow(v) => v.span,
            AstNodeKind::ObjectiveRequirementUsage(v) => v.span,
            AstNodeKind::VariantReference(v) => v.span,
            AstNodeKind::TriggerFeatureValue(v) => v.span,
            AstNodeKind::UnioningPart(v) => v.span,
            AstNodeKind::SpecificType(v) => v.span,
            AstNodeKind::MetadataUsageDeclaration(v) => v.span,
            AstNodeKind::TypeFeaturingPart(v) => v.span,
            AstNodeKind::TypeRelationshipPart(_v) => { Span::default() },
            AstNodeKind::TypedBy(v) => v.span,
            AstNodeKind::FunctionReferenceExpression(v) => v.span,
            AstNodeKind::TransitionUsage(v) => v.span,
            AstNodeKind::StakeholderMember(v) => v.span,
            AstNodeKind::FilterPackage(v) => v.span,
            AstNodeKind::BodyArgumentMember(v) => v.span,
            AstNodeKind::BindingConnector(v) => v.span,
            AstNodeKind::OwnedFeatureMember(v) => v.span,
            AstNodeKind::SelectExpression(v) => v.span,
            AstNodeKind::DependencyDeclaration(v) => v.span,
            AstNodeKind::FeatureElement(_v) => { Span::default() },
            AstNodeKind::ViewpointUsage(v) => v.span,
            AstNodeKind::SequenceOperatorExpression(v) => v.span,
            AstNodeKind::TargetSuccession(v) => v.span,
            AstNodeKind::MetadataBodyUsageMember(v) => v.span,
            AstNodeKind::SuccessionDeclaration(v) => v.span,
            AstNodeKind::CalculationBody(v) => v.span,
            AstNodeKind::NamespaceMember(_v) => { Span::default() },
            AstNodeKind::Dependency(v) => v.span,
            AstNodeKind::OwnedFeatureTyping(v) => v.span,
            AstNodeKind::ActionUsageDeclaration(v) => v.span,
            AstNodeKind::DecisionNode(v) => v.span,
            AstNodeKind::EffectBehaviorUsage(_v) => { Span::default() },
            AstNodeKind::FunctionReferenceArgumentValue(v) => v.span,
            AstNodeKind::ViewDefinitionBodyItem(v) => v.span,
            AstNodeKind::MetadataBodyElement(_v) => { Span::default() },
            AstNodeKind::SequenceExpression(v) => v.span,
            AstNodeKind::ConstraintDefinition(v) => v.span,
            AstNodeKind::TriggerActionMember(v) => v.span,
            AstNodeKind::MetadataArgumentMember(v) => v.span,
            AstNodeKind::Identification(v) => v.span,
            AstNodeKind::OwnedRedefinition(v) => v.span,
            AstNodeKind::EmptyMultiplicity(v) => v.span,
            AstNodeKind::OwnedAnnotation(v) => v.span,
            AstNodeKind::InterfaceNonOccurrenceUsageElement(_v) => { Span::default() },
            AstNodeKind::Metaclass(v) => v.span,
            AstNodeKind::OwnedExpressionReferenceMember(v) => v.span,
            AstNodeKind::OwnedSpecialization(v) => v.span,
            AstNodeKind::NonFeatureMember(v) => v.span,
            AstNodeKind::ConnectorEnd(v) => v.span,
            AstNodeKind::TypeResultMember(v) => v.span,
            AstNodeKind::MultiplicityExpressionMember(v) => v.span,
            AstNodeKind::AssertConstraintUsage(v) => v.span,
            AstNodeKind::Invariant(v) => v.span,
            AstNodeKind::Conjugation(v) => v.span,
            AstNodeKind::SourceSuccession(v) => v.span,
            AstNodeKind::ActorMember(v) => v.span,
            AstNodeKind::ViewRenderingUsage(v) => v.span,
            AstNodeKind::UsagePrefix(v) => v.span,
            AstNodeKind::BracketExpression(v) => v.span,
            AstNodeKind::ExtendedDefinition(v) => v.span,
            AstNodeKind::OccurrenceUsageElement(_v) => { Span::default() },
            AstNodeKind::MultiplicityBounds(v) => v.span,
            AstNodeKind::Behavior(v) => v.span,
            AstNodeKind::InitialNodeMember(v) => v.span,
            AstNodeKind::PortionUsage(v) => v.span,
            AstNodeKind::FunctionBody(v) => v.span,
            AstNodeKind::PrimaryArgumentMember(v) => v.span,
            AstNodeKind::NonOccurrenceUsageElement(_v) => { Span::default() },
            AstNodeKind::TransitionSuccessionMember(v) => v.span,
            AstNodeKind::NaryConnectorDeclaration(v) => v.span,
            AstNodeKind::AllocationUsage(v) => v.span,
            AstNodeKind::FeatureReferenceExpression(v) => v.span,
            AstNodeKind::EnumerationDefinition(v) => v.span,
            AstNodeKind::Type(v) => v.span,
            AstNodeKind::StatePerformActionUsage(v) => v.span,
            AstNodeKind::ArgumentValue(v) => v.span,
            AstNodeKind::MessageEvent(v) => v.span,
            AstNodeKind::AnnotatingElement(_v) => { Span::default() },
            AstNodeKind::CalculationBodyPart(v) => v.span,
            AstNodeKind::OwnedCrossMultiplicityMember(v) => v.span,
            AstNodeKind::BinaryOperatorExpression(v) => v.span,
            AstNodeKind::ResultExpressionMember(v) => v.span,
            AstNodeKind::LiteralReal(v) => v.span,
            AstNodeKind::ConstructorExpression(v) => v.span,
            AstNodeKind::ActionBodyParameterMember(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryExpression(_v) => { Span::default() },
            AstNodeKind::NonFeatureChainPrimaryArgumentMember(v) => v.span,
            AstNodeKind::BodyExpression(v) => v.span,
            AstNodeKind::VariantUsageMember(v) => v.span,
            AstNodeKind::TypeBody(v) => v.span,
            AstNodeKind::FunctionOperationExpression(v) => v.span,
            AstNodeKind::BooleanValue(v) => v.span,
            AstNodeKind::RequirementVerificationMember(v) => v.span,
            AstNodeKind::NamedArgumentList(v) => v.span,
            AstNodeKind::ActionNodePrefix(v) => v.span,
            AstNodeKind::SenderReceiverPart(v) => v.span,
            AstNodeKind::UnaryOperatorExpression(v) => v.span,
            AstNodeKind::RealValue(v) => v.span,
            AstNodeKind::DefinitionElement(_v) => { Span::default() },
            AstNodeKind::Connector(v) => v.span,
            AstNodeKind::VerificationCaseUsage(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryArgument(v) => v.span,
            AstNodeKind::Crosses(v) => v.span,
            AstNodeKind::BasicFeaturePrefix(v) => v.span,
            AstNodeKind::NonFeatureChainPrimaryArgumentValue(v) => v.span,
            AstNodeKind::ExhibitStateUsage(v) => v.span,
            AstNodeKind::NonOccurrenceUsageMember(v) => v.span,
            AstNodeKind::ExtendedUsage(v) => v.span,
            AstNodeKind::References(v) => v.span,
            AstNodeKind::PayloadFeature(v) => v.span,
            AstNodeKind::InterfaceNonOccurrenceUsageMember(v) => v.span,
            AstNodeKind::DoActionMember(v) => v.span,
            AstNodeKind::ConjugatedPortDefinition(v) => v.span,
            AstNodeKind::DefaultReferenceUsage(v) => v.span,
            AstNodeKind::NonBehaviorBodyItem(v) => v.span,
            AstNodeKind::ConstructorResultMember(v) => v.span,
            AstNodeKind::JoinNode(v) => v.span,
            AstNodeKind::ActionNodeMember(v) => v.span,
            AstNodeKind::TransitionSuccession(v) => v.span,
            AstNodeKind::SuccessionFlowUsage(v) => v.span,
            AstNodeKind::ViewRenderingMember(v) => v.span,
            AstNodeKind::EmptyParameterMember(v) => v.span,
            AstNodeKind::Multiplicity(_v) => { Span::default() },
            AstNodeKind::EnumeratedValue(v) => v.span,
            AstNodeKind::StakeholderUsage(v) => v.span,
            AstNodeKind::TransitionPerformActionUsage(v) => v.span,
            AstNodeKind::BasicDefinitionPrefix(v) => v.span,
            AstNodeKind::UsageCompletion(v) => v.span,
            AstNodeKind::SpecializationPart(v) => v.span,
            AstNodeKind::PartDefinition(v) => v.span,
            AstNodeKind::RequirementBodyItem(v) => v.span,
            AstNodeKind::MetadataBodyUsage(v) => v.span,
            AstNodeKind::ChainingPart(v) => v.span,
            AstNodeKind::Feature(v) => v.span,
            AstNodeKind::EndFeaturePrefix(v) => v.span,
            AstNodeKind::InterfacePart(_v) => { Span::default() },
            AstNodeKind::TransitionAcceptActionUsage(v) => v.span,
            AstNodeKind::FeatureInverting(v) => v.span,
            AstNodeKind::BindingConnectorAsUsage(v) => v.span,
            AstNodeKind::UsageDeclaration(v) => v.span,
            AstNodeKind::ControlNodePrefix(v) => v.span,
            AstNodeKind::CaseBodyItem(v) => v.span,
            AstNodeKind::OwnedCrossFeature(v) => v.span,
            AstNodeKind::GeneralType(v) => v.span,
            AstNodeKind::UseCaseUsage(v) => v.span,
            AstNodeKind::ItemDefinition(v) => v.span,
            AstNodeKind::Disjoining(v) => v.span,
            AstNodeKind::StructureUsageMember(v) => v.span,
            AstNodeKind::ElementFilterMember(v) => v.span,
            AstNodeKind::Intersecting(v) => v.span,
            AstNodeKind::FramedConcernUsage(v) => v.span,
            AstNodeKind::InterfaceUsageDeclaration(v) => v.span,
            AstNodeKind::LiteralExpression(_v) => { Span::default() },
            AstNodeKind::Step(v) => v.span,
            AstNodeKind::PerformActionUsage(v) => v.span,
            AstNodeKind::MetadataUsage(v) => v.span,
            AstNodeKind::Namespace(v) => v.span,
            AstNodeKind::FlowEnd(v) => v.span,
            AstNodeKind::RenderingUsage(v) => v.span,
            AstNodeKind::OwnedReferenceSubsetting(v) => v.span,
            AstNodeKind::StateAcceptActionUsage(v) => v.span,
            AstNodeKind::ViewDefinition(v) => v.span,
            AstNodeKind::FlowFeatureRedefinition(v) => v.span,
            AstNodeKind::BodyArgumentValue(v) => v.span,
            AstNodeKind::BehaviorUsageMember(v) => v.span,
            AstNodeKind::Structure(v) => v.span,
            AstNodeKind::EntryTransitionMember(v) => v.span,
            AstNodeKind::AcceptNodeDeclaration(v) => v.span,
            AstNodeKind::FeatureBinding(v) => v.span,
            AstNodeKind::DataType(v) => v.span,
            AstNodeKind::BinaryOperator(v) => v.span,
            AstNodeKind::Predicate(v) => v.span,
            AstNodeKind::EmptyFeature(v) => v.span,
            AstNodeKind::RenderingDefinition(v) => v.span,
            AstNodeKind::CalculationDefinition(v) => v.span,
            AstNodeKind::TerminateNode(v) => v.span,
            AstNodeKind::LiteralInfinity(v) => v.span,
            AstNodeKind::PayloadParameterMember(v) => v.span,
            AstNodeKind::OccurrenceUsageMember(v) => v.span,
            AstNodeKind::UsageExtensionKeyword(v) => v.span,
            AstNodeKind::ConjugatedPortDefinitionMember(v) => v.span,
            AstNodeKind::TriggerAction(v) => v.span,
            AstNodeKind::PrefixMetadataMember(v) => v.span,
            AstNodeKind::ViewBody(v) => v.span,
            AstNodeKind::MetaCastOperator(v) => v.span,
            AstNodeKind::DefinitionDeclaration(v) => v.span,
            AstNodeKind::SourceSuccessionMember(v) => v.span,
            AstNodeKind::MetadataFeatureDeclaration(v) => v.span,
            AstNodeKind::OccurrenceUsage(v) => v.span,
            AstNodeKind::Specialization(v) => v.span,
            AstNodeKind::AssignmentNodeDeclaration(v) => v.span,
            AstNodeKind::ConcernUsage(v) => v.span,
            AstNodeKind::StateAssignmentActionUsage(v) => v.span,
            AstNodeKind::MetadataArgument(v) => v.span,
            AstNodeKind::MetadataDefinition(v) => v.span,
            AstNodeKind::ActionDefinition(v) => v.span,
            AstNodeKind::IfNode(v) => v.span,
            AstNodeKind::NaryConnectorPart(v) => v.span,
            AstNodeKind::DifferencingPart(v) => v.span,
            AstNodeKind::BooleanExpression(v) => v.span,
            AstNodeKind::FlowFeature(v) => v.span,
            AstNodeKind::RequirementDefinition(v) => v.span,
            AstNodeKind::GuardedTargetSuccession(v) => v.span,
            AstNodeKind::PackageBody(v) => v.span,
            AstNodeKind::FunctionReferenceMember(v) => v.span,
            AstNodeKind::DefinitionPrefix(v) => v.span,
            AstNodeKind::ConditionalBinaryOperatorExpression(v) => v.span,
            AstNodeKind::BinaryInterfacePart(v) => v.span,
            AstNodeKind::ActionTargetSuccessionMember(v) => v.span,
            AstNodeKind::SatisfactionSubjectMember(v) => v.span,
            AstNodeKind::MessageEventMember(v) => v.span,
            AstNodeKind::ActionNodeUsageDeclaration(v) => v.span,
            AstNodeKind::InterfaceEnd(v) => v.span,
            AstNodeKind::PackageDeclaration(v) => v.span,
            AstNodeKind::PayloadFeatureMember(v) => v.span,
            AstNodeKind::FlowDeclaration(v) => v.span,
            AstNodeKind::StateUsage(v) => v.span,
            AstNodeKind::OwnedSubclassification(v) => v.span,
            AstNodeKind::PrefixMetadataFeature(v) => v.span,
            AstNodeKind::EmptyActionUsage(v) => v.span,
            AstNodeKind::RequirementConstraintMember(v) => v.span,
            AstNodeKind::FeaturePrefix(v) => v.span,
            AstNodeKind::Argument(v) => v.span,
            AstNodeKind::MultiplicityPart(v) => v.span,
            AstNodeKind::UsageBody(v) => v.span,
            AstNodeKind::Annotation(v) => v.span,
            AstNodeKind::OccurrenceDefinitionPrefix(v) => v.span,
            AstNodeKind::Expression(v) => v.span,
            AstNodeKind::OwnedFeatureInverting(v) => v.span,
            AstNodeKind::PayloadFeatureSpecializationPart(v) => v.span,
            AstNodeKind::TypeBodyElement(v) => v.span,
            AstNodeKind::OwnedCrossSubsetting(v) => v.span,
            AstNodeKind::Class(v) => v.span,
            AstNodeKind::LibraryPackage(v) => v.span,
            AstNodeKind::PackageBodyElement(v) => v.span,
            AstNodeKind::ViewpointDefinition(v) => v.span,
            AstNodeKind::TransitionAssignmentActionUsage(v) => v.span,
            AstNodeKind::MetadataValue(v) => v.span,
            AstNodeKind::InterfaceOccurrenceUsageElement(_v) => { Span::default() },
            AstNodeKind::ViewBodyItem(v) => v.span,
            AstNodeKind::RequirementBody(v) => v.span,
            AstNodeKind::EventOccurrenceUsage(v) => v.span,
            AstNodeKind::FlowEndMember(v) => v.span,
            AstNodeKind::MetadataFeature(v) => v.span,
            AstNodeKind::UnextendedUsagePrefix(_v) => { Span::default() },
            AstNodeKind::BinaryConnectorDeclaration(v) => v.span,
            AstNodeKind::AnnotatingMember(v) => v.span,
            AstNodeKind::TransitionSendActionUsage(v) => v.span,
            AstNodeKind::StateBodyItem(v) => v.span,
            AstNodeKind::SequenceExpressionList(v) => v.span,
            AstNodeKind::EmptyEndMember(v) => v.span,
            AstNodeKind::PortionKind(v) => v.span,
            AstNodeKind::RequirementKind(v) => v.span,
            AstNodeKind::TriggerExpression(v) => v.span,
            AstNodeKind::ForLoopNode(v) => v.span,
            AstNodeKind::RefPrefix(v) => v.span,
            AstNodeKind::ActionNode(_v) => { Span::default() },
            AstNodeKind::AttributeUsage(v) => v.span,
            AstNodeKind::NamedArgumentMember(v) => v.span,
            AstNodeKind::FlowDefinition(v) => v.span,
            AstNodeKind::FunctionReferenceArgumentMember(v) => v.span,
            AstNodeKind::FeatureTyping(_v) => { Span::default() },
            AstNodeKind::AliasMember(v) => v.span,
            AstNodeKind::Subclassification(v) => v.span,
            AstNodeKind::MetadataBodyFeature(v) => v.span,
            AstNodeKind::ElementReferenceMember(v) => v.span,
            AstNodeKind::AssignmentTargetParameter(v) => v.span,
            AstNodeKind::VisibilityIndicator(v) => v.span,
            AstNodeKind::SuccessionFlow(v) => v.span,
            AstNodeKind::MultiplicitySubset(v) => v.span,
            AstNodeKind::MembershipImport(v) => v.span,
            AstNodeKind::NonFeatureElement(_v) => { Span::default() },
            AstNodeKind::PortConjugation(v) => v.span,
            AstNodeKind::ViewUsage(v) => v.span,
            AstNodeKind::SatisfyRequirementUsage(v) => v.span,
            AstNodeKind::InvertingPart(v) => v.span,
            AstNodeKind::FunctionReferenceArgument(v) => v.span,
            AstNodeKind::EnumerationBody(v) => v.span,
            AstNodeKind::MetadataBody(v) => v.span,
            AstNodeKind::ReturnParameterMember(v) => v.span,
            AstNodeKind::Classifier(v) => v.span,
            AstNodeKind::NamespaceBody(v) => v.span,
            AstNodeKind::CaseBody(v) => v.span,
            AstNodeKind::QualifiedName(v) => v.span,
            AstNodeKind::PackageMember(v) => v.span,
            AstNodeKind::InterfaceOccurrenceUsageMember(v) => v.span,
            AstNodeKind::ActionBodyParameter(v) => v.span,
            AstNodeKind::ConjugationPart(v) => v.span,
            AstNodeKind::SendNode(v) => v.span,
            AstNodeKind::ForVariableDeclaration(v) => v.span,
            AstNodeKind::AnalysisCaseUsage(v) => v.span,
            AstNodeKind::DefinitionExtensionKeyword(v) => v.span,
            AstNodeKind::FeatureReference(v) => v.span,
            AstNodeKind::OwnedFeatureChain(v) => v.span,
            AstNodeKind::PositionalArgumentList(v) => v.span,
            AstNodeKind::NullExpression(v) => v.span,
            AstNodeKind::TypeReference(v) => v.span,
            AstNodeKind::FeatureReferenceMember(v) => v.span,
            AstNodeKind::MultiplicityRange(v) => v.span,
            AstNodeKind::EnumerationUsage(v) => v.span,
            AstNodeKind::PayloadParameter(v) => v.span,
            AstNodeKind::LiteralBoolean(v) => v.span,
            AstNodeKind::ParameterRedefinition(v) => v.span,
            AstNodeKind::NamedArgument(v) => v.span,
            AstNodeKind::MergeNode(v) => v.span,
            AstNodeKind::MembershipExpose(v) => v.span,
            AstNodeKind::StateActionUsage(v) => v.span,
            AstNodeKind::FeatureChainExpression(v) => v.span,
            AstNodeKind::AssignmentNode(v) => v.span,
            AstNodeKind::TypeFeatureMember(v) => v.span,
            AstNodeKind::NodeParameter(v) => v.span,
            AstNodeKind::OwnedCrossMultiplicity(v) => v.span,
            AstNodeKind::FlowPayloadFeatureMember(v) => v.span,
            AstNodeKind::CalculationUsage(v) => v.span,
            AstNodeKind::SubjectMember(v) => v.span,
            AstNodeKind::ArgumentExpressionValue(v) => v.span,
            AstNodeKind::NamespaceBodyElement(v) => v.span,
            AstNodeKind::PrefixMetadataAnnotation(v) => v.span,
            AstNodeKind::TypePrefix(v) => v.span,
            AstNodeKind::SuccessionAsUsage(v) => v.span,
            AstNodeKind::PrefixMetadataUsage(v) => v.span,
            AstNodeKind::FeatureDirection(v) => v.span,
            AstNodeKind::MetaclassificationExpression(v) => v.span,
            AstNodeKind::FunctionBodyPart(v) => v.span,
            AstNodeKind::IndividualDefinition(v) => v.span,
            AstNodeKind::UnaryOperator(v) => v.span,
            AstNodeKind::ClassificationTestOperator(v) => v.span,
            AstNodeKind::CastOperator(v) => v.span,
            AstNodeKind::InterfaceBody(v) => v.span,
            AstNodeKind::DisjoiningPart(v) => v.span,
            AstNodeKind::FeatureChainPrefix(v) => v.span,
            AstNodeKind::RequirementUsage(v) => v.span,
            AstNodeKind::EmptyResultMember(v) => v.span,
            AstNodeKind::ReferenceUsage(v) => v.span,
            AstNodeKind::MetadataAccessExpression(v) => v.span,
            AstNodeKind::CaseDefinition(v) => v.span,
            AstNodeKind::FeatureChain(v) => v.span,
            AstNodeKind::Succession(v) => v.span,
            AstNodeKind::UsageElement(_v) => { Span::default() },
            AstNodeKind::IncludeUseCaseUsage(v) => v.span,
            AstNodeKind::ConnectorPart(_v) => { Span::default() },
            AstNodeKind::TargetTransitionUsageMember(v) => v.span,
            AstNodeKind::FeatureSpecialization(_v) => { Span::default() },
            AstNodeKind::LiteralInteger(v) => v.span,
            AstNodeKind::InterfaceUsage(v) => v.span,
            AstNodeKind::RequirementConstraintUsage(v) => v.span,
            AstNodeKind::FeatureValue(v) => v.span,
            AstNodeKind::TriggerValuePart(v) => v.span,
            AstNodeKind::AcceptNode(v) => v.span,
            AstNodeKind::FilterPackageMember(v) => v.span,
            AstNodeKind::SourceEnd(v) => v.span,
            AstNodeKind::Association(v) => v.span,
            AstNodeKind::ExtentExpression(v) => v.span,
            AstNodeKind::WhileLoopNode(v) => v.span,
            AstNodeKind::DefinitionBody(v) => v.span,
            AstNodeKind::ImportDeclaration(_v) => { Span::default() },
            AstNodeKind::ClassificationExpression(v) => v.span,
            AstNodeKind::OwnedFeatureChaining(v) => v.span,
            AstNodeKind::NaryInterfacePart(v) => v.span,
            AstNodeKind::ConnectorDeclaration(_v) => { Span::default() },
            AstNodeKind::GuardExpressionMember(v) => v.span,
            AstNodeKind::OccurrenceDefinition(v) => v.span,
            AstNodeKind::Differencing(v) => v.span,
            AstNodeKind::FramedConcernMember(v) => v.span,
            AstNodeKind::ExitActionMember(v) => v.span,
            AstNodeKind::NamespaceExpose(v) => v.span,
            AstNodeKind::AssignmentTargetBinding(v) => v.span,
            AstNodeKind::PartUsage(v) => v.span,
            AstNodeKind::ForVariableDeclarationMember(v) => v.span,
            AstNodeKind::StateSendActionUsage(v) => v.span,
            AstNodeKind::MetadataBodyFeatureMember(v) => v.span,
            AstNodeKind::ConnectionUsage(v) => v.span,
            AstNodeKind::NodeParameterMember(v) => v.span,
            AstNodeKind::Function(v) => v.span,
            AstNodeKind::VariantUsageElement(_v) => { Span::default() },
            AstNodeKind::FeatureIdentification(v) => v.span,
            AstNodeKind::OwnedFeatureChainMember(v) => v.span,
            AstNodeKind::FlowEndSubsetting(v) => v.span,
            AstNodeKind::ConjugatedPortTyping(v) => v.span,
            AstNodeKind::TargetTransitionUsage(v) => v.span,
            AstNodeKind::RequirementVerificationUsage(v) => v.span,
            AstNodeKind::ActionUsage(v) => v.span,
            AstNodeKind::CaseUsage(v) => v.span,
            AstNodeKind::TextualRepresentation(v) => v.span,
            AstNodeKind::ConcernDefinition(v) => v.span,
            AstNodeKind::OwnedCrossFeatureMember(v) => v.span,
            AstNodeKind::PrimaryArgumentValue(v) => v.span,
            AstNodeKind::OccurrenceUsagePrefix(v) => v.span,
            AstNodeKind::Comment(v) => v.span,
            AstNodeKind::AllocationDefinition(v) => v.span,
            AstNodeKind::ActorUsage(v) => v.span,
            AstNodeKind::MetadataReference(v) => v.span,
            AstNodeKind::EmptyUsage(v) => v.span,
            AstNodeKind::FeatureDeclaration(v) => v.span,
            AstNodeKind::RelationshipBody(v) => v.span,
            AstNodeKind::FeatureRelationshipPart(_v) => { Span::default() },
            AstNodeKind::ConditionalBinaryOperator(v) => v.span,
            AstNodeKind::BehaviorUsageElement(_v) => { Span::default() },
            AstNodeKind::ActionBehaviorMember(_v) => { Span::default() },
            AstNodeKind::ValuePart(v) => v.span,
            AstNodeKind::StateDefBody(v) => v.span,
            AstNodeKind::Redefinition(v) => v.span,
            AstNodeKind::ActionBodyItem(v) => v.span,
            AstNodeKind::ClassifierDeclaration(v) => v.span,
            AstNodeKind::SatisfactionReferenceExpression(v) => v.span,
            AstNodeKind::Subsetting(v) => v.span,
            AstNodeKind::RelationshipOwnedElement(v) => v.span,
            AstNodeKind::ArgumentMember(v) => v.span,
            AstNodeKind::Subsets(v) => v.span,
            AstNodeKind::ArgumentList(v) => v.span,
            AstNodeKind::Redefinitions(v) => v.span,
            AstNodeKind::IndividualUsage(v) => v.span,
            AstNodeKind::GuardedSuccession(v) => v.span,
            AstNodeKind::EntryActionMember(v) => v.span,
            AstNodeKind::PrimaryExpression(_v) => { Span::default() },
            AstNodeKind::BindingConnectorDeclaration(v) => v.span,
            AstNodeKind::OwnedExpressionMember(v) => v.span,
            AstNodeKind::InterfaceEndMember(v) => v.span,
            AstNodeKind::SendNodeDeclaration(v) => v.span,
            AstNodeKind::ReferenceTyping(v) => v.span,
            AstNodeKind::InstantiatedTypeReference(v) => v.span,
            AstNodeKind::Subsettings(v) => v.span,
            AstNodeKind::OwnedMultiplicity(v) => v.span,
            AstNodeKind::AttributeDefinition(v) => v.span,
            AstNodeKind::PortDefinition(v) => v.span,
            AstNodeKind::OwnedMultiplicityRange(v) => v.span,
            AstNodeKind::UseCaseDefinition(v) => v.span,
            AstNodeKind::GuardedSuccessionMember(v) => v.span,
            AstNodeKind::AssociationStructure(v) => v.span,
            AstNodeKind::ExpressionBody(v) => v.span,
            AstNodeKind::ConstraintUsage(v) => v.span,
            AstNodeKind::PrimaryArgument(v) => v.span,
            AstNodeKind::Import(v) => v.span,
            AstNodeKind::ReturnFeatureMember(v) => v.span,
            AstNodeKind::FeatureMember(_v) => { Span::default() },
            AstNodeKind::ForkNode(v) => v.span,
            AstNodeKind::ConstructorResult(v) => v.span,
            AstNodeKind::StructureUsageElement(_v) => { Span::default() },
            AstNodeKind::NamespaceFeatureMember(v) => v.span,
            AstNodeKind::MetaclassificationTestOperator(v) => v.span,
            AstNodeKind::ViewDefinitionBody(v) => v.span,
            AstNodeKind::ConnectionDefinition(v) => v.span,
            AstNodeKind::OwnedDisjoining(v) => v.span,
            AstNodeKind::ConditionalExpression(v) => v.span,
            AstNodeKind::PerformActionUsageDeclaration(v) => v.span,
            AstNodeKind::Package(v) => v.span,
            AstNodeKind::SequenceExpressionListMember(v) => v.span,
            AstNodeKind::StateUsageBody(v) => v.span,
            AstNodeKind::CalculationBodyItem(v) => v.span,
            AstNodeKind::CollectExpression(v) => v.span,
            AstNodeKind::OwnedRelatedElement(_v) => { Span::default() },
            AstNodeKind::ConnectorEndMember(v) => v.span,
            AstNodeKind::LiteralString(v) => v.span,
            AstNodeKind::EnumerationUsageMember(v) => v.span,
            AstNodeKind::MessageDeclaration(v) => v.span,
            AstNodeKind::CalculationUsageDeclaration(v) => v.span,
            AstNodeKind::FilterPackageImport(v) => v.span,
            AstNodeKind::InvocationTypeMember(v) => v.span,
        }
    }

    /// Return the variant name as a string.
    pub fn kind_name(&self) -> &'static str {
        match self {
            AstNodeKind::AssignmentTargetMember(_) => "AssignmentTargetMember",
            AstNodeKind::ExpressionBodyMember(_) => "ExpressionBodyMember",
            AstNodeKind::InterfaceDefinition(_) => "InterfaceDefinition",
            AstNodeKind::FlowPayloadFeature(_) => "FlowPayloadFeature",
            AstNodeKind::ActionBody(_) => "ActionBody",
            AstNodeKind::StateDefinition(_) => "StateDefinition",
            AstNodeKind::SubjectUsage(_) => "SubjectUsage",
            AstNodeKind::Expose(_) => "Expose",
            AstNodeKind::EmptyMultiplicityMember(_) => "EmptyMultiplicityMember",
            AstNodeKind::Definition(_) => "Definition",
            AstNodeKind::SuperclassingPart(_) => "SuperclassingPart",
            AstNodeKind::Documentation(_) => "Documentation",
            AstNodeKind::FeatureSpecializationPart(_) => "FeatureSpecializationPart",
            AstNodeKind::Interaction(_) => "Interaction",
            AstNodeKind::InterfaceBodyItem(_) => "InterfaceBodyItem",
            AstNodeKind::OwnedTypeFeaturing(_) => "OwnedTypeFeaturing",
            AstNodeKind::EndUsagePrefix(_) => "EndUsagePrefix",
            AstNodeKind::DefaultInterfaceEnd(_) => "DefaultInterfaceEnd",
            AstNodeKind::ObjectiveMember(_) => "ObjectiveMember",
            AstNodeKind::ArgumentExpressionMember(_) => "ArgumentExpressionMember",
            AstNodeKind::NamespaceImport(_) => "NamespaceImport",
            AstNodeKind::ActionTargetSuccession(_) => "ActionTargetSuccession",
            AstNodeKind::TypeReferenceMember(_) => "TypeReferenceMember",
            AstNodeKind::VerificationCaseDefinition(_) => "VerificationCaseDefinition",
            AstNodeKind::AcceptParameterPart(_) => "AcceptParameterPart",
            AstNodeKind::TypeFeaturing(_) => "TypeFeaturing",
            AstNodeKind::BodyArgument(_) => "BodyArgument",
            AstNodeKind::BinaryConnectorPart(_) => "BinaryConnectorPart",
            AstNodeKind::ControlNode(_) => "ControlNode",
            AstNodeKind::MemberPrefix(_) => "MemberPrefix",
            AstNodeKind::FlowUsage(_) => "FlowUsage",
            AstNodeKind::InvocationExpression(_) => "InvocationExpression",
            AstNodeKind::FlowFeatureMember(_) => "FlowFeatureMember",
            AstNodeKind::Message(_) => "Message",
            AstNodeKind::ConstraintUsageDeclaration(_) => "ConstraintUsageDeclaration",
            AstNodeKind::BaseExpression(_) => "BaseExpression",
            AstNodeKind::SourceEndMember(_) => "SourceEndMember",
            AstNodeKind::DefaultTargetSuccession(_) => "DefaultTargetSuccession",
            AstNodeKind::AnalysisCaseDefinition(_) => "AnalysisCaseDefinition",
            AstNodeKind::OwnedExpression(_) => "OwnedExpression",
            AstNodeKind::FunctionReference(_) => "FunctionReference",
            AstNodeKind::MemberElement(_) => "MemberElement",
            AstNodeKind::AllocationUsageDeclaration(_) => "AllocationUsageDeclaration",
            AstNodeKind::ArgumentExpression(_) => "ArgumentExpression",
            AstNodeKind::OwnedExpressionReference(_) => "OwnedExpressionReference",
            AstNodeKind::RootNamespace(_) => "RootNamespace",
            AstNodeKind::SatisfactionFeatureValue(_) => "SatisfactionFeatureValue",
            AstNodeKind::Typings(_) => "Typings",
            AstNodeKind::IndexExpression(_) => "IndexExpression",
            AstNodeKind::IntersectingPart(_) => "IntersectingPart",
            AstNodeKind::DefinitionBodyItem(_) => "DefinitionBodyItem",
            AstNodeKind::SubclassificationPart(_) => "SubclassificationPart",
            AstNodeKind::ExpressionParameterMember(_) => "ExpressionParameterMember",
            AstNodeKind::SatisfactionParameter(_) => "SatisfactionParameter",
            AstNodeKind::EffectBehaviorMember(_) => "EffectBehaviorMember",
            AstNodeKind::Usage(_) => "Usage",
            AstNodeKind::DefinitionMember(_) => "DefinitionMember",
            AstNodeKind::ItemUsage(_) => "ItemUsage",
            AstNodeKind::BasicUsagePrefix(_) => "BasicUsagePrefix",
            AstNodeKind::OwnedSubsetting(_) => "OwnedSubsetting",
            AstNodeKind::InstantiatedTypeMember(_) => "InstantiatedTypeMember",
            AstNodeKind::FeatureChainMember(_) => "FeatureChainMember",
            AstNodeKind::NamespaceDeclaration(_) => "NamespaceDeclaration",
            AstNodeKind::Unioning(_) => "Unioning",
            AstNodeKind::OwnedConjugation(_) => "OwnedConjugation",
            AstNodeKind::Redefines(_) => "Redefines",
            AstNodeKind::PortUsage(_) => "PortUsage",
            AstNodeKind::TypeDeclaration(_) => "TypeDeclaration",
            AstNodeKind::IfNodeParameterMember(_) => "IfNodeParameterMember",
            AstNodeKind::TransitionUsageMember(_) => "TransitionUsageMember",
            AstNodeKind::Flow(_) => "Flow",
            AstNodeKind::ObjectiveRequirementUsage(_) => "ObjectiveRequirementUsage",
            AstNodeKind::VariantReference(_) => "VariantReference",
            AstNodeKind::TriggerFeatureValue(_) => "TriggerFeatureValue",
            AstNodeKind::UnioningPart(_) => "UnioningPart",
            AstNodeKind::SpecificType(_) => "SpecificType",
            AstNodeKind::MetadataUsageDeclaration(_) => "MetadataUsageDeclaration",
            AstNodeKind::TypeFeaturingPart(_) => "TypeFeaturingPart",
            AstNodeKind::TypeRelationshipPart(_) => "TypeRelationshipPart",
            AstNodeKind::TypedBy(_) => "TypedBy",
            AstNodeKind::FunctionReferenceExpression(_) => "FunctionReferenceExpression",
            AstNodeKind::TransitionUsage(_) => "TransitionUsage",
            AstNodeKind::StakeholderMember(_) => "StakeholderMember",
            AstNodeKind::FilterPackage(_) => "FilterPackage",
            AstNodeKind::BodyArgumentMember(_) => "BodyArgumentMember",
            AstNodeKind::BindingConnector(_) => "BindingConnector",
            AstNodeKind::OwnedFeatureMember(_) => "OwnedFeatureMember",
            AstNodeKind::SelectExpression(_) => "SelectExpression",
            AstNodeKind::DependencyDeclaration(_) => "DependencyDeclaration",
            AstNodeKind::FeatureElement(_) => "FeatureElement",
            AstNodeKind::ViewpointUsage(_) => "ViewpointUsage",
            AstNodeKind::SequenceOperatorExpression(_) => "SequenceOperatorExpression",
            AstNodeKind::TargetSuccession(_) => "TargetSuccession",
            AstNodeKind::MetadataBodyUsageMember(_) => "MetadataBodyUsageMember",
            AstNodeKind::SuccessionDeclaration(_) => "SuccessionDeclaration",
            AstNodeKind::CalculationBody(_) => "CalculationBody",
            AstNodeKind::NamespaceMember(_) => "NamespaceMember",
            AstNodeKind::Dependency(_) => "Dependency",
            AstNodeKind::OwnedFeatureTyping(_) => "OwnedFeatureTyping",
            AstNodeKind::ActionUsageDeclaration(_) => "ActionUsageDeclaration",
            AstNodeKind::DecisionNode(_) => "DecisionNode",
            AstNodeKind::EffectBehaviorUsage(_) => "EffectBehaviorUsage",
            AstNodeKind::FunctionReferenceArgumentValue(_) => "FunctionReferenceArgumentValue",
            AstNodeKind::ViewDefinitionBodyItem(_) => "ViewDefinitionBodyItem",
            AstNodeKind::MetadataBodyElement(_) => "MetadataBodyElement",
            AstNodeKind::SequenceExpression(_) => "SequenceExpression",
            AstNodeKind::ConstraintDefinition(_) => "ConstraintDefinition",
            AstNodeKind::TriggerActionMember(_) => "TriggerActionMember",
            AstNodeKind::MetadataArgumentMember(_) => "MetadataArgumentMember",
            AstNodeKind::Identification(_) => "Identification",
            AstNodeKind::OwnedRedefinition(_) => "OwnedRedefinition",
            AstNodeKind::EmptyMultiplicity(_) => "EmptyMultiplicity",
            AstNodeKind::OwnedAnnotation(_) => "OwnedAnnotation",
            AstNodeKind::InterfaceNonOccurrenceUsageElement(_) => "InterfaceNonOccurrenceUsageElement",
            AstNodeKind::Metaclass(_) => "Metaclass",
            AstNodeKind::OwnedExpressionReferenceMember(_) => "OwnedExpressionReferenceMember",
            AstNodeKind::OwnedSpecialization(_) => "OwnedSpecialization",
            AstNodeKind::NonFeatureMember(_) => "NonFeatureMember",
            AstNodeKind::ConnectorEnd(_) => "ConnectorEnd",
            AstNodeKind::TypeResultMember(_) => "TypeResultMember",
            AstNodeKind::MultiplicityExpressionMember(_) => "MultiplicityExpressionMember",
            AstNodeKind::AssertConstraintUsage(_) => "AssertConstraintUsage",
            AstNodeKind::Invariant(_) => "Invariant",
            AstNodeKind::Conjugation(_) => "Conjugation",
            AstNodeKind::SourceSuccession(_) => "SourceSuccession",
            AstNodeKind::ActorMember(_) => "ActorMember",
            AstNodeKind::ViewRenderingUsage(_) => "ViewRenderingUsage",
            AstNodeKind::UsagePrefix(_) => "UsagePrefix",
            AstNodeKind::BracketExpression(_) => "BracketExpression",
            AstNodeKind::ExtendedDefinition(_) => "ExtendedDefinition",
            AstNodeKind::OccurrenceUsageElement(_) => "OccurrenceUsageElement",
            AstNodeKind::MultiplicityBounds(_) => "MultiplicityBounds",
            AstNodeKind::Behavior(_) => "Behavior",
            AstNodeKind::InitialNodeMember(_) => "InitialNodeMember",
            AstNodeKind::PortionUsage(_) => "PortionUsage",
            AstNodeKind::FunctionBody(_) => "FunctionBody",
            AstNodeKind::PrimaryArgumentMember(_) => "PrimaryArgumentMember",
            AstNodeKind::NonOccurrenceUsageElement(_) => "NonOccurrenceUsageElement",
            AstNodeKind::TransitionSuccessionMember(_) => "TransitionSuccessionMember",
            AstNodeKind::NaryConnectorDeclaration(_) => "NaryConnectorDeclaration",
            AstNodeKind::AllocationUsage(_) => "AllocationUsage",
            AstNodeKind::FeatureReferenceExpression(_) => "FeatureReferenceExpression",
            AstNodeKind::EnumerationDefinition(_) => "EnumerationDefinition",
            AstNodeKind::Type(_) => "Type",
            AstNodeKind::StatePerformActionUsage(_) => "StatePerformActionUsage",
            AstNodeKind::ArgumentValue(_) => "ArgumentValue",
            AstNodeKind::MessageEvent(_) => "MessageEvent",
            AstNodeKind::AnnotatingElement(_) => "AnnotatingElement",
            AstNodeKind::CalculationBodyPart(_) => "CalculationBodyPart",
            AstNodeKind::OwnedCrossMultiplicityMember(_) => "OwnedCrossMultiplicityMember",
            AstNodeKind::BinaryOperatorExpression(_) => "BinaryOperatorExpression",
            AstNodeKind::ResultExpressionMember(_) => "ResultExpressionMember",
            AstNodeKind::LiteralReal(_) => "LiteralReal",
            AstNodeKind::ConstructorExpression(_) => "ConstructorExpression",
            AstNodeKind::ActionBodyParameterMember(_) => "ActionBodyParameterMember",
            AstNodeKind::NonFeatureChainPrimaryExpression(_) => "NonFeatureChainPrimaryExpression",
            AstNodeKind::NonFeatureChainPrimaryArgumentMember(_) => "NonFeatureChainPrimaryArgumentMember",
            AstNodeKind::BodyExpression(_) => "BodyExpression",
            AstNodeKind::VariantUsageMember(_) => "VariantUsageMember",
            AstNodeKind::TypeBody(_) => "TypeBody",
            AstNodeKind::FunctionOperationExpression(_) => "FunctionOperationExpression",
            AstNodeKind::BooleanValue(_) => "BooleanValue",
            AstNodeKind::RequirementVerificationMember(_) => "RequirementVerificationMember",
            AstNodeKind::NamedArgumentList(_) => "NamedArgumentList",
            AstNodeKind::ActionNodePrefix(_) => "ActionNodePrefix",
            AstNodeKind::SenderReceiverPart(_) => "SenderReceiverPart",
            AstNodeKind::UnaryOperatorExpression(_) => "UnaryOperatorExpression",
            AstNodeKind::RealValue(_) => "RealValue",
            AstNodeKind::DefinitionElement(_) => "DefinitionElement",
            AstNodeKind::Connector(_) => "Connector",
            AstNodeKind::VerificationCaseUsage(_) => "VerificationCaseUsage",
            AstNodeKind::NonFeatureChainPrimaryArgument(_) => "NonFeatureChainPrimaryArgument",
            AstNodeKind::Crosses(_) => "Crosses",
            AstNodeKind::BasicFeaturePrefix(_) => "BasicFeaturePrefix",
            AstNodeKind::NonFeatureChainPrimaryArgumentValue(_) => "NonFeatureChainPrimaryArgumentValue",
            AstNodeKind::ExhibitStateUsage(_) => "ExhibitStateUsage",
            AstNodeKind::NonOccurrenceUsageMember(_) => "NonOccurrenceUsageMember",
            AstNodeKind::ExtendedUsage(_) => "ExtendedUsage",
            AstNodeKind::References(_) => "References",
            AstNodeKind::PayloadFeature(_) => "PayloadFeature",
            AstNodeKind::InterfaceNonOccurrenceUsageMember(_) => "InterfaceNonOccurrenceUsageMember",
            AstNodeKind::DoActionMember(_) => "DoActionMember",
            AstNodeKind::ConjugatedPortDefinition(_) => "ConjugatedPortDefinition",
            AstNodeKind::DefaultReferenceUsage(_) => "DefaultReferenceUsage",
            AstNodeKind::NonBehaviorBodyItem(_) => "NonBehaviorBodyItem",
            AstNodeKind::ConstructorResultMember(_) => "ConstructorResultMember",
            AstNodeKind::JoinNode(_) => "JoinNode",
            AstNodeKind::ActionNodeMember(_) => "ActionNodeMember",
            AstNodeKind::TransitionSuccession(_) => "TransitionSuccession",
            AstNodeKind::SuccessionFlowUsage(_) => "SuccessionFlowUsage",
            AstNodeKind::ViewRenderingMember(_) => "ViewRenderingMember",
            AstNodeKind::EmptyParameterMember(_) => "EmptyParameterMember",
            AstNodeKind::Multiplicity(_) => "Multiplicity",
            AstNodeKind::EnumeratedValue(_) => "EnumeratedValue",
            AstNodeKind::StakeholderUsage(_) => "StakeholderUsage",
            AstNodeKind::TransitionPerformActionUsage(_) => "TransitionPerformActionUsage",
            AstNodeKind::BasicDefinitionPrefix(_) => "BasicDefinitionPrefix",
            AstNodeKind::UsageCompletion(_) => "UsageCompletion",
            AstNodeKind::SpecializationPart(_) => "SpecializationPart",
            AstNodeKind::PartDefinition(_) => "PartDefinition",
            AstNodeKind::RequirementBodyItem(_) => "RequirementBodyItem",
            AstNodeKind::MetadataBodyUsage(_) => "MetadataBodyUsage",
            AstNodeKind::ChainingPart(_) => "ChainingPart",
            AstNodeKind::Feature(_) => "Feature",
            AstNodeKind::EndFeaturePrefix(_) => "EndFeaturePrefix",
            AstNodeKind::InterfacePart(_) => "InterfacePart",
            AstNodeKind::TransitionAcceptActionUsage(_) => "TransitionAcceptActionUsage",
            AstNodeKind::FeatureInverting(_) => "FeatureInverting",
            AstNodeKind::BindingConnectorAsUsage(_) => "BindingConnectorAsUsage",
            AstNodeKind::UsageDeclaration(_) => "UsageDeclaration",
            AstNodeKind::ControlNodePrefix(_) => "ControlNodePrefix",
            AstNodeKind::CaseBodyItem(_) => "CaseBodyItem",
            AstNodeKind::OwnedCrossFeature(_) => "OwnedCrossFeature",
            AstNodeKind::GeneralType(_) => "GeneralType",
            AstNodeKind::UseCaseUsage(_) => "UseCaseUsage",
            AstNodeKind::ItemDefinition(_) => "ItemDefinition",
            AstNodeKind::Disjoining(_) => "Disjoining",
            AstNodeKind::StructureUsageMember(_) => "StructureUsageMember",
            AstNodeKind::ElementFilterMember(_) => "ElementFilterMember",
            AstNodeKind::Intersecting(_) => "Intersecting",
            AstNodeKind::FramedConcernUsage(_) => "FramedConcernUsage",
            AstNodeKind::InterfaceUsageDeclaration(_) => "InterfaceUsageDeclaration",
            AstNodeKind::LiteralExpression(_) => "LiteralExpression",
            AstNodeKind::Step(_) => "Step",
            AstNodeKind::PerformActionUsage(_) => "PerformActionUsage",
            AstNodeKind::MetadataUsage(_) => "MetadataUsage",
            AstNodeKind::Namespace(_) => "Namespace",
            AstNodeKind::FlowEnd(_) => "FlowEnd",
            AstNodeKind::RenderingUsage(_) => "RenderingUsage",
            AstNodeKind::OwnedReferenceSubsetting(_) => "OwnedReferenceSubsetting",
            AstNodeKind::StateAcceptActionUsage(_) => "StateAcceptActionUsage",
            AstNodeKind::ViewDefinition(_) => "ViewDefinition",
            AstNodeKind::FlowFeatureRedefinition(_) => "FlowFeatureRedefinition",
            AstNodeKind::BodyArgumentValue(_) => "BodyArgumentValue",
            AstNodeKind::BehaviorUsageMember(_) => "BehaviorUsageMember",
            AstNodeKind::Structure(_) => "Structure",
            AstNodeKind::EntryTransitionMember(_) => "EntryTransitionMember",
            AstNodeKind::AcceptNodeDeclaration(_) => "AcceptNodeDeclaration",
            AstNodeKind::FeatureBinding(_) => "FeatureBinding",
            AstNodeKind::DataType(_) => "DataType",
            AstNodeKind::BinaryOperator(_) => "BinaryOperator",
            AstNodeKind::Predicate(_) => "Predicate",
            AstNodeKind::EmptyFeature(_) => "EmptyFeature",
            AstNodeKind::RenderingDefinition(_) => "RenderingDefinition",
            AstNodeKind::CalculationDefinition(_) => "CalculationDefinition",
            AstNodeKind::TerminateNode(_) => "TerminateNode",
            AstNodeKind::LiteralInfinity(_) => "LiteralInfinity",
            AstNodeKind::PayloadParameterMember(_) => "PayloadParameterMember",
            AstNodeKind::OccurrenceUsageMember(_) => "OccurrenceUsageMember",
            AstNodeKind::UsageExtensionKeyword(_) => "UsageExtensionKeyword",
            AstNodeKind::ConjugatedPortDefinitionMember(_) => "ConjugatedPortDefinitionMember",
            AstNodeKind::TriggerAction(_) => "TriggerAction",
            AstNodeKind::PrefixMetadataMember(_) => "PrefixMetadataMember",
            AstNodeKind::ViewBody(_) => "ViewBody",
            AstNodeKind::MetaCastOperator(_) => "MetaCastOperator",
            AstNodeKind::DefinitionDeclaration(_) => "DefinitionDeclaration",
            AstNodeKind::SourceSuccessionMember(_) => "SourceSuccessionMember",
            AstNodeKind::MetadataFeatureDeclaration(_) => "MetadataFeatureDeclaration",
            AstNodeKind::OccurrenceUsage(_) => "OccurrenceUsage",
            AstNodeKind::Specialization(_) => "Specialization",
            AstNodeKind::AssignmentNodeDeclaration(_) => "AssignmentNodeDeclaration",
            AstNodeKind::ConcernUsage(_) => "ConcernUsage",
            AstNodeKind::StateAssignmentActionUsage(_) => "StateAssignmentActionUsage",
            AstNodeKind::MetadataArgument(_) => "MetadataArgument",
            AstNodeKind::MetadataDefinition(_) => "MetadataDefinition",
            AstNodeKind::ActionDefinition(_) => "ActionDefinition",
            AstNodeKind::IfNode(_) => "IfNode",
            AstNodeKind::NaryConnectorPart(_) => "NaryConnectorPart",
            AstNodeKind::DifferencingPart(_) => "DifferencingPart",
            AstNodeKind::BooleanExpression(_) => "BooleanExpression",
            AstNodeKind::FlowFeature(_) => "FlowFeature",
            AstNodeKind::RequirementDefinition(_) => "RequirementDefinition",
            AstNodeKind::GuardedTargetSuccession(_) => "GuardedTargetSuccession",
            AstNodeKind::PackageBody(_) => "PackageBody",
            AstNodeKind::FunctionReferenceMember(_) => "FunctionReferenceMember",
            AstNodeKind::DefinitionPrefix(_) => "DefinitionPrefix",
            AstNodeKind::ConditionalBinaryOperatorExpression(_) => "ConditionalBinaryOperatorExpression",
            AstNodeKind::BinaryInterfacePart(_) => "BinaryInterfacePart",
            AstNodeKind::ActionTargetSuccessionMember(_) => "ActionTargetSuccessionMember",
            AstNodeKind::SatisfactionSubjectMember(_) => "SatisfactionSubjectMember",
            AstNodeKind::MessageEventMember(_) => "MessageEventMember",
            AstNodeKind::ActionNodeUsageDeclaration(_) => "ActionNodeUsageDeclaration",
            AstNodeKind::InterfaceEnd(_) => "InterfaceEnd",
            AstNodeKind::PackageDeclaration(_) => "PackageDeclaration",
            AstNodeKind::PayloadFeatureMember(_) => "PayloadFeatureMember",
            AstNodeKind::FlowDeclaration(_) => "FlowDeclaration",
            AstNodeKind::StateUsage(_) => "StateUsage",
            AstNodeKind::OwnedSubclassification(_) => "OwnedSubclassification",
            AstNodeKind::PrefixMetadataFeature(_) => "PrefixMetadataFeature",
            AstNodeKind::EmptyActionUsage(_) => "EmptyActionUsage",
            AstNodeKind::RequirementConstraintMember(_) => "RequirementConstraintMember",
            AstNodeKind::FeaturePrefix(_) => "FeaturePrefix",
            AstNodeKind::Argument(_) => "Argument",
            AstNodeKind::MultiplicityPart(_) => "MultiplicityPart",
            AstNodeKind::UsageBody(_) => "UsageBody",
            AstNodeKind::Annotation(_) => "Annotation",
            AstNodeKind::OccurrenceDefinitionPrefix(_) => "OccurrenceDefinitionPrefix",
            AstNodeKind::Expression(_) => "Expression",
            AstNodeKind::OwnedFeatureInverting(_) => "OwnedFeatureInverting",
            AstNodeKind::PayloadFeatureSpecializationPart(_) => "PayloadFeatureSpecializationPart",
            AstNodeKind::TypeBodyElement(_) => "TypeBodyElement",
            AstNodeKind::OwnedCrossSubsetting(_) => "OwnedCrossSubsetting",
            AstNodeKind::Class(_) => "Class",
            AstNodeKind::LibraryPackage(_) => "LibraryPackage",
            AstNodeKind::PackageBodyElement(_) => "PackageBodyElement",
            AstNodeKind::ViewpointDefinition(_) => "ViewpointDefinition",
            AstNodeKind::TransitionAssignmentActionUsage(_) => "TransitionAssignmentActionUsage",
            AstNodeKind::MetadataValue(_) => "MetadataValue",
            AstNodeKind::InterfaceOccurrenceUsageElement(_) => "InterfaceOccurrenceUsageElement",
            AstNodeKind::ViewBodyItem(_) => "ViewBodyItem",
            AstNodeKind::RequirementBody(_) => "RequirementBody",
            AstNodeKind::EventOccurrenceUsage(_) => "EventOccurrenceUsage",
            AstNodeKind::FlowEndMember(_) => "FlowEndMember",
            AstNodeKind::MetadataFeature(_) => "MetadataFeature",
            AstNodeKind::UnextendedUsagePrefix(_) => "UnextendedUsagePrefix",
            AstNodeKind::BinaryConnectorDeclaration(_) => "BinaryConnectorDeclaration",
            AstNodeKind::AnnotatingMember(_) => "AnnotatingMember",
            AstNodeKind::TransitionSendActionUsage(_) => "TransitionSendActionUsage",
            AstNodeKind::StateBodyItem(_) => "StateBodyItem",
            AstNodeKind::SequenceExpressionList(_) => "SequenceExpressionList",
            AstNodeKind::EmptyEndMember(_) => "EmptyEndMember",
            AstNodeKind::PortionKind(_) => "PortionKind",
            AstNodeKind::RequirementKind(_) => "RequirementKind",
            AstNodeKind::TriggerExpression(_) => "TriggerExpression",
            AstNodeKind::ForLoopNode(_) => "ForLoopNode",
            AstNodeKind::RefPrefix(_) => "RefPrefix",
            AstNodeKind::ActionNode(_) => "ActionNode",
            AstNodeKind::AttributeUsage(_) => "AttributeUsage",
            AstNodeKind::NamedArgumentMember(_) => "NamedArgumentMember",
            AstNodeKind::FlowDefinition(_) => "FlowDefinition",
            AstNodeKind::FunctionReferenceArgumentMember(_) => "FunctionReferenceArgumentMember",
            AstNodeKind::FeatureTyping(_) => "FeatureTyping",
            AstNodeKind::AliasMember(_) => "AliasMember",
            AstNodeKind::Subclassification(_) => "Subclassification",
            AstNodeKind::MetadataBodyFeature(_) => "MetadataBodyFeature",
            AstNodeKind::ElementReferenceMember(_) => "ElementReferenceMember",
            AstNodeKind::AssignmentTargetParameter(_) => "AssignmentTargetParameter",
            AstNodeKind::VisibilityIndicator(_) => "VisibilityIndicator",
            AstNodeKind::SuccessionFlow(_) => "SuccessionFlow",
            AstNodeKind::MultiplicitySubset(_) => "MultiplicitySubset",
            AstNodeKind::MembershipImport(_) => "MembershipImport",
            AstNodeKind::NonFeatureElement(_) => "NonFeatureElement",
            AstNodeKind::PortConjugation(_) => "PortConjugation",
            AstNodeKind::ViewUsage(_) => "ViewUsage",
            AstNodeKind::SatisfyRequirementUsage(_) => "SatisfyRequirementUsage",
            AstNodeKind::InvertingPart(_) => "InvertingPart",
            AstNodeKind::FunctionReferenceArgument(_) => "FunctionReferenceArgument",
            AstNodeKind::EnumerationBody(_) => "EnumerationBody",
            AstNodeKind::MetadataBody(_) => "MetadataBody",
            AstNodeKind::ReturnParameterMember(_) => "ReturnParameterMember",
            AstNodeKind::Classifier(_) => "Classifier",
            AstNodeKind::NamespaceBody(_) => "NamespaceBody",
            AstNodeKind::CaseBody(_) => "CaseBody",
            AstNodeKind::QualifiedName(_) => "QualifiedName",
            AstNodeKind::PackageMember(_) => "PackageMember",
            AstNodeKind::InterfaceOccurrenceUsageMember(_) => "InterfaceOccurrenceUsageMember",
            AstNodeKind::ActionBodyParameter(_) => "ActionBodyParameter",
            AstNodeKind::ConjugationPart(_) => "ConjugationPart",
            AstNodeKind::SendNode(_) => "SendNode",
            AstNodeKind::ForVariableDeclaration(_) => "ForVariableDeclaration",
            AstNodeKind::AnalysisCaseUsage(_) => "AnalysisCaseUsage",
            AstNodeKind::DefinitionExtensionKeyword(_) => "DefinitionExtensionKeyword",
            AstNodeKind::FeatureReference(_) => "FeatureReference",
            AstNodeKind::OwnedFeatureChain(_) => "OwnedFeatureChain",
            AstNodeKind::PositionalArgumentList(_) => "PositionalArgumentList",
            AstNodeKind::NullExpression(_) => "NullExpression",
            AstNodeKind::TypeReference(_) => "TypeReference",
            AstNodeKind::FeatureReferenceMember(_) => "FeatureReferenceMember",
            AstNodeKind::MultiplicityRange(_) => "MultiplicityRange",
            AstNodeKind::EnumerationUsage(_) => "EnumerationUsage",
            AstNodeKind::PayloadParameter(_) => "PayloadParameter",
            AstNodeKind::LiteralBoolean(_) => "LiteralBoolean",
            AstNodeKind::ParameterRedefinition(_) => "ParameterRedefinition",
            AstNodeKind::NamedArgument(_) => "NamedArgument",
            AstNodeKind::MergeNode(_) => "MergeNode",
            AstNodeKind::MembershipExpose(_) => "MembershipExpose",
            AstNodeKind::StateActionUsage(_) => "StateActionUsage",
            AstNodeKind::FeatureChainExpression(_) => "FeatureChainExpression",
            AstNodeKind::AssignmentNode(_) => "AssignmentNode",
            AstNodeKind::TypeFeatureMember(_) => "TypeFeatureMember",
            AstNodeKind::NodeParameter(_) => "NodeParameter",
            AstNodeKind::OwnedCrossMultiplicity(_) => "OwnedCrossMultiplicity",
            AstNodeKind::FlowPayloadFeatureMember(_) => "FlowPayloadFeatureMember",
            AstNodeKind::CalculationUsage(_) => "CalculationUsage",
            AstNodeKind::SubjectMember(_) => "SubjectMember",
            AstNodeKind::ArgumentExpressionValue(_) => "ArgumentExpressionValue",
            AstNodeKind::NamespaceBodyElement(_) => "NamespaceBodyElement",
            AstNodeKind::PrefixMetadataAnnotation(_) => "PrefixMetadataAnnotation",
            AstNodeKind::TypePrefix(_) => "TypePrefix",
            AstNodeKind::SuccessionAsUsage(_) => "SuccessionAsUsage",
            AstNodeKind::PrefixMetadataUsage(_) => "PrefixMetadataUsage",
            AstNodeKind::FeatureDirection(_) => "FeatureDirection",
            AstNodeKind::MetaclassificationExpression(_) => "MetaclassificationExpression",
            AstNodeKind::FunctionBodyPart(_) => "FunctionBodyPart",
            AstNodeKind::IndividualDefinition(_) => "IndividualDefinition",
            AstNodeKind::UnaryOperator(_) => "UnaryOperator",
            AstNodeKind::ClassificationTestOperator(_) => "ClassificationTestOperator",
            AstNodeKind::CastOperator(_) => "CastOperator",
            AstNodeKind::InterfaceBody(_) => "InterfaceBody",
            AstNodeKind::DisjoiningPart(_) => "DisjoiningPart",
            AstNodeKind::FeatureChainPrefix(_) => "FeatureChainPrefix",
            AstNodeKind::RequirementUsage(_) => "RequirementUsage",
            AstNodeKind::EmptyResultMember(_) => "EmptyResultMember",
            AstNodeKind::ReferenceUsage(_) => "ReferenceUsage",
            AstNodeKind::MetadataAccessExpression(_) => "MetadataAccessExpression",
            AstNodeKind::CaseDefinition(_) => "CaseDefinition",
            AstNodeKind::FeatureChain(_) => "FeatureChain",
            AstNodeKind::Succession(_) => "Succession",
            AstNodeKind::UsageElement(_) => "UsageElement",
            AstNodeKind::IncludeUseCaseUsage(_) => "IncludeUseCaseUsage",
            AstNodeKind::ConnectorPart(_) => "ConnectorPart",
            AstNodeKind::TargetTransitionUsageMember(_) => "TargetTransitionUsageMember",
            AstNodeKind::FeatureSpecialization(_) => "FeatureSpecialization",
            AstNodeKind::LiteralInteger(_) => "LiteralInteger",
            AstNodeKind::InterfaceUsage(_) => "InterfaceUsage",
            AstNodeKind::RequirementConstraintUsage(_) => "RequirementConstraintUsage",
            AstNodeKind::FeatureValue(_) => "FeatureValue",
            AstNodeKind::TriggerValuePart(_) => "TriggerValuePart",
            AstNodeKind::AcceptNode(_) => "AcceptNode",
            AstNodeKind::FilterPackageMember(_) => "FilterPackageMember",
            AstNodeKind::SourceEnd(_) => "SourceEnd",
            AstNodeKind::Association(_) => "Association",
            AstNodeKind::ExtentExpression(_) => "ExtentExpression",
            AstNodeKind::WhileLoopNode(_) => "WhileLoopNode",
            AstNodeKind::DefinitionBody(_) => "DefinitionBody",
            AstNodeKind::ImportDeclaration(_) => "ImportDeclaration",
            AstNodeKind::ClassificationExpression(_) => "ClassificationExpression",
            AstNodeKind::OwnedFeatureChaining(_) => "OwnedFeatureChaining",
            AstNodeKind::NaryInterfacePart(_) => "NaryInterfacePart",
            AstNodeKind::ConnectorDeclaration(_) => "ConnectorDeclaration",
            AstNodeKind::GuardExpressionMember(_) => "GuardExpressionMember",
            AstNodeKind::OccurrenceDefinition(_) => "OccurrenceDefinition",
            AstNodeKind::Differencing(_) => "Differencing",
            AstNodeKind::FramedConcernMember(_) => "FramedConcernMember",
            AstNodeKind::ExitActionMember(_) => "ExitActionMember",
            AstNodeKind::NamespaceExpose(_) => "NamespaceExpose",
            AstNodeKind::AssignmentTargetBinding(_) => "AssignmentTargetBinding",
            AstNodeKind::PartUsage(_) => "PartUsage",
            AstNodeKind::ForVariableDeclarationMember(_) => "ForVariableDeclarationMember",
            AstNodeKind::StateSendActionUsage(_) => "StateSendActionUsage",
            AstNodeKind::MetadataBodyFeatureMember(_) => "MetadataBodyFeatureMember",
            AstNodeKind::ConnectionUsage(_) => "ConnectionUsage",
            AstNodeKind::NodeParameterMember(_) => "NodeParameterMember",
            AstNodeKind::Function(_) => "Function",
            AstNodeKind::VariantUsageElement(_) => "VariantUsageElement",
            AstNodeKind::FeatureIdentification(_) => "FeatureIdentification",
            AstNodeKind::OwnedFeatureChainMember(_) => "OwnedFeatureChainMember",
            AstNodeKind::FlowEndSubsetting(_) => "FlowEndSubsetting",
            AstNodeKind::ConjugatedPortTyping(_) => "ConjugatedPortTyping",
            AstNodeKind::TargetTransitionUsage(_) => "TargetTransitionUsage",
            AstNodeKind::RequirementVerificationUsage(_) => "RequirementVerificationUsage",
            AstNodeKind::ActionUsage(_) => "ActionUsage",
            AstNodeKind::CaseUsage(_) => "CaseUsage",
            AstNodeKind::TextualRepresentation(_) => "TextualRepresentation",
            AstNodeKind::ConcernDefinition(_) => "ConcernDefinition",
            AstNodeKind::OwnedCrossFeatureMember(_) => "OwnedCrossFeatureMember",
            AstNodeKind::PrimaryArgumentValue(_) => "PrimaryArgumentValue",
            AstNodeKind::OccurrenceUsagePrefix(_) => "OccurrenceUsagePrefix",
            AstNodeKind::Comment(_) => "Comment",
            AstNodeKind::AllocationDefinition(_) => "AllocationDefinition",
            AstNodeKind::ActorUsage(_) => "ActorUsage",
            AstNodeKind::MetadataReference(_) => "MetadataReference",
            AstNodeKind::EmptyUsage(_) => "EmptyUsage",
            AstNodeKind::FeatureDeclaration(_) => "FeatureDeclaration",
            AstNodeKind::RelationshipBody(_) => "RelationshipBody",
            AstNodeKind::FeatureRelationshipPart(_) => "FeatureRelationshipPart",
            AstNodeKind::ConditionalBinaryOperator(_) => "ConditionalBinaryOperator",
            AstNodeKind::BehaviorUsageElement(_) => "BehaviorUsageElement",
            AstNodeKind::ActionBehaviorMember(_) => "ActionBehaviorMember",
            AstNodeKind::ValuePart(_) => "ValuePart",
            AstNodeKind::StateDefBody(_) => "StateDefBody",
            AstNodeKind::Redefinition(_) => "Redefinition",
            AstNodeKind::ActionBodyItem(_) => "ActionBodyItem",
            AstNodeKind::ClassifierDeclaration(_) => "ClassifierDeclaration",
            AstNodeKind::SatisfactionReferenceExpression(_) => "SatisfactionReferenceExpression",
            AstNodeKind::Subsetting(_) => "Subsetting",
            AstNodeKind::RelationshipOwnedElement(_) => "RelationshipOwnedElement",
            AstNodeKind::ArgumentMember(_) => "ArgumentMember",
            AstNodeKind::Subsets(_) => "Subsets",
            AstNodeKind::ArgumentList(_) => "ArgumentList",
            AstNodeKind::Redefinitions(_) => "Redefinitions",
            AstNodeKind::IndividualUsage(_) => "IndividualUsage",
            AstNodeKind::GuardedSuccession(_) => "GuardedSuccession",
            AstNodeKind::EntryActionMember(_) => "EntryActionMember",
            AstNodeKind::PrimaryExpression(_) => "PrimaryExpression",
            AstNodeKind::BindingConnectorDeclaration(_) => "BindingConnectorDeclaration",
            AstNodeKind::OwnedExpressionMember(_) => "OwnedExpressionMember",
            AstNodeKind::InterfaceEndMember(_) => "InterfaceEndMember",
            AstNodeKind::SendNodeDeclaration(_) => "SendNodeDeclaration",
            AstNodeKind::ReferenceTyping(_) => "ReferenceTyping",
            AstNodeKind::InstantiatedTypeReference(_) => "InstantiatedTypeReference",
            AstNodeKind::Subsettings(_) => "Subsettings",
            AstNodeKind::OwnedMultiplicity(_) => "OwnedMultiplicity",
            AstNodeKind::AttributeDefinition(_) => "AttributeDefinition",
            AstNodeKind::PortDefinition(_) => "PortDefinition",
            AstNodeKind::OwnedMultiplicityRange(_) => "OwnedMultiplicityRange",
            AstNodeKind::UseCaseDefinition(_) => "UseCaseDefinition",
            AstNodeKind::GuardedSuccessionMember(_) => "GuardedSuccessionMember",
            AstNodeKind::AssociationStructure(_) => "AssociationStructure",
            AstNodeKind::ExpressionBody(_) => "ExpressionBody",
            AstNodeKind::ConstraintUsage(_) => "ConstraintUsage",
            AstNodeKind::PrimaryArgument(_) => "PrimaryArgument",
            AstNodeKind::Import(_) => "Import",
            AstNodeKind::ReturnFeatureMember(_) => "ReturnFeatureMember",
            AstNodeKind::FeatureMember(_) => "FeatureMember",
            AstNodeKind::ForkNode(_) => "ForkNode",
            AstNodeKind::ConstructorResult(_) => "ConstructorResult",
            AstNodeKind::StructureUsageElement(_) => "StructureUsageElement",
            AstNodeKind::NamespaceFeatureMember(_) => "NamespaceFeatureMember",
            AstNodeKind::MetaclassificationTestOperator(_) => "MetaclassificationTestOperator",
            AstNodeKind::ViewDefinitionBody(_) => "ViewDefinitionBody",
            AstNodeKind::ConnectionDefinition(_) => "ConnectionDefinition",
            AstNodeKind::OwnedDisjoining(_) => "OwnedDisjoining",
            AstNodeKind::ConditionalExpression(_) => "ConditionalExpression",
            AstNodeKind::PerformActionUsageDeclaration(_) => "PerformActionUsageDeclaration",
            AstNodeKind::Package(_) => "Package",
            AstNodeKind::SequenceExpressionListMember(_) => "SequenceExpressionListMember",
            AstNodeKind::StateUsageBody(_) => "StateUsageBody",
            AstNodeKind::CalculationBodyItem(_) => "CalculationBodyItem",
            AstNodeKind::CollectExpression(_) => "CollectExpression",
            AstNodeKind::OwnedRelatedElement(_) => "OwnedRelatedElement",
            AstNodeKind::ConnectorEndMember(_) => "ConnectorEndMember",
            AstNodeKind::LiteralString(_) => "LiteralString",
            AstNodeKind::EnumerationUsageMember(_) => "EnumerationUsageMember",
            AstNodeKind::MessageDeclaration(_) => "MessageDeclaration",
            AstNodeKind::CalculationUsageDeclaration(_) => "CalculationUsageDeclaration",
            AstNodeKind::FilterPackageImport(_) => "FilterPackageImport",
            AstNodeKind::InvocationTypeMember(_) => "InvocationTypeMember",
        }
    }
}

