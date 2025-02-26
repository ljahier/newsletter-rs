create table if not exists contacts (
  id text primary key,
  first_name text,
  last_name text,
  address text,
  postal_code text,
  city text,
  email text not null unique,
  custom_fields text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists contact_lists (
  id text primary key,
  name text not null,
  type text check (type in ('automatic', 'manual')),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists contact_list_members (
  contact_id text,
  list_id text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  primary key (contact_id, list_id),
  foreign key (contact_id) references contacts (id) on delete cascade,
  foreign key (list_id) references contact_lists (id) on delete cascade
);
create table if not exists sendings (
  id text primary key,
  type text check (
    type in (
      'newsletter',
      'general_communication',
      'targeted_announcement'
    )
  ),
  name text not null,
  send_date timestamp with time zone,
  status text check (status in ('pending', 'sent', 'failed')),
  content text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists themes (
  id text primary key,
  name text not null,
  header text,
  footer text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists sending_history (
  id text primary key,
  sending_id text,
  sending_date timestamp with time zone,
  status text check (status in ('sent', 'failed')),
  sending_type text check (
    sending_type in (
      'newsletter',
      'general_communication',
      'targeted_announcement'
    )
  ),
  contacts text,
  content text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  foreign key (sending_id) references sendings (id) on delete cascade
);
create table if not exists unsubscriptions (
  id text primary key,
  email text not null,
  request_date timestamp with time zone,
  reason text,
  status text check (status in ('pending', 'processed')),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists gdpr_compliance (
  id text primary key,
  sending_id text,
  last_processing_date timestamp with time zone,
  legal_mentions text,
  unsubscription_link text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  foreign key (sending_id) references sendings (id) on delete cascade
);
create table if not exists csv_imports (
  id text primary key,
  file_name text not null,
  uploaded_at timestamp with time zone,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists users (
  id text primary key,
  email text not null unique,
  password text not null,
  role text check (role in ('admin', 'user')) default 'user',
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);