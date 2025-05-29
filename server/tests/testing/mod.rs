pub mod labman {
    use labman_server::core::Labman;
    const IN_MEMORY_DATABASE_URL: &str = ":memory:";

    pub fn in_memory() -> Labman {
        Labman::new(IN_MEMORY_DATABASE_URL).expect("Failed to create Labman")
    }
}
