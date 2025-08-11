pub mod data {
    use native_db::{db_type, Builder, Database, Models};
    use once_cell::sync::Lazy;
    use crate::seed::seed;


    pub static MODELS: Lazy<Models> = Lazy::new(|| {
        let mut models = Models::new();

        models.define::<seed::v1::Seed>().unwrap();
        models
    });

    pub fn init() -> Result<(), db_type::Error> {
        let db = Builder::new().create(&MODELS, "gardenwatch.data")?;
        Ok(())
    }

    pub fn open() -> Result<Database<'static>, db_type::Error> {
        let builder: Builder = Builder::new();
        let db = builder.open(&MODELS, "gardenwatch.data")?;

        Ok(db)
    }
}