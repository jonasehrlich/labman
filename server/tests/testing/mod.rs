pub mod labman {
    use labman_server::core::Labman;
    const IN_MEMORY_DATABASE_URL: &str = ":memory:";

    pub fn in_memory() -> Labman {
        let mut lm = Labman::new(IN_MEMORY_DATABASE_URL).expect("Failed to create Labman");
        lm.run_migrations().expect("Failed to run migrations");
        lm
    }
}
