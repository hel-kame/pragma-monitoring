-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.future_entry
(
    network character varying(255) COLLATE pg_catalog."default",
    pair_id character varying(255) COLLATE pg_catalog."default",
    data_id character varying(255) COLLATE pg_catalog."default",
    block_hash character varying(255) COLLATE pg_catalog."default",
    block_number bigint,
    block_timestamp timestamp without time zone,
    transaction_hash character varying(255) COLLATE pg_catalog."default",
    price numeric,
    "timestamp" timestamp without time zone,
    publisher character varying(255) COLLATE pg_catalog."default",
    source character varying(255) COLLATE pg_catalog."default",
    volume numeric,
    expiration_timestamp timestamp without time zone,
    _cursor bigint
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.future_entry
    OWNER to postgres;

REVOKE ALL ON TABLE public.future_entry FROM indexed_data_read_only;

GRANT SELECT ON TABLE public.future_entry TO indexed_data_read_only;

GRANT ALL ON TABLE public.future_entry TO postgres;
-- Index: future_entry_pair_id_publisher_source_timestamp_index

-- DROP INDEX IF EXISTS public.future_entry_pair_id_publisher_source_timestamp_index;

CREATE INDEX IF NOT EXISTS future_entry_pair_id_publisher_source_timestamp_index
    ON public.future_entry USING btree
    (pair_id COLLATE pg_catalog."default" ASC NULLS LAST, publisher COLLATE pg_catalog."default" ASC NULLS LAST, source COLLATE pg_catalog."default" ASC NULLS LAST, "timestamp" ASC NULLS LAST)
    TABLESPACE pg_default;