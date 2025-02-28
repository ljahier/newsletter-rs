create table if not exists contacts (
  id text primary key,
  first_name text,
  last_name text,
  address text,
  postal_code text,
  city text,
  email text not null unique,
  unsubscribe_token text,
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
create table if not exists themes (
  id text primary key,
  name text not null,
  header text,
  footer text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);
create table if not exists users (
  id text primary key,
  email text not null unique,
  password text not null,
  role text check (role in ('admin', 'user')) default 'user',
  auth_info text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
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
  status text check (status in ('pending', 'sent', 'failed', 'draft')),
  content_html text,
  content_plain text,
  theme_id text,
  sent_at timestamp with time zone,
  sent_by text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  foreign key (theme_id) references themes (id),
  foreign key (sent_by) references users (id)
);
create table if not exists sending_contact_lists (
  sending_id text,
  contact_list_id text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp,
  primary key (sending_id, contact_list_id),
  foreign key (sending_id) references sendings (id) on delete cascade,
  foreign key (contact_list_id) references contact_lists (id) on delete cascade
);