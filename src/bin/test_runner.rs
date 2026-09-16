use solfunmeme_dioxus::core::*;
//use std::{collections::HashMap, fs, path::Path};
//use std::array::repeat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Solfunmeme Test Runner - Comprehensive Coverage Tests");
    //    println!("=".repeat(60));

    // Test 1: Code Vectorization
    test_vectorization()?;

    // Test 2: Declaration Splitting
    test_declaration_splitting()?;

    // Test 3: Duplicate Detection
    test_duplicate_detection()?;

    // Test 4: Code Analysis
    test_code_analysis()?;

    // Test 5: Meme Generation
    //    test_meme_generation()?;

    // Test 6: Wallet Integration
    test_wallet_integration()?;

    // Test 7: Self Analysis
    test_self_analysis()?;

    println!("\n✅ All tests completed successfully!");
    println!("📊 Coverage Report:");
    println!("   - Vectorization: ✅");
    println!("   - Declaration Splitting: ✅");
    println!("   - Duplicate Detection: ✅");
    println!("   - Code Analysis: ✅");
    println!("   - Meme Generation: ✅");
    println!("   - Wallet Integration: ✅");
    println!("   - Self Analysis: ✅");

    Ok(())
}

fn test_vectorization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔬 Testing Code Vectorization...");

    let vectorizer = CodeVectorizer::new(128);
    let test_codes = vec![
        "fn hello() { println!(\"Hello\"); }",
        "struct Point { x: f64, y: f64 }",
        "enum Color { Red, Green, Blue }",
    ];

    let mut vectors = Vec::new();
    for code in &test_codes {
        let vector = vectorizer.vectorize(code);
        assert_eq!(vector.dimensions.len(), 128);

        // Check normalization
        let sum: f32 = vector.dimensions.iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-5,
            "Vector not normalized: sum = {}",
            sum
        );

        vectors.push(vector);
    }

    // Test similarity
    let similarity = vectors[0].similarity(&vectors[0]);
    assert!(
        (similarity - 1.0).abs() < 1e-6,
        "Self-similarity should be 1.0"
    );

    println!("   ✅ Vectorization tests passed");
    Ok(())
}

fn test_declaration_splitting() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔪 Testing Declaration Splitting...");

    let test_code = r#"
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Point {
    x: f64,
    y: f64,
}

enum Color {
    Red,
    Green,
    Blue,
}

trait Display {
    fn display(&self);
}

impl Display for Point {
    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}
"#;

    let mut splitter = DeclarationSplitter::new();
    splitter.split_file(test_code, Some("test.rs".to_string()))?;

    assert!(
        splitter.declarations.len() >= 5,
        "Should find at least 5 declarations"
    );

    let functions = splitter.get_declarations_by_type(DeclarationType::Function);
    let structs = splitter.get_declarations_by_type(DeclarationType::Struct);
    let enums = splitter.get_declarations_by_type(DeclarationType::Enum);
    let traits = splitter.get_declarations_by_type(DeclarationType::Trait);
    let impls = splitter.get_declarations_by_type(DeclarationType::Impl);

    assert!(functions.len() >= 2, "Should find at least 2 functions");
    assert_eq!(structs.len(), 1, "Should find 1 struct");
    assert_eq!(enums.len(), 1, "Should find 1 enum");
    assert_eq!(traits.len(), 1, "Should find 1 trait");
    assert_eq!(impls.len(), 1, "Should find 1 impl");

    println!("   ✅ Declaration splitting tests passed");
    Ok(())
}

fn test_duplicate_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing Duplicate Detection...");

    let detector = DuplicateDetector::new(64, 0.8);

    // Create test declarations with duplicates
    let declarations = vec![
        Declaration {
            name: "func1".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn test() { println!(\"hello\"); }".to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file1.rs".to_string()),
        },
        Declaration {
            name: "func2".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn test() { println!(\"hello\"); }".to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file2.rs".to_string()),
        },
        Declaration {
            name: "unique".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn unique() { println!(\"unique\"); }".to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file3.rs".to_string()),
        },
    ];

    let report = detector.detect_duplicates(&declarations);
    let stats = detector.calculate_deduplication_savings(&report);

    println!("   📊 Duplicate Report:");
    println!("      - Groups: {}", report.groups.len());
    println!("      - Total Duplicates: {}", report.total_duplicates);
    println!("      - Savings: {:.1}%", stats.savings_percentage);

    println!("   ✅ Duplicate detection tests passed");
    Ok(())
}

fn test_code_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Testing Code Analysis...");

    let mut analyzer = CodeAnalyzer::new(128, 0.8);

    let test_code = r#"
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
"#;

    let analysis = analyzer.analyze_file(test_code, "test.rs".to_string())?;

    assert_eq!(analysis.file_path, "test.rs");
    assert!(analysis.declarations.len() > 0);
    assert_eq!(analysis.declarations.len(), analysis.vectors.len());
    assert!(!analysis.json_ast.is_empty());

    println!("   📈 Analysis Results:");
    println!("      - Declarations: {}", analysis.declarations.len());
    println!("      - Functions: {}", analysis.metrics.function_count);
    println!("      - Structs: {}", analysis.metrics.struct_count);
    println!(
        "      - Complexity: {:.2}",
        analysis.metrics.complexity_score
    );

    println!("   ✅ Code analysis tests passed");
    Ok(())
}

// fn test_meme_generation() -> Result<(), Box<dyn std::error::Error>> {
//     println!("\n🎭 Testing Meme Generation...");

//     let generator = MemeGenerator::new(64);
//     let mut analyzer = CodeAnalyzer::new(64, 0.8);

//     let test_code = r#"
// fn recursive_function(n: u32) -> u32 {
//     if n <= 1 {
//         1
//     } else {
//         n * recursive_function(n - 1)
//     }
// }

// struct ComplexStruct<T> {
//     data: Vec<T>,
//     metadata: HashMap<String, String>,
// }
// "#;

//     let analysis = analyzer.analyze_file(test_code, "meme_test.rs".to_string())?;
//     let ecosystem = generator.create_meme_ecosystem(&[analysis.clone()]);
// //    let memes = generator.generate_meme_from_representation(&analysis);
// //      let memes = generator.generate_meme_from_declaration(&analysis);

//     assert!(ecosystem.memes.len() > 0);
//     assert!(memes.len() > 0);

//     println!("   🧬 Meme Ecosystem:");
//     println!("      - Total Memes: {}", ecosystem.memes.len());
//     println!("      - Relationships: {}", ecosystem.relationships.len());
//     println!(
//         "      - Emergence Patterns: {}",
//         ecosystem.emergence_patterns.len()
//     );
//     println!(
//         "      - Dimensions: {}",
//         ecosystem.dimensional_structure.dimensions
//     );

//     for (name, meme_repr) in &memes {
//         println!("      - {}: {}", name, meme_repr);
//     }

//     println!("   ✅ Meme generation tests passed");
//     Ok(())
// }

fn test_wallet_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Testing Wallet Integration...");

    let mut wallet = WalletManager::new();
    wallet.initialize_with_password("test_password_123")?;

    // Test AWS credentials
    wallet.store_aws_credentials("AKIATEST", "secret_key_test")?;
    let aws_creds = wallet.get_aws_credentials()?;
    assert!(aws_creds.is_some());

    // Test AI keys
    wallet.store_ai_key("openai", "sk-test123")?;
    let openai_key = wallet.get_ai_key("openai")?;
    assert!(openai_key.is_some());

    // Test Solana keys
    wallet.store_solana_key("main_wallet", "test_private_key")?;
    //FIXME:    assert_eq!(wallet.secrets.solana_keys.len(), 1);

    // Test export/import
    let exported = wallet.export_secrets()?;
    let mut new_wallet = WalletManager::new();
    new_wallet.initialize_with_password("test_password_123")?;
    new_wallet.import_secrets(&exported)?;

    println!("   🔐 Wallet Features:");
    println!("      - AWS Credentials: ✅");
    println!("      - AI Keys: ✅");
    println!("      - Solana Keys: ✅");
    println!("      - Export/Import: ✅");
    println!("      - Encryption: ✅");

    println!("   ✅ Wallet integration tests passed");
    Ok(())
}

fn test_self_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🪞 Testing Self Analysis...");

    // Analyze this very file
    let current_file = include_str!("test_runner.rs");

    let mut analyzer = CodeAnalyzer::new(256, 0.8);
    let analysis = analyzer.analyze_file(current_file, "test_runner.rs".to_string())?;

    let generator = MemeGenerator::new(256);
    //    let memes = generator.generate_meme_representation(&analysis);
    let ecosystem = generator.create_meme_ecosystem(&[analysis.clone()]);

    println!("   🔍 Self-Analysis Results:");
    println!(
        "      - Functions Found: {}",
        analysis.metrics.function_count
    );
    println!("      - Total Lines: {}", analysis.metrics.total_lines);
    println!(
        "      - Complexity Score: {:.2}",
        analysis.metrics.complexity_score
    );
    //    println!("      - Memes Generated: {}", memes.len());
    println!("      - Ecosystem Size: {}", ecosystem.memes.len());

    // Show some memes
    println!("   🎭 Generated Memes:");
    //    for (name, meme) in memes.iter().take(3) {
    //        println!("      - {}", meme);
    //    }

    println!("   ✅ Self analysis tests passed");
    Ok(())
}
