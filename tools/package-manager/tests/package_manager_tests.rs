use techscript_package_manager::{DependencyResolver, Package};

#[test]
fn test_package_resolver() {
    let resolver = DependencyResolver::new("https://registry.techscript.org");
    let package = Package {
        name: "test-lib".to_string(),
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    let chain = resolver.resolve(&package).expect("resolve should succeed");
    assert_eq!(chain.len(), 0);
}
