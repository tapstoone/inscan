-- blocks

-- events
create table public.inscan_events (
    height integer,
    blocktime integer,
    txhash VARCHAR(255),
    txindex integer,
    protocol VARCHAR(255),
    payload JSONB
);
CREATE INDEX inscan_events_height_idx ON public.inscan_events USING btree (height);
CREATE INDEX inscan_events_blocktime_idx ON public.inscan_events USING btree (blocktime);
CREATE INDEX inscan_events_protocol_idx ON public.inscan_events USING btree (protocol);
CREATE INDEX inscan_events_payload_idx ON public.inscan_events USING GIN(payload);
