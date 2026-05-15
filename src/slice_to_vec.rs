#[macro_export]
macro_rules! slice_to_vec_recursive {
    // Base case: depth 0 or simple slice
    ($slice:expr) => {
        $slice.to_vec()
    };

    // Recursive: keep converting until no more nesting
    ($slice:expr; nested) => {
        $slice
            .iter()
            .map(|inner| slice_to_vec_recursive!(inner))
            .collect::<Vec<_>>()
    };
}
