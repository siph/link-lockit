create table links
(
    links_id      uuid primary key     default uuid_generate_v1mc(),
    original_link text unique not null,
    short_link    text unique not null,
    description   text        not null default '',
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);

select trigger_updated_at('links');
