use crate::model::{
    metameme::{MetaMemeOntology, MetaMemes},
    PrimeOntology, SemanticConcept,
};
use rrust_kontekst_base::{McpError, McpToolInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

/// Bridge between ontology systems and MCP protocol
/// Provides semantic organization and categorization of tools
#[derive(Debug, Clone, Serialize)]
pub struct OntologyMcpBridge {
    pub prime_ontology: PrimeOntology,
    pub metameme_ontology: MetaMemeOntology,
    pub tool_categories: HashMap<String, ToolCategory>,
    pub semantic_tool_mapping: HashMap<String, SemanticToolInfo>,
    pub ontology_tool_registry: HashMap<SemanticConcept, Vec<String>>,
}

/// Semantic categorization of tools based on prime ontology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCategory {
    pub name: String,
    pub prime_association: u64,
    pub semantic_concept: SemanticConcept,
    pub description: String,
    pub tools: Vec<String>,
    pub priority: f32,
}

/// Enhanced tool information with semantic context
#[derive(Debug, Clone, Serialize)]
pub struct SemanticToolInfo {
    #[serde(skip)]
    pub base_info: McpToolInfo,
    pub semantic_category: SemanticConcept,
    pub prime_encoding: Vec<u64>,
    pub semantic_vector: Vec<f32>,
    pub ontological_context: OntologicalContext,
    pub language_associations: Vec<MetaMemes>,
    pub complexity_score: f32,
    pub semantic_relationships: Vec<String>,
}

/// Ontological context for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologicalContext {
    pub primary_domain: OntologyDomain,
    pub secondary_domains: Vec<OntologyDomain>,
    pub conceptual_level: ConceptualLevel,
    pub abstraction_layer: AbstractionLayer,
    pub emergence_properties: EmergenceProperties,
}

/// Domains within the ontological system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OntologyDomain {
    /// 0 - Void - System initialization, null operations
    Void,
    /// 1 - Unity - Core system operations, foundational tools
    Unity,
    /// 2 - Duality - Binary operations, comparisons, choices
    Duality,
    /// 3 - Trinity - Compilation, synthesis, transformation
    Trinity,
    /// 5 - Quintessence - Life cycle, ecosystem management
    Quintessence,
    /// 7 - Completion - Finalization, optimization, perfection
    Completion,
    /// 11 - Transcendence - Meta-programming, reflection, proofs
    Transcendence,
    /// 13 - Transformation - Metamorphosis, evolution, adaptation
    Transformation,
    /// 17 - Star - Guidance, navigation, discovery
    Star,
    /// 23 - Cosmic Order - Universal structure, coordination
    CosmicOrder,
}

/// Levels of conceptual abstraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptualLevel {
    /// Direct hardware/system interaction
    Hardware,
    /// Operating system level
    System,
    /// Runtime environment
    Runtime,
    /// Application framework
    Framework,
    /// Business logic
    Application,
    /// User interface
    Interface,
    /// Conceptual/abstract
    Conceptual,
    /// Meta-conceptual
    Meta,
}

/// Abstraction layers in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbstractionLayer {
    /// Physical layer (hardware)
    Physical,
    /// Data layer (storage, memory)
    Data,
    /// Logic layer (computation)
    Logic,
    /// Service layer (APIs, protocols)
    Service,
    /// Presentation layer (UI, visualization)
    Presentation,
    /// Semantic layer (meaning, understanding)
    Semantic,
    /// Ontological layer (being, existence)
    Ontological,
}

/// Properties of emergence in tool interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceProperties {
    pub has_emergent_behavior: bool,
    pub complexity_amplification: f32,
    pub network_effects: bool,
    pub self_organization: bool,
    pub adaptive_capacity: f32,
    pub recursive_depth: u8,
}

/// Enhanced tool execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologicalExecutionContext {
    pub tool_name: String,
    pub semantic_context: SemanticContext,
    pub prime_state: PrimeState,
    pub execution_metadata: ExecutionMetadata,
    pub relationship_graph: Vec<ToolRelationship>,
}

/// Semantic context for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticContext {
    pub active_concepts: Vec<SemanticConcept>,
    pub conceptual_weights: HashMap<SemanticConcept, f32>,
    pub semantic_momentum: Vec<f32>,
    pub contextual_primes: Vec<u64>,
}

/// State of prime ontology during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeState {
    pub active_primes: HashSet<u64>,
    pub prime_activations: HashMap<u64, f32>,
    pub harmonic_resonance: f32,
    pub ontological_stability: f32,
}

/// Metadata about tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub execution_id: String,
    pub timestamp: u64,
    pub complexity_score: f32,
    pub predicted_duration: f32,
    pub resource_requirements: ResourceRequirements,
}

/// Resource requirements for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cognitive_load: f32,
    pub computational_complexity: f32,
    pub memory_requirements: f32,
    pub semantic_depth: f32,
    pub ontological_bandwidth: f32,
}

/// Relationships between tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRelationship {
    pub target_tool: String,
    pub relationship_type: RelationshipType,
    pub strength: f32,
    pub semantic_compatibility: f32,
    pub prime_resonance: f32,
}

/// Types of relationships between tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Tools that work together synergistically
    Synergistic,
    /// Tools that are alternatives to each other
    Alternative,
    /// Tools that depend on each other
    Dependent,
    /// Tools that conflict with each other
    Conflicting,
    /// Tools that complement each other
    Complementary,
    /// Tools that enhance each other
    Enhancing,
    /// Tools that transform output of other tools
    Transformative,
}

impl OntologyMcpBridge {
    /// Creates a new ontology-MCP bridge
    pub fn new() -> Self {
        let prime_ontology = PrimeOntology::new();
        let metameme_ontology = MetaMemeOntology::new();

        let mut tool_categories = HashMap::new();
        let mut semantic_tool_mapping = HashMap::new();
        let mut ontology_tool_registry = HashMap::new();

        // Initialize tool categories based on prime ontology
        for (&prime, concept) in &prime_ontology.semantic_mappings {
            let category = ToolCategory {
                name: format!("{:?}", concept),
                prime_association: prime,
                semantic_concept: concept.clone(),
                description: Self::get_concept_description(concept),
                tools: Vec::new(),
                priority: Self::compute_priority(prime),
            };
            tool_categories.insert(format!("{:?}", concept), category);
            ontology_tool_registry.insert(concept.clone(), Vec::new());
        }

        Self {
            prime_ontology,
            metameme_ontology,
            tool_categories,
            semantic_tool_mapping,
            ontology_tool_registry,
        }
    }

    /// Registers a tool with semantic context
    pub fn register_semantic_tool(&mut self, tool_info: McpToolInfo) -> Result<String, McpError> {
        let semantic_info = self.analyze_tool_semantics(&tool_info);
        let tool_id = format!("{}::{}", tool_info.component_name, tool_info.tool_name);

        // Add tool to appropriate category
        if let Some(category) = self
            .tool_categories
            .get_mut(&format!("{:?}", semantic_info.semantic_category))
        {
            category.tools.push(tool_id.clone());
        }

        // Add to ontology registry
        if let Some(tools) = self
            .ontology_tool_registry
            .get_mut(&semantic_info.semantic_category)
        {
            tools.push(tool_id.clone());
        }

        self.semantic_tool_mapping
            .insert(tool_id.clone(), semantic_info);

        Ok(tool_id)
    }

    /// Analyzes tool semantics to determine categorization
    fn analyze_tool_semantics(&self, tool_info: &McpToolInfo) -> SemanticToolInfo {
        let semantic_category = self.infer_semantic_category(tool_info);
        let prime_encoding = self.encode_tool_with_primes(tool_info);
        let semantic_vector = self.prime_ontology.encode_concept(tool_info.description);

        let ontological_context = OntologicalContext {
            primary_domain: self.map_concept_to_domain(&semantic_category),
            secondary_domains: self.infer_secondary_domains(tool_info),
            conceptual_level: self.infer_conceptual_level(tool_info),
            abstraction_layer: self.infer_abstraction_layer(tool_info),
            emergence_properties: self.analyze_emergence_properties(tool_info),
        };

        SemanticToolInfo {
            base_info: tool_info.clone(),
            semantic_category,
            prime_encoding,
            semantic_vector,
            ontological_context,
            language_associations: self.infer_language_associations(tool_info),
            complexity_score: self.compute_complexity_score(tool_info),
            semantic_relationships: self.find_semantic_relationships(tool_info),
        }
    }

    /// Infers semantic category from tool information
    fn infer_semantic_category(&self, tool_info: &McpToolInfo) -> SemanticConcept {
        let description = tool_info.description.to_lowercase();
        let tool_name = tool_info.tool_name.to_lowercase();

        // Pattern matching for semantic categorization
        if description.contains("compile")
            || description.contains("build")
            || tool_name.contains("compile")
        {
            SemanticConcept::Trinity
        } else if description.contains("proof")
            || description.contains("theorem")
            || tool_name.contains("proof")
        {
            SemanticConcept::Transcendence
        } else if description.contains("transform")
            || description.contains("convert")
            || tool_name.contains("transform")
        {
            SemanticConcept::Transformation
        } else if description.contains("analyze")
            || description.contains("search")
            || tool_name.contains("analyze")
        {
            SemanticConcept::Star
        } else if description.contains("manage")
            || description.contains("coordinate")
            || tool_name.contains("manage")
        {
            SemanticConcept::CosmicOrder
        } else if description.contains("create")
            || description.contains("generate")
            || tool_name.contains("create")
        {
            SemanticConcept::Quintessence
        } else if description.contains("optimize")
            || description.contains("complete")
            || tool_name.contains("optimize")
        {
            SemanticConcept::Completion
        } else if description.contains("choose")
            || description.contains("select")
            || tool_name.contains("choose")
        {
            SemanticConcept::Duality
        } else if description.contains("init")
            || description.contains("start")
            || tool_name.contains("init")
        {
            SemanticConcept::Unity
        } else {
            SemanticConcept::Void
        }
    }

    /// Encodes tool with prime numbers based on characteristics
    fn encode_tool_with_primes(&self, tool_info: &McpToolInfo) -> Vec<u64> {
        let mut encoding = Vec::new();

        // Base prime from semantic category
        let semantic_category = self.infer_semantic_category(tool_info);
        if let Some((&prime, _)) = self
            .prime_ontology
            .semantic_mappings
            .iter()
            .find(|(_, concept)| **concept == semantic_category)
        {
            encoding.push(prime);
        }

        // Additional primes based on features
        if tool_info.mcp_enabled {
            encoding.push(2); // Duality for MCP capability
        }

        if tool_info.visible {
            encoding.push(3); // Trinity for visibility
        }

        // Order influences prime selection
        if tool_info.order > 0 {
            let order_prime = self
                .prime_ontology
                .nearest_prime(tool_info.order as u64)
                .unwrap_or(1);
            encoding.push(order_prime);
        }

        encoding
    }

    /// Maps semantic concept to ontology domain
    fn map_concept_to_domain(&self, concept: &SemanticConcept) -> OntologyDomain {
        match concept {
            SemanticConcept::Void => OntologyDomain::Void,
            SemanticConcept::Unity => OntologyDomain::Unity,
            SemanticConcept::Duality => OntologyDomain::Duality,
            SemanticConcept::Trinity => OntologyDomain::Trinity,
            SemanticConcept::Quintessence => OntologyDomain::Quintessence,
            SemanticConcept::Completion => OntologyDomain::Completion,
            SemanticConcept::Transcendence => OntologyDomain::Transcendence,
            SemanticConcept::Transformation => OntologyDomain::Transformation,
            SemanticConcept::Star => OntologyDomain::Star,
            SemanticConcept::CosmicOrder => OntologyDomain::CosmicOrder,
        }
    }

    /// Infers secondary domains based on tool complexity
    fn infer_secondary_domains(&self, tool_info: &McpToolInfo) -> Vec<OntologyDomain> {
        let mut domains = Vec::new();

        // Complex tools span multiple domains
        if tool_info.parameters.len() > 3 {
            domains.push(OntologyDomain::CosmicOrder);
        }

        if tool_info.description.len() > 100 {
            domains.push(OntologyDomain::Completion);
        }

        domains
    }

    /// Infers conceptual level of the tool
    fn infer_conceptual_level(&self, tool_info: &McpToolInfo) -> ConceptualLevel {
        let description = tool_info.description.to_lowercase();

        if description.contains("hardware") || description.contains("system") {
            ConceptualLevel::Hardware
        } else if description.contains("runtime") || description.contains("execution") {
            ConceptualLevel::Runtime
        } else if description.contains("framework") || description.contains("library") {
            ConceptualLevel::Framework
        } else if description.contains("application") || description.contains("business") {
            ConceptualLevel::Application
        } else if description.contains("interface") || description.contains("ui") {
            ConceptualLevel::Interface
        } else if description.contains("concept") || description.contains("abstract") {
            ConceptualLevel::Conceptual
        } else if description.contains("meta") || description.contains("ontology") {
            ConceptualLevel::Meta
        } else {
            ConceptualLevel::System
        }
    }

    /// Infers abstraction layer
    fn infer_abstraction_layer(&self, tool_info: &McpToolInfo) -> AbstractionLayer {
        let description = tool_info.description.to_lowercase();

        if description.contains("physical") || description.contains("hardware") {
            AbstractionLayer::Physical
        } else if description.contains("data") || description.contains("storage") {
            AbstractionLayer::Data
        } else if description.contains("logic") || description.contains("computation") {
            AbstractionLayer::Logic
        } else if description.contains("service") || description.contains("api") {
            AbstractionLayer::Service
        } else if description.contains("presentation") || description.contains("ui") {
            AbstractionLayer::Presentation
        } else if description.contains("semantic") || description.contains("meaning") {
            AbstractionLayer::Semantic
        } else if description.contains("ontological") || description.contains("ontology") {
            AbstractionLayer::Ontological
        } else {
            AbstractionLayer::Logic
        }
    }

    /// Analyzes emergence properties
    fn analyze_emergence_properties(&self, tool_info: &McpToolInfo) -> EmergenceProperties {
        let description = tool_info.description.to_lowercase();

        EmergenceProperties {
            has_emergent_behavior: description.contains("emergent")
                || description.contains("complex"),
            complexity_amplification: if tool_info.parameters.len() > 2 {
                1.5
            } else {
                1.0
            },
            network_effects: description.contains("network") || description.contains("distributed"),
            self_organization: description.contains("self") || description.contains("auto"),
            adaptive_capacity: if description.contains("adaptive") {
                1.0
            } else {
                0.5
            },
            recursive_depth: if description.contains("recursive") {
                3
            } else {
                1
            },
        }
    }

    /// Infers language associations
    fn infer_language_associations(&self, tool_info: &McpToolInfo) -> Vec<MetaMemes> {
        let description = tool_info.description.to_lowercase();
        let mut associations = Vec::new();

        if description.contains("rust") {
            associations.push(MetaMemes::Rust);
        }
        if description.contains("haskell") {
            associations.push(MetaMemes::Haskell);
        }
        if description.contains("lean") {
            associations.push(MetaMemes::Lean4);
        }
        if description.contains("gcc") {
            associations.push(MetaMemes::Gcc);
        }
        if description.contains("llvm") {
            associations.push(MetaMemes::LLVM);
        }

        associations
    }

    /// Computes complexity score
    fn compute_complexity_score(&self, tool_info: &McpToolInfo) -> f32 {
        let base_score = 0.5;
        let param_score = tool_info.parameters.len() as f32 * 0.1;
        let description_score = (tool_info.description.len() as f32 / 100.0) * 0.2;
        let order_score = (tool_info.order as f32 / 10.0) * 0.1;

        base_score + param_score + description_score + order_score
    }

    /// Finds semantic relationships with other tools
    fn find_semantic_relationships(&self, tool_info: &McpToolInfo) -> Vec<String> {
        let mut relationships = Vec::new();

        // For now, return empty - would need existing tool registry
        // This would be populated as more tools are registered

        relationships
    }

    /// Gets description for semantic concept
    fn get_concept_description(concept: &SemanticConcept) -> String {
        match concept {
            SemanticConcept::Void => {
                "Tools for initialization, void operations, and system bootstrapping".to_string()
            }
            SemanticConcept::Unity => {
                "Core foundational tools, unity operations, and system unity".to_string()
            }
            SemanticConcept::Duality => {
                "Binary operations, choices, comparisons, and dualistic processes".to_string()
            }
            SemanticConcept::Trinity => {
                "Compilation, synthesis, transformation, and triadic operations".to_string()
            }
            SemanticConcept::Quintessence => {
                "Life cycle management, ecosystem tools, and vital processes".to_string()
            }
            SemanticConcept::Completion => {
                "Finalization, optimization, completion, and perfection tools".to_string()
            }
            SemanticConcept::Transcendence => {
                "Meta-programming, proofs, transcendent operations, and higher-order tools"
                    .to_string()
            }
            SemanticConcept::Transformation => {
                "Metamorphosis, evolution, adaptation, and transformative processes".to_string()
            }
            SemanticConcept::Star => {
                "Guidance, navigation, discovery, and illumination tools".to_string()
            }
            SemanticConcept::CosmicOrder => {
                "Universal structure, coordination, and cosmic ordering tools".to_string()
            }
        }
    }

    /// Computes priority based on prime number
    fn compute_priority(prime: u64) -> f32 {
        match prime {
            0 => 0.1,  // Void - lowest priority
            1 => 1.0,  // Unity - highest priority
            2 => 0.8,  // Duality - high priority
            3 => 0.9,  // Trinity - very high priority
            5 => 0.7,  // Quintessence - medium-high priority
            7 => 0.6,  // Completion - medium priority
            11 => 0.5, // Transcendence - medium-low priority
            13 => 0.4, // Transformation - low-medium priority
            17 => 0.3, // Star - low priority
            23 => 0.2, // CosmicOrder - very low priority
            _ => 0.1,
        }
    }

    /// Finds tools by semantic category
    pub fn find_tools_by_concept(&self, concept: &SemanticConcept) -> Vec<&SemanticToolInfo> {
        self.semantic_tool_mapping
            .values()
            .filter(|tool| tool.semantic_category == *concept)
            .collect()
    }

    /// Recommends tools based on semantic similarity
    pub fn recommend_tools(&self, query: &str, limit: usize) -> Vec<(&String, f32)> {
        let query_encoding = self.prime_ontology.encode_concept(query);

        let mut similarities: Vec<(&String, f32)> = self
            .semantic_tool_mapping
            .iter()
            .map(|(name, tool)| {
                let similarity =
                    self.compute_semantic_similarity(&query_encoding, &tool.semantic_vector);
                (name, similarity)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(limit);

        similarities
    }

    /// Computes semantic similarity between vectors
    fn compute_semantic_similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() {
            return 0.0;
        }

        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 > 0.0 && norm2 > 0.0 {
            dot_product / (norm1 * norm2)
        } else {
            0.0
        }
    }

    /// Creates execution context for a tool
    pub fn create_execution_context(&self, tool_name: &str) -> Option<OntologicalExecutionContext> {
        let tool_info = self.semantic_tool_mapping.get(tool_name)?;

        let semantic_context = SemanticContext {
            active_concepts: vec![tool_info.semantic_category.clone()],
            conceptual_weights: {
                let mut weights = HashMap::new();
                weights.insert(tool_info.semantic_category.clone(), 1.0);
                weights
            },
            semantic_momentum: tool_info.semantic_vector.clone(),
            contextual_primes: tool_info.prime_encoding.clone(),
        };

        let prime_state = PrimeState {
            active_primes: tool_info.prime_encoding.iter().cloned().collect(),
            prime_activations: {
                let mut activations = HashMap::new();
                for &prime in &tool_info.prime_encoding {
                    activations.insert(prime, 1.0);
                }
                activations
            },
            harmonic_resonance: 0.618, // Golden ratio
            ontological_stability: 0.8,
        };

        Some(OntologicalExecutionContext {
            tool_name: tool_name.to_string(),
            semantic_context,
            prime_state,
            execution_metadata: ExecutionMetadata {
                execution_id: format!("exec_{}", chrono::Utc::now().timestamp()),
                timestamp: chrono::Utc::now().timestamp() as u64,
                complexity_score: tool_info.complexity_score,
                predicted_duration: tool_info.complexity_score * 1000.0, // ms
                resource_requirements: ResourceRequirements {
                    cognitive_load: tool_info.complexity_score * 0.8,
                    computational_complexity: tool_info.complexity_score,
                    memory_requirements: tool_info.complexity_score * 0.6,
                    semantic_depth: tool_info.complexity_score * 0.9,
                    ontological_bandwidth: tool_info.complexity_score * 0.7,
                },
            },
            relationship_graph: Vec::new(), // Would be populated with actual relationships
        })
    }
}

impl Default for OntologyMcpBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrust_kontekst_base::McpToolInfo;

    #[test]
    fn test_ontology_mcp_bridge_creation() {
        let bridge = OntologyMcpBridge::new();
        assert_eq!(bridge.tool_categories.len(), 10); // One for each prime concept
        assert_eq!(bridge.ontology_tool_registry.len(), 10);
    }

    #[test]
    fn test_semantic_tool_registration() {
        let mut bridge = OntologyMcpBridge::new();

        let tool_info = McpToolInfo {
            component_name: "test_component",
            tool_name: "compile_tool",
            menu_type: "build",
            label: "Compile Code",
            emoji: "⚙️",
            description: "Compile source code to executable",
            visible: true,
            order: 1,
            mcp_enabled: true,
            parameters: &["source_file", "output_file"],
            returns: "compilation_result",
        };

        let result = bridge.register_semantic_tool(tool_info);
        assert!(result.is_ok());

        let tool_id = result.unwrap();
        assert!(bridge.semantic_tool_mapping.contains_key(&tool_id));
    }

    #[test]
    fn test_tool_recommendation() {
        let mut bridge = OntologyMcpBridge::new();

        let tool_info = McpToolInfo {
            component_name: "test_component",
            tool_name: "analyze_tool",
            menu_type: "analysis",
            label: "Analyze Code",
            emoji: "🔍",
            description: "Analyze source code for patterns",
            visible: true,
            order: 2,
            mcp_enabled: true,
            parameters: &["source_file"],
            returns: "analysis_result",
        };

        let _ = bridge.register_semantic_tool(tool_info);

        let recommendations = bridge.recommend_tools("analyze code", 5);
        assert!(recommendations.len() > 0);
    }

    #[test]
    fn test_execution_context_creation() {
        let mut bridge = OntologyMcpBridge::new();

        let tool_info = McpToolInfo {
            component_name: "test_component",
            tool_name: "test_tool",
            menu_type: "test",
            label: "Test Tool",
            emoji: "🧪",
            description: "Test tool for analysis",
            visible: true,
            order: 1,
            mcp_enabled: true,
            parameters: &["input"],
            returns: "output",
        };

        let tool_id = bridge.register_semantic_tool(tool_info).unwrap();
        let context = bridge.create_execution_context(&tool_id);

        assert!(context.is_some());
        let ctx = context.unwrap();
        assert_eq!(ctx.tool_name, tool_id);
        assert!(ctx.semantic_context.active_concepts.len() > 0);
    }
}
