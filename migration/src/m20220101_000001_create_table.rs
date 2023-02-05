use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alpha::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alpha::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bravo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bravo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Charlie::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Charlie::AlphaId).integer().not_null())
                    .col(ColumnDef::new(Charlie::BravoId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alpha_id-charlie")
                            .from(Charlie::Table, Charlie::AlphaId)
                            .to(Alpha::Table, Alpha::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bravo_id-charlie")
                            .from(Charlie::Table, Charlie::AlphaId)
                            .to(Bravo::Table, Bravo::Id),
                    )
                    .primary_key(Index::create().col(Charlie::AlphaId).col(Charlie::BravoId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Delta::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Delta::AlphaId).integer().not_null())
                    .col(ColumnDef::new(Delta::BravoId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alpha_id-delta")
                            .from(Delta::Table, Delta::AlphaId)
                            .to(Alpha::Table, Alpha::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bravo_id-delta")
                            .from(Delta::Table, Delta::AlphaId)
                            .to(Bravo::Table, Bravo::Id),
                    )
                    .primary_key(Index::create().col(Delta::AlphaId).col(Delta::BravoId))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Delta::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Charlie::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bravo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Alpha::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Alpha {
    Table,
    Id,
}

#[derive(Iden)]
enum Bravo {
    Table,
    Id,
}

#[derive(Iden)]
enum Charlie {
    Table,
    AlphaId,
    BravoId,
}

#[derive(Iden)]
enum Delta {
    Table,
    AlphaId,
    BravoId,
}
