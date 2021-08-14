--
-- PostgreSQL database dump
--

-- Dumped from database version 11.4
-- Dumped by pg_dump version 11.4

-- Started on 2021-08-06 21:50:33 EDT

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

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 197 (class 1259 OID 16412)
-- Name: exercises; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.exercises (
    id bigint NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.exercises OWNER TO postgres;

--
-- TOC entry 198 (class 1259 OID 16418)
-- Name: exercises_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.exercises_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.exercises_id_seq OWNER TO postgres;

--
-- TOC entry 3163 (class 0 OID 0)
-- Dependencies: 198
-- Name: exercises_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.exercises_id_seq OWNED BY public.exercises.id;


--
-- TOC entry 200 (class 1259 OID 16451)
-- Name: muscle_group_exercise; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.muscle_group_exercise (
    muscle_group_id bigint NOT NULL,
    exercise_id bigint NOT NULL,
    worked boolean NOT NULL
);


ALTER TABLE public.muscle_group_exercise OWNER TO postgres;

--
-- TOC entry 202 (class 1259 OID 16460)
-- Name: muscle_group_exercise_exercise_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.muscle_group_exercise_exercise_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.muscle_group_exercise_exercise_id_seq OWNER TO postgres;

--
-- TOC entry 3164 (class 0 OID 0)
-- Dependencies: 202
-- Name: muscle_group_exercise_exercise_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.muscle_group_exercise_exercise_id_seq OWNED BY public.muscle_group_exercise.exercise_id;


--
-- TOC entry 201 (class 1259 OID 16454)
-- Name: muscle_group_exercise_muscle_group_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.muscle_group_exercise_muscle_group_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.muscle_group_exercise_muscle_group_id_seq OWNER TO postgres;

--
-- TOC entry 3165 (class 0 OID 0)
-- Dependencies: 201
-- Name: muscle_group_exercise_muscle_group_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.muscle_group_exercise_muscle_group_id_seq OWNED BY public.muscle_group_exercise.muscle_group_id;


--
-- TOC entry 196 (class 1259 OID 16409)
-- Name: muscle_groups; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.muscle_groups (
    id bigint NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.muscle_groups OWNER TO postgres;

--
-- TOC entry 199 (class 1259 OID 16440)
-- Name: muscle_groups_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.muscle_groups_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.muscle_groups_id_seq OWNER TO postgres;

--
-- TOC entry 3166 (class 0 OID 0)
-- Dependencies: 199
-- Name: muscle_groups_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.muscle_groups_id_seq OWNED BY public.muscle_groups.id;


--
-- TOC entry 3019 (class 2604 OID 16420)
-- Name: exercises id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.exercises ALTER COLUMN id SET DEFAULT nextval('public.exercises_id_seq'::regclass);


--
-- TOC entry 3020 (class 2604 OID 16456)
-- Name: muscle_group_exercise muscle_group_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_group_exercise ALTER COLUMN muscle_group_id SET DEFAULT nextval('public.muscle_group_exercise_muscle_group_id_seq'::regclass);


--
-- TOC entry 3021 (class 2604 OID 16462)
-- Name: muscle_group_exercise exercise_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_group_exercise ALTER COLUMN exercise_id SET DEFAULT nextval('public.muscle_group_exercise_exercise_id_seq'::regclass);


--
-- TOC entry 3018 (class 2604 OID 16442)
-- Name: muscle_groups id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_groups ALTER COLUMN id SET DEFAULT nextval('public.muscle_groups_id_seq'::regclass);


--
-- TOC entry 3152 (class 0 OID 16412)
-- Dependencies: 197
-- Data for Name: exercises; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.exercises (id, name) FROM stdin;
1	Squat
2	Leg Press
3	Lunge
4	Deadlift
5	Leg Extension
6	Leg Curl
7	Standing Calf Raise
8	Seating Calf Raise
9	Hip Adductor
10	Bench Press
11	Chest Fly
12	Push-up
13	Pull-down
14	Pull-up
15	Bent-over Row
16	Upright Row
17	Shoulder Press
18	Lateral Raise
19	Shoulder Shrug
20	Pushdown
21	Triceps Extension
22	Biceps Curl
23	Crunch
24	Russian Twist
25	Leg Raise
26	Back Extension
\.


--
-- TOC entry 3155 (class 0 OID 16451)
-- Dependencies: 200
-- Data for Name: muscle_group_exercise; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.muscle_group_exercise (muscle_group_id, exercise_id, worked) FROM stdin;
1	1	t
1	2	t
1	3	f
1	4	t
1	5	f
1	6	t
1	7	t
1	8	t
1	26	t
2	1	t
2	2	t
2	3	t
2	4	t
2	5	t
3	1	t
3	2	t
3	3	t
3	4	t
3	6	t
3	26	t
4	1	t
4	2	t
4	3	t
4	4	t
4	26	t
5	1	t
5	3	t
5	4	t
5	9	t
5	25	t
6	1	t
6	4	t
6	26	t
7	13	t
7	14	t
7	15	t
8	4	t
8	13	t
8	14	t
8	15	t
8	16	t
8	17	t
8	18	t
8	19	t
9	1	t
9	4	t
9	23	t
9	24	t
9	25	t
10	10	t
10	11	t
10	12	t
11	10	t
11	11	t
11	12	t
11	16	t
11	17	t
11	18	t
12	10	t
12	12	t
12	17	t
12	20	t
12	21	t
13	13	t
13	14	t
13	15	t
13	16	t
13	22	t
14	4	t
14	14	t
14	19	t
14	21	t
14	22	t
\.


--
-- TOC entry 3151 (class 0 OID 16409)
-- Dependencies: 196
-- Data for Name: muscle_groups; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.muscle_groups (id, name) FROM stdin;
1	Calves
2	Quadriceps
3	Hamstrings
4	Gluteus
5	Hips Other
6	Lower Back
7	Lats
8	Trapezius
9	Abdominals
10	Pectorals
11	Deltoids
12	Triceps
13	Biceps
14	Forearms
\.


--
-- TOC entry 3167 (class 0 OID 0)
-- Dependencies: 198
-- Name: exercises_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.exercises_id_seq', 1, false);


--
-- TOC entry 3168 (class 0 OID 0)
-- Dependencies: 202
-- Name: muscle_group_exercise_exercise_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.muscle_group_exercise_exercise_id_seq', 1, false);


--
-- TOC entry 3169 (class 0 OID 0)
-- Dependencies: 201
-- Name: muscle_group_exercise_muscle_group_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.muscle_group_exercise_muscle_group_id_seq', 1, false);


--
-- TOC entry 3170 (class 0 OID 0)
-- Dependencies: 199
-- Name: muscle_groups_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.muscle_groups_id_seq', 1, false);


--
-- TOC entry 3025 (class 2606 OID 16428)
-- Name: exercises exercises_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.exercises
    ADD CONSTRAINT exercises_pkey PRIMARY KEY (id);


--
-- TOC entry 3027 (class 2606 OID 16467)
-- Name: muscle_group_exercise muscle_group_exercise_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_group_exercise
    ADD CONSTRAINT muscle_group_exercise_pkey PRIMARY KEY (muscle_group_id, exercise_id);


--
-- TOC entry 3023 (class 2606 OID 16450)
-- Name: muscle_groups muscle_groups_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_groups
    ADD CONSTRAINT muscle_groups_pkey PRIMARY KEY (id);


--
-- TOC entry 3029 (class 2606 OID 16473)
-- Name: muscle_group_exercise muscle_group_exercise_exercise_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_group_exercise
    ADD CONSTRAINT muscle_group_exercise_exercise_id_fkey FOREIGN KEY (exercise_id) REFERENCES public.exercises(id);


--
-- TOC entry 3028 (class 2606 OID 16468)
-- Name: muscle_group_exercise muscle_group_exercise_muscle_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.muscle_group_exercise
    ADD CONSTRAINT muscle_group_exercise_muscle_group_id_fkey FOREIGN KEY (muscle_group_id) REFERENCES public.muscle_groups(id);


-- Completed on 2021-08-06 21:50:33 EDT

--
-- PostgreSQL database dump complete
--

