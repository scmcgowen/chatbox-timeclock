
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE players (
    id serial NOT NULL,
    name citext NOT NULL,
    uuid uuid NOT NULL,
    admin boolean DEFAULT false NOT NULL
);


CREATE TABLE public.timecards (
    id serial NOT NULL,
    user_id uuid NOT NULL,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone,
    description text
);
