SELECT 
    fk.name AS fk_name,
    OBJECT_SCHEMA_NAME(fk.parent_object_id) AS schema_origem,
    OBJECT_NAME(fk.parent_object_id)        AS tabela_origem,
    COL_NAME(fkc.parent_object_id, fkc.parent_column_id) AS coluna_origem,
    OBJECT_SCHEMA_NAME(fk.referenced_object_id) AS schema_destino,
    OBJECT_NAME(fk.referenced_object_id)        AS tabela_destino,
    COL_NAME(fkc.referenced_object_id, fkc.referenced_column_id) AS coluna_destino
FROM sys.foreign_keys fk
JOIN sys.foreign_key_columns fkc 
    ON fk.object_id = fkc.constraint_object_id;