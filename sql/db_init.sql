-- blocks

-- events
create table public.inscan_events (
    height integer,
    blocktime timestamp,
    txhash VARCHAR(52),
    txindex integer,
    protocol integer,
    payload JSONB
);
CREATE INDEX inscan_events_height_idx ON public.inscan_events USING btree (height);
CREATE INDEX inscan_events_blocktime_idx ON public.inscan_events USING btree (blocktime);
CREATE INDEX inscan_events_protocol_idx ON public.inscan_events USING btree (protocol);
CREATE INDEX inscan_events_payload_idx ON public.inscan_events USING GIN(payload);
