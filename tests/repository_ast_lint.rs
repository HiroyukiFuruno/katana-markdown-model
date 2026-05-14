#[test]
fn repository_ast_lint() {
    katana_ast_lint::KatanaAstLint::from_workspace().assert_clean();
}
