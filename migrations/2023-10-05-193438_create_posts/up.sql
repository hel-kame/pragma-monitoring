-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.storage
(
    id uuid NOT NULL,
    network character varying COLLATE pg_catalog."default" NOT NULL,
    data_type character varying COLLATE pg_catalog."default" NOT NULL,
    block_hash character varying COLLATE pg_catalog."default" NOT NULL,
    block_number numeric NOT NULL,
    block_timestamp timestamp without time zone NOT NULL,
    transaction_hash character varying COLLATE pg_catalog."default" NOT NULL,
    source character varying COLLATE pg_catalog."default",
    price real,
    CONSTRAINT storage_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.storage
    OWNER to postgres;