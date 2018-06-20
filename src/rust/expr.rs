#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_expr {
    // Entry point. Dup tokens.
    {
        $caller:tt
        input = [{ $($input:tt)* }]
    } => {
        private_parse_expr! {
            $caller
            tokens = [{ $($input)* }]
            _tokens = [{ $($input)* }]
        }
    };

    // TODO: our own expression parser for better error recovery
    // TODO: support non-empty `rest`
    {
        $caller:tt
        tokens = [{ $e:expr }]
        _tokens = [{ $($tokens:tt)* }]
    } => {
        tt_return! {
            $caller
            expr = [{ $($tokens)* }]
            rest = [{ }]
        }
    };

    // Unexpected: input does not match $:expr.
    {
        $caller:tt
        tokens = [{ $($unexpected:tt)+ }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        error_unexpected! {
            $($unexpected)*
        }
    };
}
