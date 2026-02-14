--
-- PostgreSQL database dump
--

\restrict msu4ebbEpvzmw0vS4EESPw71KFRfJiC6D7XrDuiD02QFJBo97acwboYP23RIfLC

-- Dumped from database version 16.11
-- Dumped by pg_dump version 16.11

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: citext; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;


--
-- Name: EXTENSION citext; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: players; Type: TABLE; Schema: public; Owner: herrkatze
--

CREATE TABLE public.players (
    id integer NOT NULL,
    name public.citext NOT NULL,
    uuid uuid NOT NULL,
    admin boolean DEFAULT false NOT NULL
);


ALTER TABLE public.players OWNER TO herrkatze;

--
-- Name: players_id_seq; Type: SEQUENCE; Schema: public; Owner: herrkatze
--

CREATE SEQUENCE public.players_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.players_id_seq OWNER TO herrkatze;

--
-- Name: players_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: herrkatze
--

ALTER SEQUENCE public.players_id_seq OWNED BY public.players.id;


--
-- Name: timecards; Type: TABLE; Schema: public; Owner: herrkatze
--

CREATE TABLE public.timecards (
    id integer NOT NULL,
    user_id uuid NOT NULL,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone,
    description text
);


ALTER TABLE public.timecards OWNER TO herrkatze;

--
-- Name: timecards_id_seq; Type: SEQUENCE; Schema: public; Owner: herrkatze
--

CREATE SEQUENCE public.timecards_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.timecards_id_seq OWNER TO herrkatze;

--
-- Name: timecards_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: herrkatze
--

ALTER SEQUENCE public.timecards_id_seq OWNED BY public.timecards.id;


--
-- Name: players id; Type: DEFAULT; Schema: public; Owner: herrkatze
--

ALTER TABLE ONLY public.players ALTER COLUMN id SET DEFAULT nextval('public.players_id_seq'::regclass);


--
-- Name: timecards id; Type: DEFAULT; Schema: public; Owner: herrkatze
--

ALTER TABLE ONLY public.timecards ALTER COLUMN id SET DEFAULT nextval('public.timecards_id_seq'::regclass);


--
-- Data for Name: players; Type: TABLE DATA; Schema: public; Owner: herrkatze
--

COPY public.players (id, name, uuid, admin) FROM stdin;
1	HerrKatzeGaming	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	t
5	GPLv3	d5100e46-f630-4c48-a062-b5b75569a576	t
4	minecartchris	1de578d0-4eae-48db-abc9-7bf3354f809b	f
6	SethGamer1223	3aa9db3e-ffdb-46e3-85b2-5740e4e9a46e	f
7	HerrKatzeAlt	78aaf36a-f37b-4f5a-9b79-24ebad9253d6	t
\.


--
-- Data for Name: timecards; Type: TABLE DATA; Schema: public; Owner: herrkatze
--

COPY public.timecards (id, user_id, start_time, end_time, description) FROM stdin;
1	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 11:54:43.665933-08	2026-01-24 11:55:07.31421-08	\N
2	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 14:56:26.626772-08	2026-01-24 14:56:43.021316-08	\N
3	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 14:57:39.441146-08	2026-01-24 14:57:46.662637-08	\N
4	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 15:03:15.450828-08	2026-01-24 15:03:23.917623-08	\N
5	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 15:09:37.633512-08	2026-01-24 15:15:33.939179-08	\N
6	1f558cbb-0752-49c0-ace4-7f9ed0506fe3	2026-01-24 18:01:29.342358-08	2026-01-24 18:02:02.844756-08	\N
7	78aaf36a-f37b-4f5a-9b79-24ebad9253d6	2026-01-24 23:34:02.070855-08	2026-01-24 23:34:11.804221-08	\N
8	78aaf36a-f37b-4f5a-9b79-24ebad9253d6	2026-01-24 23:34:43.783303-08	2026-01-24 23:34:50.859676-08	\N
9	78aaf36a-f37b-4f5a-9b79-24ebad9253d6	2026-01-24 23:35:00.62712-08	2026-01-24 23:35:10.410953-08	\N
10	78aaf36a-f37b-4f5a-9b79-24ebad9253d6	2026-01-24 23:44:11.681853-08	2026-01-24 23:44:19.650777-08	db test
\.


--
-- Name: players_id_seq; Type: SEQUENCE SET; Schema: public; Owner: herrkatze
--

SELECT pg_catalog.setval('public.players_id_seq', 7, true);


--
-- Name: timecards_id_seq; Type: SEQUENCE SET; Schema: public; Owner: herrkatze
--

SELECT pg_catalog.setval('public.timecards_id_seq', 10, true);


--
-- Name: players players_pkey; Type: CONSTRAINT; Schema: public; Owner: herrkatze
--

ALTER TABLE ONLY public.players
    ADD CONSTRAINT players_pkey PRIMARY KEY (id);


--
-- Name: timecards timecards_pkey; Type: CONSTRAINT; Schema: public; Owner: herrkatze
--

ALTER TABLE ONLY public.timecards
    ADD CONSTRAINT timecards_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict msu4ebbEpvzmw0vS4EESPw71KFRfJiC6D7XrDuiD02QFJBo97acwboYP23RIfLC

