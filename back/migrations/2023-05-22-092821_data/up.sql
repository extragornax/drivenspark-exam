CREATE TABLE data (
  id BIGSERIAL PRIMARY KEY NOT NULL,
	title varchar NOT NULL,
	description varchar NOT NULL,
	"date" date NOT NULL,
	priority varchar NOT NULL,
	duration integer NOT NULL,
	status varchar NOT NULL
);