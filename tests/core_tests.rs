use solfunmeme_dioxus::core::*;

#[test]
fn test_vectorization() {
    let vectorizer = CodeVectorizer::new(64);
    let vector = vectorizer.vectorize("fn test() {}");
    assert_eq!(vector.dimensions.len(), 64);

    let sum: f32 = vector.dimensions.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
}

#[test]
fn test_declaration_splitting() {
    let mut splitter = DeclarationSplitter::new();
    let code = "fn main() {} struct Point { x: f64 }";

    let result = splitter.split_file(code, Some("test.rs".to_string()));
    assert!(result.is_ok());
    assert!(splitter.declarations.len() >= 2);
}

#[test]
fn test_duplicate_detection() {
    let detector = DuplicateDetector::new(64, 0.8);
    let declarations = vec![
        Declaration {
            name: "test1".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn test() {}".to_string(),
            line_start: 1,
            line_end: 1,
            file_path: None,
        },
        Declaration {
            name: "test2".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn test() {}".to_string(),
            line_start: 1,
            line_end: 1,
            file_path: None,
        },
    ];

    let report = detector.detect_duplicates(&declarations);
    assert!(report.total_duplicates > 0 || report.canonical_count > 0);
}

#[test]
fn test_code_analysis() {
    let mut analyzer = CodeAnalyzer::new(64, 0.8);
    let code = "fn hello() { println!(\"world\"); }";

    let result = analyzer.analyze_file(code, "test.rs".to_string());
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.file_path, "test.rs");
    assert!(analysis.declarations.len() > 0);
}

#[test]
fn test_meme_generation() {
    let generator = MemeGenerator::new(64);
    let declaration = Declaration {
        name: "test_fn".to_string(),
        declaration_type: DeclarationType::Function,
        content: "fn test_fn() {}".to_string(),
        line_start: 1,
        line_end: 1,
        file_path: Some("test.rs".to_string()),
    };
    let vector = CodeVector::new(vec![0.1; 64]);

    let meme = generator.generate_meme_from_declaration(&declaration, &vector);
    assert_eq!(meme.name, "test_fn");
    assert_eq!(meme.emoji, "🔧");
}

#[test]
fn test_wallet_integration() {
    let mut wallet = WalletManager::new();
    assert!(wallet.initialize_with_password("test123").is_ok());

    assert!(wallet.store_ai_key("openai", "sk-test").is_ok());
    let key = wallet.get_ai_key("openai").unwrap();
    assert_eq!(key, Some("sk-test".to_string()));
}
